use crate::ast::*;
use crate::lexer::{PunctKind, QuartzToken};
use langbox::*;
use std::borrow::Cow;

trait TokenStreamEx {
    fn current_span(&self) -> TextSpan;
}

impl<'a, Kind> TokenStreamEx for TokenStream<'a, Kind> {
    fn current_span(&self) -> TextSpan {
        if let Some(current) = self.peek() {
            current.span
        } else {
            self.empty_span()
        }
    }
}

const KEYWORDS: phf::Map<&'static str, KeywordKind> = phf_macros::phf_map!(
    "mod" => KeywordKind::Mod,
    "struct" => KeywordKind::Struct,
    "enum" => KeywordKind::Enum,
    "fn" => KeywordKind::Fn,
    "inout" => KeywordKind::InOut,
    "in" => KeywordKind::In,
    "out" => KeywordKind::Out,
    "sig" => KeywordKind::Sig,
    "reg" => KeywordKind::Reg,
    "let" => KeywordKind::Let,
    "const" => KeywordKind::Const,
    "comb" => KeywordKind::Comb,
    "proc" => KeywordKind::Proc,
    "if" => KeywordKind::If,
    "else" => KeywordKind::Else,
    "match" => KeywordKind::Match,
    "while" => KeywordKind::While,
    "for" => KeywordKind::For,
    "rising" => KeywordKind::Rising,
    "falling" => KeywordKind::Falling,
    "as" => KeywordKind::As,
);

#[derive(Debug)]
pub struct QuartzParserErrror {
    pub message: Cow<'static, str>,
    pub span: TextSpan,
}

pub trait QuartzParser<T> = Parser<QuartzToken, T, QuartzParserErrror>;

macro_rules! err {
    ($msg:expr) => {
        |input| QuartzParserErrror {
            message: $msg.into(),
            span: input.current_span(),
        }
    };
}

fn punct<const N: usize>(punct: impl Into<[PunctKind; N]>) -> impl QuartzParser<Punct> {
    let punct = punct.into();

    parse_fn!(|input| {
        if let Some(token) = input.peek() {
            if let QuartzToken::Punct(found_punct) = &token.kind
              && punct.iter().find(|p| (*p).eq(found_punct)).is_some() {
                ParseResult::Match {
                    value: Punct::new(
                        *found_punct,
                        token.span,
                    ),
                    span: token.span,
                    remaining: input.advance(),
                }
            } else {
                ParseResult::NoMatch
            }
        } else {
            ParseResult::NoMatch
        }
    })
}

fn ident() -> impl QuartzParser<Ident> {
    parse_fn!(|input| {
        if let Some(token) = input.peek() {
            match &token.kind {
                QuartzToken::Ident(name) => {
                    if KEYWORDS.contains_key(name) {
                        ParseResult::Err(QuartzParserErrror {
                            message: format!(
                                "'{}' is a reserved keyword and cannot be used as an identifier",
                                name
                            )
                            .into(),
                            span: token.span,
                        })
                    } else {
                        ParseResult::Match {
                            value: Ident::new(name, token.span),
                            span: token.span,
                            remaining: input.advance(),
                        }
                    }
                }
                QuartzToken::InvalidIdent(name) => ParseResult::Match {
                    value: Ident::new(name, token.span),
                    span: token.span,
                    remaining: input.advance(),
                },
                _ => ParseResult::NoMatch,
            }
        } else {
            ParseResult::NoMatch
        }
    })
}

fn kw(kw: KeywordKind) -> impl QuartzParser<Keyword> {
    parse_fn!(|input| {
        if let Some(token) = input.peek() {
            match &token.kind {
                QuartzToken::Ident(name) => {
                    if let Some(found_kw) = KEYWORDS.get(name) && kw.eq(found_kw) {
                        ParseResult::Match {
                            value: Keyword::new(kw, token.span),
                            span: token.span,
                            remaining: input.advance(),
                        }
                    } else {
                        ParseResult::NoMatch
                    }
                }
                _ => ParseResult::NoMatch,
            }
        } else {
            ParseResult::NoMatch
        }
    })
}

