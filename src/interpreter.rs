//! interpreter based on Abstract Syntax Tree walking

use std::{fmt::Display, vec::IntoIter};

use crate::{
    ast::*, environment::Environment, object::Object::*, object::*, stdlib, token::Token,
    token::TokenKind as tk,
};

#[derive(Debug, Clone)]
pub struct Interpreter {
    curtok: Token,
    ast: IntoIter<AstNode>,
    pub env: Environment,
    pub stdout: String,
}

impl Interpreter {
    pub fn new(ast: Program) -> Self {
        Self {
            curtok: ast[0].1.clone(),
            ast: ast.into_iter(),
            env: Environment::from(stdlib::get_lib()),
            stdout: String::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        while let Some((statement, matching_token)) = self.ast.next() {
            self.curtok = matching_token;
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
            Statement::While { mut condition, body } => {
                let mut ret = None;
                self.env.enter_scope();

                'outer: loop {
                    if let Object::Bool(cond) = self.run_expression(condition.clone())? {

                        if cond == false {
                            break;
                        }

                        for v in &body {
                            if let Some(retval) = self.run_statement(v.clone())? {
                                ret = Some(retval);
                                break 'outer;
                            }
                        }

                    } else {
                        return Err(self.error("Expression after while isn't a boolean"));
                    }                    
                }
                self.env.exit_scope();
                return Ok(ret);
            }
            Statement::Return(expr) => {
                return Ok(Some(self.run_expression(expr)?));
            }
            Statement::Block(block) => {
                self.env.enter_scope();
                for statement in block {
                    self.run_statement(statement)?;
                }
                self.env.exit_scope();
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
                        return Err(self.error(format!(
                            "Unsupported operation {:?} between {} and {}",
                            op, left, right
                        )));
                    }
                }
            }
            Expression::Call { name, args } => {
                let mut processed_args = vec![];

                for arg in args.into_iter() {
                    processed_args.push(self.run_expression(arg)?);
                }

                self.run_function(&name, processed_args)?
            }
            Expression::Identifier(name) => {
                if let Some(val) = self.env.get(&name) {
                    val.clone()
                } else {
                    dbg!(&self.env);
                    return Err(self.error(format!("Identifier `{name}` does not exist")));
                }
            }
            Expression::Assign(name, value) => self.run_assign(*name, *value)?,
        };
        return Ok(res);
    }

    fn run_function(&mut self, name: &String, args: Vec<Object>) -> Result<Object, String> {
        if let Some(_) = self.env.contains(name) {
            let v = self.env.get(name).unwrap().clone();
            match v {
                Object::NativeFunction(f) => {
                    if args.len() < 1 {
                        return Err(
                            self.error(format!("No arguments provided to function `{name}`"))
                        );
                    }
                    let retval = (f.callback)(args, self);
                    return Ok(retval);
                }
                Object::Function(f) => {
                    if f.params.len() != args.len() {
                        return Err(self.error(format!(
                            "Arguments of length {} don't match parameters of length {}",
                            args.len(),
                            f.params.len()
                        )));
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
                _ => Err(self.error(format!("`{name}` is not a function"))),
            }
        } else {
            Err(self.error(format!("`{name}` is not defined")))
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
            match self.env.contains(name) {
                Some(index) => {
                    self.env.insert_at(name.clone(), new_value, index);
                    return Ok(Object::Unit);
                }
                None => {
                    return Err(self.error(format!("Identifier `{name}` hasn't been declared")));
                }
            }
        }

        Err(format!("`{name:?}` is not an identifier"))
    }

    fn error<S: Into<String> + Display>(&self, text: S) -> String {
        format!("(Ln {}, Col {}) {}", self.curtok.ln, self.curtok.col, text)
    }
}
