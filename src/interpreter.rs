use std::{collections::HashMap, vec::IntoIter};

use crate::{ast::*, environment::Value::*, environment::*, stdlib, token::TokenKind::*};

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

    pub fn run(&mut self) -> Result<(), String> {
        while let Some(statement) = self.ast.next() {
            self.run_statement(statement)?;
        }
        Ok(())
    }

    pub fn run_statement(&mut self, statement: Statement) -> Result<Option<Value>, String> {
        match statement {
            Statement::Expression(expr) => {
                self.run_expression(expr)?;
            }
            Statement::VarDeclaration { name, value } => {
                self.run_var(&name, value)?;
            }
            Statement::FunctionDeclaration { name, params, body } => {
                self.globals.insert(
                    name.clone(),
                    Value::Function(Box::from(FunctionDef {
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                    })),
                );
            }
            Statement::While { condition, body } => loop {
                if let Value::Bool(cond) = self.run_expression(condition.clone())? {
                    if cond == false {
                        break;
                    }

                    for statement in &body {
                        self.run_statement(statement.clone())?;
                    }
                }
            },
            Statement::Return(expr) => {
                return Ok(Some(self.run_expression(expr)?));
            }
        }
        Ok(Option::None)
    }

    pub fn run_expression(&mut self, expression: Expression) -> Result<Value, String> {
        let res = match expression {
            // Literals
            Expression::Number(v) => Number(Box::from(v)),
            Expression::Str(v) => Str(v),
            Expression::Bool(v) => Bool(v),
            // Binary Operation
            Expression::BinOp(left, op, right) => {
                let left = self.run_expression(*left)?;
                let right = self.run_expression(*right)?;

                match (&left, &op, &right) {
                    (Number(l), Plus, Number(r)) => Number(Box::from(**l + **r)),
                    (Number(l), Mul, Number(r)) => Number(Box::from(**l * **r)),
                    (Number(l), Slash, Number(r)) => Number(Box::from(**l / **r)),
                    (Number(l), Minus, Number(r)) => Number(Box::from(**l + **r)),
                    (Number(l), Equals, Number(r)) => Bool(*l == *r),
                    (Number(l), NotEquals, Number(r)) => Bool(l != r),
                    (Number(l), LessThan, Number(r)) => Bool(l < r),
                    (Number(l), GreaterThan, Number(r)) => Bool(l > r),
                    (Number(l), LessEquals, Number(r)) => Bool(l <= r),
                    (Number(l), GreaterEquals, Number(r)) => Bool(l >= r),

                    (Bool(l), Equals, Bool(r)) => Bool(l == r),
                    (Bool(l), NotEquals, Bool(r)) => Bool(l != r),
                    _ => {
                        return Err(format!(
                            "Unsupported operation {:?} between {:?} and {:?}",
                            op, left, right
                        ));
                    }
                }
            }
            Expression::Call { name, args } => {
                let mut processed_args = Vec::new();

                for arg in args.into_iter() {
                    processed_args.push(self.run_expression(arg)?);
                }

                self.run_function(&name, processed_args)?
            }
            Expression::Identifier(name) => {
                if let Some(val) = self.globals.get(&name) {
                    val.clone()
                } else {
                    return Err(format!("Identifier {} does not exist", name));
                }
            }
            Expression::Nil => Value::Nil,
            Expression::Assign(name, value) => self.run_assign(*name, *value)?,
        };
        return Ok(res);
    }

    fn run_function(&mut self, name: &String, args: Vec<Value>) -> Result<Value, String> {
        if self.globals.contains_key(name) {
            let v = self.globals[name].clone();
            match v {
                Value::NativeFunction(f) => {
                    if args.len() < 1 { 
                        return Err(format!("No arguments provided to function {name}"));
                    }
                    let retval = (f.callback)(args, self);
                    return Ok(retval);
                }
                Value::Function(f) => {
                    if f.params.len() != args.len() {
                        return Err(format!(
                            "Arguments of length {} don't match paramters of length {}",
                            args.len(),
                            f.params.len()
                        ));
                    }

                    for i in 0..f.params.len() {
                        self.globals.insert(f.params[i].clone(), args[i].clone());
                    }

                    for statement in f.body.iter() {
                        if let Some(retval) = self.run_statement(statement.clone())? {
                            return Ok(retval);
                        }
                    }

                    return Ok(Value::Nil);
                }
                _ => Err(format!("{} is not a function", name)),
            }
        } else {
            Err(format!("{} is not defined", name))
        }
    }

    fn run_var(&mut self, name: &String, value: Expression) -> Result<(), String> {
        let right = self.run_expression(value)?;
        self.globals.insert(name.clone(), right);
        Ok(())
    }

    fn run_assign(&mut self, name: Expression, value: Expression) -> Result<Value, String> {
        let new_value = self.run_expression(value)?;

        if let Expression::Identifier(name) = &name {
            match self.globals.get(name) {
                Some(_old_value) => {
                    self.globals.insert(name.clone(), new_value);
                    return Ok(Value::Nil);
                }
                None => {
                    return Err(format!("Identifier {} hasn't been declared", name));
                }
            }
        }

        Err(format!("{:?} is not an identifier", name))
    }
}