fn literal() -> impl QuartzParser<Literal> {
    parse_fn!(|input| {
        if let Some(token) = input.peek() {
            match &token.kind {
                QuartzToken::Literal(value) => ParseResult::Match {
                    value: Literal::new(*value, token.span),
                    span: token.span,
                    remaining: input.advance(),
                },
                QuartzToken::InvalidLiteral(_) => ParseResult::Match {
                    value: Literal::new(0, token.span),
                    span: token.span,
                    remaining: input.advance(),
                },
                _ => ParseResult::NoMatch,
            }
        } else {
            ParseResult::NoMatch
        }
    })
}

fn path() -> impl QuartzParser<Path> {
    let segment = parser!(
        (
            (
                {punct(PunctKind::Period)}->[|punct| PathSeparator::new(PathSeparatorKind::Period, punct.span())]
                <|> {punct(PunctKind::DoubleColon)}->[|punct| PathSeparator::new(PathSeparatorKind::DoubleColon, punct.span())]
            )
            <.> {ident()}!![err!("expected identifier")]
        )
        ->[|(sep, ident)| PathSegment::new(sep, ident)]
    );

    parser!(({ident()} <.> *segment)->[|(head, tail)| Path::new(head, tail)])
}

fn paren_expr(simple: bool) -> impl QuartzParser<Expr> {
    parser!(
        (
            {punct(PunctKind::OpenParen)}
            <.> {expr(simple)}
            <.> {punct(PunctKind::CloseParen)}!![err!("expected `)`")]
        )
        ->[|((open_paren, expr), close_paren)| Expr::Paren(
            ParenExpr::new(open_paren, expr, close_paren)
        )]
    )
}

fn call_expr(simple: bool) -> impl QuartzParser<Expr> {
    parser!(
        {sequence!(
            ident(),
            punct(PunctKind::OpenParen),
            sep_by(expr(simple), punct(PunctKind::Comma), true, true),
            parser!({punct(PunctKind::CloseParen)}!![err!("expected `)`")])
        )}
        ->[|(func, open_paren, args, close_paren)| Expr::Call(
            CallExpr::new(func, open_paren, args, close_paren)
        )]
    )
}

fn construct_expr() -> impl QuartzParser<Expr> {
    let field = parser!(
        ({ident()} <.> {punct(PunctKind::Colon)}!![err!("expected `:`")] <.> {expr(false)}!![err!("expected expression")])
        ->[|((field, sep), value)| FieldAssign::new(field, sep, value)]
    );

    parser!(
        {sequence!(
            ty(),
            punct(PunctKind::OpenCurl),
            sep_by(field, punct(PunctKind::Comma), true, true),
            parser!({punct(PunctKind::CloseCurl)}!![err!("expected `}`")])
        )}
        ->[|(ty, open_curl, fields, close_curl)| Expr::Construct(
            ConstructExpr::new(ty, open_curl, fields, close_curl)
        )]
    )
}

fn if_expr() -> impl QuartzParser<Expr> {
    let else_if_block = parser!(
        {sequence!(
            kw(KeywordKind::Else),
            kw(KeywordKind::If),
            parser!({expr(true)}!![err!("expected expression")]),
            parser!({block()}!![err!("expected block")]),
        )}
        ->[|(else_kw, if_kw, cond, body)| ElseIfBlock::new(else_kw, if_kw, cond, body)]
    );

    let else_block = parser!(
        ({kw(KeywordKind::Else)} <.> {block()}!![err!("expected block")])
        ->[|(else_kw, body)| ElseBlock::new(else_kw, body)]
    );

    parser!(
        {sequence!(
            kw(KeywordKind::If),
            parser!({expr(true)}!![err!("expected expression")]),
            parser!({block()}!![err!("expected block")]),
            parser!(*else_if_block),
            parser!(?else_block),
        )}
        ->[|(if_kw, cond, body, else_if_blocks, else_block)| Expr::If(
            IfExpr::new(if_kw, cond, body, else_if_blocks, else_block)
        )]
    )
}

