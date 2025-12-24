use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tree_sitter::{Node, Parser};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeadCodeItem {
    pub file: String,
    pub line: usize,
    pub name: String,
    pub code_type: String,
    pub confidence: u8,
    pub size: usize,
}

#[derive(Debug, Clone)]
struct Definition {
    file: String,
    line: usize,
    size: usize,
    code_type: String,
    is_method: bool,
    is_decorated: bool,
    is_magic: bool,
    is_test: bool,
    is_private: bool,
    parent_class: Option<String>,
}

pub struct DeadCodeAnalyzer {
    min_confidence: u8,
    exclude_patterns: Vec<String>,
    definitions: HashMap<String, Vec<Definition>>,
    function_calls: HashSet<String>,
    exports: HashSet<String>,
}

impl DeadCodeAnalyzer {
    pub fn new(min_confidence: u8, exclude_patterns: Vec<&str>) -> Self {
        Self {
            min_confidence,
            exclude_patterns: exclude_patterns.iter().map(|s| s.to_string()).collect(),
            definitions: HashMap::new(),
            function_calls: HashSet::new(),
            exports: HashSet::new(),
        }
    }

    pub fn analyze_path(&mut self, path: &PathBuf) -> Result<()> {
        if path.is_file() {
            if !self.should_exclude(path) {
                self.analyze_file(path)?;
            }
        } else if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "py"))
            {
                let entry_path = entry.path().to_path_buf();
                if !self.should_exclude(&entry_path) {
                    self.analyze_file(&entry_path)?;
                }
            }
        }
        Ok(())
    }

    fn should_exclude(&self, path: &PathBuf) -> bool {
        let path_str = path.to_string_lossy();
        for pattern in &self.exclude_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
        false
    }

    fn analyze_file(&mut self, file_path: &PathBuf) -> Result<()> {
        let content = std::fs::read_to_string(file_path)?;
        let mut parser = Parser::new();
        let python_language = tree_sitter_python::language();
        parser.set_language(&python_language)?;

        let tree = parser.parse(&content, None).ok_or_else(|| {
            anyhow::anyhow!("Failed to parse Python file")
        })?;

        let root = tree.root_node();

        // First pass: extract definitions
        self.extract_definitions(&root, &content, file_path, None);
        
        // Second pass: extract usage
        self.extract_usage(&root, &content);
        
        // Third pass: check for __all__ exports
        self.extract_exports(&root, &content);

        Ok(())
    }

    fn extract_definitions(
        &mut self,
        node: &Node,
        content: &str,
        file_path: &PathBuf,
        parent_class: Option<String>,
    ) {
        let kind = node.kind();

        if kind == "function_definition" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = name_node.utf8_text(content.as_bytes()).unwrap_or("");
                let line = node.start_position().row + 1;
                let size = node.end_byte() - node.start_byte();
                
                let is_method = parent_class.is_some();
                let is_magic = name.starts_with("__") && name.ends_with("__");
                let is_private = name.starts_with('_') && !is_magic;
                let is_test = name.starts_with("test_") || name == "setUp" || name == "tearDown";
                let is_decorated = self.is_decorated(node);

                let definition = Definition {
                    file: file_path.to_string_lossy().to_string(),
                    line,
                    size,
                    code_type: if is_method { "method".to_string() } else { "function".to_string() },
                    is_method,
                    is_decorated,
                    is_magic,
                    is_test,
                    is_private,
                    parent_class: parent_class.clone(),
                };

                self.definitions
                    .entry(name.to_string())
                    .or_insert_with(Vec::new)
                    .push(definition);
            }
        } else if kind == "class_definition" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = name_node.utf8_text(content.as_bytes()).unwrap_or("");
                let line = node.start_position().row + 1;
                let size = node.end_byte() - node.start_byte();
                
                let is_private = name.starts_with('_');
                let is_decorated = self.is_decorated(node);

                let definition = Definition {
                    file: file_path.to_string_lossy().to_string(),
                    line,
                    size,
                    code_type: "class".to_string(),
                    is_method: false,
                    is_decorated,
                    is_magic: false,
                    is_test: name.starts_with("Test"),
                    is_private,
                    parent_class: None,
                };

                self.definitions
                    .entry(name.to_string())
                    .or_insert_with(Vec::new)
                    .push(definition);

                // Recursively process methods within this class
                if let Some(body) = node.child_by_field_name("body") {
                    self.extract_definitions(&body, content, file_path, Some(name.to_string()));
                }
                return; // Don't process children again below
            }
        }

        // Recursively process children
        for child in node.children(&mut node.walk()) {
            self.extract_definitions(&child, content, file_path, parent_class.clone());
        }
    }

    fn is_decorated(&self, node: &Node) -> bool {
        if let Some(parent) = node.parent() {
            parent.kind() == "decorated_definition"
        } else {
            false
        }
    }

    fn extract_usage(&mut self, node: &Node, content: &str) {
        let kind = node.kind();

        // Detect function calls (identifier followed by argument_list)
        if kind == "call" {
            if let Some(func_node) = node.child_by_field_name("function") {
                let func_text = self.extract_call_name(&func_node, content);
                if !func_text.is_empty() {
                    self.function_calls.insert(func_text);
                }
            }
        }

        // Recursively process children
        for child in node.children(&mut node.walk()) {
            self.extract_usage(&child, content);
        }
    }

    fn extract_call_name(&self, node: &Node, content: &str) -> String {
        match node.kind() {
            "identifier" => {
                node.utf8_text(content.as_bytes()).unwrap_or("").to_string()
            }
            "attribute" => {
                // For method calls like obj.method(), extract just "method"
                if let Some(attr_node) = node.child_by_field_name("attribute") {
                    attr_node.utf8_text(content.as_bytes()).unwrap_or("").to_string()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }

    fn extract_exports(&mut self, node: &Node, content: &str) {
        // Look for __all__ = [...]
        if node.kind() == "assignment" {
            if let Some(left) = node.child_by_field_name("left") {
                if left.utf8_text(content.as_bytes()).unwrap_or("") == "__all__" {
                    if let Some(right) = node.child_by_field_name("right") {
                        self.extract_list_strings(&right, content);
                    }
                }
            }
        }

        for child in node.children(&mut node.walk()) {
            self.extract_exports(&child, content);
        }
    }

    fn extract_list_strings(&mut self, node: &Node, content: &str) {
        if node.kind() == "list" {
            for child in node.children(&mut node.walk()) {
                if child.kind() == "string" {
                    let text = child.utf8_text(content.as_bytes()).unwrap_or("");
                    // Remove quotes
                    let cleaned = text.trim_matches(|c| c == '"' || c == '\'');
                    self.exports.insert(cleaned.to_string());
                }
            }
        }
    }

    pub fn get_results(&self) -> Vec<DeadCodeItem> {
        let mut results = Vec::new();

        for (name, definitions) in &self.definitions {
            for def in definitions {
                let mut confidence = 80;

                // Skip if it's used
                if self.function_calls.contains(name) {
                    continue;
                }

                // Skip magic methods (always used by Python)
                if def.is_magic {
                    continue;
                }

                // Skip test functions (used by test frameworks)
                if def.is_test {
                    continue;
                }

                // Skip if exported in __all__
                if self.exports.contains(name) {
                    continue;
                }

                // Lower confidence for decorated functions (might be used by framework)
                if def.is_decorated {
                    confidence = 60;
                }

                // Lower confidence for methods (might be called dynamically)
                if def.is_method {
                    confidence = 70;
                }

                // Private functions/methods are more likely to be dead
                if def.is_private && !def.is_method {
                    confidence = 85;
                }

                // Skip if below minimum confidence
                if confidence < self.min_confidence {
                    continue;
                }

                // Build code_type string with parent class if applicable
                let code_type = if let Some(ref parent) = def.parent_class {
                    format!("{}.{}", parent, def.code_type)
                } else {
                    def.code_type.clone()
                };

                results.push(DeadCodeItem {
                    file: def.file.clone(),
                    line: def.line,
                    name: name.clone(),
                    code_type,
                    confidence,
                    size: def.size,
                });
            }
        }

        results
    }
}
