/// Type mapping between Python and Rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PythonType {
    Int,
    Float,
    String,
    Bool,
    List,
    Dict,
    Tuple,
    Set,
    None,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RustType {
    I32,
    I64,
    F32,
    F64,
    String,
    Bool,
    Vec,
    HashMap,
    Tuple,
    HashSet,
    Option,
    Custom(String),
}

pub struct TypeMapper {
    mapping: HashMap<PythonType, RustType>,
}

impl TypeMapper {
    pub fn new() -> Self {
        let mut mapping = HashMap::new();
        mapping.insert(PythonType::Int, RustType::I64);
        mapping.insert(PythonType::Float, RustType::F64);
        mapping.insert(PythonType::String, RustType::String);
        mapping.insert(PythonType::Bool, RustType::Bool);
        mapping.insert(PythonType::List, RustType::Vec);
        mapping.insert(PythonType::Dict, RustType::HashMap);
        mapping.insert(PythonType::None, RustType::Option);

        Self { mapping }
    }

    pub fn map_type(&self, py_type: &PythonType) -> Option<RustType> {
        self.mapping.get(py_type).cloned()
    }
}
