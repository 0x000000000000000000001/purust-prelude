pub fn Record_Unsafe_unsafeGet(field: String, record: crate::UnknownType) -> crate::UnknownType {
    record
        .__purust_get_field(&field)
        .unwrap_or_else(|| panic!("Record.Unsafe.unsafeGet: missing field '{}'", field))
}

pub fn Record_Unsafe_unsafeHas(field: String, record: crate::UnknownType) -> bool {
    record.__purust_get_field(&field).is_some()
}

pub fn Record_Unsafe_unsafeSet(
    field: String,
    value: crate::UnknownType,
    record: crate::UnknownType,
) -> crate::UnknownType {
    record.__purust_set_field(&field, value)
}
