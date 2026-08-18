pub fn Data_Semiring_intAdd(mut a: crate::UnknownType, mut b: crate::UnknownType) -> crate::UnknownType {
    crate::mk_int(a.init_int.unwrap() + b.init_int.unwrap())
}

pub fn Data_Semiring_intMul(mut a: crate::UnknownType, mut b: crate::UnknownType) -> crate::UnknownType {
    crate::mk_int(a.init_int.unwrap() * b.init_int.unwrap())
}

pub fn Data_Semiring_numAdd(mut a: crate::UnknownType, mut b: crate::UnknownType) -> crate::UnknownType {
    crate::mk_number(a.init_number.unwrap() + b.init_number.unwrap())
}

pub fn Data_Semiring_numMul(mut a: crate::UnknownType, mut b: crate::UnknownType) -> crate::UnknownType {
    crate::mk_number(a.init_number.unwrap() * b.init_number.unwrap())
}
