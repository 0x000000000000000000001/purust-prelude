pub fn Data_Functor_arrayMap(mut f: purust_core::Func1<crate::UnknownType, crate::UnknownType>, mut arr: crate::UnknownType) -> crate::UnknownType {
    let mut result = Vec::new();
    let a = arr.unwrap_array();
    for i in 0..a.len() {
        result.push(f(a[i].clone()));
    }
    crate::mk_array(result)
}
