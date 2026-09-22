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

// ECMAScript `Number::toString` (shortest round-trip digits, decimal notation
// for 10^-6 <= |n| < 10^21, exponential outside), which is what the JS FFI
// uses. Rust's `Display` never switches to exponent notation.
fn js_number_to_string(n: f64) -> String {
    if n.is_nan() {
        return "NaN".to_owned();
    }
    if n == 0.0 {
        return "0".to_owned();
    }
    if n < 0.0 {
        return format!("-{}", js_number_to_string(-n));
    }
    if n.is_infinite() {
        return "Infinity".to_owned();
    }
    // `{:e}` already gives the shortest round-trip digits and an exponent.
    let formatted = format!("{:e}", n);
    let (mantissa, exponent) = formatted.split_once('e').expect("exponent notation");
    let exponent: i32 = exponent.parse().expect("decimal exponent");
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let k = digits.len() as i32;
    let point = exponent + 1;
    if k <= point && point <= 21 {
        format!("{}{}", digits, "0".repeat((point - k) as usize))
    } else if 0 < point && point <= 21 {
        let (head, tail) = digits.split_at(point as usize);
        format!("{}.{}", head, tail)
    } else if -6 < point && point <= 0 {
        format!("0.{}{}", "0".repeat((-point) as usize), digits)
    } else {
        let exponent = point - 1;
        let sign = if exponent >= 0 { "+" } else { "-" };
        let (head, tail) = digits.split_at(1);
        if tail.is_empty() {
            format!("{}e{}{}", head, sign, exponent.abs())
        } else {
            format!("{}.{}e{}{}", head, tail, sign, exponent.abs())
        }
    }
}

pub fn Data_Show_showNumberImpl(mut a0: f64) -> String {
    let str = js_number_to_string(a0);
    // PureScript appends ".0" whenever the JavaScript text has no dot and no
    // exponent (`isNaN(str + ".0")`).
    let with_zero = format!("{}.0", str);
    if with_zero.parse::<f64>().is_ok() { with_zero } else { str }
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
