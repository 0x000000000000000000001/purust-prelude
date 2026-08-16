use std::rc::Rc;

pub fn Control_Bind_arrayBind(mut arr: crate::UnknownType, mut f: crate::UnknownType) -> crate::UnknownType {
    if arr.init_array.is_none() {
        panic!("arrayBind called with non-array! arr tag: {}, init_string: {:?}, init_int: {:?}", arr.tag, arr.init_string, arr.init_int);
    }
    
    let a = arr.init_array.as_ref().unwrap();
    let mut result = Vec::new();
    
    for item in a.iter() {
        let mapped = f.call.clone().unwrap()(item.clone());
        let mapped_arr = mapped.init_array.as_ref().unwrap();
        result.extend(mapped_arr.iter().cloned());
    }
    
    crate::UnknownType::new(crate::Record_a {
        init_array: Some(Rc::new(result)),
        ..Default::default()
    })
}
