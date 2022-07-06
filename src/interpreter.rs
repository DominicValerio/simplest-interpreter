use std::collections::HashMap;
use std::vec::IntoIter;

use crate::token::*;
use crate::ast::*;
use crate::environment::*;
use crate::stdlib;

type ReturnVal = Value;

#[derive(Debug, Clone)]
pub struct Interpreter {
  ast: IntoIter<Statement>,
  pub globals: HashMap<String, Value>,
  pub stdout: String,
}

impl Interpreter {
  pub fn new(ast: Program) -> Self {
    Self {
      ast: ast.into_iter(),
      globals: stdlib::get_lib(),
      stdout: String::new(),
    }
  }

  pub fn run(&mut self) {
    while let Some(statement) = self.ast.next() {
      self.run_statement(&statement.clone());
    }
  }

  pub fn run_statement(&mut self, statement: &Statement) -> Option<ReturnVal> {
    match statement {
      Statement::Expression(expr) => {
        self.run_expression(expr);
      }
      Statement::VarDeclaration{name, value} => {
        self.run_var(name, value); 
      }
      Statement::FunctionDeclaration{name, params, body} => {
        self.globals.insert(name.clone(), Value::Function{
          name: name.clone(), params: params.clone(),body: body.clone()
        });
      }
      Statement::While{condition, body} => {
        loop {
          if let Value::Bool(cond) = self.run_expression(condition) {
            if cond == false { 
              break; 
            }

            for statement in body {
              self.run_statement(statement);
            }
          }
        }
      }
      Statement::Return(expr) => {
        return Some(self.run_expression(expr));
      }
    }
    Option::None
  }

  pub fn run_expression(&mut self, expression: &Expression) -> Value {
    use Value::*;
    use TokenKind::*;

    match expression {
      // Literals
      Expression::Int(v) => Int(*v),
      Expression::Str(v) => Str(v.clone()),
      Expression::Bool(v) => Bool(*v),
      // Binary Operation
      Expression::BinOp(left, op, right) => {
        let left = self.run_expression(&**left);
        let right = self.run_expression(&**right);

        match (left.clone(), op, right.clone()) {
          (Int(l), Plus, Int(r)) => return  Int(l + r),
          (Int(l), Mul, Int(r)) => return  Int(l * r),
          (Int(l), Slash, Int(r)) => return  Int(l / r),
          (Int(l), Minus, Int(r)) => return Int(l - r),
          (Int(l), Equals, Int(r)) => return Bool(l == r),
          (Int(l), NotEquals, Int(r)) => return Bool(l != r),
          (Int(l), LessThan, Int(r)) => return Bool(l < r),
          (Int(l), GreaterThan, Int(r)) => return Bool(l > r),
          (Int(l), LessEquals, Int(r)) => return Bool(l <= r),
          (Int(l), GreaterEquals, Int(r)) => return Bool(l >= r),

          (Bool(l), Equals, Bool(r)) => return Bool(l == r),
          (Bool(l), NotEquals, Bool(r)) => return Bool(l != r),
          _=> {
            eprintln!("Invalid/unsupported operation.");
            dbg!(left, op, right);
            unimplemented!();
          }
        }
      }
      Expression::Call{name, args} => {
        let mut processed_args: Vec<Value> = Vec::new();

        for arg in args.into_iter() {
            processed_args.push(self.run_expression(arg));
        }

        return self.run_function(name, processed_args);
      }
      Expression::Identifier(name) => {
        // TODO: wtf does this mean?
        if let Some(val) = self.globals.get(name) {
          return val.clone();
        } else {
          unreachable!();
        }
      }
      Expression::None => {
        Value::None
      }
      Expression::Assign(name, value) => {
        self.run_assign(name, value);
        Value::None
      }
    }
  }

  fn run_function(&mut self, name: &String, args: Vec<Value>) -> Value {
    if let Some(v) = self.globals.clone().get(name) {
      match v {
        Value::NativeFunction{callback, ..} => {
          let retval = callback(args, self);
          return retval;
        },
        Value::Function{params, body, ..} => {
          if params.len() != args.len() {
            eprintln!("Arguments of length {} don't match paramters of length {}", args.len(), params.len());
            unimplemented!();
          }

          for i in 0..params.len() {
            self.globals.insert(params[i].clone(), args[i].clone()); 
          }

          //dbg!(&self.globals)
          //let mut return_val: Option<Value> = None;

          for statement in body {
            let res = self.run_statement(statement);

            if let Some(ret_val) = res {
              return ret_val;
            }
          }

          return Value::None;
        }
        _=> unreachable!(),
      }
    } else {
      unimplemented!(); // function doesn't exist
    }
  }

  fn run_var(&mut self, name: &String, value: &Expression) {
    let res = self.run_expression(value);
    self.globals.insert(name.clone(), res);
  }

  fn run_assign(&mut self, name: &Expression, value: &Expression) -> Value {
    let new_value = self.run_expression(value);

    if let Expression::Identifier(name) = name {
      if let Some(_old_value) = self.globals.get(name) {
        self.globals.insert(name.clone(), new_value);
        return Value::None;
      } 
    }

    eprintln!("{:?} doesn't exist", name);
    unimplemented!();
  }
}



