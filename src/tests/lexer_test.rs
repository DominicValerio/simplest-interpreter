use crate::lexer::*;

#[test]
fn cols_linenums() {
    let src = "1 3 5  8\n\r1 3 5  8";
    let l = Lexer::new(src);
    let res = l.parse();
    dbg!(&res);
    let it = res.iter();
}
