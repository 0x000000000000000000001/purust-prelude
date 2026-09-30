use std::rc::Rc;

pub fn Control_Bind_arrayBind(mut arr: crate::UnknownType, mut f: purust_core::Func1<crate::UnknownType, crate::UnknownType>) -> crate::UnknownType {
    if !arr.is_array() {
        panic!("arrayBind called with non-array!");
    }
    
    let mut result = Vec::new();
    
    for item in arr.array_iter() {
        let mapped = f(item);
        result.extend(mapped.array_iter());
    }
    
    crate::Value::Array(Rc::new(result))
}