fn match_expr() -> impl QuartzParser<Expr> {
    let pattern = choice!(
        parser!({literal()}->[MatchPattern::Literal]),
        parser!({path()}->[MatchPattern::Path]),
    );

    let body = choice!(
        parser!({expr(false)}->[MatchBody::Expr]),
        parser!({block()}->[MatchBody::Block]),
    );

    let branch = parser!(
        (
            {sep_by(pattern, punct(PunctKind::Or), false, false)}
            <.> {punct(PunctKind::FatRightArrow)}!![err!("expected `=>`")]
            <.> body!![err!("expected expression or block")]
        )
        ->[|((patterns, arrow), body)| MatchBranch::new(patterns, arrow, body)]
    );

    parser!(
        {sequence!(
            kw(KeywordKind::Match),
            parser!({expr(true)}!![err!("expected expression")]),
            parser!({punct(PunctKind::OpenCurl)}!![err!("expected `{`")]),
            sep_by(branch, punct(PunctKind::Comma), true, true),
            parser!({punct(PunctKind::CloseCurl)}!![err!("expected `}`")]),
        )}
        ->[|(match_kw, value, open_curl, branches, close_curl)| Expr::Match(
            MatchExpr::new(match_kw, value, open_curl, branches, close_curl)
        )]
    )
}

fn leaf_expr() -> impl QuartzParser<Expr> {
    choice!(
        parser!({literal()}->[Expr::Literal]),
        if_expr(),
        match_expr(),
        call_expr(false),
        construct_expr(),
        parser!({path()}->[Expr::Path]),
        paren_expr(false),
    )
}

fn leaf_expr_simple() -> impl QuartzParser<Expr> {
    choice!(
        parser!({literal()}->[Expr::Literal]),
        call_expr(true),
        parser!({path()}->[Expr::Path]),
        paren_expr(true),
    )
}

fn indexer() -> impl QuartzParser<Indexer> {
    let range_end = parser!(
        {punct(PunctKind::DoublePeriod)}
        .> {expr(true)}!![err!("expected expression")]
    );

    parser!(
        (
            {punct(PunctKind::OpenBracket)}
            <.> {expr(true)}!![err!("expected expression")]
            <.> ?range_end
            <.> {punct(PunctKind::CloseBracket)}!![err!("expected `]`")]
        )
        ->[|(((open_paren, start), end), close_paren)| Indexer::new(
            open_paren,
            if let Some(end) = end {
                IndexKind::Range(start..end)
            } else {
                IndexKind::Single(start)
            },
            close_paren,
        )]
    )
}

fn concat_unary_suffix_exprs(base: Expr, indexers: Vec<Indexer>) -> Expr {
    let mut result = base;
    for indexer in indexers.into_iter() {
        result = Expr::Index(IndexExpr::new(result, indexer));
    }
    result
}

fn unary_suffix_expr(simple: bool) -> impl QuartzParser<Expr> {
    parse_fn!(|input| {
        let leaf_expr_result = if simple {
            leaf_expr_simple().run(input)?
        } else {
            leaf_expr().run(input)?
        };

        match leaf_expr_result {
            InfallibleParseResult::Match {
                value: base,
                span: s1,
                remaining,
            } => match parser!(*{ indexer() }).run(remaining)? {
                InfallibleParseResult::Match {
                    value: indexers,
                    span: s2,
                    remaining,
                } => ParseResult::Match {
                    value: concat_unary_suffix_exprs(base, indexers),
                    span: s1.join(&s2),
                    remaining,
                },
                InfallibleParseResult::NoMatch => ParseResult::NoMatch,
            },
            InfallibleParseResult::NoMatch => ParseResult::NoMatch,
        }
    })
}

fn concat_unary_prefix_exprs(ops: (Vec<Punct>, Expr)) -> Expr {
    let mut result = ops.1;
    for punct in ops.0.into_iter().rev() {
        macro_rules! expr {
            ($variant:ident) => {
                Expr::$variant(UnaryExpr::new(punct, result))
            };
        }

        result = match punct.kind() {
            PunctKind::Add => expr!(Pos),
            PunctKind::Sub => expr!(Neg),
            PunctKind::Not => expr!(Not),
            _ => unreachable!("invalid unary operator"),
        };
    }
    result
}

