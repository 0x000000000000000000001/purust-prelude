use std::rc::Rc;

pub fn Control_Bind_arrayBind(mut arr: crate::UnknownType, mut f: std::rc::Rc<dyn Fn(crate::UnknownType) -> crate::UnknownType>) -> crate::UnknownType {
    if !matches!(arr, crate::Value::Array(_)) {
        panic!("arrayBind called with non-array!");
    }
    
    let a = arr.unwrap_array();
    let mut result = Vec::new();
    
    for item in a.iter() {
        let mapped = f(item.clone());
        let mapped_arr = mapped.unwrap_array();
        result.extend(mapped_arr.iter().cloned());
    }
    
    crate::Value::Array(Rc::new(result))
}
