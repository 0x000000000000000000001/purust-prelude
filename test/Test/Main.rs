pub fn Test_Main_testNumberShow(
    show_number: purust_core::Func1<f64, String>,
) -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |_| {
        let cases = [
            // Within Int range
            (0.0, "0.0"),
            (1.0, "1.0"),
            (-1.0, "-1.0"),
            (500.0, "500.0"),
            // Outside Int range
            (1e10, "10000000000.0"),
            (1e10 + 0.5, "10000000000.5"),
            (-1e10, "-10000000000.0"),
            (-1e10 - 0.5, "-10000000000.5"),
            // With exponent
            (1e21, "1e+21"),
            (1e-21, "1e-21"),
            // With decimal and exponent
            (1.5e21, "1.5e+21"),
            (1.5e-10, "1.5e-10"),
            (f64::NAN, "NaN"),
            (f64::INFINITY, "Infinity"),
            (f64::NEG_INFINITY, "-Infinity"),
        ];
        for (number, expected) in cases {
            let actual = show_number(number);
            if actual != expected {
                panic!("For {}, expected {}, got: {}.", number, expected, actual);
            }
        }
        crate::Value::Unit
    })))
}

pub fn Test_Main_makeArray(length: i64) -> crate::UnknownType {
    crate::Value::Array(std::rc::Rc::new(
        (0..length).map(crate::Value::Int).collect(),
    ))
}