fn unary_prefix_expr(simple: bool) -> impl QuartzParser<Expr> {
    let op = punct([PunctKind::Add, PunctKind::Sub, PunctKind::Not]);
    parser!((*op <.> {unary_suffix_expr(simple)})->[concat_unary_prefix_exprs])
}

fn concat_cast_exprs(value: Expr, targets: Vec<(Keyword, Type)>) -> Expr {
    let mut result = value;
    for (as_kw, target_ty) in targets.into_iter() {
        result = Expr::Cast(CastExpr::new(result, as_kw, target_ty));
    }
    result
}

fn cast_expr(simple: bool) -> impl QuartzParser<Expr> {
    parse_fn!(|input| {
        match unary_prefix_expr(simple).run(input)? {
            InfallibleParseResult::Match {
                value,
                span: s1,
                remaining,
            } => {
                let tail = parser!({kw(KeywordKind::As)} <.> {ty()});
                match parser!(*tail).run(remaining)? {
                    InfallibleParseResult::Match {
                        value: targets,
                        span: s2,
                        remaining,
                    } => ParseResult::Match {
                        value: concat_cast_exprs(value, targets),
                        span: s1.join(&s2),
                        remaining,
                    },
                    InfallibleParseResult::NoMatch => ParseResult::NoMatch,
                }
            }
            InfallibleParseResult::NoMatch => ParseResult::NoMatch,
        }
    })
}

fn concat_binary_exprs(terms: (Expr, Vec<(Punct, Expr)>)) -> Expr {
    let mut result = terms.0;
    for (punct, expr) in terms.1.into_iter() {
        macro_rules! expr {
            ($variant:ident) => {
                Expr::$variant(BinaryExpr::new(result, punct, expr))
            };
        }

        result = match punct.kind() {
            PunctKind::AtSymbol => expr!(Concat),
            PunctKind::SingleQuote => todo!(),
            PunctKind::Lt => expr!(Lt),
            PunctKind::Lte => expr!(Lte),
            PunctKind::Gt => expr!(Gt),
            PunctKind::Gte => expr!(Gte),
            PunctKind::Slt => expr!(Slt),
            PunctKind::Slte => expr!(Slte),
            PunctKind::Sgt => expr!(Sgt),
            PunctKind::Sgte => expr!(Sgte),
            PunctKind::Eq => expr!(Eq),
            PunctKind::Ne => expr!(Ne),
            PunctKind::Add => expr!(Add),
            PunctKind::Sub => expr!(Sub),
            PunctKind::Mul => expr!(Mul),
            PunctKind::Div => expr!(Div),
            PunctKind::Rem => expr!(Rem),
            PunctKind::And => expr!(And),
            PunctKind::Or => expr!(Or),
            PunctKind::Xor => expr!(Xor),
            PunctKind::Shl => expr!(Shl),
            PunctKind::Asr => expr!(Asr),
            PunctKind::Lsr => expr!(Lsr),
            _ => unreachable!("invalid binary operator"),
        };
    }
    result
}

fn binary_expr<const N: usize>(
    term: impl QuartzParser<Expr>,
    ops: impl Into<[PunctKind; N]>,
) -> impl QuartzParser<Expr> {
    let tail = parser!({punct(ops)} <.> term);
    parser!((term <.> *tail)->[concat_binary_exprs])
}

macro_rules! def_binary_expr {
    ($term:ident => $name:ident: $($op:ident),+) => {
        #[inline]
        fn $name(simple: bool) -> impl QuartzParser<Expr> {
            binary_expr($term(simple), [$(PunctKind::$op),+])
        }
    };
}

