pub fn Data_Semigroup_concatString(mut a0: String, mut a1: String) -> String {
    format!("{}{}", a0, a1)
}

pub fn Data_Semigroup_concatArray(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let mut vec = a0.init_array.as_ref().unwrap().to_vec();
    vec.extend(a1.init_array.as_ref().unwrap().to_vec());
    crate::mk_array(vec)
}
