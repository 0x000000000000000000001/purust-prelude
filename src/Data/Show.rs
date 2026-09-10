// FFI implementation for Data.Show in Rust

fn purust_show_escape(code: u16) -> Option<&'static str> {
    match code {
        7 => Some("\\a"), 8 => Some("\\b"), 9 => Some("\\t"),
        10 => Some("\\n"), 11 => Some("\\v"), 12 => Some("\\f"),
        13 => Some("\\r"), _ => None,
    }
}

pub fn Data_Show_showStringImpl(s: String) -> String {
    let mut output = String::from("\"");
    let mut chars = s.chars().peekable();
    while let Some(character) = chars.next() {
        let code = purust_core::purust_char_to_code_unit(character);
        if character == '"' || character == '\\' {
            output.push('\\');
            output.push(character);
        } else if let Some(escape) = purust_show_escape(code) {
            output.push_str(escape);
        } else if code < 0x20 || code == 0x7f {
            output.push_str(&format!("\\{}", code));
            if chars.peek().is_some_and(|next| next.is_ascii_digit()) {
                output.push_str("\\&");
            }
        } else {
            // Retain encoded units, including unpaired surrogates, verbatim.
            output.push(character);
        }
    }
    output.push('"');
    output
}

pub fn Data_Show_showIntImpl(mut a0: i64) -> String {
    a0.to_string()
}

pub fn Data_Show_showNumberImpl(mut a0: f64) -> String {
    a0.to_string()
}

pub fn Data_Show_showCharImpl(character: char) -> String {
    let code = purust_core::purust_char_to_code_unit(character);
    if let Some(escape) = purust_show_escape(code) {
        format!("'{}'", escape)
    } else if code < 0x20 || code == 0x7f {
        format!("'\\{}'", code)
    } else if character == '\'' || character == '\\' {
        format!("'\\{}'", character)
    } else {
        format!("'{}'", character)
    }
}

pub fn Data_Show_showArrayImpl(mut f: purust_core::Func1<crate::UnknownType, String>, mut a0: crate::UnknownType) -> String {
    let arr = a0.unwrap_array();
    let mut s = String::from("[");
    for (i, x) in arr.iter().enumerate() {
        if i > 0 {
            s.push_str(",");
        }
        let res = f(x.clone());
        s.push_str(&res);
    }
    s.push_str("]");
    s
}
