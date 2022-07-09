// Used to set the functions that are usable by the interpreter
#![allow(non_upper_case_globals)]
use crate::object::{NativeFunctionCallback, NativeFunctionDef, Object, Object::*};
use std::collections::HashMap;


const print: NativeFunctionCallback = |args, i| {
    let mut output = String::default();
    for v in args.into_iter() {
        output.push_str(format!("{v}").as_str());
    }
    i.stdout.push_str(&output);
    print!("{}", output);

    return Unit;
};

const println: NativeFunctionCallback = |args, i| {
    let ret = print(args, i);
    print(vec![Str("\n".to_string())], i);
    return ret;
};

pub fn get_lib() -> HashMap<String, Object> {
    [("print", print), ("println", println)]
        .into_iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                NativeFunction(Box::from(NativeFunctionDef {
                    name: k.to_string(),
                    callback: v,
                })),
            )
        })
        .collect()
}
