use crate::environment::{NativeFunctionCallback, NativeFunctionDef, Value};
use std::{collections::HashMap, mem::size_of_val};

#[allow(non_upper_case_globals)]
const print: NativeFunctionCallback = |args, i| {
    let mut output = String::default();
    for v in args.into_iter() {
        output.push_str(&format!("{}", v).to_string());
    }
    i.stdout.push_str(&output);
    print!("{}", output);

    return Value::Nil;
};

#[allow(non_upper_case_globals)]
const println: NativeFunctionCallback = |args, i| {
    let ret = print(args, i);
    print(vec![Value::Str("\n".to_string())], i);
    return ret
};

#[allow(non_upper_case_globals)]
const size_of: NativeFunctionCallback = |args, i| {
    use Value::*;
    let size;

    match args[0].clone() {
        Nil => size = size_of_val(&args[0]),
        Str(s) => size = size_of_val(&s) + size_of_val(&*s),
        Bool(b) => size = size_of_val(&b),
        Number(v) => size = size_of_val(&v) + size_of_val(&*v), // size of the ReferenceCount + size of the actual value
        Function(f) => size = size_of_val(&f) + size_of_val(&*f),

        NativeFunction(f) => size = size_of_val(&f),
    }
    return Value::Number(Box::from(size as f64));
};

pub fn get_lib() -> HashMap<String, Value> {
    [
        ("print", print), 
        ("println", println), 
        ("size_of", size_of)
    ]
        .into_iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                Value::NativeFunction(Box::from(NativeFunctionDef {
                    name: k.to_string(),
                    callback: v,
                })),
            )
        })
        .collect()
}