def_binary_expr!(cast_expr => concat_expr: AtSymbol);
def_binary_expr!(concat_expr => mul_expr: Mul, Div, Rem);
def_binary_expr!(mul_expr => add_expr: Add, Sub);
def_binary_expr!(add_expr => shift_expr: Shl, Lsr, Asr);
def_binary_expr!(shift_expr => cmp_expr: Lt, Lte, Gt, Gte, Slt, Slte, Sgt, Sgte);
def_binary_expr!(cmp_expr => eq_expr: Eq, Ne);
def_binary_expr!(eq_expr => and_expr: And);
def_binary_expr!(and_expr => xor_expr: Xor);
def_binary_expr!(xor_expr => or_expr: Or);

fn expr(simple: bool) -> impl QuartzParser<Expr> {
    or_expr(simple)
}

fn decl() -> impl QuartzParser<Declaration> {
    parser!(
        {sequence!(
            kw(KeywordKind::Let),
            parser!({ident()}!![err!("expected identifier")]),
            parser!({punct(PunctKind::Assign)}!![err!("expected `=`")]),
            parser!({expr(false)}!![err!("expected expression")]),
            parser!({punct(PunctKind::Semicolon)}!![err!("expected `;`")]),
        )}
        ->[|(let_kw, name, assign, value, _)| Declaration::new(let_kw, name, assign, value)]
    )
}

fn punct_to_assign_op(punct: Punct) -> AssignOp {
    let kind = match punct.kind() {
        PunctKind::Assign => AssignKind::Assign,
        PunctKind::AddAssign => AssignKind::AddAssign,
        PunctKind::SubAssign => AssignKind::SubAssign,
        PunctKind::MulAssign => AssignKind::MulAssign,
        PunctKind::DivAssign => AssignKind::DivAssign,
        PunctKind::RemAssign => AssignKind::RemAssign,
        PunctKind::AndAssign => AssignKind::AndAssign,
        PunctKind::OrAssign => AssignKind::OrAssign,
        PunctKind::XorAssign => AssignKind::XorAssign,
        PunctKind::ShlAssign => AssignKind::ShlAssign,
        PunctKind::AsrAssign => AssignKind::AsrAssign,
        PunctKind::LsrAssign => AssignKind::LsrAssign,
        _ => unreachable!("invalid assignment operator"),
    };

    AssignOp::new(kind, punct.span())
}

fn assign() -> impl QuartzParser<Assignment> {
    let target = parser!(
        ({path()} <.> *{indexer()})
        ->[|(path, indexers)| AssignTarget::new(path, indexers)]
    );

    let op = parser!({punct([
        PunctKind::Assign,
        PunctKind::AddAssign,
        PunctKind::SubAssign,
        PunctKind::MulAssign,
        PunctKind::DivAssign,
        PunctKind::RemAssign,
        PunctKind::AndAssign,
        PunctKind::OrAssign,
        PunctKind::XorAssign,
        PunctKind::ShlAssign,
        PunctKind::AsrAssign,
        PunctKind::LsrAssign,
    ])}->[punct_to_assign_op]);

    parser!(
        (target <.> op <.> {expr(false)}!![err!("expected expression")] <. {punct(PunctKind::Semicolon)}!![err!("expected `;`")])
        ->[|((target, op), value)| Assignment::new(target, op, value)]
    )
}

fn while_loop() -> impl QuartzParser<WhileLoop> {
    parser!(
        ({kw(KeywordKind::While)} <.> {expr(true)}!![err!("expected expression")] <.> {block()}!![err!("expected block")])
        ->[|((while_kw, condition), body)| WhileLoop::new(while_kw, condition, body)]
    )
}

fn for_loop() -> impl QuartzParser<ForLoop> {
    parser!(
        {sequence!(
            kw(KeywordKind::For),
            parser!({ident()}!![err!("expected identifier")]),
            parser!({kw(KeywordKind::In)}!![err!("expected `in`")]),
            parser!({expr(true)}!![err!("expected expression")]),
            parser!({punct(PunctKind::DoublePeriod)}!![err!("expected `..`")]),
            parser!({expr(true)}!![err!("expected expression")]),
            parser!({block()}!![err!("expected block")]),
        )}
        ->[|(for_kw, item_name, in_kw, start, _, end, body)| ForLoop::new(for_kw, item_name, in_kw, start..end, body)]
    )
}

