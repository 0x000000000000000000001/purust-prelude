pub fn Data_Bounded_bottomChar() -> char {
    '\u{0}'
}

pub fn Data_Bounded_topChar() -> char {
    '\u{FFFF}'
}

pub fn Data_Bounded_bottomInt() -> i64 {
    -2147483648
}

pub fn Data_Bounded_topInt() -> i64 {
    2147483647
}

pub fn Data_Bounded_bottomNumber() -> f64 {
    std::f64::NEG_INFINITY
}

pub fn Data_Bounded_topNumber() -> f64 {
    std::f64::INFINITY
}
