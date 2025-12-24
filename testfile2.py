"""
Comprehensive Dead Code Detection Benchmark
This file contains various edge cases to stress-test dead code detectors
"""

# ============================================================================
# UNUSED IMPORTS - Various complexity levels
# ============================================================================
import os  # DEAD - never used
import sys  # USED - sys.exit called
from pathlib import Path  # DEAD - imported but never used
from typing import Dict, List, Tuple, Optional, Any  # MIXED - some used, some not
from collections import defaultdict, Counter, deque  # MIXED
import json  # DEAD
import re  # USED - in regex pattern
from functools import wraps, lru_cache, partial  # MIXED
from itertools import chain, permutations  # DEAD - both unused
import threading  # DEAD
from datetime import datetime, timedelta  # USED - datetime used

# Conditional imports
try:
    import numpy as np  # DEAD - imported but never used
except ImportError:
    np = None

# Star imports - hard to track
from math import *  # Makes tracking difficult

# ============================================================================
# UNUSED VARIABLES - Global scope
# ============================================================================
DEAD_CONSTANT = 42  # DEAD
USED_CONSTANT = 100  # USED below
unused_global_var = "I'm forgotten"  # DEAD
used_global = "I'm alive"  # USED

# Complex unused assignments
x, y, z = 1, 2, 3  # MIXED - x used, y and z dead
a = b = c = 0  # ALL DEAD

# ============================================================================
# EDGE CASE: Dynamic attribute access
# ============================================================================
class DynamicClass:
    """Methods might be called via getattr/setattr"""

    def __init__(self):
        self.value = 10
        self.unused_attr = 20  # Might be accessed dynamically

    def dynamic_method(self):  # Might be called via getattr
        return "dynamic"

    def truly_unused_method(self):  # DEAD
        return "never called"

    def _internal_method(self):  # Used by __getattribute__
        return "internal"

    def __getattribute__(self, name):
        """Override makes tracking hard"""
        if name == "special":
            return super().__getattribute__("_internal_method")()
        return super().__getattribute__(name)


# ============================================================================
# EDGE CASE: Metaclasses and class decorators
# ============================================================================
class MetaClass(type):
    """Metaclass might use methods automatically"""
    def __new__(mcs, name, bases, dct):
        # Might call methods during class creation
        if 'auto_init' in dct:
            dct['auto_init']()  # String-based lookup
        return super().__new__(mcs, name, bases, dct)


class AutoClass(metaclass=MetaClass):
    """Methods might be used by metaclass"""

    @staticmethod
    def auto_init():  # Called by metaclass
        pass

    def unused_in_auto(self):  # DEAD
        pass


# ============================================================================
# EDGE CASE: Decorators that might use code
# ============================================================================
def register_function(func):
    """Decorator that registers functions"""
    REGISTRY[func.__name__] = func  # Function stored for later use
    return func

REGISTRY = {}  # Global registry

@register_function
def registered_unused():  # Looks dead but stored in registry
    return "registered"

@lru_cache(maxsize=128)
def cached_unused(n):  # DEAD but decorated
    return n * 2


# ============================================================================
# EDGE CASE: String-based execution (eval/exec)
# ============================================================================
def dangerous_eval_user():  # Might be called via eval
    return "eval danger"

def definitely_dead_eval():  # DEAD
    return "truly dead"

def eval_caller():
    """Uses eval to call functions"""
    func_name = "dangerous_eval_user"
    result = eval(f"{func_name}()")  # Dynamic call
    return result


# ============================================================================
# EDGE CASE: Reflection and introspection
# ============================================================================
class ReflectiveClass:
    """Class that uses reflection"""

    def method_a(self):  # Might be found via introspection
        return "a"

    def method_b(self):  # Might be found via introspection
        return "b"

    def unused_reflective(self):  # DEAD
        return "dead"

    def call_methods_dynamically(self):
        """Calls methods based on their names"""
        for name in dir(self):
            if name.startswith('method_'):
                method = getattr(self, name)
                if callable(method):
                    method()  # Dynamic call


# ============================================================================
# EDGE CASE: Closures and nested functions
# ============================================================================
def outer_function():
    """Nested functions create complex scope"""
    dead_in_outer = 10  # DEAD - never used
    used_in_outer = 20  # USED - accessed by closure

    def inner_function():
        return used_in_outer * 2  # Closure access

    def dead_inner():  # DEAD - never called
        return "dead inner"

    return inner_function()