fn statement() -> impl QuartzParser<Statement> {
    choice!(
        parser!({if_expr()}->[Statement::Expr]),
        parser!({match_expr()}->[Statement::Expr]),
        parser!({decl()}->[Statement::Declaration]),
        parser!({while_loop()}->[Statement::WhileLoop]),
        parser!({for_loop()}->[Statement::ForLoop]),
        parser!({block()}->[|block| Statement::Expr(Expr::Block(Box::new(block)))]),
        parser!({expr(false)}->[Statement::Expr] <. {punct(PunctKind::Semicolon)}),
        parser!({assign()}->[Statement::Assignment]),
    )
}

fn block() -> impl QuartzParser<Block> {
    parser!(
        {sequence!(
            punct(PunctKind::OpenCurl),
            sep_by(statement(), parser!(*{punct(PunctKind::Semicolon)}), true, false),
            parser!(*{punct(PunctKind::Semicolon)}),
            parser!(?{expr(false)}),
            parser!({punct(PunctKind::CloseCurl)}!![err!("expected `}`")]),
        )}
        ->[|(open_curl, mut statements, trailing_semis, last_expr, close_curl)| {
            if let Some(last_expr) = last_expr {
                Block::new(open_curl, statements, Some(last_expr), close_curl)
            } else {
                if let Some(Statement::Expr(_)) = statements.last() && (trailing_semis.len() == 0) {
                    if let Some(Statement::Expr(last_expr)) = statements.pop() {
                        Block::new(open_curl, statements, Some(last_expr), close_curl)
                    } else {
                        unreachable!()
                    }
                } else {
                    Block::new(open_curl, statements, None, close_curl)
                }
            }
        }]
    )
}

fn named_ty() -> impl QuartzParser<NamedType> {
    let generic_arg = parser!(
        {literal()}->[GenericTypeArg::Literal]
        <|> {path()}->[GenericTypeArg::Path]
        <|> (
            {punct(PunctKind::OpenCurl)}
            .> {expr(true)}!![err!("expected expression")]
            <. {punct(PunctKind::CloseCurl)}!![err!("expected `}`")]
        )->[GenericTypeArg::Expr]
    );

    let generic_args = parser!(
        {sequence!(
            punct(PunctKind::Lt),
            parser!({sep_by(generic_arg, punct(PunctKind::Comma), false, true)}!![err!("expected generic arguments")]),
            parser!({punct(PunctKind::Gt)}!![err!("expected `>`")]),
        )}
        ->[|(open_paren, exprs, close_paren)| GenericTypeArgs::new(open_paren, exprs, close_paren)]
    );

    parser!(
        ({ident()} <.> ?generic_args)
        ->[|(name, generic_args)| NamedType::new(name, generic_args)]
    )
}

fn array_ty() -> impl QuartzParser<ArrayType> {
    parser!(
        {sequence!(
            punct(PunctKind::OpenBracket),
            parser!({ty()}!![err!("expected type")]),
            parser!({punct(PunctKind::Semicolon)}!![err!("expected `;`")]),
            parser!({expr(true)}!![err!("expected expression")]),
            parser!({punct(PunctKind::CloseBracket)}!![err!("expected `]`")]),
        )}
        ->[|(open_bracket, ty, sep, len, close_bracket)| ArrayType::new(
            open_bracket,
            ty,
            sep,
            len,
            close_bracket,
        )]
    )
}

fn ty() -> impl QuartzParser<Type> {
    choice!(
        parser!({named_ty()}->[Type::Named]),
        parser!({array_ty()}->[Type::Array]),
    )
}

fn generic_args() -> impl QuartzParser<GenericStructArgs> {
    parser!(
        (
            {punct(PunctKind::Lt)}
            <.> {sep_by(ident(), punct(PunctKind::Comma), false, true)}!![err!("expected generic arguments")]
            <.> {punct(PunctKind::Gt)}!![err!("expected `>`")]
        )
        ->[|((open_paren, args), close_paren)| GenericStructArgs::new(open_paren, args, close_paren)]
    )
}

