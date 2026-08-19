import fs from 'fs';
let code = fs.readFileSync('src/Data/Ord.rs', 'utf8');

code = code.replace(
    '        if res.tag == "LT" {\n            return res.init_int.unwrap();\n        } else if res.tag == "GT" {\n            return res.init_int.unwrap();\n        }',
    '        let cmp = res.init_int.unwrap();\n        if cmp != 0 {\n            return cmp;\n        }'
);

fs.writeFileSync('src/Data/Ord.rs', code);
