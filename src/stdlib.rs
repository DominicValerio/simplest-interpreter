use crate::environment::{Value, NativeFunctionCallback};
use std::collections::HashMap;

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
    let mut args = args.clone();
    args.push(Value::Str("\n".to_string()));
    return print(args, i);
};

pub fn get_lib() -> HashMap<String, Value> {
    [
        ("print", print),
        ("println", println),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), Value::NativeFunction {
        name: k.to_string(),
        callback: v,
    }))
    .collect()
   
}
