use crate::parser::{
    ast::{Expr, Identifier, RangeInterval},
    parse_comments::comment,
    parse_ident::parse_ident,
    parse_scope::parse_scope,
    parse_var_types::{parse_as_variable, parse_var_expr},
    tokens::{Span, COMMA, FOREACH, IN, L_PAREN, R_PAREN},
    tools::get_interval,
};
use nom::{
    branch::alt, bytes::complete::tag, combinator::opt, error::ParseError, sequence::preceded, *,
};

fn pars_args<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E> {
    let (s, _) = preceded(comment, tag(COMMA))(s)?;
    let (s, ident) = parse_ident(s)?;
    Ok((s, ident))
}

pub fn parse_for<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, _) = preceded(comment, tag(FOREACH))(s)?;
    let (s, start) = get_interval(s)?;

    let (s, _) = preceded(comment, tag(L_PAREN))(s)?;
    let (s, ident) = parse_ident(s)?;
    let (s, opt) = opt(pars_args)(s)?;
    let (s, _) = preceded(comment, tag(R_PAREN))(s)?;

    let (s, _) = preceded(comment, tag(IN))(s)?;
    let (s, expr) = alt((parse_as_variable, parse_var_expr))(s)?;
    let (s, block) = parse_scope(s)?;

    let (s, end) = get_interval(s)?;

    Ok((
        s,
        Expr::ForExpr(
            ident,
            opt,
            Box::new(expr),
            block,
            RangeInterval { start, end },
        ),
    ))
}
