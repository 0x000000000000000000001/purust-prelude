import fs from 'fs';
let code = fs.readFileSync('src/Data/Ord.rs', 'utf8');

code = code.replace(
    'pub fn Data_Ord_ordArrayImpl(mut f: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {',
    'pub fn Data_Ord_ordArrayImpl(mut f: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> i64 {'
);

code = code.replace(
    'return res;',
    'return res.init_int.unwrap();'
);
code = code.replace(
    'return res;',
    'return res.init_int.unwrap();'
);

code = code.replace(
    'std::cmp::Ordering::Less => crate::UnknownType::new(crate::Record_a { tag: "LT", ..Default::default() }),',
    'std::cmp::Ordering::Less => -1,'
);
code = code.replace(
    'std::cmp::Ordering::Equal => crate::UnknownType::new(crate::Record_a { tag: "EQ", ..Default::default() }),',
    'std::cmp::Ordering::Equal => 0,'
);
code = code.replace(
    'std::cmp::Ordering::Greater => crate::UnknownType::new(crate::Record_a { tag: "GT", ..Default::default() }),',
    'std::cmp::Ordering::Greater => 1,'
);

fs.writeFileSync('src/Data/Ord.rs', code);
