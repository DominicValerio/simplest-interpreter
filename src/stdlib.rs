use crate::interpreter::Interpreter;
use std::collections::HashMap;
use crate::environment::Value;


macro_rules! register {
  ($name_str: expr, $callback: ident, $map: ident) => {
    $map.insert($name_str.to_string(), Value::NativeFunction{name: $name_str.to_string(), callback: $callback});
  };
}

fn print(args: Vec<Value>, i: &mut Interpreter) -> Value {
  let mut output = String::default();
  for v in args.into_iter() {
    output.push_str(&format!("{}", v).to_string());
  }
  i.stdout.push_str(&output);
  print!("{}", output);

  return Value::Nil;
}

fn println(args: Vec<Value>, i: &mut Interpreter) -> Value {
  let mut args = args.clone();
  args.push(Value::Str("\n".to_string()));
  return print(args, i);
}

pub fn get_lib() -> HashMap<String, Value> {
  let mut map = HashMap::new();
  
  register!("print", print, map);
  register!("println", println, map);

  return map;
}

  

