pub fn Data_Ring_intSub(mut a: crate::UnknownType, mut b: crate::UnknownType) -> crate::UnknownType {
    crate::mk_int(a.init_int.unwrap() - b.init_int.unwrap())
}

pub fn Data_Ring_numSub(mut a: crate::UnknownType, mut b: crate::UnknownType) -> crate::UnknownType {
    crate::mk_number(a.init_number.unwrap() - b.init_number.unwrap())
}