# ============================================================================
# EDGE CASE: Generator expressions and comprehensions
# ============================================================================
def generator_deadness():
    """Generators might hide usage"""
    unused_in_gen = 100  # DEAD
    used_in_gen = 50  # USED

    # Variable used in generator
    gen = (x * used_in_gen for x in range(10))

    dead_gen = (x * 2 for x in range(5))  # DEAD - generator never consumed

    return list(gen)


# ============================================================================
# EDGE CASE: Lambda functions
# ============================================================================
dead_lambda = lambda x: x * 2  # DEAD
used_lambda = lambda x: x * 3  # USED

lambda_holder = {
    'func': lambda y: y + 1  # Stored in dict, might be used
}


# ============================================================================
# EDGE CASE: Type hints and forward references
# ============================================================================
def function_with_hints(param: List[int]) -> Dict[str, Any]:  # Types used in hints
    """Type hints reference imported types"""
    return {"value": param[0] if param else 0}

# Forward reference
def create_node() -> 'TreeNode':  # Forward reference
    return TreeNode()

class TreeNode:
    """Class used in forward reference"""
    def __init__(self):
        self.left: Optional['TreeNode'] = None  # Self-reference
        self.right: Optional['TreeNode'] = None


# ============================================================================
# EDGE CASE: Context managers and magic methods
# ============================================================================
class ContextClass:
    """Context manager with magic methods"""

    def __enter__(self):  # Magic method - looks unused but used by 'with'
        return self

    def __exit__(self, *args):  # Magic method
        pass

    def unused_context_method(self):  # DEAD
        pass

    def __del__(self):  # Destructor - called automatically
        pass


# ============================================================================
# EDGE CASE: Operator overloading
# ============================================================================
class OperatorClass:
    def __init__(self, value):
        self.value = value

    def __add__(self, other):  # Looks unused but called by +
        return OperatorClass(self.value + other.value)

    def __lt__(self, other):  # DEAD - comparison never used
        return self.value < other.value

    def unused_operator_helper(self):  # DEAD
        return self.value * 2


# ============================================================================
# EDGE CASE: Properties and descriptors
# ============================================================================
class DescriptorClass:
    """Properties that look like attributes"""

    def __init__(self):
        self._hidden = 10
        self._dead_hidden = 20  # DEAD

    @property
    def computed_value(self):  # Looks unused but accessed as attribute
        return self._hidden * 2

    @computed_value.setter
    def computed_value(self, value):  # Setter - might not be used
        self._hidden = value

    @property
    def dead_property(self):  # DEAD
        return self._dead_hidden


# ============================================================================
# EDGE CASE: Abstract base classes
# ============================================================================
from abc import ABC, abstractmethod

class AbstractBase(ABC):
    """Abstract class - methods must exist but aren't called here"""

    @abstractmethod
    def required_method(self):  # Required but looks unused
        pass

    def concrete_unused(self):  # DEAD
        pass


class ConcreteImpl(AbstractBase):
    """Implementation of abstract class"""

    def required_method(self):  # Implements abstract
        return "implemented"


# ============================================================================
# EDGE CASE: Recursive and mutually recursive functions
# ============================================================================
def factorial_recursive(n):  # DEAD but recursive
    """Recursive function"""
    if n <= 1:
        return 1
    return n * factorial_recursive(n - 1)  # Self-call

# Mutual recursion
def is_even(n):  # DEAD but mutually recursive
    if n == 0:
        return True
    return is_odd(n - 1)

def is_odd(n):  # DEAD but mutually recursive
    if n == 0:
        return False
    return is_even(n - 1)


# ============================================================================
# EDGE CASE: Potential infinite loops
# ============================================================================
def infinite_while():  # DEAD - and would loop forever
    """Infinite loop pattern"""
    counter = 0
    while True:
        counter += 1
        if counter > 1000000:  # Never breaks effectively
            continue
        # No break condition

def infinite_recursion(n):  # DEAD - and dangerous
    """Recursion without base case"""
    return infinite_recursion(n) + 1  # Stack overflow

def tricky_infinite(x):  # DEAD
    """Looks like it terminates but might not"""
    while x > 0:
        x = x + 1  # Always increases, never terminates
    return x


# ============================================================================
# EDGE CASE: Class variables vs instance variables
# ============================================================================
class MixedVariables:
    class_var = 100  # USED as class variable
    dead_class_var = 200  # DEAD

    def __init__(self):
        self.instance_var = 300  # USED
        self.dead_instance = 400  # DEAD

    def use_class_var(self):
        return MixedVariables.class_var


