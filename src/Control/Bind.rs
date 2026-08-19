use std::rc::Rc;

pub fn Control_Bind_arrayBind(mut arr: crate::UnknownType, mut f: crate::UnknownType) -> crate::UnknownType {
    if !matches!(arr, crate::Value::Array(_)) {
        panic!("arrayBind called with non-array!");
    }
    
    let a = arr.unwrap_array();
    let mut result = Vec::new();
    
    for item in a.iter() {
        let mapped = f.unwrap_func()(item.clone());
        let mapped_arr = mapped.unwrap_array();
        result.extend(mapped_arr.iter().cloned());
    }
    
    crate::Value::Array(Rc::new(result))
}