fn field() -> impl QuartzParser<Field> {
    parser!(
        (
            {ident()}
            <.> {punct(PunctKind::Colon)}!![err!("expected `:`")]
            <.> {ty()}!![err!("expected type")]
        )
        ->[|((name, sep), ty)| Field::new(name, sep, ty)]
    )
}

fn struct_def() -> impl QuartzParser<Struct> {
    parser!(
        {sequence!(
            kw(KeywordKind::Struct),
            parser!({ident()}!![err!("expected identifier")]),
            parser!(?{generic_args()}),
            parser!({punct(PunctKind::OpenCurl)}!![err!("expected `{`")]),
            sep_by(field(), punct(PunctKind::Comma), true, true),
            parser!({punct(PunctKind::CloseCurl)}!![err!("expected `}`")]),
        )}
        ->[|(struct_kw, name, generic_args, open_curl, fields, close_curl)| Struct::new(
            struct_kw,
            name,
            generic_args,
            open_curl,
            fields,
            close_curl,
        )]
    )
}

fn variant() -> impl QuartzParser<Variant> {
    let value = parser!(
        {punct(PunctKind::Assign)}
        .> {expr(true)}!![err!("expected expression")]
    );

    parser!(
        ({ident()} <.> ?value)
        ->[|(name, value)| Variant::new(name, value)]
    )
}

fn enum_def() -> impl QuartzParser<Enum> {
    parser!(
        {sequence!(
            kw(KeywordKind::Enum),
            parser!({ident()}!![err!("expected identifier")]),
            parser!({punct(PunctKind::Colon)}!![err!("expected `:`")]),
            parser!({ty()}!![err!("expected type")]),
            parser!({punct(PunctKind::OpenCurl)}!![err!("expected `{`")]),
            sep_by(variant(), punct(PunctKind::Comma), true, true),
            parser!({punct(PunctKind::CloseCurl)}!![err!("expected  `}`")]),
        )}
        ->[|(enum_kw, name, ty_sep, base_ty, open_curl, variants, close_curl)| Enum::new(
            enum_kw,
            name,
            ty_sep,
            base_ty,
            open_curl,
            variants,
            close_curl,
        )]
    )
}

fn const_def() -> impl QuartzParser<Const> {
    parser!(
        {sequence!(
            kw(KeywordKind::Const),
            parser!({ident()}!![err!("expected identifier")]),
            parser!({punct(PunctKind::Assign)}!![err!("expected `=`")]),
            parser!({expr(false)}!![err!("expected expression")]),
            parser!({punct(PunctKind::Semicolon)}!![err!("expected `;`")]),
        )}
        ->[|(const_kw, name, assign, value, _)| Const::new(const_kw, name, assign, value)]
    )
}

fn logic_mode() -> impl QuartzParser<LogicMode> {
    choice!(
        parser!({kw(KeywordKind::Sig)}->[|kw| LogicMode::new(LogicKind::Signal, kw.span())]),
        parser!({kw(KeywordKind::Reg)}->[|kw| LogicMode::new(LogicKind::Register, kw.span())]),
        parser!({kw(KeywordKind::Let)}->[|kw| LogicMode::new(LogicKind::Module, kw.span())]),
    )
}

fn port_mode() -> impl QuartzParser<PortMode> {
    choice!(
        parser!({kw(KeywordKind::In)}->[|kw| PortMode::new(Direction::In, kw.span())]),
        parser!({kw(KeywordKind::Out)}->[|kw| PortMode::new(Direction::Out, kw.span())]),
        parser!({kw(KeywordKind::InOut)}->[|kw| PortMode::new(Direction::InOut, kw.span())]),
    )
}

fn port() -> impl QuartzParser<Port> {
    parser!(
        {sequence!(
            port_mode(),
            parser!({logic_mode()}!![err!("expected `sig` or `reg`")]),
            parser!({ident()}!![err!("expected identifier")]),
            parser!({punct(PunctKind::Colon)}!![err!("expected `:`")]),
            parser!({ty()}!![err!("expected type")]),
        )}
        ->[|(mode, logic_mode, name, sep, ty)| {
            Port::new(mode, logic_mode, name, sep, ty)
        }]
    )
}

