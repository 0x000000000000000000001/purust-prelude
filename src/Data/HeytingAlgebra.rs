pub fn Data_HeytingAlgebra_boolConj(mut a: crate::UnknownType, mut b: crate::UnknownType) -> crate::UnknownType {
    crate::mk_bool(a.init_bool.unwrap() && b.init_bool.unwrap())
}

pub fn Data_HeytingAlgebra_boolDisj(mut a: crate::UnknownType, mut b: crate::UnknownType) -> crate::UnknownType {
    crate::mk_bool(a.init_bool.unwrap() || b.init_bool.unwrap())
}

pub fn Data_HeytingAlgebra_boolNot(mut a: crate::UnknownType) -> crate::UnknownType {
    crate::mk_bool(!a.init_bool.unwrap())
}
