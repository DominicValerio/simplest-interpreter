use crate::object::{NativeFunctionCallback, NativeFunctionDef, Object};
use std::{collections::HashMap};

#[allow(non_upper_case_globals)]
const print: NativeFunctionCallback = |args, i| {
    let mut output = String::default();
    for v in args.into_iter() {
        output.push_str(format!("{v}").as_str());
    }
    i.stdout.push_str(&output);
    print!("{}", output);

    return Object::Unit;
};

#[allow(non_upper_case_globals)]
const println: NativeFunctionCallback = |args, i| {
    let ret = print(args, i);
    print(vec![Object::Str("\n".to_string())], i);
    return ret
};

pub fn get_lib() -> HashMap<String, Object> {
    [
        ("print", print), 
        ("println", println), 
    ]
        .into_iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                Object::NativeFunction(Box::from(NativeFunctionDef {
                    name: k.to_string(),
                    callback: v,
                })),
            )
        })
        .collect()
}
