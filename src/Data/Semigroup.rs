pub fn Data_Semigroup_concatString(mut a0: String, mut a1: String) -> String {
    format!("{}{}", a0, a1)
}

pub fn Data_Semigroup_concatArray<a: Clone + 'static>(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let mut vec = a0.unwrap_array().to_vec();
    vec.extend(a1.unwrap_array().to_vec());
    crate::mk_array(vec)
}
