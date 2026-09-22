pub fn Test_Utils_throwErr(message: String) -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |_| {
        panic!("{}", message);
    })))
}
