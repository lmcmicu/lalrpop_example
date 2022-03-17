#[macro_use]
extern crate lalrpop_util;

mod ast;

lalrpop_mod!(pub structure);

fn main() {
    let leif = structure::ExpressionParser::new()
        .parse(r"foo(bar, jar, mar.lar, scar=tar, hor(odecki), /bin/d, s/foo/var/sde)")
        .unwrap();
    eprintln!("PARSED:\n{:#?}", leif);
}