fn member() -> impl QuartzParser<Member> {
    let logic = parser!(
        (
            {logic_mode()}
            <.> {ident()}!![err!("expected identifier")]
            <.> {punct(PunctKind::Colon)}!![err!("expected `:`")]
            <.> {ty()}!![err!("expected type")]
            <. {punct(PunctKind::Semicolon)}!![err!("expected `;`")]
        )
        ->[|(((kind, name), sep), ty)| LogicMember::new(kind, name, sep, ty)]
    );

    let edge = choice!(
        parser!({kw(KeywordKind::Rising)}->[|kw| Edge::new(EdgeKind::Rising, kw.span())]),
        parser!({kw(KeywordKind::Rising)}->[|kw| Edge::new(EdgeKind::Rising, kw.span())]),
    );

    let sens = parser!(
        (edge <.> {punct(PunctKind::OpenParen)} <.> {path()} <.> {punct(PunctKind::CloseParen)})
        ->[|(((edge, open_paren), sig), close_paren)| Sens::new(edge, open_paren, sig, close_paren)]
    );

    let proc = parser!(
        (
            {kw(KeywordKind::Proc)}
            <.> {sep_by(sens, punct(PunctKind::Or), false, false)}!![err!("expected sensitivity list")]
            <.> {block()}
        )
        ->[|((proc_kw, sens), body)| ProcMember::new(proc_kw, sens, body)]
    );

    let comb = parser!(
        ({kw(KeywordKind::Comb)} <.> {block()})
        ->[|(comb_kw, body)| CombMember::new(comb_kw, body)]
    );

    choice!(
        parser!(logic->[Member::Logic]),
        parser!({const_def()}->[Member::Const]),
        parser!(proc->[Member::Proc]),
        parser!(comb->[Member::Comb]),
    )
}

fn module_def() -> impl QuartzParser<Module> {
    parser!(
        {sequence!(
            kw(KeywordKind::Mod),
            parser!({ident()}!![err!("expected identifier")]),
            parser!(?{generic_args()}),
            parser!({punct(PunctKind::OpenParen)}!![err!("expected `(`")]),
            sep_by(port(), punct(PunctKind::Comma), true, true),
            parser!({punct(PunctKind::CloseParen)}!![err!("expected `)`")]),
            parser!({punct(PunctKind::OpenCurl)}!![err!("expected `{`")]),
            parser!(*{member()}),
            parser!({punct(PunctKind::CloseCurl)}!![err!("expected `}`")]),
        )}
        ->[|(mod_kw, name, generic_args, open_paren, ports, close_paren, open_curl, members, close_curl)| {
            Module::new(mod_kw, name, generic_args, open_paren, ports, close_paren, open_curl, members, close_curl)
        }]
    )
}

fn fn_def() -> impl QuartzParser<Func> {
    parser!(
        {sequence!(
            kw(KeywordKind::Fn),
            parser!({ident()}!![err!("expected identifier")]),
            parser!({punct(PunctKind::OpenParen)}!![err!("expected `(`")]),
            sep_by(ident(), punct(PunctKind::Comma), true, true),
            parser!({punct(PunctKind::CloseParen)}!![err!("expected `)`")]),
            parser!({block()}!![err!("expected block")]),
        )}
        ->[|(fn_kw, name, open_paren, args, close_paren, body)| {
            Func::new(fn_kw, name, open_paren, args, close_paren, body)
        }]
    )
}

fn item() -> impl QuartzParser<Item> {
    choice!(
        parser!({struct_def()}->[Item::Struct]),
        parser!({enum_def()}->[Item::Enum]),
        parser!({const_def()}->[Item::Const]),
        parser!({module_def()}->[Item::Module]),
        parser!({fn_def()}->[Item::Func]),
    )
}

pub fn design() -> impl QuartzParser<Vec<Item>> {
    parser!(*{ item() })
}
