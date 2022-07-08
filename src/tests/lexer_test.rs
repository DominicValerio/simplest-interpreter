use crate::lexer::*;

#[test]
fn cols_linenums() {
    let src = "1 3 5  8\n\r1 3 5  8";
    let mut l = Lexer::new(src);
    let res = l.parse();
    dbg!(&res);
    let it = res.iter();
}

#[test]
fn test_easy() {
    let src = " + -     /\n++";
    let mut l = Lexer::new(src);
    let res = l.parse();
    dbg!(&res);
    let it = res.iter();
}

#[test]
fn string() {
    let src = r#"bobby boi   "#;
    let mut l = Lexer::new(src);
    let res = l.parse();
    dbg!(&res);
    let it = res.iter();
}

#[test]
fn numbers() {
    let src = r#"3.0    69 0"#;
    let mut l = Lexer::new(src);
    let res = l.parse();
    dbg!(&res);
    let it = res.iter();
}

#[test]
fn combined_ops() {
    let src = r#"= == < <= > >= ! !="#;
    let mut l = Lexer::new(src);
    let res = l.parse();
    dbg!(&res);
    let it = res.iter();
}