# ============================================================================
# EDGE CASE: Module-level __getattr__
# ============================================================================
def __getattr__(name):
    """Module-level getattr makes everything potentially accessible"""
    if name == "dynamic_constant":
        return 999
    raise AttributeError(f"module has no attribute {name}")


# ============================================================================
# EDGE CASE: Conditional definitions
# ============================================================================
if sys.platform == "linux":
    def platform_specific_linux():  # Platform-specific
        return "linux only"
else:
    def platform_specific_other():  # Platform-specific
        return "not linux"

# Version-based definition
if sys.version_info >= (3, 10):
    def modern_python_feature():  # Version-specific
        return "match statement"


# ============================================================================
# EDGE CASE: Import tricks
# ============================================================================
# Lazy import
_json_module = None
def get_json():
    global _json_module
    if _json_module is None:
        import json as _json_module  # Lazy load
    return _json_module


# ============================================================================
# EDGE CASE: Monkey patching
# ============================================================================
class OriginalClass:
    def original_method(self):
        return "original"

def replacement_method(self):  # Looks dead but used for monkey patching
    return "replaced"

# Monkey patch
OriginalClass.original_method = replacement_method


# ============================================================================
# EDGE CASE: __all__ exports
# ============================================================================
__all__ = [
    'DynamicClass',
    'used_global',
    'function_with_hints',
    'USED_CONSTANT',
    'ContextClass',
    'TreeNode'
]


# ============================================================================
# EDGE CASE: Complex control flow
# ============================================================================
def complex_control_flow(x, y, z):
    """Multiple nested conditions"""
    result = 0
    temp_dead = 100  # DEAD
    temp_used = 50  # USED

    if x > 0:
        if y > 0:
            if z > 0:
                result = temp_used
            else:
                for i in range(10):
                    if i % 2 == 0:
                        result += i
                    else:
                        continue
        elif y < 0:
            while z > 0:
                z -= 1
                result += z
    else:
        try:
            result = x / y
        except ZeroDivisionError:
            result = 0
        finally:
            pass

    return result


# ============================================================================
# EDGE CASE: Walrus operator
# ============================================================================
def walrus_usage():
    """Python 3.8+ walrus operator"""
    data = [1, 2, 3, 4, 5]

    # Variable defined in walrus
    if (n := len(data)) > 3:  # n used here
        return n

    # Dead walrus
    unused_walrus = [(x, y) for x in data if (y := x * 2)]  # DEAD - list unused

    return 0


# ============================================================================
# MAIN EXECUTION - Determines what's actually used
# ============================================================================
def main():
    """Main entry point"""
    # Use some functions
    print(f"Global: {used_global}")
    print(f"Constant: {USED_CONSTANT}")
    print(f"X value: {x}")  # Uses x from tuple unpacking

    # Use datetime
    now = datetime.now()
    print(f"Time: {now}")

    # Use regex
    pattern = re.compile(r"\d+")
    print(f"Pattern: {pattern}")

    # Create instances
    dyn = DynamicClass()
    print(getattr(dyn, "dynamic_method")())

    # Use context manager
    with ContextClass() as ctx:
        print("Context used")

    # Use operators
    op1 = OperatorClass(10)
    op2 = OperatorClass(20)
    result = op1 + op2
    print(f"Operator result: {result.value}")

    # Use descriptor
    desc = DescriptorClass()
    print(f"Computed: {desc.computed_value}")

    # Use concrete implementation
    impl = ConcreteImpl()
    print(impl.required_method())

    # Use lambda
    print(used_lambda(5))

    # Use function from lambda holder
    print(lambda_holder['func'](10))

    # Use outer function
    print(outer_function())

    # Use generator
    print(generator_deadness())

    # Use eval caller
    print(eval_caller())

    # Use reflective class
    ref = ReflectiveClass()
    ref.call_methods_dynamically()

    # Use class variable
    mv = MixedVariables()
    print(mv.use_class_var())
    print(mv.instance_var)

    # Use complex control flow
    print(complex_control_flow(1, 2, 3))

    # Use walrus
    print(walrus_usage())

    # Use type hinted function
    print(function_with_hints([1, 2, 3]))

    # Use tree node
    node = create_node()

    # Use monkey patched
    orig = OriginalClass()
    print(orig.original_method())

    # Use lazy import
    json_mod = get_json()

    # Platform specific
    if sys.platform == "linux":
        print(platform_specific_linux())
    else:
        print(platform_specific_other())


if __name__ == "__main__":
    main()
    sys.exit(0)
