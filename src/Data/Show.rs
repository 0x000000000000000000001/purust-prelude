// FFI implementation for Data.Show in Rust

pub fn Data_Show_showStringImpl(mut s: String) -> String {
    format!("{:?}", s)
}

pub fn Data_Show_showIntImpl(mut a0: i64) -> String {
    a0.to_string()
}

pub fn Data_Show_showNumberImpl(mut a0: f64) -> String {
    a0.to_string()
}

pub fn Data_Show_showCharImpl(mut a0: char) -> String {
    format!("'{}'", a0)
}

pub fn Data_Show_showArrayImpl(mut f: crate::UnknownType, mut a0: crate::UnknownType) -> String {
    let arr = a0.init_array.as_ref().unwrap();
    let mut s = String::from("[");
    for (i, x) in arr.iter().enumerate() {
        if i > 0 {
            s.push_str(",");
        }
        let res = f.call.as_ref().unwrap()(x.clone());
        s.push_str(res.init_string.as_ref().unwrap());
    }
    s.push_str("]");
    s
}
