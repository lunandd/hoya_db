use crate::parser::ast::Expr;
use combine::error::ParseError;
use combine::parser::char::{char, digit, letter, spaces, string, tab};
use combine::parser::EasyParser;
use combine::stream::{position, Stream};
use combine::{
    attempt, between, many1, optional, satisfy, sep_by, sep_by1, skip_many, token, Parser,
};

fn whitespace<Input>() -> impl Parser<Input>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().or(skip_many(tab()))
}

fn lex_char<Input>(c: char) -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let skip_spaces = || whitespace().silent();

    char(c).skip(skip_spaces())
}

fn int<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (optional(char('-')), many1(digit())).map(|(sign, digits): (Option<char>, String)| {
        Expr::Number(if sign.is_some() {
            -digits.parse::<isize>().unwrap()
        } else {
            digits.parse::<isize>().unwrap()
        })
    })
}

fn float<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        optional(char('-')),
        many1(digit()),
        char('.'),
        many1(digit()),
    )
        .map(
            |(sign, first, _, second): (Option<char>, String, char, String)| {
                Expr::Float(if sign.is_some() {
                    -format!("{first}.{second}").parse::<f64>().unwrap()
                } else {
                    format!("{first}.{second}").parse::<f64>().unwrap()
                })
            },
        )
}

fn bool<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    string("true")
        .or(string("false"))
        .map(|chosen| Expr::Boolean(chosen == "true"))
}

fn text<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(
        token('\"'),
        token('\"'),
        many1(satisfy(|c| c != '"' && c != '\'')),
    )
    .map(Expr::Text)
}

fn list_<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let comma_list = sep_by(expr(), lex_char(','));

    between(lex_char('['), lex_char(']'), comma_list).map(Expr::List)
}

fn name<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(letter().or(digit())).map(Expr::Identifier)
}

fn identifier<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (lex_char('('), name(), lex_char(')')).map(|(_, n, _)| n)
}

fn call<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let expr_list = sep_by1(expr(), whitespace());

    (
        lex_char('('),
        name(),
        whitespace(),
        expr_list,
        lex_char(')'),
    )
        .map(|(_, name, _, args, _)| Expr::Call(Box::new(name), args))
}

pub fn parse(
    code: &str,
) -> Result<
    (Expr, position::Stream<&str, position::SourcePosition>),
    combine::easy::Errors<char, &str, position::SourcePosition>,
> {
    expr().easy_parse(position::Stream::new(code))
}

parser! {
    pub fn list[Input]()(Input) -> Expr
    where [Input: Stream<Token = char>]
    {
        list_()
    }
}

parser! {
    pub fn expr[Input]()(Input) -> Expr
    where [Input: Stream<Token = char>]
    {
        choice!(attempt(bool()), attempt(float()),attempt(int()), attempt(text()), attempt(list()), attempt(call()), attempt(identifier()))
    }
}
