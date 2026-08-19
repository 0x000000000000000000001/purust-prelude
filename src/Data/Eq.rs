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

pub fn Data_Eq_eqArrayImpl(mut f: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> bool {
    let arr1 = a0.init_array.as_ref().unwrap();
    let arr2 = a1.init_array.as_ref().unwrap();
    if arr1.len() != arr2.len() {
        return false;
    }
    for (x, y) in arr1.iter().zip(arr2.iter()) {
        let res = f.call.as_ref().unwrap()(x.clone()).call.as_ref().unwrap()(y.clone());
        if !res.init_bool.unwrap() {
            return false;
        }
    }
    true
}
