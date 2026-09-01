# Catalyst Conversion Guide

## Overview

This guide helps you convert Python projects to Rust using Catalyst.

## Step 1: Analysis

```bash
catalyst analyze ./my_python_project
```

This analyzes your Python code and identifies:
- Complexity hotspots
- Type requirements
- Performance bottlenecks
- Dependencies

## Step 2: Conversion

```bash
catalyst convert input.py --output output.rs
```

Converts Python syntax to Rust equivalents.

## Step 3: Optimization

```bash
catalyst optimize output.rs
```

Applies performance optimizations:
- Memory layout improvements
- Algorithm optimizations
- Parallel processing hints

## Step 4: Testing

```bash
cargo test
```

Run comprehensive tests to validate conversion.

## Best Practices

1. **Start Small** - Convert one module at a time
2. **Preserve Behavior** - Test thoroughly
3. **Leverage Rust Features** - Use ownership, borrowing, pattern matching
4. **Document Changes** - Keep mapping of Python to Rust code
5. **Benchmark** - Compare performance before/after

## Common Patterns

### Python Lists → Rust Vectors

```python
# Python
my_list = [1, 2, 3]
my_list.append(4)
```

```rust
// Rust
let mut my_list = vec![1, 2, 3];
my_list.push(4);
```

### Python Dicts → Rust HashMaps

```python
# Python
my_dict = {"key": "value"}
my_dict["new_key"] = "new_value"
```

```rust
// Rust
use std::collections::HashMap;
let mut my_dict = HashMap::new();
my_dict.insert("key", "value");
my_dict.insert("new_key", "new_value");
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [PyO3 Documentation](https://pyo3.rs/)
- [Catalyst Examples](../examples/)
