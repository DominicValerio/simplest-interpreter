use std::{vec::IntoIter};

use crate::{ast::*, object::*, object::Object::*, environment::Environment, stdlib, token::TokenKind as tk};

#[derive(Debug, Clone)]
pub struct Interpreter {
    ast: IntoIter<Statement>,
    pub env: Environment,
    pub stdout: String,
}

impl Interpreter {
    pub fn new(ast: Program) -> Self {
        Self {
            ast: ast.into_iter(),
            env: Environment::from(stdlib::get_lib()),
            stdout: String::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        while let Some(statement) = self.ast.next() {
            self.run_statement(statement)?;
        }
        Ok(())
    }

    fn run_statement(&mut self, statement: Statement) -> Result<Option<Object>, String> {
        match statement {
            Statement::Expression(expr) => {
                self.run_expression(expr)?;
            }
            Statement::VarDeclaration { name, value } => {
                self.run_var(&name, value)?;
            }
            Statement::FunctionDeclaration { name, params, body } => {
                self.env.insert(
                    name.clone(),
                    Object::Function(Box::from(FunctionDef {
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                    })),
                );
            }
            Statement::While { condition, body } => {
                self.env.enter_scope();
                loop {
                    if let Object::Bool(cond) = self.run_expression(condition.clone())? {
                        if cond == false {
                            self.env.exit_scope();
                            return Ok(Option::None);
                        }

                        for v in &body {
                            if let Some(retval) = self.run_statement(v.clone())? {
                                self.env.exit_scope();
                                return Ok(Some(retval));
                            }
                        }
    
                    } else {
                        return Err("Expression after while isn't a boolean".to_string());
                    }
                }
            },
            Statement::Return(expr) => {
                return Ok(Some(self.run_expression(expr)?));
            }
        }
        Ok(Option::None)
    }

    fn run_expression(&mut self, expression: Expression) -> Result<Object, String> {
        let res = match expression {
            // Literals
            Expression::Number(v) => Number(v),
            Expression::Str(v) => Str(v),
            Expression::Bool(v) => Bool(v),
            //Binary Operation
            Expression::BinOp(left, op, right) => {
                let left = self.run_expression(*left)?;
                let right = self.run_expression(*right)?;

                use tk::*;
                match (&left, &op, &right) {

                    (Number(l), Plus, Number(r)) => Number(l + r),
                    (Number(l), Minus, Number(r)) => Number(l - r),
                    (Number(l), Equals, Number(r)) => Bool(l == r),
                    (Number(l), LessThan, Number(r)) => Bool(l < r),
                    (Bool(l), Equals, Bool(r)) => Bool(l == r),
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
                if let Some(val) = self.env.get(&name) {
                    val.clone()
                } else {
                    //dbg!(&self.env);
                    return Err(format!("Identifier {} does not exist", name));
                }
            }
            Expression::Assign(name, value) => self.run_assign(*name, *value)?,
        };
        return Ok(res);
    }

    fn run_function(&mut self, name: &String, args: Vec<Object>) -> Result<Object, String> {
        if self.env.contains(name) {
            let v = self.env.get(name).unwrap().clone();
            match v {
                Object::NativeFunction(f) => {
                    if args.len() < 1 { 
                        return Err(format!("No arguments provided to function {name}"));
                    }
                    let retval = (f.callback)(args, self);
                    return Ok(retval);
                }
                Object::Function(f) => {
                    if f.params.len() != args.len() {
                        return Err(format!(
                            "Arguments of length {} don't match paramters of length {}",
                            args.len(),
                            f.params.len()
                        ));
                    }

                    self.env.enter_scope();

                    for i in 0..f.params.len() {
                        self.env.insert(f.params[i].clone(), args[i].clone());
                    }

                    let mut retval = Object::Unit;

                    for v in f.body {
                        if let Some(_retval) = self.run_statement(v)? {
                            retval = _retval;
                            break;
                        }
                    }

                    self.env.exit_scope();

                    return Ok(retval);
                }
                _ => Err(format!("{} is not a function", name)),
            }
        } else {
            Err(format!("{} is not defined", name))
        }
    }

    fn run_var(&mut self, name: &String, value: Expression) -> Result<(), String> {
        let right = self.run_expression(value)?;
        self.env.insert(name.clone(), right);
        Ok(())
    }

    fn run_assign(&mut self, name: Expression, value: Expression) -> Result<Object, String> {
        let new_value = self.run_expression(value)?;

        if let Expression::Identifier(name) = &name {
            match self.env.get(name) {
                Some(_old_value) => {
                    self.env.insert(name.clone(), new_value);
                    return Ok(Object::Unit);
                }
                None => {
                    return Err(format!("Identifier {} hasn't been declared", name));
                }
            }
        }

        Err(format!("{:?} is not an identifier", name))
    }
}
