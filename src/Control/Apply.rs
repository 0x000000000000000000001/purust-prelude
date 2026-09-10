pub fn Control_Apply_arrayApply(functions: crate::UnknownType, values: crate::UnknownType) -> crate::UnknownType {
    let functions = functions.unwrap_array();
    let values = values.unwrap_array();
    let mut result = Vec::new();
    for function in functions.iter() {
        let function = function.unwrap_func1();
        for value in values.iter() {
            result.push(function(value.clone()));
        }
    }
    crate::mk_array(result)
}
