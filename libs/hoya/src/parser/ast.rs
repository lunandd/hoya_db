#[derive(Debug, Clone)]
pub enum Ast {
    // Expressions
    Number(isize),
    Float(f64),
    Boolean(bool),
    Text(String),
    List(Vec<Ast>),
    Identifier(String),
    Call(Box<Ast>, Vec<Ast>),
    Unit(()),
    // Statements
}
