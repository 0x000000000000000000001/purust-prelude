pub fn Data_Functor_arrayMap(mut f: purust_core::Func1<crate::UnknownType, crate::UnknownType>, mut arr: crate::UnknownType) -> crate::UnknownType {
    let mut result = Vec::with_capacity(arr.array_len());
    for item in arr.array_iter() {
        result.push(f(item));
    }
    crate::mk_array(result)
}
