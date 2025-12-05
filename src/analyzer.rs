use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tree_sitter::Parser;
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

pub struct DeadCodeAnalyzer {
    #[allow(dead_code)]
    min_confidence: u8,
    #[allow(dead_code)]
    exclude_patterns: Vec<String>,
    defined_names: HashMap<String, Vec<(String, usize)>>,
    used_names: HashMap<String, usize>,
    #[allow(dead_code)]
    results: Vec<DeadCodeItem>,
}

impl DeadCodeAnalyzer {
    pub fn new(min_confidence: u8, exclude_patterns: Vec<&str>) -> Self {
        Self {
            min_confidence,
            exclude_patterns: exclude_patterns.iter().map(|s| s.to_string()).collect(),
            defined_names: HashMap::new(),
            used_names: HashMap::new(),
            results: Vec::new(),
        }
    }

    pub fn analyze_path(&mut self, path: &PathBuf) -> Result<()> {
        if path.is_file() {
            self.analyze_file(path)?;
        } else if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "py"))
            {
                self.analyze_file(&entry.path().to_path_buf())?;
            }
        }
        Ok(())
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

        // Extract definitions and usage
        self.extract_definitions(&root, &content, file_path);
        self.extract_usage(&root, &content);

        Ok(())
    }

    fn extract_definitions(&mut self, node: &tree_sitter::Node, content: &str, file_path: &PathBuf) {
        if node.kind() == "function_definition" || node.kind() == "decorated_definition" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = name_node.utf8_text(content.as_bytes()).unwrap_or("");
                let line = node.start_position().row + 1;
                let _size = node.end_byte() - node.start_byte();
                
                self.defined_names
                    .entry(name.to_string())
                    .or_insert_with(Vec::new)
                    .push((file_path.to_string_lossy().to_string(), line));
                
                // Check for imports
                if node.kind() == "import_statement" {
                    self.defined_names
                        .entry(name.to_string())
                        .or_insert_with(Vec::new)
                        .push((file_path.to_string_lossy().to_string(), line));
                }
            }
        }

        for child in node.children(&mut node.walk()) {
            self.extract_definitions(&child, content, file_path);
        }
    }

    fn extract_usage(&mut self, node: &tree_sitter::Node, content: &str) {
        if node.kind() == "identifier" || node.kind() == "attribute" {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                *self.used_names.entry(text.to_string()).or_insert(0) += 1;
            }
        }

        for child in node.children(&mut node.walk()) {
            self.extract_usage(&child, content);
        }
    }

    pub fn get_results(&self) -> Vec<DeadCodeItem> {
        let mut results = Vec::new();

        for (name, locations) in &self.defined_names {
            if !self.used_names.contains_key(name) && !name.starts_with('_') {
                for (file, line) in locations {
                    results.push(DeadCodeItem {
                        file: file.clone(),
                        line: *line,
                        name: name.clone(),
                        code_type: "function/class".to_string(),
                        confidence: 80,
                        size: 0,
                    });
                }
            }
        }

        results
    }
}
