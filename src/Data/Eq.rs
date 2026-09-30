pub fn Data_Eq_eqIntImpl(mut a0: i64, mut a1: i64) -> bool {
    a0 == a1
}

pub fn Data_Eq_eqNumberImpl(mut a0: f64, mut a1: f64) -> bool {
    a0 == a1
}

pub fn Data_Eq_eqCharImpl(mut a0: char, mut a1: char) -> bool {
    a0 == a1
}

pub fn Data_Eq_eqStringImpl(mut a0: String, mut a1: String) -> bool {
    a0 == a1
}

pub fn Data_Eq_eqBooleanImpl(mut a0: bool, mut a1: bool) -> bool {
    a0 == a1
}

pub fn Data_Eq_eqArrayImpl(mut f: purust_core::Func2<crate::UnknownType, crate::UnknownType, bool>, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> bool {
    let arr1 = a0.array_iter();
    let arr2 = a1.array_iter();
    if arr1.len() != arr2.len() {
        return false;
    }
    for (x, y) in arr1.zip(arr2) {
        let res = f(x, y);
        if !res {
            return false;
        }
    }
    true
}
