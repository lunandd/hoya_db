#[derive(Debug, Clone)]
pub enum Expr {
    Number(isize),
    Float(f64),
    Boolean(bool),
    Text(String),
    List(Vec<Expr>),
    Identifier(String),
    Call(Box<Expr>, Vec<Expr>),
    Unit(()),
}

// TODO: Function definitions, if statements, variable arguments, quoting, comments
