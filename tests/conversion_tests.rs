#[cfg(test)]
mod tests {
    use catalyst::*;

    #[test]
    fn test_conversion_config_default() {
        let config = ConversionConfig::default();
        assert!(config.strict_types);
        assert!(config.optimize);
        assert!(config.generate_tests);
        assert!(config.include_benchmarks);
    }

    #[test]
    fn test_type_mapper() {
        use catalyst::types::*;

        let mapper = TypeMapper::new();
        assert_eq!(
            mapper.map_type(&PythonType::Int),
            Some(RustType::I64)
        );
        assert_eq!(
            mapper.map_type(&PythonType::String),
            Some(RustType::String)
        );
    }
}
