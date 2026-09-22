pub fn Data_Show_Generic_intercalate(separator: String, values: crate::UnknownType) -> String {
    values
        .unwrap_array()
        .iter()
        .map(|value| value.unwrap_string())
        .collect::<Vec<_>>()
        .join(&separator)
}
