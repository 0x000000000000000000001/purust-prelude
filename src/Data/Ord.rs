pub fn Data_Ord_ordIntImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a: i64, mut b: i64) -> crate::UnknownType {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordNumberImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a: f64, mut b: f64) -> crate::UnknownType {
    if a < b { lt }
    else if a > b { gt }
    else { eq }
}

pub fn Data_Ord_ordCharImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a: char, mut b: char) -> crate::UnknownType {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordStringImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a: String, mut b: String) -> crate::UnknownType {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordBooleanImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a: bool, mut b: bool) -> crate::UnknownType {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordArrayImpl(mut f: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> i64 {
    let arr1 = a0.unwrap_array();
    let arr2 = a1.unwrap_array();
    let len = std::cmp::min(arr1.len(), arr2.len());
    for i in 0..len {
        let res = f.unwrap_func()(arr1[i].clone()).unwrap_func()(arr2[i].clone());
        let cmp = res.unwrap_int();
        if cmp != 0 {
            return cmp;
        }
    }
    match arr1.len().cmp(&arr2.len()) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}
