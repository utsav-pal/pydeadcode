"""
Comprehensive test file for dead code detection
"""

# DEAD CODE - should be detected
def unused_function():
    """This function is never called"""
    return "I'm dead code"


# DEAD CODE - should be detected
def another_unused():
    """Another unused function"""
    pass


# USED CODE - should NOT be detected
def used_function():
    """This function is called"""
    return "I'm alive"


# USED CODE - should NOT be detected
def helper_function():
    """Called by main"""
    return "helping"


# DEAD CODE - but decorated, lower confidence
@property
def decorated_unused():
    """Decorated but unused - framework might use it"""
    return "maybe used"


# Class with mixed usage
class MyClass:
    # USED CODE - __init__ is magic method
    def __init__(self):
        self.value = 10
    
    # USED CODE - called explicitly
    def used_method(self):
        return self.value * 2
    
    # DEAD CODE - never called
    def unused_method(self):
        return self.value * 3
    
    # USED CODE - magic method
    def __str__(self):
        return f"MyClass({self.value})"


# DEAD CODE - class never instantiated
class UnusedClass:
    def __init__(self):
        pass
    
    def some_method(self):
        pass


# USED CODE - test function (pytest/unittest pattern)
def test_something():
    """Test functions shouldn't be flagged"""
    assert True


# USED CODE - private but used
def _private_helper():
    """Private but used"""
    return "secret"


# DEAD CODE - private and unused
def _unused_private():
    """Private and unused"""
    return "dead secret"


# Module exports - these should NOT be flagged as dead
__all__ = ['used_function', 'MyClass']


# Entry point - functions called here are USED
if __name__ == "__main__":
    print(used_function())
    print(helper_function())
    
    obj = MyClass()
    print(obj.used_method())
    print(obj)  # Calls __str__
    
    result = _private_helper()
    print(result)
