#![feature(prelude_import)]
#![feature(trait_alias)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod ast {
    #![allow(dead_code)]
    use crate::fmt::{DisplayScoped, ScopedFormatter};
    use crate::lexer::PunctKind;
    use crate::{default_display_impl, SharedString};
    use langbox::TextSpan;
    use std::fmt::Write;
    use std::hash::Hash;
    use std::ops::Range;
    pub trait Spanned {
        fn span(&self) -> TextSpan;
    }
    pub enum KeywordKind {
        Mod,
        Struct,
        Enum,
        Fn,
        InOut,
        In,
        Out,
        Sig,
        Reg,
        Let,
        Const,
        Comb,
        Proc,
        If,
        Else,
        Match,
        While,
        For,
        Rising,
        Falling,
        As,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for KeywordKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                KeywordKind::Mod => ::core::fmt::Formatter::write_str(f, "Mod"),
                KeywordKind::Struct => ::core::fmt::Formatter::write_str(f, "Struct"),
                KeywordKind::Enum => ::core::fmt::Formatter::write_str(f, "Enum"),
                KeywordKind::Fn => ::core::fmt::Formatter::write_str(f, "Fn"),
                KeywordKind::InOut => ::core::fmt::Formatter::write_str(f, "InOut"),
                KeywordKind::In => ::core::fmt::Formatter::write_str(f, "In"),
                KeywordKind::Out => ::core::fmt::Formatter::write_str(f, "Out"),
                KeywordKind::Sig => ::core::fmt::Formatter::write_str(f, "Sig"),
                KeywordKind::Reg => ::core::fmt::Formatter::write_str(f, "Reg"),
                KeywordKind::Let => ::core::fmt::Formatter::write_str(f, "Let"),
                KeywordKind::Const => ::core::fmt::Formatter::write_str(f, "Const"),
                KeywordKind::Comb => ::core::fmt::Formatter::write_str(f, "Comb"),
                KeywordKind::Proc => ::core::fmt::Formatter::write_str(f, "Proc"),
                KeywordKind::If => ::core::fmt::Formatter::write_str(f, "If"),
                KeywordKind::Else => ::core::fmt::Formatter::write_str(f, "Else"),
                KeywordKind::Match => ::core::fmt::Formatter::write_str(f, "Match"),
                KeywordKind::While => ::core::fmt::Formatter::write_str(f, "While"),
                KeywordKind::For => ::core::fmt::Formatter::write_str(f, "For"),
                KeywordKind::Rising => ::core::fmt::Formatter::write_str(f, "Rising"),
                KeywordKind::Falling => ::core::fmt::Formatter::write_str(f, "Falling"),
                KeywordKind::As => ::core::fmt::Formatter::write_str(f, "As"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for KeywordKind {
        #[inline]
        fn clone(&self) -> KeywordKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for KeywordKind {}
    impl ::core::marker::StructuralPartialEq for KeywordKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for KeywordKind {
        #[inline]
        fn eq(&self, other: &KeywordKind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    impl ::core::marker::StructuralEq for KeywordKind {}
    #[automatically_derived]
    impl ::core::cmp::Eq for KeywordKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for KeywordKind {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    impl DisplayScoped for KeywordKind {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&match self {
                    Self::Mod => "mod",
                    Self::Struct => "struct",
                    Self::Enum => "enum",
                    Self::Fn => "fn",
                    Self::InOut => "inout",
                    Self::In => "in",
                    Self::Out => "out",
                    Self::Sig => "sig",
                    Self::Reg => "reg",
                    Self::Let => "let",
                    Self::Const => "const",
                    Self::Comb => "comb",
                    Self::Proc => "proc",
                    Self::If => "if",
                    Self::Else => "else",
                    Self::Match => "match",
                    Self::While => "while",
                    Self::For => "for",
                    Self::Rising => "rising",
                    Self::Falling => "falling",
                    Self::As => "as",
                })],
            ))
        }
    }
    pub struct Keyword {
        kind: KeywordKind,
        span: TextSpan,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Keyword {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Keyword",
                "kind",
                &&self.kind,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Keyword {
        #[inline]
        fn clone(&self) -> Keyword {
            let _: ::core::clone::AssertParamIsClone<KeywordKind>;
            let _: ::core::clone::AssertParamIsClone<TextSpan>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Keyword {}
    impl Keyword {
        #[inline]
        pub fn new(kind: KeywordKind, span: TextSpan) -> Self {
            Self { kind, span }
        }
        #[inline]
        pub fn kind(&self) -> KeywordKind {
            self.kind
        }
    }
    impl Spanned for Keyword {
        #[inline]
        fn span(&self) -> TextSpan {
            self.span
        }
    }
    impl DisplayScoped for Keyword {
        #[inline]
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            DisplayScoped::fmt(&self.kind, f)
        }
    }
    impl std::fmt::Display for Keyword {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Ident {
        value: SharedString,
        span: TextSpan,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Ident {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Ident",
                "value",
                &&self.value,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Ident {
        #[inline]
        fn clone(&self) -> Ident {
            Ident {
                value: ::core::clone::Clone::clone(&self.value),
                span: ::core::clone::Clone::clone(&self.span),
            }
        }
    }
    impl Ident {
        #[inline]
        pub fn new(value: &SharedString, span: TextSpan) -> Self {
            Self {
                value: SharedString::clone(value),
                span,
            }
        }
        #[inline]
        pub fn as_string(&self) -> SharedString {
            SharedString::clone(&self.value)
        }
    }
    impl Spanned for Ident {
        #[inline]
        fn span(&self) -> TextSpan {
            self.span
        }
    }
    impl DisplayScoped for Ident {
        #[inline]
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            DisplayScoped::fmt(&self.value, f)
        }
    }
    impl std::fmt::Display for Ident {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    impl AsRef<str> for Ident {
        #[inline]
        fn as_ref(&self) -> &str {
            &self.value
        }
    }
    pub struct Literal {
        value: i64,
        span: TextSpan,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Literal {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Literal",
                "value",
                &&self.value,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Literal {
        #[inline]
        fn clone(&self) -> Literal {
            let _: ::core::clone::AssertParamIsClone<i64>;
            let _: ::core::clone::AssertParamIsClone<TextSpan>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Literal {}
    impl Literal {
        #[inline]
        pub fn new(value: i64, span: TextSpan) -> Self {
            Self { value, span }
        }
        #[inline]
        pub fn value(&self) -> i64 {
            self.value
        }
    }
    impl Spanned for Literal {
        #[inline]
        fn span(&self) -> TextSpan {
            self.span
        }
    }
    impl DisplayScoped for Literal {
        #[inline]
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            DisplayScoped::fmt(&self.value, f)
        }
    }
    impl std::fmt::Display for Literal {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    impl AsRef<i64> for Literal {
        #[inline]
        fn as_ref(&self) -> &i64 {
            &self.value
        }
    }
    pub struct Punct {
        kind: PunctKind,
        span: TextSpan,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Punct {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Punct",
                "kind",
                &&self.kind,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Punct {
        #[inline]
        fn clone(&self) -> Punct {
            let _: ::core::clone::AssertParamIsClone<PunctKind>;
            let _: ::core::clone::AssertParamIsClone<TextSpan>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Punct {}
    impl Punct {
        #[inline]
        pub fn new(kind: PunctKind, span: TextSpan) -> Self {
            Self { kind, span }
        }
        #[inline]
        pub fn kind(&self) -> PunctKind {
            self.kind
        }
    }
    impl Spanned for Punct {
        #[inline]
        fn span(&self) -> TextSpan {
            self.span
        }
    }
    impl DisplayScoped for Punct {
        #[inline]
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            DisplayScoped::fmt(&self.kind, f)
        }
    }
    impl std::fmt::Display for Punct {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum PathSeparatorKind {
        Period,
        DoubleColon,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PathSeparatorKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                PathSeparatorKind::Period => ::core::fmt::Formatter::write_str(f, "Period"),
                PathSeparatorKind::DoubleColon => {
                    ::core::fmt::Formatter::write_str(f, "DoubleColon")
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PathSeparatorKind {
        #[inline]
        fn clone(&self) -> PathSeparatorKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for PathSeparatorKind {}
    impl ::core::marker::StructuralPartialEq for PathSeparatorKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PathSeparatorKind {
        #[inline]
        fn eq(&self, other: &PathSeparatorKind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    impl ::core::marker::StructuralEq for PathSeparatorKind {}
    #[automatically_derived]
    impl ::core::cmp::Eq for PathSeparatorKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for PathSeparatorKind {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    impl DisplayScoped for PathSeparatorKind {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Period => f.write_fmt(::core::fmt::Arguments::new_v1(&["."], &[])),
                Self::DoubleColon => f.write_fmt(::core::fmt::Arguments::new_v1(&["::"], &[])),
            }
        }
    }
    impl std::fmt::Display for PathSeparatorKind {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct PathSeparator {
        kind: PathSeparatorKind,
        span: TextSpan,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PathSeparator {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "PathSeparator",
                "kind",
                &&self.kind,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PathSeparator {
        #[inline]
        fn clone(&self) -> PathSeparator {
            let _: ::core::clone::AssertParamIsClone<PathSeparatorKind>;
            let _: ::core::clone::AssertParamIsClone<TextSpan>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for PathSeparator {}
    impl PathSeparator {
        #[inline]
        pub fn new(kind: PathSeparatorKind, span: TextSpan) -> Self {
            Self { kind, span }
        }
        #[inline]
        pub fn kind(&self) -> PathSeparatorKind {
            self.kind
        }
    }
    impl Spanned for PathSeparator {
        #[inline]
        fn span(&self) -> TextSpan {
            self.span
        }
    }
    impl DisplayScoped for PathSeparator {
        #[inline]
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            DisplayScoped::fmt(&self.kind, f)
        }
    }
    impl std::fmt::Display for PathSeparator {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct PathSegment {
        sep: PathSeparator,
        ident: Ident,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PathSegment {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "PathSegment",
                "sep",
                &&self.sep,
                "ident",
                &&self.ident,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PathSegment {
        #[inline]
        fn clone(&self) -> PathSegment {
            PathSegment {
                sep: ::core::clone::Clone::clone(&self.sep),
                ident: ::core::clone::Clone::clone(&self.ident),
            }
        }
    }
    impl PathSegment {
        #[inline]
        pub fn new(sep: PathSeparator, ident: Ident) -> Self {
            Self { sep, ident }
        }
        #[inline]
        pub fn sep(&self) -> &PathSeparator {
            &self.sep
        }
        #[inline]
        pub fn ident(&self) -> &Ident {
            &self.ident
        }
    }
    impl Spanned for PathSegment {
        fn span(&self) -> TextSpan {
            self.sep.span().join(&self.ident.span())
        }
    }
    impl DisplayScoped for PathSegment {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.sep),
                    ::core::fmt::ArgumentV1::new_display(&self.ident),
                ],
            ))
        }
    }
    impl std::fmt::Display for PathSegment {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Path {
        head: Ident,
        tail: Vec<PathSegment>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Path {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Path",
                "head",
                &&self.head,
                "tail",
                &&self.tail,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Path {
        #[inline]
        fn clone(&self) -> Path {
            Path {
                head: ::core::clone::Clone::clone(&self.head),
                tail: ::core::clone::Clone::clone(&self.tail),
            }
        }
    }
    impl Path {
        #[inline]
        pub fn new(head: Ident, tail: Vec<PathSegment>) -> Self {
            Self { head, tail }
        }
        #[inline]
        pub fn head(&self) -> &Ident {
            &self.head
        }
        #[inline]
        pub fn tail(&self) -> &[PathSegment] {
            &self.tail
        }
    }
    impl Spanned for Path {
        fn span(&self) -> TextSpan {
            if let Some(last) = self.tail.last() {
                self.head.span().join(&last.span())
            } else {
                self.head.span()
            }
        }
    }
    impl DisplayScoped for Path {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.head)],
            ))?;
            for segment in self.tail.iter() {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&segment)],
                ))?;
            }
            Ok(())
        }
    }
    impl std::fmt::Display for Path {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct ParenExpr {
        open_paren: Punct,
        inner: Box<Expr>,
        close_paren: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParenExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ParenExpr",
                "open_paren",
                &&self.open_paren,
                "inner",
                &&self.inner,
                "close_paren",
                &&self.close_paren,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ParenExpr {
        #[inline]
        fn clone(&self) -> ParenExpr {
            ParenExpr {
                open_paren: ::core::clone::Clone::clone(&self.open_paren),
                inner: ::core::clone::Clone::clone(&self.inner),
                close_paren: ::core::clone::Clone::clone(&self.close_paren),
            }
        }
    }
    impl ParenExpr {
        #[inline]
        pub fn new(open_paren: Punct, inner: Expr, close_paren: Punct) -> Self {
            Self {
                open_paren,
                inner: Box::new(inner),
                close_paren,
            }
        }
        #[inline]
        pub fn open_paren(&self) -> &Punct {
            &self.open_paren
        }
        #[inline]
        pub fn inner(&self) -> &Expr {
            &self.inner
        }
        #[inline]
        pub fn close_paren(&self) -> &Punct {
            &self.close_paren
        }
    }
    impl Spanned for ParenExpr {
        fn span(&self) -> TextSpan {
            self.open_paren.span().join(&self.close_paren.span())
        }
    }
    impl DisplayScoped for ParenExpr {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", "", ""],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.open_paren),
                    ::core::fmt::ArgumentV1::new_display(&self.inner),
                    ::core::fmt::ArgumentV1::new_display(&self.close_paren),
                ],
            ))
        }
    }
    impl std::fmt::Display for ParenExpr {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct CallExpr {
        func: Ident,
        open_paren: Punct,
        args: Vec<Expr>,
        close_paren: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CallExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "CallExpr",
                "func",
                &&self.func,
                "open_paren",
                &&self.open_paren,
                "args",
                &&self.args,
                "close_paren",
                &&self.close_paren,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CallExpr {
        #[inline]
        fn clone(&self) -> CallExpr {
            CallExpr {
                func: ::core::clone::Clone::clone(&self.func),
                open_paren: ::core::clone::Clone::clone(&self.open_paren),
                args: ::core::clone::Clone::clone(&self.args),
                close_paren: ::core::clone::Clone::clone(&self.close_paren),
            }
        }
    }
    impl CallExpr {
        #[inline]
        pub fn new(func: Ident, open_paren: Punct, args: Vec<Expr>, close_paren: Punct) -> Self {
            Self {
                func,
                open_paren,
                args,
                close_paren,
            }
        }
        #[inline]
        pub fn func(&self) -> &Ident {
            &self.func
        }
        #[inline]
        pub fn open_paren(&self) -> &Punct {
            &self.open_paren
        }
        #[inline]
        pub fn args(&self) -> &[Expr] {
            &self.args
        }
        #[inline]
        pub fn close_paren(&self) -> &Punct {
            &self.close_paren
        }
    }
    impl Spanned for CallExpr {
        fn span(&self) -> TextSpan {
            self.open_paren.span().join(&self.close_paren.span())
        }
    }
    impl DisplayScoped for CallExpr {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.func),
                    ::core::fmt::ArgumentV1::new_display(&self.open_paren),
                ],
            ))?;
            for (i, arg) in self.args.iter().enumerate() {
                if i > 0 {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[", "],
                        &[::core::fmt::ArgumentV1::new_display(&arg)],
                    ))?;
                } else {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&arg)],
                    ))?;
                }
            }
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.close_paren)],
            ))
        }
    }
    impl std::fmt::Display for CallExpr {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct FieldAssign {
        field: Ident,
        sep: Punct,
        value: Expr,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FieldAssign {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "FieldAssign",
                "field",
                &&self.field,
                "sep",
                &&self.sep,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FieldAssign {
        #[inline]
        fn clone(&self) -> FieldAssign {
            FieldAssign {
                field: ::core::clone::Clone::clone(&self.field),
                sep: ::core::clone::Clone::clone(&self.sep),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl FieldAssign {
        #[inline]
        pub fn new(field: Ident, sep: Punct, value: Expr) -> Self {
            Self { field, sep, value }
        }
        #[inline]
        pub fn field(&self) -> &Ident {
            &self.field
        }
        #[inline]
        pub fn sep(&self) -> &Punct {
            &self.sep
        }
        #[inline]
        pub fn value(&self) -> &Expr {
            &self.value
        }
    }
    impl Spanned for FieldAssign {
        fn span(&self) -> TextSpan {
            self.field.span().join(&self.value.span())
        }
    }
    impl DisplayScoped for FieldAssign {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", "", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.field),
                    ::core::fmt::ArgumentV1::new_display(&self.sep),
                    ::core::fmt::ArgumentV1::new_display(&self.value),
                ],
            ))
        }
    }
    impl std::fmt::Display for FieldAssign {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct ConstructExpr {
        ty: Box<Type>,
        open_curl: Punct,
        fields: Vec<FieldAssign>,
        close_curl: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstructExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "ConstructExpr",
                "ty",
                &&self.ty,
                "open_curl",
                &&self.open_curl,
                "fields",
                &&self.fields,
                "close_curl",
                &&self.close_curl,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstructExpr {
        #[inline]
        fn clone(&self) -> ConstructExpr {
            ConstructExpr {
                ty: ::core::clone::Clone::clone(&self.ty),
                open_curl: ::core::clone::Clone::clone(&self.open_curl),
                fields: ::core::clone::Clone::clone(&self.fields),
                close_curl: ::core::clone::Clone::clone(&self.close_curl),
            }
        }
    }
    impl ConstructExpr {
        #[inline]
        pub fn new(
            ty: Type,
            open_curl: Punct,
            fields: Vec<FieldAssign>,
            close_curl: Punct,
        ) -> Self {
            Self {
                ty: Box::new(ty),
                open_curl,
                fields,
                close_curl,
            }
        }
        #[inline]
        pub fn ty(&self) -> &Type {
            &self.ty
        }
        #[inline]
        pub fn open_curl(&self) -> &Punct {
            &self.open_curl
        }
        #[inline]
        pub fn fields(&self) -> &[FieldAssign] {
            &self.fields
        }
        #[inline]
        pub fn close_curl(&self) -> &Punct {
            &self.close_curl
        }
    }
    impl Spanned for ConstructExpr {
        fn span(&self) -> TextSpan {
            self.ty.span().join(&self.close_curl.span())
        }
    }
    impl DisplayScoped for ConstructExpr {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", "\n"],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.ty),
                    ::core::fmt::ArgumentV1::new_display(&self.open_curl),
                ],
            ))?;
            f.enter_scope();
            for field in self.fields.iter() {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ",\n"],
                    &[::core::fmt::ArgumentV1::new_display(&field)],
                ))?;
            }
            f.exit_scope();
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.close_curl)],
            ))
        }
    }
    impl std::fmt::Display for ConstructExpr {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct ElseIfBlock {
        else_kw: Keyword,
        if_kw: Keyword,
        condition: Box<Expr>,
        body: Box<Block>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ElseIfBlock {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "ElseIfBlock",
                "else_kw",
                &&self.else_kw,
                "if_kw",
                &&self.if_kw,
                "condition",
                &&self.condition,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ElseIfBlock {
        #[inline]
        fn clone(&self) -> ElseIfBlock {
            ElseIfBlock {
                else_kw: ::core::clone::Clone::clone(&self.else_kw),
                if_kw: ::core::clone::Clone::clone(&self.if_kw),
                condition: ::core::clone::Clone::clone(&self.condition),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl ElseIfBlock {
        #[inline]
        pub fn new(else_kw: Keyword, if_kw: Keyword, condition: Expr, body: Block) -> Self {
            Self {
                else_kw,
                if_kw,
                condition: Box::new(condition),
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn else_kw(&self) -> &Keyword {
            &self.else_kw
        }
        #[inline]
        pub fn if_kw(&self) -> &Keyword {
            &self.if_kw
        }
        #[inline]
        pub fn condition(&self) -> &Expr {
            &self.condition
        }
        #[inline]
        pub fn body(&self) -> &Block {
            &self.body
        }
    }
    impl Spanned for ElseIfBlock {
        fn span(&self) -> TextSpan {
            self.else_kw.span().join(&self.body.span())
        }
    }
    impl DisplayScoped for ElseIfBlock {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", " ", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.else_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.if_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.condition),
                    ::core::fmt::ArgumentV1::new_display(&self.body.as_ref()),
                ],
            ))
        }
    }
    impl std::fmt::Display for ElseIfBlock {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct ElseBlock {
        else_kw: Keyword,
        body: Box<Block>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ElseBlock {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ElseBlock",
                "else_kw",
                &&self.else_kw,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ElseBlock {
        #[inline]
        fn clone(&self) -> ElseBlock {
            ElseBlock {
                else_kw: ::core::clone::Clone::clone(&self.else_kw),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl ElseBlock {
        #[inline]
        pub fn new(else_kw: Keyword, body: Block) -> Self {
            Self {
                else_kw,
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn else_kw(&self) -> &Keyword {
            &self.else_kw
        }
        #[inline]
        pub fn body(&self) -> &Block {
            &self.body
        }
    }
    impl Spanned for ElseBlock {
        fn span(&self) -> TextSpan {
            self.else_kw.span().join(&self.body.span())
        }
    }
    impl DisplayScoped for ElseBlock {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.else_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.body.as_ref()),
                ],
            ))
        }
    }
    impl std::fmt::Display for ElseBlock {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct IfExpr {
        if_kw: Keyword,
        condition: Box<Expr>,
        body: Box<Block>,
        else_if_blocks: Vec<ElseIfBlock>,
        else_block: Option<ElseBlock>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IfExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "IfExpr",
                "if_kw",
                &&self.if_kw,
                "condition",
                &&self.condition,
                "body",
                &&self.body,
                "else_if_blocks",
                &&self.else_if_blocks,
                "else_block",
                &&self.else_block,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for IfExpr {
        #[inline]
        fn clone(&self) -> IfExpr {
            IfExpr {
                if_kw: ::core::clone::Clone::clone(&self.if_kw),
                condition: ::core::clone::Clone::clone(&self.condition),
                body: ::core::clone::Clone::clone(&self.body),
                else_if_blocks: ::core::clone::Clone::clone(&self.else_if_blocks),
                else_block: ::core::clone::Clone::clone(&self.else_block),
            }
        }
    }
    impl IfExpr {
        #[inline]
        pub fn new(
            if_kw: Keyword,
            condition: Expr,
            body: Block,
            else_if_blocks: Vec<ElseIfBlock>,
            else_block: Option<ElseBlock>,
        ) -> Self {
            Self {
                if_kw,
                condition: Box::new(condition),
                body: Box::new(body),
                else_if_blocks,
                else_block,
            }
        }
        #[inline]
        pub fn if_kw(&self) -> &Keyword {
            &self.if_kw
        }
        #[inline]
        pub fn condition(&self) -> &Expr {
            &self.condition
        }
        #[inline]
        pub fn body(&self) -> &Block {
            &self.body
        }
        #[inline]
        pub fn else_if_blocks(&self) -> &[ElseIfBlock] {
            &self.else_if_blocks
        }
        #[inline]
        pub fn else_block(&self) -> Option<&ElseBlock> {
            self.else_block.as_ref()
        }
    }
    impl Spanned for IfExpr {
        fn span(&self) -> TextSpan {
            if let Some(else_block) = &self.else_block {
                self.if_kw.span().join(&else_block.span())
            } else if let Some(last_block) = self.else_if_blocks.last() {
                self.if_kw.span().join(&last_block.span())
            } else {
                self.if_kw.span().join(&self.body.span())
            }
        }
    }
    impl DisplayScoped for IfExpr {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.if_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.condition),
                    ::core::fmt::ArgumentV1::new_display(&self.body.as_ref()),
                ],
            ))?;
            for else_if_block in self.else_if_blocks.iter() {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &[" "],
                    &[::core::fmt::ArgumentV1::new_display(&else_if_block)],
                ))?;
            }
            if let Some(else_block) = &self.else_block {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &[" "],
                    &[::core::fmt::ArgumentV1::new_display(&else_block)],
                ))?;
            }
            Ok(())
        }
    }
    impl std::fmt::Display for IfExpr {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum MatchPattern {
        Literal(Literal),
        Path(Path),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MatchPattern {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                MatchPattern::Literal(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Literal", &__self_0)
                }
                MatchPattern::Path(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Path", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MatchPattern {
        #[inline]
        fn clone(&self) -> MatchPattern {
            match self {
                MatchPattern::Literal(__self_0) => {
                    MatchPattern::Literal(::core::clone::Clone::clone(__self_0))
                }
                MatchPattern::Path(__self_0) => {
                    MatchPattern::Path(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl Spanned for MatchPattern {
        fn span(&self) -> TextSpan {
            match self {
                Self::Literal(l) => l.span(),
                Self::Path(p) => p.span(),
            }
        }
    }
    impl DisplayScoped for MatchPattern {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Literal(l) => DisplayScoped::fmt(l, f),
                Self::Path(p) => DisplayScoped::fmt(p, f),
            }
        }
    }
    impl std::fmt::Display for MatchPattern {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum MatchBody {
        Expr(Expr),
        Block(Block),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MatchBody {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                MatchBody::Expr(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Expr", &__self_0)
                }
                MatchBody::Block(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Block", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MatchBody {
        #[inline]
        fn clone(&self) -> MatchBody {
            match self {
                MatchBody::Expr(__self_0) => MatchBody::Expr(::core::clone::Clone::clone(__self_0)),
                MatchBody::Block(__self_0) => {
                    MatchBody::Block(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl Spanned for MatchBody {
        fn span(&self) -> TextSpan {
            match self {
                Self::Expr(expr) => expr.span(),
                Self::Block(block) => block.span(),
            }
        }
    }
    impl DisplayScoped for MatchBody {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Expr(expr) => DisplayScoped::fmt(expr, f),
                Self::Block(block) => DisplayScoped::fmt(block, f),
            }
        }
    }
    impl std::fmt::Display for MatchBody {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct MatchBranch {
        patterns: Vec<MatchPattern>,
        arrow: Punct,
        body: Box<MatchBody>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MatchBranch {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "MatchBranch",
                "patterns",
                &&self.patterns,
                "arrow",
                &&self.arrow,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MatchBranch {
        #[inline]
        fn clone(&self) -> MatchBranch {
            MatchBranch {
                patterns: ::core::clone::Clone::clone(&self.patterns),
                arrow: ::core::clone::Clone::clone(&self.arrow),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl MatchBranch {
        #[inline]
        pub fn new(patterns: Vec<MatchPattern>, arrow: Punct, body: MatchBody) -> Self {
            Self {
                patterns,
                arrow,
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn patterns(&self) -> &[MatchPattern] {
            &self.patterns
        }
        #[inline]
        pub fn arrow(&self) -> &Punct {
            &self.arrow
        }
        #[inline]
        pub fn body(&self) -> &MatchBody {
            &self.body
        }
    }
    impl Spanned for MatchBranch {
        fn span(&self) -> TextSpan {
            self.patterns
                .first()
                .unwrap()
                .span()
                .join(&self.body.span())
        }
    }
    impl DisplayScoped for MatchBranch {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            for (i, pattern) in self.patterns.iter().enumerate() {
                if i > 0 {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[" | "],
                        &[::core::fmt::ArgumentV1::new_display(&pattern)],
                    ))?;
                } else {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&pattern)],
                    ))?;
                }
            }
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[" ", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.arrow),
                    ::core::fmt::ArgumentV1::new_display(&self.body),
                ],
            ))
        }
    }
    impl std::fmt::Display for MatchBranch {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct MatchExpr {
        match_kw: Keyword,
        value: Box<Expr>,
        open_curl: Punct,
        branches: Vec<MatchBranch>,
        close_curl: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MatchExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "MatchExpr",
                "match_kw",
                &&self.match_kw,
                "value",
                &&self.value,
                "open_curl",
                &&self.open_curl,
                "branches",
                &&self.branches,
                "close_curl",
                &&self.close_curl,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MatchExpr {
        #[inline]
        fn clone(&self) -> MatchExpr {
            MatchExpr {
                match_kw: ::core::clone::Clone::clone(&self.match_kw),
                value: ::core::clone::Clone::clone(&self.value),
                open_curl: ::core::clone::Clone::clone(&self.open_curl),
                branches: ::core::clone::Clone::clone(&self.branches),
                close_curl: ::core::clone::Clone::clone(&self.close_curl),
            }
        }
    }
    impl MatchExpr {
        #[inline]
        pub fn new(
            match_kw: Keyword,
            value: Expr,
            open_curl: Punct,
            branches: Vec<MatchBranch>,
            close_curl: Punct,
        ) -> Self {
            Self {
                match_kw,
                value: Box::new(value),
                open_curl,
                branches,
                close_curl,
            }
        }
        #[inline]
        pub fn match_kw(&self) -> &Keyword {
            &self.match_kw
        }
        #[inline]
        pub fn value(&self) -> &Expr {
            &self.value
        }
        #[inline]
        pub fn open_curl(&self) -> &Punct {
            &self.open_curl
        }
        #[inline]
        pub fn branches(&self) -> &[MatchBranch] {
            &self.branches
        }
        #[inline]
        pub fn close_curl(&self) -> &Punct {
            &self.close_curl
        }
    }
    impl Spanned for MatchExpr {
        fn span(&self) -> TextSpan {
            self.match_kw.span().join(&self.close_curl.span())
        }
    }
    impl DisplayScoped for MatchExpr {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", " ", "\n"],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.match_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.value),
                    ::core::fmt::ArgumentV1::new_display(&self.open_curl),
                ],
            ))?;
            f.enter_scope();
            for branch in self.branches.iter() {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ",\n"],
                    &[::core::fmt::ArgumentV1::new_display(&branch)],
                ))?;
            }
            f.exit_scope();
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.close_curl)],
            ))
        }
    }
    impl std::fmt::Display for MatchExpr {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum IndexKind {
        Single(Expr),
        Range(Range<Expr>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IndexKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                IndexKind::Single(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Single", &__self_0)
                }
                IndexKind::Range(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Range", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for IndexKind {
        #[inline]
        fn clone(&self) -> IndexKind {
            match self {
                IndexKind::Single(__self_0) => {
                    IndexKind::Single(::core::clone::Clone::clone(__self_0))
                }
                IndexKind::Range(__self_0) => {
                    IndexKind::Range(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl Spanned for IndexKind {
        fn span(&self) -> TextSpan {
            match self {
                Self::Single(index) => index.span(),
                Self::Range(range) => range.start.span().join(&range.end.span()),
            }
        }
    }
    impl DisplayScoped for IndexKind {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Single(index) => DisplayScoped::fmt(index, f),
                Self::Range(range) => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ".."],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&range.start),
                        ::core::fmt::ArgumentV1::new_display(&range.end),
                    ],
                )),
            }
        }
    }
    impl std::fmt::Display for IndexKind {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Indexer {
        open_paren: Punct,
        index: Box<IndexKind>,
        close_paren: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Indexer {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Indexer",
                "open_paren",
                &&self.open_paren,
                "index",
                &&self.index,
                "close_paren",
                &&self.close_paren,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Indexer {
        #[inline]
        fn clone(&self) -> Indexer {
            Indexer {
                open_paren: ::core::clone::Clone::clone(&self.open_paren),
                index: ::core::clone::Clone::clone(&self.index),
                close_paren: ::core::clone::Clone::clone(&self.close_paren),
            }
        }
    }
    impl Indexer {
        #[inline]
        pub fn new(open_paren: Punct, index: IndexKind, close_paren: Punct) -> Self {
            Self {
                open_paren,
                index: Box::new(index),
                close_paren,
            }
        }
        #[inline]
        pub fn open_paren(&self) -> &Punct {
            &self.open_paren
        }
        #[inline]
        pub fn index(&self) -> &IndexKind {
            &self.index
        }
        #[inline]
        pub fn close_paren(&self) -> &Punct {
            &self.close_paren
        }
    }
    impl Spanned for Indexer {
        fn span(&self) -> TextSpan {
            self.open_paren.span().join(&self.close_paren.span())
        }
    }
    impl DisplayScoped for Indexer {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", "", ""],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.open_paren),
                    ::core::fmt::ArgumentV1::new_display(&self.index),
                    ::core::fmt::ArgumentV1::new_display(&self.close_paren),
                ],
            ))
        }
    }
    impl std::fmt::Display for Indexer {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct IndexExpr {
        base: Box<Expr>,
        indexer: Indexer,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IndexExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "IndexExpr",
                "base",
                &&self.base,
                "indexer",
                &&self.indexer,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for IndexExpr {
        #[inline]
        fn clone(&self) -> IndexExpr {
            IndexExpr {
                base: ::core::clone::Clone::clone(&self.base),
                indexer: ::core::clone::Clone::clone(&self.indexer),
            }
        }
    }
    impl IndexExpr {
        #[inline]
        pub fn new(base: Expr, indexer: Indexer) -> Self {
            Self {
                base: Box::new(base),
                indexer,
            }
        }
        #[inline]
        pub fn base(&self) -> &Expr {
            &self.base
        }
        #[inline]
        pub fn indexer(&self) -> &Indexer {
            &self.indexer
        }
    }
    impl Spanned for IndexExpr {
        fn span(&self) -> TextSpan {
            self.base.span().join(&self.indexer.span())
        }
    }
    impl DisplayScoped for IndexExpr {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.base),
                    ::core::fmt::ArgumentV1::new_display(&self.indexer),
                ],
            ))
        }
    }
    impl std::fmt::Display for IndexExpr {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct UnaryExpr {
        op: Punct,
        inner: Box<Expr>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UnaryExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "UnaryExpr",
                "op",
                &&self.op,
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnaryExpr {
        #[inline]
        fn clone(&self) -> UnaryExpr {
            UnaryExpr {
                op: ::core::clone::Clone::clone(&self.op),
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    impl UnaryExpr {
        #[inline]
        pub fn new(op: Punct, inner: Expr) -> Self {
            Self {
                op,
                inner: Box::new(inner),
            }
        }
        #[inline]
        pub fn op(&self) -> &Punct {
            &self.op
        }
        #[inline]
        pub fn inner(&self) -> &Expr {
            &self.inner
        }
    }
    impl Spanned for UnaryExpr {
        fn span(&self) -> TextSpan {
            self.op.span().join(&self.inner.span())
        }
    }
    impl DisplayScoped for UnaryExpr {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.op),
                    ::core::fmt::ArgumentV1::new_display(&self.inner),
                ],
            ))
        }
    }
    impl std::fmt::Display for UnaryExpr {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct CastExpr {
        value: Box<Expr>,
        as_kw: Keyword,
        target_ty: Box<Type>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CastExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "CastExpr",
                "value",
                &&self.value,
                "as_kw",
                &&self.as_kw,
                "target_ty",
                &&self.target_ty,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CastExpr {
        #[inline]
        fn clone(&self) -> CastExpr {
            CastExpr {
                value: ::core::clone::Clone::clone(&self.value),
                as_kw: ::core::clone::Clone::clone(&self.as_kw),
                target_ty: ::core::clone::Clone::clone(&self.target_ty),
            }
        }
    }
    impl CastExpr {
        #[inline]
        pub fn new(value: Expr, as_kw: Keyword, target_ty: Type) -> Self {
            Self {
                value: Box::new(value),
                as_kw,
                target_ty: Box::new(target_ty),
            }
        }
        #[inline]
        pub fn value(&self) -> &Expr {
            &self.value
        }
        #[inline]
        pub fn as_kw(&self) -> &Keyword {
            &self.as_kw
        }
        #[inline]
        pub fn target_ty(&self) -> &Type {
            &self.target_ty
        }
    }
    impl Spanned for CastExpr {
        fn span(&self) -> TextSpan {
            self.value.span().join(&self.target_ty.span())
        }
    }
    impl DisplayScoped for CastExpr {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.value),
                    ::core::fmt::ArgumentV1::new_display(&self.as_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.target_ty),
                ],
            ))
        }
    }
    impl std::fmt::Display for CastExpr {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct BinaryExpr {
        lhs: Box<Expr>,
        op: Punct,
        rhs: Box<Expr>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BinaryExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "BinaryExpr",
                "lhs",
                &&self.lhs,
                "op",
                &&self.op,
                "rhs",
                &&self.rhs,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for BinaryExpr {
        #[inline]
        fn clone(&self) -> BinaryExpr {
            BinaryExpr {
                lhs: ::core::clone::Clone::clone(&self.lhs),
                op: ::core::clone::Clone::clone(&self.op),
                rhs: ::core::clone::Clone::clone(&self.rhs),
            }
        }
    }
    impl BinaryExpr {
        #[inline]
        pub fn new(lhs: Expr, op: Punct, rhs: Expr) -> Self {
            Self {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        }
        #[inline]
        pub fn lhs(&self) -> &Expr {
            &self.lhs
        }
        #[inline]
        pub fn op(&self) -> &Punct {
            &self.op
        }
        #[inline]
        pub fn rhs(&self) -> &Expr {
            &self.rhs
        }
    }
    impl Spanned for BinaryExpr {
        fn span(&self) -> TextSpan {
            self.lhs.span().join(&self.rhs.span())
        }
    }
    impl DisplayScoped for BinaryExpr {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.lhs),
                    ::core::fmt::ArgumentV1::new_display(&self.op),
                    ::core::fmt::ArgumentV1::new_display(&self.rhs),
                ],
            ))
        }
    }
    impl std::fmt::Display for BinaryExpr {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum Expr {
        Literal(Literal),
        Path(Path),
        Paren(ParenExpr),
        Call(CallExpr),
        Construct(ConstructExpr),
        If(IfExpr),
        Match(MatchExpr),
        Block(Box<Block>),
        Index(IndexExpr),
        Pos(UnaryExpr),
        Neg(UnaryExpr),
        Not(UnaryExpr),
        Cast(CastExpr),
        Concat(BinaryExpr),
        Lt(BinaryExpr),
        Lte(BinaryExpr),
        Gt(BinaryExpr),
        Gte(BinaryExpr),
        Slt(BinaryExpr),
        Slte(BinaryExpr),
        Sgt(BinaryExpr),
        Sgte(BinaryExpr),
        Eq(BinaryExpr),
        Ne(BinaryExpr),
        Add(BinaryExpr),
        Sub(BinaryExpr),
        Mul(BinaryExpr),
        Div(BinaryExpr),
        Rem(BinaryExpr),
        And(BinaryExpr),
        Xor(BinaryExpr),
        Or(BinaryExpr),
        Shl(BinaryExpr),
        Lsr(BinaryExpr),
        Asr(BinaryExpr),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Expr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Expr::Literal(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Literal", &__self_0)
                }
                Expr::Path(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Path", &__self_0)
                }
                Expr::Paren(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Paren", &__self_0)
                }
                Expr::Call(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Call", &__self_0)
                }
                Expr::Construct(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Construct", &__self_0)
                }
                Expr::If(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "If", &__self_0)
                }
                Expr::Match(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Match", &__self_0)
                }
                Expr::Block(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Block", &__self_0)
                }
                Expr::Index(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Index", &__self_0)
                }
                Expr::Pos(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Pos", &__self_0)
                }
                Expr::Neg(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Neg", &__self_0)
                }
                Expr::Not(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Not", &__self_0)
                }
                Expr::Cast(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Cast", &__self_0)
                }
                Expr::Concat(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Concat", &__self_0)
                }
                Expr::Lt(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Lt", &__self_0)
                }
                Expr::Lte(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Lte", &__self_0)
                }
                Expr::Gt(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Gt", &__self_0)
                }
                Expr::Gte(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Gte", &__self_0)
                }
                Expr::Slt(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Slt", &__self_0)
                }
                Expr::Slte(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Slte", &__self_0)
                }
                Expr::Sgt(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Sgt", &__self_0)
                }
                Expr::Sgte(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Sgte", &__self_0)
                }
                Expr::Eq(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Eq", &__self_0)
                }
                Expr::Ne(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Ne", &__self_0)
                }
                Expr::Add(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Add", &__self_0)
                }
                Expr::Sub(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Sub", &__self_0)
                }
                Expr::Mul(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Mul", &__self_0)
                }
                Expr::Div(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Div", &__self_0)
                }
                Expr::Rem(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Rem", &__self_0)
                }
                Expr::And(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "And", &__self_0)
                }
                Expr::Xor(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Xor", &__self_0)
                }
                Expr::Or(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Or", &__self_0)
                }
                Expr::Shl(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Shl", &__self_0)
                }
                Expr::Lsr(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Lsr", &__self_0)
                }
                Expr::Asr(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Asr", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Expr {
        #[inline]
        fn clone(&self) -> Expr {
            match self {
                Expr::Literal(__self_0) => Expr::Literal(::core::clone::Clone::clone(__self_0)),
                Expr::Path(__self_0) => Expr::Path(::core::clone::Clone::clone(__self_0)),
                Expr::Paren(__self_0) => Expr::Paren(::core::clone::Clone::clone(__self_0)),
                Expr::Call(__self_0) => Expr::Call(::core::clone::Clone::clone(__self_0)),
                Expr::Construct(__self_0) => Expr::Construct(::core::clone::Clone::clone(__self_0)),
                Expr::If(__self_0) => Expr::If(::core::clone::Clone::clone(__self_0)),
                Expr::Match(__self_0) => Expr::Match(::core::clone::Clone::clone(__self_0)),
                Expr::Block(__self_0) => Expr::Block(::core::clone::Clone::clone(__self_0)),
                Expr::Index(__self_0) => Expr::Index(::core::clone::Clone::clone(__self_0)),
                Expr::Pos(__self_0) => Expr::Pos(::core::clone::Clone::clone(__self_0)),
                Expr::Neg(__self_0) => Expr::Neg(::core::clone::Clone::clone(__self_0)),
                Expr::Not(__self_0) => Expr::Not(::core::clone::Clone::clone(__self_0)),
                Expr::Cast(__self_0) => Expr::Cast(::core::clone::Clone::clone(__self_0)),
                Expr::Concat(__self_0) => Expr::Concat(::core::clone::Clone::clone(__self_0)),
                Expr::Lt(__self_0) => Expr::Lt(::core::clone::Clone::clone(__self_0)),
                Expr::Lte(__self_0) => Expr::Lte(::core::clone::Clone::clone(__self_0)),
                Expr::Gt(__self_0) => Expr::Gt(::core::clone::Clone::clone(__self_0)),
                Expr::Gte(__self_0) => Expr::Gte(::core::clone::Clone::clone(__self_0)),
                Expr::Slt(__self_0) => Expr::Slt(::core::clone::Clone::clone(__self_0)),
                Expr::Slte(__self_0) => Expr::Slte(::core::clone::Clone::clone(__self_0)),
                Expr::Sgt(__self_0) => Expr::Sgt(::core::clone::Clone::clone(__self_0)),
                Expr::Sgte(__self_0) => Expr::Sgte(::core::clone::Clone::clone(__self_0)),
                Expr::Eq(__self_0) => Expr::Eq(::core::clone::Clone::clone(__self_0)),
                Expr::Ne(__self_0) => Expr::Ne(::core::clone::Clone::clone(__self_0)),
                Expr::Add(__self_0) => Expr::Add(::core::clone::Clone::clone(__self_0)),
                Expr::Sub(__self_0) => Expr::Sub(::core::clone::Clone::clone(__self_0)),
                Expr::Mul(__self_0) => Expr::Mul(::core::clone::Clone::clone(__self_0)),
                Expr::Div(__self_0) => Expr::Div(::core::clone::Clone::clone(__self_0)),
                Expr::Rem(__self_0) => Expr::Rem(::core::clone::Clone::clone(__self_0)),
                Expr::And(__self_0) => Expr::And(::core::clone::Clone::clone(__self_0)),
                Expr::Xor(__self_0) => Expr::Xor(::core::clone::Clone::clone(__self_0)),
                Expr::Or(__self_0) => Expr::Or(::core::clone::Clone::clone(__self_0)),
                Expr::Shl(__self_0) => Expr::Shl(::core::clone::Clone::clone(__self_0)),
                Expr::Lsr(__self_0) => Expr::Lsr(::core::clone::Clone::clone(__self_0)),
                Expr::Asr(__self_0) => Expr::Asr(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    impl Spanned for Expr {
        fn span(&self) -> TextSpan {
            match self {
                Self::Literal(l) => l.span(),
                Self::Path(p) => p.span(),
                Self::Paren(expr) => expr.span(),
                Self::Call(expr) => expr.span(),
                Self::Construct(expr) => expr.span(),
                Self::If(expr) => expr.span(),
                Self::Match(expr) => expr.span(),
                Self::Block(b) => b.span(),
                Self::Index(expr) => expr.span(),
                Self::Pos(expr) => expr.span(),
                Self::Neg(expr) => expr.span(),
                Self::Not(expr) => expr.span(),
                Self::Cast(expr) => expr.span(),
                Self::Concat(expr) => expr.span(),
                Self::Lt(expr) => expr.span(),
                Self::Lte(expr) => expr.span(),
                Self::Gt(expr) => expr.span(),
                Self::Gte(expr) => expr.span(),
                Self::Slt(expr) => expr.span(),
                Self::Slte(expr) => expr.span(),
                Self::Sgt(expr) => expr.span(),
                Self::Sgte(expr) => expr.span(),
                Self::Eq(expr) => expr.span(),
                Self::Ne(expr) => expr.span(),
                Self::Add(expr) => expr.span(),
                Self::Sub(expr) => expr.span(),
                Self::Mul(expr) => expr.span(),
                Self::Div(expr) => expr.span(),
                Self::Rem(expr) => expr.span(),
                Self::And(expr) => expr.span(),
                Self::Xor(expr) => expr.span(),
                Self::Or(expr) => expr.span(),
                Self::Shl(expr) => expr.span(),
                Self::Lsr(expr) => expr.span(),
                Self::Asr(expr) => expr.span(),
            }
        }
    }
    impl DisplayScoped for Expr {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Literal(l) => DisplayScoped::fmt(l, f),
                Self::Path(p) => DisplayScoped::fmt(p, f),
                Self::Paren(expr) => DisplayScoped::fmt(expr, f),
                Self::Call(expr) => DisplayScoped::fmt(expr, f),
                Self::Construct(expr) => DisplayScoped::fmt(expr, f),
                Self::If(expr) => DisplayScoped::fmt(expr, f),
                Self::Match(expr) => DisplayScoped::fmt(expr, f),
                Self::Block(b) => DisplayScoped::fmt(b.as_ref(), f),
                Self::Index(expr) => DisplayScoped::fmt(expr, f),
                Self::Pos(expr) => DisplayScoped::fmt(expr, f),
                Self::Neg(expr) => DisplayScoped::fmt(expr, f),
                Self::Not(expr) => DisplayScoped::fmt(expr, f),
                Self::Cast(expr) => DisplayScoped::fmt(expr, f),
                Self::Concat(expr) => DisplayScoped::fmt(expr, f),
                Self::Lt(expr) => DisplayScoped::fmt(expr, f),
                Self::Lte(expr) => DisplayScoped::fmt(expr, f),
                Self::Gt(expr) => DisplayScoped::fmt(expr, f),
                Self::Gte(expr) => DisplayScoped::fmt(expr, f),
                Self::Slt(expr) => DisplayScoped::fmt(expr, f),
                Self::Slte(expr) => DisplayScoped::fmt(expr, f),
                Self::Sgt(expr) => DisplayScoped::fmt(expr, f),
                Self::Sgte(expr) => DisplayScoped::fmt(expr, f),
                Self::Eq(expr) => DisplayScoped::fmt(expr, f),
                Self::Ne(expr) => DisplayScoped::fmt(expr, f),
                Self::Add(expr) => DisplayScoped::fmt(expr, f),
                Self::Sub(expr) => DisplayScoped::fmt(expr, f),
                Self::Mul(expr) => DisplayScoped::fmt(expr, f),
                Self::Div(expr) => DisplayScoped::fmt(expr, f),
                Self::Rem(expr) => DisplayScoped::fmt(expr, f),
                Self::And(expr) => DisplayScoped::fmt(expr, f),
                Self::Xor(expr) => DisplayScoped::fmt(expr, f),
                Self::Or(expr) => DisplayScoped::fmt(expr, f),
                Self::Shl(expr) => DisplayScoped::fmt(expr, f),
                Self::Lsr(expr) => DisplayScoped::fmt(expr, f),
                Self::Asr(expr) => DisplayScoped::fmt(expr, f),
            }
        }
    }
    impl std::fmt::Display for Expr {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Declaration {
        let_kw: Keyword,
        name: Ident,
        assign: Punct,
        value: Expr,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Declaration {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Declaration",
                "let_kw",
                &&self.let_kw,
                "name",
                &&self.name,
                "assign",
                &&self.assign,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Declaration {
        #[inline]
        fn clone(&self) -> Declaration {
            Declaration {
                let_kw: ::core::clone::Clone::clone(&self.let_kw),
                name: ::core::clone::Clone::clone(&self.name),
                assign: ::core::clone::Clone::clone(&self.assign),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl Declaration {
        #[inline]
        pub fn new(let_kw: Keyword, name: Ident, assign: Punct, value: Expr) -> Self {
            Self {
                let_kw,
                name,
                assign,
                value,
            }
        }
        #[inline]
        pub fn let_kw(&self) -> &Keyword {
            &self.let_kw
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn assign(&self) -> &Punct {
            &self.assign
        }
        #[inline]
        pub fn value(&self) -> &Expr {
            &self.value
        }
    }
    impl Spanned for Declaration {
        fn span(&self) -> TextSpan {
            self.let_kw.span().join(&self.value.span())
        }
    }
    impl DisplayScoped for Declaration {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", " ", " ", ";"],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.let_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.name),
                    ::core::fmt::ArgumentV1::new_display(&self.assign),
                    ::core::fmt::ArgumentV1::new_display(&self.value),
                ],
            ))
        }
    }
    impl std::fmt::Display for Declaration {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct AssignTarget {
        path: Path,
        indexers: Vec<Indexer>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AssignTarget {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "AssignTarget",
                "path",
                &&self.path,
                "indexers",
                &&self.indexers,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AssignTarget {
        #[inline]
        fn clone(&self) -> AssignTarget {
            AssignTarget {
                path: ::core::clone::Clone::clone(&self.path),
                indexers: ::core::clone::Clone::clone(&self.indexers),
            }
        }
    }
    impl AssignTarget {
        #[inline]
        pub fn new(path: Path, indexers: Vec<Indexer>) -> Self {
            Self { path, indexers }
        }
        #[inline]
        pub fn path(&self) -> &Path {
            &self.path
        }
        #[inline]
        pub fn indexers(&self) -> &[Indexer] {
            &self.indexers
        }
    }
    impl Spanned for AssignTarget {
        fn span(&self) -> TextSpan {
            if let Some(last) = self.indexers.last() {
                self.path.span().join(&last.span())
            } else {
                self.path.span()
            }
        }
    }
    impl DisplayScoped for AssignTarget {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.path)],
            ))?;
            for indexer in self.indexers.iter() {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&indexer)],
                ))?;
            }
            Ok(())
        }
    }
    impl std::fmt::Display for AssignTarget {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum AssignKind {
        /// `=`
        Assign,
        /// `+=`
        AddAssign,
        /// `-=`
        SubAssign,
        /// `*=`
        MulAssign,
        /// `/=`
        DivAssign,
        /// `%=`
        RemAssign,
        /// `&=`
        AndAssign,
        /// `|=`
        OrAssign,
        /// `^=`
        XorAssign,
        /// `<<=`
        ShlAssign,
        /// `>>>=`
        AsrAssign,
        /// `>>=`
        LsrAssign,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AssignKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AssignKind::Assign => ::core::fmt::Formatter::write_str(f, "Assign"),
                AssignKind::AddAssign => ::core::fmt::Formatter::write_str(f, "AddAssign"),
                AssignKind::SubAssign => ::core::fmt::Formatter::write_str(f, "SubAssign"),
                AssignKind::MulAssign => ::core::fmt::Formatter::write_str(f, "MulAssign"),
                AssignKind::DivAssign => ::core::fmt::Formatter::write_str(f, "DivAssign"),
                AssignKind::RemAssign => ::core::fmt::Formatter::write_str(f, "RemAssign"),
                AssignKind::AndAssign => ::core::fmt::Formatter::write_str(f, "AndAssign"),
                AssignKind::OrAssign => ::core::fmt::Formatter::write_str(f, "OrAssign"),
                AssignKind::XorAssign => ::core::fmt::Formatter::write_str(f, "XorAssign"),
                AssignKind::ShlAssign => ::core::fmt::Formatter::write_str(f, "ShlAssign"),
                AssignKind::AsrAssign => ::core::fmt::Formatter::write_str(f, "AsrAssign"),
                AssignKind::LsrAssign => ::core::fmt::Formatter::write_str(f, "LsrAssign"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AssignKind {
        #[inline]
        fn clone(&self) -> AssignKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AssignKind {}
    impl ::core::marker::StructuralPartialEq for AssignKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AssignKind {
        #[inline]
        fn eq(&self, other: &AssignKind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    impl ::core::marker::StructuralEq for AssignKind {}
    #[automatically_derived]
    impl ::core::cmp::Eq for AssignKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for AssignKind {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    impl DisplayScoped for AssignKind {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&match self {
                    Self::Assign => "=",
                    Self::AddAssign => "+=",
                    Self::SubAssign => "-=",
                    Self::MulAssign => "*=",
                    Self::DivAssign => "/=",
                    Self::RemAssign => "%=",
                    Self::AndAssign => "&=",
                    Self::OrAssign => "|=",
                    Self::XorAssign => "^=",
                    Self::ShlAssign => "<<=",
                    Self::AsrAssign => ">>>=",
                    Self::LsrAssign => ">>=",
                })],
            ))
        }
    }
    impl std::fmt::Display for AssignKind {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct AssignOp {
        kind: AssignKind,
        span: TextSpan,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AssignOp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "AssignOp",
                "kind",
                &&self.kind,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AssignOp {
        #[inline]
        fn clone(&self) -> AssignOp {
            let _: ::core::clone::AssertParamIsClone<AssignKind>;
            let _: ::core::clone::AssertParamIsClone<TextSpan>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AssignOp {}
    impl AssignOp {
        #[inline]
        pub fn new(kind: AssignKind, span: TextSpan) -> Self {
            Self { kind, span }
        }
        #[inline]
        pub fn kind(&self) -> AssignKind {
            self.kind
        }
    }
    impl Spanned for AssignOp {
        #[inline]
        fn span(&self) -> TextSpan {
            self.span
        }
    }
    impl DisplayScoped for AssignOp {
        #[inline]
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            DisplayScoped::fmt(&self.kind, f)
        }
    }
    impl std::fmt::Display for AssignOp {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Assignment {
        target: AssignTarget,
        op: AssignOp,
        value: Expr,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Assignment {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Assignment",
                "target",
                &&self.target,
                "op",
                &&self.op,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Assignment {
        #[inline]
        fn clone(&self) -> Assignment {
            Assignment {
                target: ::core::clone::Clone::clone(&self.target),
                op: ::core::clone::Clone::clone(&self.op),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl Assignment {
        #[inline]
        pub fn new(target: AssignTarget, op: AssignOp, value: Expr) -> Self {
            Self { target, op, value }
        }
        #[inline]
        pub fn target(&self) -> &AssignTarget {
            &self.target
        }
        #[inline]
        pub fn op(&self) -> &AssignOp {
            &self.op
        }
        #[inline]
        pub fn value(&self) -> &Expr {
            &self.value
        }
    }
    impl Spanned for Assignment {
        fn span(&self) -> TextSpan {
            self.target.span().join(&self.value.span())
        }
    }
    impl DisplayScoped for Assignment {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", " ", ";"],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.target),
                    ::core::fmt::ArgumentV1::new_display(&self.op),
                    ::core::fmt::ArgumentV1::new_display(&self.value),
                ],
            ))
        }
    }
    impl std::fmt::Display for Assignment {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct WhileLoop {
        while_kw: Keyword,
        condition: Expr,
        body: Box<Block>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for WhileLoop {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "WhileLoop",
                "while_kw",
                &&self.while_kw,
                "condition",
                &&self.condition,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for WhileLoop {
        #[inline]
        fn clone(&self) -> WhileLoop {
            WhileLoop {
                while_kw: ::core::clone::Clone::clone(&self.while_kw),
                condition: ::core::clone::Clone::clone(&self.condition),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl WhileLoop {
        #[inline]
        pub fn new(while_kw: Keyword, condition: Expr, body: Block) -> Self {
            Self {
                while_kw,
                condition,
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn while_kw(&self) -> &Keyword {
            &self.while_kw
        }
        #[inline]
        pub fn condition(&self) -> &Expr {
            &self.condition
        }
        #[inline]
        pub fn body(&self) -> &Block {
            &self.body
        }
    }
    impl Spanned for WhileLoop {
        fn span(&self) -> TextSpan {
            self.while_kw.span().join(&self.body.span())
        }
    }
    impl DisplayScoped for WhileLoop {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.while_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.condition),
                    ::core::fmt::ArgumentV1::new_display(&self.body),
                ],
            ))
        }
    }
    impl std::fmt::Display for WhileLoop {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct ForLoop {
        for_kw: Keyword,
        item_name: Ident,
        in_kw: Keyword,
        range: Range<Expr>,
        body: Box<Block>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ForLoop {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "ForLoop",
                "for_kw",
                &&self.for_kw,
                "item_name",
                &&self.item_name,
                "in_kw",
                &&self.in_kw,
                "range",
                &&self.range,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ForLoop {
        #[inline]
        fn clone(&self) -> ForLoop {
            ForLoop {
                for_kw: ::core::clone::Clone::clone(&self.for_kw),
                item_name: ::core::clone::Clone::clone(&self.item_name),
                in_kw: ::core::clone::Clone::clone(&self.in_kw),
                range: ::core::clone::Clone::clone(&self.range),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl ForLoop {
        #[inline]
        pub fn new(
            for_kw: Keyword,
            item_name: Ident,
            in_kw: Keyword,
            range: Range<Expr>,
            body: Block,
        ) -> Self {
            Self {
                for_kw,
                item_name,
                in_kw,
                range,
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn for_kw(&self) -> &Keyword {
            &self.for_kw
        }
        #[inline]
        pub fn item_name(&self) -> &Ident {
            &self.item_name
        }
        #[inline]
        pub fn in_kw(&self) -> &Keyword {
            &self.in_kw
        }
        #[inline]
        pub fn range(&self) -> &Range<Expr> {
            &self.range
        }
        #[inline]
        pub fn body(&self) -> &Block {
            &self.body
        }
    }
    impl Spanned for ForLoop {
        fn span(&self) -> TextSpan {
            self.for_kw.span().join(&self.body.span())
        }
    }
    impl DisplayScoped for ForLoop {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", " ", " ", "..", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.for_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.item_name),
                    ::core::fmt::ArgumentV1::new_display(&self.in_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.range.start),
                    ::core::fmt::ArgumentV1::new_display(&self.range.end),
                    ::core::fmt::ArgumentV1::new_display(&self.body),
                ],
            ))
        }
    }
    impl std::fmt::Display for ForLoop {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum Statement {
        Expr(Expr),
        Declaration(Declaration),
        Assignment(Assignment),
        WhileLoop(WhileLoop),
        ForLoop(ForLoop),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Statement {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Statement::Expr(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Expr", &__self_0)
                }
                Statement::Declaration(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Declaration", &__self_0)
                }
                Statement::Assignment(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Assignment", &__self_0)
                }
                Statement::WhileLoop(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "WhileLoop", &__self_0)
                }
                Statement::ForLoop(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ForLoop", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Statement {
        #[inline]
        fn clone(&self) -> Statement {
            match self {
                Statement::Expr(__self_0) => Statement::Expr(::core::clone::Clone::clone(__self_0)),
                Statement::Declaration(__self_0) => {
                    Statement::Declaration(::core::clone::Clone::clone(__self_0))
                }
                Statement::Assignment(__self_0) => {
                    Statement::Assignment(::core::clone::Clone::clone(__self_0))
                }
                Statement::WhileLoop(__self_0) => {
                    Statement::WhileLoop(::core::clone::Clone::clone(__self_0))
                }
                Statement::ForLoop(__self_0) => {
                    Statement::ForLoop(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl Spanned for Statement {
        fn span(&self) -> TextSpan {
            match self {
                Self::Expr(expr) => expr.span(),
                Self::Declaration(decl) => decl.span(),
                Self::Assignment(assignment) => assignment.span(),
                Self::WhileLoop(while_loop) => while_loop.span(),
                Self::ForLoop(for_loop) => for_loop.span(),
            }
        }
    }
    impl DisplayScoped for Statement {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Expr(expr) => match expr {
                    Expr::If(_) | Expr::Match(_) | Expr::Block(_) => DisplayScoped::fmt(expr, f),
                    _ => f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["", ";"],
                        &[::core::fmt::ArgumentV1::new_display(&expr)],
                    )),
                },
                Self::Declaration(decl) => DisplayScoped::fmt(decl, f),
                Self::Assignment(assignment) => DisplayScoped::fmt(assignment, f),
                Self::WhileLoop(while_loop) => DisplayScoped::fmt(while_loop, f),
                Self::ForLoop(for_loop) => DisplayScoped::fmt(for_loop, f),
            }
        }
    }
    impl std::fmt::Display for Statement {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Block {
        open_curl: Punct,
        statements: Vec<Statement>,
        result: Option<Expr>,
        close_curl: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Block {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Block",
                "open_curl",
                &&self.open_curl,
                "statements",
                &&self.statements,
                "result",
                &&self.result,
                "close_curl",
                &&self.close_curl,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Block {
        #[inline]
        fn clone(&self) -> Block {
            Block {
                open_curl: ::core::clone::Clone::clone(&self.open_curl),
                statements: ::core::clone::Clone::clone(&self.statements),
                result: ::core::clone::Clone::clone(&self.result),
                close_curl: ::core::clone::Clone::clone(&self.close_curl),
            }
        }
    }
    impl Block {
        #[inline]
        pub fn new(
            open_curl: Punct,
            statements: Vec<Statement>,
            result: Option<Expr>,
            close_curl: Punct,
        ) -> Self {
            Self {
                open_curl,
                statements,
                result,
                close_curl,
            }
        }
        #[inline]
        pub fn open_curl(&self) -> &Punct {
            &self.open_curl
        }
        #[inline]
        pub fn statements(&self) -> &[Statement] {
            &self.statements
        }
        #[inline]
        pub fn result(&self) -> Option<&Expr> {
            self.result.as_ref()
        }
        #[inline]
        pub fn close_curl(&self) -> &Punct {
            &self.close_curl
        }
    }
    impl Spanned for Block {
        fn span(&self) -> TextSpan {
            self.open_curl.span().join(&self.close_curl.span())
        }
    }
    impl DisplayScoped for Block {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", "\n"],
                &[::core::fmt::ArgumentV1::new_display(&self.open_curl)],
            ))?;
            f.enter_scope();
            for statement in self.statements.iter() {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&statement)],
                ))?;
            }
            if let Some(result) = &self.result {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&result)],
                ))?;
            }
            f.exit_scope();
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.close_curl)],
            ))
        }
    }
    impl std::fmt::Display for Block {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum GenericTypeArg {
        Literal(Literal),
        Path(Path),
        Expr(Expr),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GenericTypeArg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                GenericTypeArg::Literal(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Literal", &__self_0)
                }
                GenericTypeArg::Path(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Path", &__self_0)
                }
                GenericTypeArg::Expr(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Expr", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GenericTypeArg {
        #[inline]
        fn clone(&self) -> GenericTypeArg {
            match self {
                GenericTypeArg::Literal(__self_0) => {
                    GenericTypeArg::Literal(::core::clone::Clone::clone(__self_0))
                }
                GenericTypeArg::Path(__self_0) => {
                    GenericTypeArg::Path(::core::clone::Clone::clone(__self_0))
                }
                GenericTypeArg::Expr(__self_0) => {
                    GenericTypeArg::Expr(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl Spanned for GenericTypeArg {
        fn span(&self) -> TextSpan {
            match self {
                Self::Literal(l) => l.span(),
                Self::Path(path) => path.span(),
                Self::Expr(expr) => expr.span(),
            }
        }
    }
    impl DisplayScoped for GenericTypeArg {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Literal(l) => DisplayScoped::fmt(l, f),
                Self::Path(path) => DisplayScoped::fmt(path, f),
                Self::Expr(expr) => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["{ ", " }"],
                    &[::core::fmt::ArgumentV1::new_display(&expr)],
                )),
            }
        }
    }
    impl std::fmt::Display for GenericTypeArg {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct GenericTypeArgs {
        open_paren: Punct,
        args: Vec<GenericTypeArg>,
        close_paren: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GenericTypeArgs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "GenericTypeArgs",
                "open_paren",
                &&self.open_paren,
                "args",
                &&self.args,
                "close_paren",
                &&self.close_paren,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GenericTypeArgs {
        #[inline]
        fn clone(&self) -> GenericTypeArgs {
            GenericTypeArgs {
                open_paren: ::core::clone::Clone::clone(&self.open_paren),
                args: ::core::clone::Clone::clone(&self.args),
                close_paren: ::core::clone::Clone::clone(&self.close_paren),
            }
        }
    }
    impl GenericTypeArgs {
        #[inline]
        pub fn new(open_paren: Punct, args: Vec<GenericTypeArg>, close_paren: Punct) -> Self {
            Self {
                open_paren,
                args,
                close_paren,
            }
        }
        #[inline]
        pub fn open_paren(&self) -> &Punct {
            &self.open_paren
        }
        #[inline]
        pub fn args(&self) -> &[GenericTypeArg] {
            &self.args
        }
        #[inline]
        pub fn close_paren(&self) -> &Punct {
            &self.close_paren
        }
    }
    impl Spanned for GenericTypeArgs {
        fn span(&self) -> TextSpan {
            self.open_paren.span().join(&self.close_paren.span())
        }
    }
    impl DisplayScoped for GenericTypeArgs {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.open_paren)],
            ))?;
            for (i, arg) in self.args.iter().enumerate() {
                if i == 0 {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&arg)],
                    ))?;
                } else {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[", "],
                        &[::core::fmt::ArgumentV1::new_display(&arg)],
                    ))?;
                }
            }
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.close_paren)],
            ))
        }
    }
    impl std::fmt::Display for GenericTypeArgs {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct NamedType {
        name: Ident,
        generic_args: Option<GenericTypeArgs>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for NamedType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "NamedType",
                "name",
                &&self.name,
                "generic_args",
                &&self.generic_args,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NamedType {
        #[inline]
        fn clone(&self) -> NamedType {
            NamedType {
                name: ::core::clone::Clone::clone(&self.name),
                generic_args: ::core::clone::Clone::clone(&self.generic_args),
            }
        }
    }
    impl NamedType {
        #[inline]
        pub fn new(name: Ident, generic_args: Option<GenericTypeArgs>) -> Self {
            Self { name, generic_args }
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn generic_args(&self) -> Option<&GenericTypeArgs> {
            self.generic_args.as_ref()
        }
    }
    impl Spanned for NamedType {
        fn span(&self) -> TextSpan {
            if let Some(generic_args) = &self.generic_args {
                self.name.span().join(&generic_args.span())
            } else {
                self.name.span()
            }
        }
    }
    impl DisplayScoped for NamedType {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            if let Some(generic_args) = &self.generic_args {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&self.name),
                        ::core::fmt::ArgumentV1::new_display(&generic_args),
                    ],
                ))
            } else {
                DisplayScoped::fmt(&self.name, f)
            }
        }
    }
    impl std::fmt::Display for NamedType {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct ArrayType {
        open_bracket: Punct,
        item_ty: Box<Type>,
        sep: Punct,
        len: Expr,
        close_bracket: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArrayType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "ArrayType",
                "open_bracket",
                &&self.open_bracket,
                "item_ty",
                &&self.item_ty,
                "sep",
                &&self.sep,
                "len",
                &&self.len,
                "close_bracket",
                &&self.close_bracket,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArrayType {
        #[inline]
        fn clone(&self) -> ArrayType {
            ArrayType {
                open_bracket: ::core::clone::Clone::clone(&self.open_bracket),
                item_ty: ::core::clone::Clone::clone(&self.item_ty),
                sep: ::core::clone::Clone::clone(&self.sep),
                len: ::core::clone::Clone::clone(&self.len),
                close_bracket: ::core::clone::Clone::clone(&self.close_bracket),
            }
        }
    }
    impl ArrayType {
        #[inline]
        pub fn new(
            open_bracket: Punct,
            item_ty: Type,
            sep: Punct,
            len: Expr,
            close_bracket: Punct,
        ) -> Self {
            Self {
                open_bracket,
                item_ty: Box::new(item_ty),
                sep,
                len,
                close_bracket,
            }
        }
        #[inline]
        pub fn open_bracket(&self) -> &Punct {
            &self.open_bracket
        }
        #[inline]
        pub fn item_ty(&self) -> &Type {
            &self.item_ty
        }
        #[inline]
        pub fn sep(&self) -> &Punct {
            &self.sep
        }
        #[inline]
        pub fn len(&self) -> &Expr {
            &self.len
        }
        #[inline]
        pub fn close_bracket(&self) -> &Punct {
            &self.close_bracket
        }
    }
    impl Spanned for ArrayType {
        fn span(&self) -> TextSpan {
            self.open_bracket.span().join(&self.close_bracket.span())
        }
    }
    impl DisplayScoped for ArrayType {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", "", "", " ", ""],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.open_bracket),
                    ::core::fmt::ArgumentV1::new_display(&self.item_ty),
                    ::core::fmt::ArgumentV1::new_display(&self.sep),
                    ::core::fmt::ArgumentV1::new_display(&self.len),
                    ::core::fmt::ArgumentV1::new_display(&self.close_bracket),
                ],
            ))
        }
    }
    impl std::fmt::Display for ArrayType {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum Type {
        Named(NamedType),
        Array(ArrayType),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Type {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Type::Named(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Named", &__self_0)
                }
                Type::Array(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Array", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Type {
        #[inline]
        fn clone(&self) -> Type {
            match self {
                Type::Named(__self_0) => Type::Named(::core::clone::Clone::clone(__self_0)),
                Type::Array(__self_0) => Type::Array(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    impl Spanned for Type {
        fn span(&self) -> TextSpan {
            match self {
                Self::Named(ty) => ty.span(),
                Self::Array(ty) => ty.span(),
            }
        }
    }
    impl DisplayScoped for Type {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Named(ty) => DisplayScoped::fmt(ty, f),
                Self::Array(ty) => DisplayScoped::fmt(ty, f),
            }
        }
    }
    impl std::fmt::Display for Type {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Field {
        name: Ident,
        sep: Punct,
        ty: Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Field {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Field",
                "name",
                &&self.name,
                "sep",
                &&self.sep,
                "ty",
                &&self.ty,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Field {
        #[inline]
        fn clone(&self) -> Field {
            Field {
                name: ::core::clone::Clone::clone(&self.name),
                sep: ::core::clone::Clone::clone(&self.sep),
                ty: ::core::clone::Clone::clone(&self.ty),
            }
        }
    }
    impl Field {
        #[inline]
        pub fn new(name: Ident, sep: Punct, ty: Type) -> Self {
            Self { name, sep, ty }
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn sep(&self) -> &Punct {
            &self.sep
        }
        #[inline]
        pub fn ty(&self) -> &Type {
            &self.ty
        }
    }
    impl Spanned for Field {
        fn span(&self) -> TextSpan {
            self.name.span().join(&self.ty.span())
        }
    }
    impl DisplayScoped for Field {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", "", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.name),
                    ::core::fmt::ArgumentV1::new_display(&self.sep),
                    ::core::fmt::ArgumentV1::new_display(&self.ty),
                ],
            ))
        }
    }
    impl std::fmt::Display for Field {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct GenericStructArgs {
        open_paren: Punct,
        args: Vec<Ident>,
        close_paren: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GenericStructArgs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "GenericStructArgs",
                "open_paren",
                &&self.open_paren,
                "args",
                &&self.args,
                "close_paren",
                &&self.close_paren,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GenericStructArgs {
        #[inline]
        fn clone(&self) -> GenericStructArgs {
            GenericStructArgs {
                open_paren: ::core::clone::Clone::clone(&self.open_paren),
                args: ::core::clone::Clone::clone(&self.args),
                close_paren: ::core::clone::Clone::clone(&self.close_paren),
            }
        }
    }
    impl GenericStructArgs {
        #[inline]
        pub fn new(open_paren: Punct, args: Vec<Ident>, close_paren: Punct) -> Self {
            Self {
                open_paren,
                args,
                close_paren,
            }
        }
        #[inline]
        pub fn open_paren(&self) -> &Punct {
            &self.open_paren
        }
        #[inline]
        pub fn args(&self) -> &[Ident] {
            &self.args
        }
        #[inline]
        pub fn close_paren(&self) -> &Punct {
            &self.close_paren
        }
    }
    impl Spanned for GenericStructArgs {
        fn span(&self) -> TextSpan {
            self.open_paren.span().join(&self.close_paren.span())
        }
    }
    impl DisplayScoped for GenericStructArgs {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.open_paren)],
            ))?;
            for (i, arg) in self.args.iter().enumerate() {
                if i == 0 {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&arg)],
                    ))?;
                } else {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[", "],
                        &[::core::fmt::ArgumentV1::new_display(&arg)],
                    ))?;
                }
            }
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.close_paren)],
            ))
        }
    }
    impl std::fmt::Display for GenericStructArgs {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Struct {
        struct_kw: Keyword,
        name: Ident,
        generic_args: Option<GenericStructArgs>,
        open_curl: Punct,
        fields: Vec<Field>,
        close_curl: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Struct {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "struct_kw",
                "name",
                "generic_args",
                "open_curl",
                "fields",
                "close_curl",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &&self.struct_kw,
                &&self.name,
                &&self.generic_args,
                &&self.open_curl,
                &&self.fields,
                &&self.close_curl,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Struct", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Struct {
        #[inline]
        fn clone(&self) -> Struct {
            Struct {
                struct_kw: ::core::clone::Clone::clone(&self.struct_kw),
                name: ::core::clone::Clone::clone(&self.name),
                generic_args: ::core::clone::Clone::clone(&self.generic_args),
                open_curl: ::core::clone::Clone::clone(&self.open_curl),
                fields: ::core::clone::Clone::clone(&self.fields),
                close_curl: ::core::clone::Clone::clone(&self.close_curl),
            }
        }
    }
    impl Struct {
        #[inline]
        pub fn new(
            struct_kw: Keyword,
            name: Ident,
            generic_args: Option<GenericStructArgs>,
            open_curl: Punct,
            fields: Vec<Field>,
            close_curl: Punct,
        ) -> Self {
            Self {
                struct_kw,
                name,
                generic_args,
                open_curl,
                fields,
                close_curl,
            }
        }
        #[inline]
        pub fn struct_kw(&self) -> &Keyword {
            &self.struct_kw
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn generic_args(&self) -> Option<&GenericStructArgs> {
            self.generic_args.as_ref()
        }
        #[inline]
        pub fn open_curl(&self) -> &Punct {
            &self.open_curl
        }
        #[inline]
        pub fn fields(&self) -> &[Field] {
            &self.fields
        }
        #[inline]
        pub fn close_curl(&self) -> &Punct {
            &self.close_curl
        }
    }
    impl Spanned for Struct {
        fn span(&self) -> TextSpan {
            self.struct_kw.span().join(&self.close_curl.span())
        }
    }
    impl DisplayScoped for Struct {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            if let Some(generic_args) = &self.generic_args {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", " ", "", " ", "\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&self.struct_kw),
                        ::core::fmt::ArgumentV1::new_display(&self.name),
                        ::core::fmt::ArgumentV1::new_display(&generic_args),
                        ::core::fmt::ArgumentV1::new_display(&self.open_curl),
                    ],
                ))?;
            } else {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", " ", " ", "\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&self.struct_kw),
                        ::core::fmt::ArgumentV1::new_display(&self.name),
                        ::core::fmt::ArgumentV1::new_display(&self.open_curl),
                    ],
                ))?;
            }
            f.enter_scope();
            for field in self.fields.iter() {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ",\n"],
                    &[::core::fmt::ArgumentV1::new_display(&field)],
                ))?;
            }
            f.exit_scope();
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.close_curl)],
            ))
        }
    }
    impl std::fmt::Display for Struct {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Variant {
        name: Ident,
        value: Option<Expr>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Variant {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Variant",
                "name",
                &&self.name,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Variant {
        #[inline]
        fn clone(&self) -> Variant {
            Variant {
                name: ::core::clone::Clone::clone(&self.name),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl Variant {
        #[inline]
        pub fn new(name: Ident, value: Option<Expr>) -> Self {
            Self { name, value }
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn value(&self) -> Option<&Expr> {
            self.value.as_ref()
        }
    }
    impl Spanned for Variant {
        fn span(&self) -> TextSpan {
            if let Some(value) = &self.value {
                self.name.span().join(&value.span())
            } else {
                self.name.span()
            }
        }
    }
    impl DisplayScoped for Variant {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            if let Some(value) = &self.value {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", " = "],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&self.name),
                        ::core::fmt::ArgumentV1::new_display(&value),
                    ],
                ))
            } else {
                DisplayScoped::fmt(&self.name, f)
            }
        }
    }
    impl std::fmt::Display for Variant {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Enum {
        enum_kw: Keyword,
        name: Ident,
        ty_sep: Punct,
        base_ty: Type,
        open_curl: Punct,
        variants: Vec<Variant>,
        close_curl: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Enum {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "enum_kw",
                "name",
                "ty_sep",
                "base_ty",
                "open_curl",
                "variants",
                "close_curl",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &&self.enum_kw,
                &&self.name,
                &&self.ty_sep,
                &&self.base_ty,
                &&self.open_curl,
                &&self.variants,
                &&self.close_curl,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Enum", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Enum {
        #[inline]
        fn clone(&self) -> Enum {
            Enum {
                enum_kw: ::core::clone::Clone::clone(&self.enum_kw),
                name: ::core::clone::Clone::clone(&self.name),
                ty_sep: ::core::clone::Clone::clone(&self.ty_sep),
                base_ty: ::core::clone::Clone::clone(&self.base_ty),
                open_curl: ::core::clone::Clone::clone(&self.open_curl),
                variants: ::core::clone::Clone::clone(&self.variants),
                close_curl: ::core::clone::Clone::clone(&self.close_curl),
            }
        }
    }
    impl Enum {
        #[inline]
        pub fn new(
            enum_kw: Keyword,
            name: Ident,
            ty_sep: Punct,
            base_ty: Type,
            open_curl: Punct,
            variants: Vec<Variant>,
            close_curl: Punct,
        ) -> Self {
            Self {
                enum_kw,
                name,
                ty_sep,
                base_ty,
                open_curl,
                variants,
                close_curl,
            }
        }
        #[inline]
        pub fn enum_kw(&self) -> &Keyword {
            &self.enum_kw
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn ty_sep(&self) -> &Punct {
            &self.ty_sep
        }
        #[inline]
        pub fn base_ty(&self) -> &Type {
            &self.base_ty
        }
        #[inline]
        pub fn open_curl(&self) -> &Punct {
            &self.open_curl
        }
        #[inline]
        pub fn variants(&self) -> &[Variant] {
            &self.variants
        }
        #[inline]
        pub fn close_curl(&self) -> &Punct {
            &self.close_curl
        }
    }
    impl Spanned for Enum {
        fn span(&self) -> TextSpan {
            self.enum_kw.span().join(&self.close_curl.span())
        }
    }
    impl DisplayScoped for Enum {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", ": ", " ", "\n"],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.enum_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.name),
                    ::core::fmt::ArgumentV1::new_display(&self.base_ty),
                    ::core::fmt::ArgumentV1::new_display(&self.open_curl),
                ],
            ))?;
            f.enter_scope();
            for variant in self.variants.iter() {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ",\n"],
                    &[::core::fmt::ArgumentV1::new_display(&variant)],
                ))?;
            }
            f.exit_scope();
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.close_curl)],
            ))
        }
    }
    impl std::fmt::Display for Enum {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum LogicKind {
        Signal,
        Register,
        Module,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LogicKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                LogicKind::Signal => ::core::fmt::Formatter::write_str(f, "Signal"),
                LogicKind::Register => ::core::fmt::Formatter::write_str(f, "Register"),
                LogicKind::Module => ::core::fmt::Formatter::write_str(f, "Module"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for LogicKind {
        #[inline]
        fn clone(&self) -> LogicKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for LogicKind {}
    impl ::core::marker::StructuralPartialEq for LogicKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for LogicKind {
        #[inline]
        fn eq(&self, other: &LogicKind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    impl ::core::marker::StructuralEq for LogicKind {}
    #[automatically_derived]
    impl ::core::cmp::Eq for LogicKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for LogicKind {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    impl DisplayScoped for LogicKind {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Signal => f.write_fmt(::core::fmt::Arguments::new_v1(&["sig"], &[])),
                Self::Register => f.write_fmt(::core::fmt::Arguments::new_v1(&["reg"], &[])),
                Self::Module => f.write_fmt(::core::fmt::Arguments::new_v1(&["let"], &[])),
            }
        }
    }
    impl std::fmt::Display for LogicKind {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct LogicMode {
        kind: LogicKind,
        span: TextSpan,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LogicMode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "LogicMode",
                "kind",
                &&self.kind,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for LogicMode {
        #[inline]
        fn clone(&self) -> LogicMode {
            let _: ::core::clone::AssertParamIsClone<LogicKind>;
            let _: ::core::clone::AssertParamIsClone<TextSpan>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for LogicMode {}
    impl LogicMode {
        #[inline]
        pub fn new(kind: LogicKind, span: TextSpan) -> Self {
            Self { kind, span }
        }
        #[inline]
        pub fn kind(&self) -> LogicKind {
            self.kind
        }
    }
    impl Spanned for LogicMode {
        #[inline]
        fn span(&self) -> TextSpan {
            self.span
        }
    }
    impl DisplayScoped for LogicMode {
        #[inline]
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            DisplayScoped::fmt(&self.kind, f)
        }
    }
    impl std::fmt::Display for LogicMode {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum Direction {
        In,
        Out,
        InOut,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Direction {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Direction::In => ::core::fmt::Formatter::write_str(f, "In"),
                Direction::Out => ::core::fmt::Formatter::write_str(f, "Out"),
                Direction::InOut => ::core::fmt::Formatter::write_str(f, "InOut"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Direction {
        #[inline]
        fn clone(&self) -> Direction {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Direction {}
    impl ::core::marker::StructuralPartialEq for Direction {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Direction {
        #[inline]
        fn eq(&self, other: &Direction) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    impl ::core::marker::StructuralEq for Direction {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Direction {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Direction {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    impl DisplayScoped for Direction {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::In => f.write_fmt(::core::fmt::Arguments::new_v1(&["in"], &[])),
                Self::Out => f.write_fmt(::core::fmt::Arguments::new_v1(&["out"], &[])),
                Self::InOut => f.write_fmt(::core::fmt::Arguments::new_v1(&["inout"], &[])),
            }
        }
    }
    impl std::fmt::Display for Direction {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct PortMode {
        dir: Direction,
        span: TextSpan,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PortMode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "PortMode",
                "dir",
                &&self.dir,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PortMode {
        #[inline]
        fn clone(&self) -> PortMode {
            let _: ::core::clone::AssertParamIsClone<Direction>;
            let _: ::core::clone::AssertParamIsClone<TextSpan>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for PortMode {}
    impl PortMode {
        #[inline]
        pub fn new(dir: Direction, span: TextSpan) -> Self {
            Self { dir, span }
        }
        #[inline]
        pub fn dir(&self) -> Direction {
            self.dir
        }
    }
    impl Spanned for PortMode {
        #[inline]
        fn span(&self) -> TextSpan {
            self.span
        }
    }
    impl DisplayScoped for PortMode {
        #[inline]
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            DisplayScoped::fmt(&self.dir, f)
        }
    }
    impl std::fmt::Display for PortMode {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Port {
        mode: PortMode,
        logic_mode: LogicMode,
        name: Ident,
        sep: Punct,
        ty: Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Port {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Port",
                "mode",
                &&self.mode,
                "logic_mode",
                &&self.logic_mode,
                "name",
                &&self.name,
                "sep",
                &&self.sep,
                "ty",
                &&self.ty,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Port {
        #[inline]
        fn clone(&self) -> Port {
            Port {
                mode: ::core::clone::Clone::clone(&self.mode),
                logic_mode: ::core::clone::Clone::clone(&self.logic_mode),
                name: ::core::clone::Clone::clone(&self.name),
                sep: ::core::clone::Clone::clone(&self.sep),
                ty: ::core::clone::Clone::clone(&self.ty),
            }
        }
    }
    impl Port {
        #[inline]
        pub fn new(
            mode: PortMode,
            logic_mode: LogicMode,
            name: Ident,
            sep: Punct,
            ty: Type,
        ) -> Self {
            Self {
                mode,
                logic_mode,
                name,
                sep,
                ty,
            }
        }
        #[inline]
        pub fn mode(&self) -> &PortMode {
            &self.mode
        }
        #[inline]
        pub fn logic_mode(&self) -> &LogicMode {
            &self.logic_mode
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn sep(&self) -> &Punct {
            &self.sep
        }
        #[inline]
        pub fn ty(&self) -> &Type {
            &self.ty
        }
    }
    impl Spanned for Port {
        fn span(&self) -> TextSpan {
            self.mode.span().join(&self.ty.span())
        }
    }
    impl DisplayScoped for Port {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", " ", "", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.mode),
                    ::core::fmt::ArgumentV1::new_display(&self.logic_mode),
                    ::core::fmt::ArgumentV1::new_display(&self.name),
                    ::core::fmt::ArgumentV1::new_display(&self.sep),
                    ::core::fmt::ArgumentV1::new_display(&self.ty),
                ],
            ))
        }
    }
    impl std::fmt::Display for Port {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Const {
        const_kw: Keyword,
        name: Ident,
        assign: Punct,
        value: Expr,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Const {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Const",
                "const_kw",
                &&self.const_kw,
                "name",
                &&self.name,
                "assign",
                &&self.assign,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Const {
        #[inline]
        fn clone(&self) -> Const {
            Const {
                const_kw: ::core::clone::Clone::clone(&self.const_kw),
                name: ::core::clone::Clone::clone(&self.name),
                assign: ::core::clone::Clone::clone(&self.assign),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl Const {
        #[inline]
        pub fn new(const_kw: Keyword, name: Ident, assign: Punct, value: Expr) -> Self {
            Self {
                const_kw,
                name,
                assign,
                value,
            }
        }
        #[inline]
        pub fn const_kw(&self) -> &Keyword {
            &self.const_kw
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn assign(&self) -> &Punct {
            &self.assign
        }
        #[inline]
        pub fn value(&self) -> &Expr {
            &self.value
        }
    }
    impl Spanned for Const {
        fn span(&self) -> TextSpan {
            self.const_kw.span().join(&self.value.span())
        }
    }
    impl DisplayScoped for Const {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", " ", " ", ";"],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.const_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.name),
                    ::core::fmt::ArgumentV1::new_display(&self.assign),
                    ::core::fmt::ArgumentV1::new_display(&self.value),
                ],
            ))
        }
    }
    impl std::fmt::Display for Const {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum EdgeKind {
        Rising,
        Falling,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EdgeKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                EdgeKind::Rising => ::core::fmt::Formatter::write_str(f, "Rising"),
                EdgeKind::Falling => ::core::fmt::Formatter::write_str(f, "Falling"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EdgeKind {
        #[inline]
        fn clone(&self) -> EdgeKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EdgeKind {}
    impl ::core::marker::StructuralPartialEq for EdgeKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EdgeKind {
        #[inline]
        fn eq(&self, other: &EdgeKind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    impl ::core::marker::StructuralEq for EdgeKind {}
    #[automatically_derived]
    impl ::core::cmp::Eq for EdgeKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for EdgeKind {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    impl DisplayScoped for EdgeKind {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Rising => f.write_fmt(::core::fmt::Arguments::new_v1(&["rising"], &[])),
                Self::Falling => f.write_fmt(::core::fmt::Arguments::new_v1(&["falling"], &[])),
            }
        }
    }
    impl std::fmt::Display for EdgeKind {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Edge {
        kind: EdgeKind,
        span: TextSpan,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Edge {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Edge",
                "kind",
                &&self.kind,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Edge {
        #[inline]
        fn clone(&self) -> Edge {
            let _: ::core::clone::AssertParamIsClone<EdgeKind>;
            let _: ::core::clone::AssertParamIsClone<TextSpan>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Edge {}
    impl Edge {
        #[inline]
        pub fn new(kind: EdgeKind, span: TextSpan) -> Self {
            Self { kind, span }
        }
        #[inline]
        pub fn kind(&self) -> EdgeKind {
            self.kind
        }
    }
    impl Spanned for Edge {
        #[inline]
        fn span(&self) -> TextSpan {
            self.span
        }
    }
    impl DisplayScoped for Edge {
        #[inline]
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            DisplayScoped::fmt(&self.kind, f)
        }
    }
    impl std::fmt::Display for Edge {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Sens {
        edge: Edge,
        open_paren: Punct,
        sig: Path,
        close_paren: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Sens {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Sens",
                "edge",
                &&self.edge,
                "open_paren",
                &&self.open_paren,
                "sig",
                &&self.sig,
                "close_paren",
                &&self.close_paren,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Sens {
        #[inline]
        fn clone(&self) -> Sens {
            Sens {
                edge: ::core::clone::Clone::clone(&self.edge),
                open_paren: ::core::clone::Clone::clone(&self.open_paren),
                sig: ::core::clone::Clone::clone(&self.sig),
                close_paren: ::core::clone::Clone::clone(&self.close_paren),
            }
        }
    }
    impl Sens {
        #[inline]
        pub fn new(edge: Edge, open_paren: Punct, sig: Path, close_paren: Punct) -> Self {
            Self {
                edge,
                open_paren,
                sig,
                close_paren,
            }
        }
        #[inline]
        pub fn edge(&self) -> &Edge {
            &self.edge
        }
        #[inline]
        pub fn open_paren(&self) -> &Punct {
            &self.open_paren
        }
        #[inline]
        pub fn sig(&self) -> &Path {
            &self.sig
        }
        #[inline]
        pub fn close_paren(&self) -> &Punct {
            &self.close_paren
        }
    }
    impl Spanned for Sens {
        fn span(&self) -> TextSpan {
            self.edge.span().join(&self.close_paren.span())
        }
    }
    impl DisplayScoped for Sens {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", "", "", ""],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.edge),
                    ::core::fmt::ArgumentV1::new_display(&self.open_paren),
                    ::core::fmt::ArgumentV1::new_display(&self.sig),
                    ::core::fmt::ArgumentV1::new_display(&self.close_paren),
                ],
            ))
        }
    }
    impl std::fmt::Display for Sens {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct LogicMember {
        mode: LogicMode,
        name: Ident,
        sep: Punct,
        ty: Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LogicMember {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "LogicMember",
                "mode",
                &&self.mode,
                "name",
                &&self.name,
                "sep",
                &&self.sep,
                "ty",
                &&self.ty,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for LogicMember {
        #[inline]
        fn clone(&self) -> LogicMember {
            LogicMember {
                mode: ::core::clone::Clone::clone(&self.mode),
                name: ::core::clone::Clone::clone(&self.name),
                sep: ::core::clone::Clone::clone(&self.sep),
                ty: ::core::clone::Clone::clone(&self.ty),
            }
        }
    }
    impl LogicMember {
        #[inline]
        pub fn new(mode: LogicMode, name: Ident, sep: Punct, ty: Type) -> Self {
            Self {
                mode,
                name,
                sep,
                ty,
            }
        }
        #[inline]
        pub fn mode(&self) -> &LogicMode {
            &self.mode
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn sep(&self) -> &Punct {
            &self.sep
        }
        #[inline]
        pub fn ty(&self) -> &Type {
            &self.ty
        }
    }
    impl Spanned for LogicMember {
        fn span(&self) -> TextSpan {
            self.mode.span().join(&self.ty.span())
        }
    }
    impl DisplayScoped for LogicMember {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", "", " ", ";"],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.mode),
                    ::core::fmt::ArgumentV1::new_display(&self.name),
                    ::core::fmt::ArgumentV1::new_display(&self.sep),
                    ::core::fmt::ArgumentV1::new_display(&self.ty),
                ],
            ))
        }
    }
    impl std::fmt::Display for LogicMember {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct ProcMember {
        proc_kw: Keyword,
        sens: Vec<Sens>,
        body: Box<Block>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ProcMember {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ProcMember",
                "proc_kw",
                &&self.proc_kw,
                "sens",
                &&self.sens,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ProcMember {
        #[inline]
        fn clone(&self) -> ProcMember {
            ProcMember {
                proc_kw: ::core::clone::Clone::clone(&self.proc_kw),
                sens: ::core::clone::Clone::clone(&self.sens),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl ProcMember {
        #[inline]
        pub fn new(proc_kw: Keyword, sens: Vec<Sens>, body: Block) -> Self {
            Self {
                proc_kw,
                sens,
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn proc_kw(&self) -> &Keyword {
            &self.proc_kw
        }
        #[inline]
        pub fn sens(&self) -> &[Sens] {
            &self.sens
        }
        #[inline]
        pub fn body(&self) -> &Block {
            &self.body
        }
    }
    impl Spanned for ProcMember {
        fn span(&self) -> TextSpan {
            self.proc_kw.span().join(&self.body.span())
        }
    }
    impl DisplayScoped for ProcMember {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " "],
                &[::core::fmt::ArgumentV1::new_display(&self.proc_kw)],
            ))?;
            for (i, sens) in self.sens.iter().enumerate() {
                if i > 0 {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[" | "],
                        &[::core::fmt::ArgumentV1::new_display(&sens)],
                    ))?;
                } else {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&sens)],
                    ))?;
                }
            }
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[" "],
                &[::core::fmt::ArgumentV1::new_display(&self.body)],
            ))
        }
    }
    impl std::fmt::Display for ProcMember {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct CombMember {
        comb_kw: Keyword,
        body: Box<Block>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CombMember {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CombMember",
                "comb_kw",
                &&self.comb_kw,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CombMember {
        #[inline]
        fn clone(&self) -> CombMember {
            CombMember {
                comb_kw: ::core::clone::Clone::clone(&self.comb_kw),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl CombMember {
        #[inline]
        pub fn new(comb_kw: Keyword, body: Block) -> Self {
            Self {
                comb_kw,
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn comb_kw(&self) -> &Keyword {
            &self.comb_kw
        }
        #[inline]
        pub fn body(&self) -> &Block {
            &self.body
        }
    }
    impl Spanned for CombMember {
        fn span(&self) -> TextSpan {
            self.comb_kw.span().join(&self.body.span())
        }
    }
    impl DisplayScoped for CombMember {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.comb_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.body),
                ],
            ))
        }
    }
    impl std::fmt::Display for CombMember {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum Member {
        Logic(LogicMember),
        Const(Const),
        Proc(ProcMember),
        Comb(CombMember),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Member {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Member::Logic(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Logic", &__self_0)
                }
                Member::Const(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Const", &__self_0)
                }
                Member::Proc(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Proc", &__self_0)
                }
                Member::Comb(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Comb", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Member {
        #[inline]
        fn clone(&self) -> Member {
            match self {
                Member::Logic(__self_0) => Member::Logic(::core::clone::Clone::clone(__self_0)),
                Member::Const(__self_0) => Member::Const(::core::clone::Clone::clone(__self_0)),
                Member::Proc(__self_0) => Member::Proc(::core::clone::Clone::clone(__self_0)),
                Member::Comb(__self_0) => Member::Comb(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    impl Spanned for Member {
        fn span(&self) -> TextSpan {
            match self {
                Self::Logic(member) => member.span(),
                Self::Const(member) => member.span(),
                Self::Proc(member) => member.span(),
                Self::Comb(member) => member.span(),
            }
        }
    }
    impl DisplayScoped for Member {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Logic(member) => DisplayScoped::fmt(member, f),
                Self::Const(member) => DisplayScoped::fmt(member, f),
                Self::Proc(member) => DisplayScoped::fmt(member, f),
                Self::Comb(member) => DisplayScoped::fmt(member, f),
            }
        }
    }
    impl std::fmt::Display for Member {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Module {
        mod_kw: Keyword,
        name: Ident,
        generic_args: Option<GenericStructArgs>,
        open_paren: Punct,
        ports: Vec<Port>,
        close_paren: Punct,
        open_curl: Punct,
        members: Vec<Member>,
        close_curl: Punct,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Module {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "mod_kw",
                "name",
                "generic_args",
                "open_paren",
                "ports",
                "close_paren",
                "open_curl",
                "members",
                "close_curl",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &&self.mod_kw,
                &&self.name,
                &&self.generic_args,
                &&self.open_paren,
                &&self.ports,
                &&self.close_paren,
                &&self.open_curl,
                &&self.members,
                &&self.close_curl,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Module", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Module {
        #[inline]
        fn clone(&self) -> Module {
            Module {
                mod_kw: ::core::clone::Clone::clone(&self.mod_kw),
                name: ::core::clone::Clone::clone(&self.name),
                generic_args: ::core::clone::Clone::clone(&self.generic_args),
                open_paren: ::core::clone::Clone::clone(&self.open_paren),
                ports: ::core::clone::Clone::clone(&self.ports),
                close_paren: ::core::clone::Clone::clone(&self.close_paren),
                open_curl: ::core::clone::Clone::clone(&self.open_curl),
                members: ::core::clone::Clone::clone(&self.members),
                close_curl: ::core::clone::Clone::clone(&self.close_curl),
            }
        }
    }
    impl Module {
        #[inline]
        pub fn new(
            mod_kw: Keyword,
            name: Ident,
            generic_args: Option<GenericStructArgs>,
            open_paren: Punct,
            ports: Vec<Port>,
            close_paren: Punct,
            open_curl: Punct,
            members: Vec<Member>,
            close_curl: Punct,
        ) -> Self {
            Self {
                mod_kw,
                name,
                generic_args,
                open_paren,
                ports,
                close_paren,
                open_curl,
                members,
                close_curl,
            }
        }
        #[inline]
        pub fn mod_kw(&self) -> &Keyword {
            &self.mod_kw
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn generic_args(&self) -> Option<&GenericStructArgs> {
            self.generic_args.as_ref()
        }
        #[inline]
        pub fn open_paren(&self) -> &Punct {
            &self.open_paren
        }
        #[inline]
        pub fn ports(&self) -> &[Port] {
            &self.ports
        }
        #[inline]
        pub fn close_paren(&self) -> &Punct {
            &self.close_paren
        }
        #[inline]
        pub fn open_curl(&self) -> &Punct {
            &self.open_curl
        }
        #[inline]
        pub fn members(&self) -> &[Member] {
            &self.members
        }
        #[inline]
        pub fn close_curl(&self) -> &Punct {
            &self.close_curl
        }
    }
    impl Spanned for Module {
        fn span(&self) -> TextSpan {
            self.mod_kw.span().join(&self.close_curl.span())
        }
    }
    impl DisplayScoped for Module {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            if let Some(generic_args) = &self.generic_args {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", " ", "", " ", "\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&self.mod_kw),
                        ::core::fmt::ArgumentV1::new_display(&self.name),
                        ::core::fmt::ArgumentV1::new_display(&generic_args),
                        ::core::fmt::ArgumentV1::new_display(&self.open_paren),
                    ],
                ))?;
            } else {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", " ", " ", "\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&self.mod_kw),
                        ::core::fmt::ArgumentV1::new_display(&self.name),
                        ::core::fmt::ArgumentV1::new_display(&self.open_paren),
                    ],
                ))?;
            }
            f.enter_scope();
            for port in self.ports.iter() {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ",\n"],
                    &[::core::fmt::ArgumentV1::new_display(&port)],
                ))?;
            }
            f.exit_scope();
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", "\n"],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.close_paren),
                    ::core::fmt::ArgumentV1::new_display(&self.open_curl),
                ],
            ))?;
            f.enter_scope();
            for member in self.members.iter() {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&member)],
                ))?;
            }
            f.exit_scope();
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self.close_curl)],
            ))
        }
    }
    impl std::fmt::Display for Module {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub struct Func {
        fn_kw: Keyword,
        name: Ident,
        open_paren: Punct,
        args: Vec<Ident>,
        close_paren: Punct,
        body: Box<Block>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Func {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &["fn_kw", "name", "open_paren", "args", "close_paren", "body"];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &&self.fn_kw,
                &&self.name,
                &&self.open_paren,
                &&self.args,
                &&self.close_paren,
                &&self.body,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Func", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Func {
        #[inline]
        fn clone(&self) -> Func {
            Func {
                fn_kw: ::core::clone::Clone::clone(&self.fn_kw),
                name: ::core::clone::Clone::clone(&self.name),
                open_paren: ::core::clone::Clone::clone(&self.open_paren),
                args: ::core::clone::Clone::clone(&self.args),
                close_paren: ::core::clone::Clone::clone(&self.close_paren),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl Func {
        #[inline]
        pub fn new(
            fn_kw: Keyword,
            name: Ident,
            open_paren: Punct,
            args: Vec<Ident>,
            close_paren: Punct,
            body: Block,
        ) -> Self {
            Self {
                fn_kw,
                name,
                open_paren,
                args,
                close_paren,
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn fn_kw(&self) -> &Keyword {
            &self.fn_kw
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn open_paren(&self) -> &Punct {
            &self.open_paren
        }
        #[inline]
        pub fn args(&self) -> &[Ident] {
            &self.args
        }
        #[inline]
        pub fn close_paren(&self) -> &Punct {
            &self.close_paren
        }
        #[inline]
        pub fn body(&self) -> &Block {
            &self.body
        }
    }
    impl Spanned for Func {
        fn span(&self) -> TextSpan {
            self.fn_kw.span().join(&self.body.span())
        }
    }
    impl DisplayScoped for Func {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " ", ""],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.fn_kw),
                    ::core::fmt::ArgumentV1::new_display(&self.name),
                    ::core::fmt::ArgumentV1::new_display(&self.open_paren),
                ],
            ))?;
            for (i, arg) in self.args.iter().enumerate() {
                if i > 0 {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[", "],
                        &[::core::fmt::ArgumentV1::new_display(&arg)],
                    ))?;
                } else {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&arg)],
                    ))?;
                }
            }
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", " "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&self.close_paren),
                    ::core::fmt::ArgumentV1::new_display(&self.body),
                ],
            ))
        }
    }
    impl std::fmt::Display for Func {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    pub enum Item {
        Struct(Struct),
        Enum(Enum),
        Const(Const),
        Module(Module),
        Func(Func),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Item {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Item::Struct(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Struct", &__self_0)
                }
                Item::Enum(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Enum", &__self_0)
                }
                Item::Const(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Const", &__self_0)
                }
                Item::Module(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Module", &__self_0)
                }
                Item::Func(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Func", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Item {
        #[inline]
        fn clone(&self) -> Item {
            match self {
                Item::Struct(__self_0) => Item::Struct(::core::clone::Clone::clone(__self_0)),
                Item::Enum(__self_0) => Item::Enum(::core::clone::Clone::clone(__self_0)),
                Item::Const(__self_0) => Item::Const(::core::clone::Clone::clone(__self_0)),
                Item::Module(__self_0) => Item::Module(::core::clone::Clone::clone(__self_0)),
                Item::Func(__self_0) => Item::Func(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    impl Item {
        pub fn name(&self) -> &Ident {
            match self {
                Self::Struct(struct_item) => struct_item.name(),
                Self::Enum(enum_item) => enum_item.name(),
                Self::Const(const_item) => const_item.name(),
                Self::Module(module_item) => module_item.name(),
                Self::Func(func_item) => func_item.name(),
            }
        }
    }
    impl Spanned for Item {
        fn span(&self) -> TextSpan {
            match self {
                Self::Struct(struct_item) => struct_item.span(),
                Self::Enum(enum_item) => enum_item.span(),
                Self::Const(const_item) => const_item.span(),
                Self::Module(module_item) => module_item.span(),
                Self::Func(func_item) => func_item.span(),
            }
        }
    }
    impl DisplayScoped for Item {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            match self {
                Self::Struct(struct_item) => DisplayScoped::fmt(struct_item, f),
                Self::Enum(enum_item) => DisplayScoped::fmt(enum_item, f),
                Self::Const(const_item) => DisplayScoped::fmt(const_item, f),
                Self::Module(module_item) => DisplayScoped::fmt(module_item, f),
                Self::Func(fn_item) => DisplayScoped::fmt(fn_item, f),
            }
        }
    }
    impl std::fmt::Display for Item {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
}
mod const_eval {
    use crate::ast::{AssignKind, Ident};
    use crate::ir::*;
    use crate::SharedString;
    use std::collections::HashMap;
    pub enum ConstValue<'a> {
        Value(i64),
        Expr(&'a ConstExpr),
    }
    pub struct VarScope<'p> {
        parent: Option<&'p mut VarScope<'p>>,
        vars: HashMap<SharedString, i64>,
    }
    impl<'p> VarScope<'p> {
        pub fn empty() -> Self {
            Self {
                parent: None,
                vars: HashMap::new(),
            }
        }
        pub fn new<'pp: 'p>(parent: &'p mut VarScope<'pp>) -> Self {
            Self {
                parent: Some(unsafe { std::mem::transmute(parent) }),
                vars: HashMap::new(),
            }
        }
        pub fn add_var(&mut self, name: &Ident, value: i64) {
            self.vars.insert(name.as_string(), value);
        }
        fn var_inner(&self, name: &Ident) -> Option<i64> {
            self.vars.get(name.as_ref()).copied().or_else(|| {
                self.parent
                    .as_ref()
                    .and_then(|parent| parent.var_inner(name))
            })
        }
        pub fn var<'a>(
            &self,
            name: &Ident,
            global_consts: &'a HashMap<SharedString, ConstExpr>,
            local_consts: Option<&'a HashMap<SharedString, ConstExpr>>,
        ) -> ConstValue<'a> {
            if let Some(value) = self.var_inner(name) {
                ConstValue::Value(value)
            } else if let Some(expr) = global_consts.get(name.as_ref()) {
                ConstValue::Expr(expr)
            } else if let Some(expr) =
                local_consts.and_then(|local_consts| local_consts.get(name.as_ref()))
            {
                ConstValue::Expr(expr)
            } else {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &[::core::fmt::ArgumentV1::new_display(
                        &::core::fmt::Arguments::new_v1(&["variable not found"], &[]),
                    )],
                ))
            }
        }
        pub fn var_mut(&mut self, name: &Ident) -> &mut i64 {
            self.vars
                .get_mut(name.as_ref())
                .or_else(|| {
                    self.parent
                        .as_mut()
                        .and_then(|parent| Some(parent.var_mut(name)))
                })
                .expect("variable not found")
        }
    }
    fn pattern_matches(
        patterns: &[ConstMatchPattern],
        value: i64,
        scope: &mut VarScope,
        consts: &HashMap<SharedString, ConstExpr>,
        funcs: &HashMap<SharedString, ConstFunc>,
    ) -> bool {
        for pattern in patterns.iter() {
            match pattern {
                ConstMatchPattern::Literal(l) => {
                    if l.value() == value {
                        return true;
                    }
                }
                ConstMatchPattern::Ident(ident) => {
                    if ident.as_ref() == "_" {
                        return true;
                    }
                    let pattern_value = eval(&consts[ident.as_ref()], scope, consts, funcs);
                    if pattern_value == value {
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn eval(
        expr: &ConstExpr,
        scope: &mut VarScope,
        consts: &HashMap<SharedString, ConstExpr>,
        funcs: &HashMap<SharedString, ConstFunc>,
    ) -> i64 {
        match expr {
            ConstExpr::Literal(l) => l.value(),
            ConstExpr::Ident(name) => match scope.var(name, consts, None) {
                ConstValue::Value(value) => value,
                ConstValue::Expr(expr) => eval(expr, scope, consts, funcs),
            },
            ConstExpr::Call(expr) => {
                let mut arg_values = Vec::with_capacity(expr.args().len());
                for arg in expr.args().iter() {
                    arg_values.push(eval(arg, scope, consts, funcs));
                }
                let func = &funcs[expr.func().as_ref()];
                let mut inner_scope = VarScope::new(scope);
                for (arg, value) in func.args().iter().zip(arg_values.into_iter()) {
                    inner_scope.add_var(arg, value);
                }
                eval_expr_block(func.body(), &mut inner_scope, consts, funcs)
            }
            ConstExpr::If(expr) => {
                if eval(expr.condition(), scope, consts, funcs) != 0 {
                    eval_expr_block(expr.body(), scope, consts, funcs)
                } else {
                    for else_if_block in expr.else_if_blocks().iter() {
                        if eval(else_if_block.condition(), scope, consts, funcs) != 0 {
                            return eval_expr_block(else_if_block.body(), scope, consts, funcs);
                        }
                    }
                    let else_block = expr.else_block().expect("missing else block");
                    eval_expr_block(else_block.body(), scope, consts, funcs)
                }
            }
            ConstExpr::Match(expr) => {
                let value = eval(expr.value(), scope, consts, funcs);
                for branch in expr.branches().iter() {
                    if pattern_matches(branch.patterns(), value, scope, consts, funcs) {
                        return match branch.body() {
                            ConstMatchBody::Expr(body) => eval(body, scope, consts, funcs),
                            ConstMatchBody::Block(body) => {
                                eval_expr_block(body, scope, consts, funcs)
                            }
                        };
                    }
                }
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &[::core::fmt::ArgumentV1::new_display(
                        &::core::fmt::Arguments::new_v1(&["unhandled match case"], &[]),
                    )],
                ));
            }
            ConstExpr::Block(block) => eval_expr_block(&block, scope, consts, funcs),
            ConstExpr::Neg(expr) => -eval(expr.inner(), scope, consts, funcs),
            ConstExpr::Not(expr) => !eval(expr.inner(), scope, consts, funcs),
            ConstExpr::Lt(expr) => {
                (eval(expr.lhs(), scope, consts, funcs) < eval(expr.rhs(), scope, consts, funcs))
                    as i64
            }
            ConstExpr::Lte(expr) => {
                (eval(expr.lhs(), scope, consts, funcs) <= eval(expr.rhs(), scope, consts, funcs))
                    as i64
            }
            ConstExpr::Gt(expr) => {
                (eval(expr.lhs(), scope, consts, funcs) > eval(expr.rhs(), scope, consts, funcs))
                    as i64
            }
            ConstExpr::Gte(expr) => {
                (eval(expr.lhs(), scope, consts, funcs) >= eval(expr.rhs(), scope, consts, funcs))
                    as i64
            }
            ConstExpr::Eq(expr) => {
                (eval(expr.lhs(), scope, consts, funcs) == eval(expr.rhs(), scope, consts, funcs))
                    as i64
            }
            ConstExpr::Ne(expr) => {
                (eval(expr.lhs(), scope, consts, funcs) != eval(expr.rhs(), scope, consts, funcs))
                    as i64
            }
            ConstExpr::Add(expr) => {
                eval(expr.lhs(), scope, consts, funcs) + eval(expr.rhs(), scope, consts, funcs)
            }
            ConstExpr::Sub(expr) => {
                eval(expr.lhs(), scope, consts, funcs) - eval(expr.rhs(), scope, consts, funcs)
            }
            ConstExpr::Mul(expr) => {
                eval(expr.lhs(), scope, consts, funcs) * eval(expr.rhs(), scope, consts, funcs)
            }
            ConstExpr::Div(expr) => {
                eval(expr.lhs(), scope, consts, funcs) / eval(expr.rhs(), scope, consts, funcs)
            }
            ConstExpr::Rem(expr) => {
                eval(expr.lhs(), scope, consts, funcs) % eval(expr.rhs(), scope, consts, funcs)
            }
            ConstExpr::And(expr) => {
                eval(expr.lhs(), scope, consts, funcs) & eval(expr.rhs(), scope, consts, funcs)
            }
            ConstExpr::Xor(expr) => {
                eval(expr.lhs(), scope, consts, funcs) ^ eval(expr.rhs(), scope, consts, funcs)
            }
            ConstExpr::Or(expr) => {
                eval(expr.lhs(), scope, consts, funcs) | eval(expr.rhs(), scope, consts, funcs)
            }
            ConstExpr::Shl(expr) => {
                eval(expr.lhs(), scope, consts, funcs) << eval(expr.rhs(), scope, consts, funcs)
            }
            ConstExpr::Lsr(expr) => {
                ((eval(expr.lhs(), scope, consts, funcs) as u64)
                    >> (eval(expr.rhs(), scope, consts, funcs) as u64)) as i64
            }
            ConstExpr::Asr(expr) => {
                eval(expr.lhs(), scope, consts, funcs) >> eval(expr.rhs(), scope, consts, funcs)
            }
        }
    }
    fn eval_statement(
        statement: &ConstStatement,
        scope: &mut VarScope,
        consts: &HashMap<SharedString, ConstExpr>,
        funcs: &HashMap<SharedString, ConstFunc>,
    ) {
        match statement {
            ConstStatement::Expr(expr) => match expr {
                ConstExpr::If(expr) => {
                    if eval(expr.condition(), scope, consts, funcs) != 0 {
                        eval_statement_block(expr.body(), scope, consts, funcs);
                    } else {
                        for else_if_block in expr.else_if_blocks().iter() {
                            if eval(else_if_block.condition(), scope, consts, funcs) != 0 {
                                eval_statement_block(else_if_block.body(), scope, consts, funcs);
                                break;
                            }
                        }
                        if let Some(else_block) = expr.else_block() {
                            eval_statement_block(else_block.body(), scope, consts, funcs);
                        }
                    }
                }
                ConstExpr::Match(expr) => {
                    let value = eval(expr.value(), scope, consts, funcs);
                    for branch in expr.branches().iter() {
                        if pattern_matches(branch.patterns(), value, scope, consts, funcs) {
                            match branch.body() {
                                ConstMatchBody::Expr(body) => {
                                    eval(body, scope, consts, funcs);
                                }
                                ConstMatchBody::Block(body) => {
                                    eval_statement_block(body, scope, consts, funcs)
                                }
                            }
                            return;
                        }
                    }
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &[::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(&["unhandled match case"], &[]),
                        )],
                    ));
                }
                ConstExpr::Block(block) => eval_statement_block(&block, scope, consts, funcs),
                _ => {
                    eval(expr, scope, consts, funcs);
                }
            },
            ConstStatement::Declaration(decl) => {
                let value = eval(decl.value(), scope, consts, funcs);
                scope.add_var(decl.name(), value);
            }
            ConstStatement::Assignment(assign) => {
                let value = eval(assign.value(), scope, consts, funcs);
                let var = scope.var_mut(assign.target());
                match assign.kind() {
                    AssignKind::Assign => *var = value,
                    AssignKind::AddAssign => *var += value,
                    AssignKind::SubAssign => *var -= value,
                    AssignKind::MulAssign => *var *= value,
                    AssignKind::DivAssign => *var /= value,
                    AssignKind::RemAssign => *var %= value,
                    AssignKind::AndAssign => *var &= value,
                    AssignKind::OrAssign => *var |= value,
                    AssignKind::XorAssign => *var ^= value,
                    AssignKind::ShlAssign => *var <<= value,
                    AssignKind::AsrAssign => *var = ((*var as u64) >> (value as u64)) as i64,
                    AssignKind::LsrAssign => *var >>= value,
                }
            }
            ConstStatement::WhileLoop(while_loop) => {
                while eval(while_loop.condition(), scope, consts, funcs) != 0 {
                    eval_statement_block(while_loop.body(), scope, consts, funcs);
                }
            }
            ConstStatement::ForLoop(for_loop) => {
                let start = eval(&for_loop.range().start, scope, consts, funcs);
                let end = eval(&for_loop.range().end, scope, consts, funcs);
                for loop_index in start..end {
                    let mut inner_scope = VarScope::new(scope);
                    inner_scope.add_var(for_loop.item_name(), loop_index);
                    eval_statement_block(for_loop.body(), &mut inner_scope, consts, funcs);
                }
            }
        }
    }
    fn eval_block(
        block: &ConstBlock,
        parent_scope: &mut VarScope,
        consts: &HashMap<SharedString, ConstExpr>,
        funcs: &HashMap<SharedString, ConstFunc>,
    ) -> Option<i64> {
        let mut scope = VarScope::new(parent_scope);
        for statement in block.statements().iter() {
            eval_statement(statement, &mut scope, consts, funcs);
        }
        if let Some(result) = block.result() {
            Some(eval(result, &mut scope, consts, funcs))
        } else {
            None
        }
    }
    fn eval_expr_block(
        block: &ConstBlock,
        parent_scope: &mut VarScope,
        consts: &HashMap<SharedString, ConstExpr>,
        funcs: &HashMap<SharedString, ConstFunc>,
    ) -> i64 {
        eval_block(block, parent_scope, consts, funcs).expect("block is not an expression")
    }
    fn eval_statement_block(
        block: &ConstBlock,
        parent_scope: &mut VarScope,
        consts: &HashMap<SharedString, ConstExpr>,
        funcs: &HashMap<SharedString, ConstFunc>,
    ) {
        if !eval_block(block, parent_scope, consts, funcs).is_none() {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["block is not a statement"],
                &[],
            ))
        };
    }
}
mod fmt {
    use std::fmt;
    pub struct ScopedFormatter<'a, 'f: 'a> {
        inner: &'a mut fmt::Formatter<'f>,
        level: usize,
        at_line_start: bool,
    }
    impl<'a, 'f: 'a> ScopedFormatter<'a, 'f> {
        #[inline]
        pub fn enter_scope(&mut self) {
            self.level += 1;
        }
        #[inline]
        pub fn exit_scope(&mut self) {
            if true {
                if !(self.level > 0) {
                    ::core::panicking::panic("assertion failed: self.level > 0")
                };
            };
            self.level -= 1;
        }
    }
    impl<'a, 'f: 'a> fmt::Write for ScopedFormatter<'a, 'f> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if s.len() > 0 {
                let indent_width = self.level * 4;
                for (i, line) in s.lines().enumerate() {
                    if i > 0 {
                        self.inner
                            .write_fmt(::core::fmt::Arguments::new_v1(&["\n"], &[]))?;
                    }
                    if line.len() > 0 {
                        if (i > 0) || self.at_line_start {
                            self.inner
                                .write_fmt(::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&""),
                                        ::core::fmt::ArgumentV1::from_usize(&indent_width),
                                    ],
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Param(1usize),
                                        },
                                    }],
                                    unsafe { ::core::fmt::UnsafeArg::new() },
                                ))?;
                        }
                        self.inner.write_str(line)?;
                    }
                }
                self.at_line_start = s.ends_with('\n');
                if self.at_line_start {
                    self.inner
                        .write_fmt(::core::fmt::Arguments::new_v1(&["\n"], &[]))?;
                }
            }
            Ok(())
        }
    }
    pub trait ToScoped<'f> {
        fn to_scoped<'a>(&'a mut self) -> ScopedFormatter<'a, 'f>
        where
            'f: 'a;
    }
    impl<'f> ToScoped<'f> for fmt::Formatter<'f> {
        fn to_scoped<'a>(&'a mut self) -> ScopedFormatter<'a, 'f>
        where
            'f: 'a,
        {
            ScopedFormatter {
                inner: self,
                level: 0,
                at_line_start: true,
            }
        }
    }
    pub trait DisplayScoped {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> fmt::Result;
    }
    impl DisplayScoped for crate::SharedString {
        #[inline]
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> fmt::Result {
            use std::fmt::Write;
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self)],
            ))
        }
    }
    impl DisplayScoped for i64 {
        #[inline]
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> fmt::Result {
            use std::fmt::Write;
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&self)],
            ))
        }
    }
}
mod ir {
    #![allow(dead_code)]
    use crate::ast::*;
    use std::ops::Range;
    pub struct ConstCallExpr {
        func: Ident,
        args: Vec<ConstExpr>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstCallExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ConstCallExpr",
                "func",
                &&self.func,
                "args",
                &&self.args,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstCallExpr {
        #[inline]
        fn clone(&self) -> ConstCallExpr {
            ConstCallExpr {
                func: ::core::clone::Clone::clone(&self.func),
                args: ::core::clone::Clone::clone(&self.args),
            }
        }
    }
    impl ConstCallExpr {
        #[inline]
        pub fn new(func: Ident, args: Vec<ConstExpr>) -> Self {
            Self { func, args }
        }
        #[inline]
        pub fn func(&self) -> &Ident {
            &self.func
        }
        #[inline]
        pub fn args(&self) -> &[ConstExpr] {
            &self.args
        }
    }
    pub struct ConstFieldAssign {
        field: Ident,
        value: ConstExpr,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstFieldAssign {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ConstFieldAssign",
                "field",
                &&self.field,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstFieldAssign {
        #[inline]
        fn clone(&self) -> ConstFieldAssign {
            ConstFieldAssign {
                field: ::core::clone::Clone::clone(&self.field),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl ConstFieldAssign {
        #[inline]
        pub fn new(field: Ident, value: ConstExpr) -> Self {
            Self { field, value }
        }
        #[inline]
        pub fn field(&self) -> &Ident {
            &self.field
        }
        #[inline]
        pub fn value(&self) -> &ConstExpr {
            &self.value
        }
    }
    pub struct ConstElseIfBlock {
        condition: Box<ConstExpr>,
        body: Box<ConstBlock>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstElseIfBlock {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ConstElseIfBlock",
                "condition",
                &&self.condition,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstElseIfBlock {
        #[inline]
        fn clone(&self) -> ConstElseIfBlock {
            ConstElseIfBlock {
                condition: ::core::clone::Clone::clone(&self.condition),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl ConstElseIfBlock {
        #[inline]
        pub fn new(condition: ConstExpr, body: ConstBlock) -> Self {
            Self {
                condition: Box::new(condition),
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn condition(&self) -> &ConstExpr {
            &self.condition
        }
        #[inline]
        pub fn body(&self) -> &ConstBlock {
            &self.body
        }
    }
    pub struct ConstElseBlock {
        body: Box<ConstBlock>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstElseBlock {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ConstElseBlock",
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstElseBlock {
        #[inline]
        fn clone(&self) -> ConstElseBlock {
            ConstElseBlock {
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl ConstElseBlock {
        #[inline]
        pub fn new(body: ConstBlock) -> Self {
            Self {
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn body(&self) -> &ConstBlock {
            &self.body
        }
    }
    pub struct ConstIfExpr {
        condition: Box<ConstExpr>,
        body: Box<ConstBlock>,
        else_if_blocks: Vec<ConstElseIfBlock>,
        else_block: Option<ConstElseBlock>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstIfExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "ConstIfExpr",
                "condition",
                &&self.condition,
                "body",
                &&self.body,
                "else_if_blocks",
                &&self.else_if_blocks,
                "else_block",
                &&self.else_block,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstIfExpr {
        #[inline]
        fn clone(&self) -> ConstIfExpr {
            ConstIfExpr {
                condition: ::core::clone::Clone::clone(&self.condition),
                body: ::core::clone::Clone::clone(&self.body),
                else_if_blocks: ::core::clone::Clone::clone(&self.else_if_blocks),
                else_block: ::core::clone::Clone::clone(&self.else_block),
            }
        }
    }
    impl ConstIfExpr {
        #[inline]
        pub fn new(
            condition: ConstExpr,
            body: ConstBlock,
            else_if_blocks: Vec<ConstElseIfBlock>,
            else_block: Option<ConstElseBlock>,
        ) -> Self {
            Self {
                condition: Box::new(condition),
                body: Box::new(body),
                else_if_blocks,
                else_block,
            }
        }
        #[inline]
        pub fn condition(&self) -> &ConstExpr {
            &self.condition
        }
        #[inline]
        pub fn body(&self) -> &ConstBlock {
            &self.body
        }
        #[inline]
        pub fn else_if_blocks(&self) -> &[ConstElseIfBlock] {
            &self.else_if_blocks
        }
        #[inline]
        pub fn else_block(&self) -> Option<&ConstElseBlock> {
            self.else_block.as_ref()
        }
    }
    pub enum ConstMatchPattern {
        Literal(Literal),
        Ident(Ident),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstMatchPattern {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ConstMatchPattern::Literal(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Literal", &__self_0)
                }
                ConstMatchPattern::Ident(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Ident", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstMatchPattern {
        #[inline]
        fn clone(&self) -> ConstMatchPattern {
            match self {
                ConstMatchPattern::Literal(__self_0) => {
                    ConstMatchPattern::Literal(::core::clone::Clone::clone(__self_0))
                }
                ConstMatchPattern::Ident(__self_0) => {
                    ConstMatchPattern::Ident(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    pub enum ConstMatchBody {
        Expr(ConstExpr),
        Block(ConstBlock),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstMatchBody {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ConstMatchBody::Expr(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Expr", &__self_0)
                }
                ConstMatchBody::Block(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Block", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstMatchBody {
        #[inline]
        fn clone(&self) -> ConstMatchBody {
            match self {
                ConstMatchBody::Expr(__self_0) => {
                    ConstMatchBody::Expr(::core::clone::Clone::clone(__self_0))
                }
                ConstMatchBody::Block(__self_0) => {
                    ConstMatchBody::Block(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    pub struct ConstMatchBranch {
        patterns: Vec<ConstMatchPattern>,
        body: Box<ConstMatchBody>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstMatchBranch {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ConstMatchBranch",
                "patterns",
                &&self.patterns,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstMatchBranch {
        #[inline]
        fn clone(&self) -> ConstMatchBranch {
            ConstMatchBranch {
                patterns: ::core::clone::Clone::clone(&self.patterns),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl ConstMatchBranch {
        #[inline]
        pub fn new(patterns: Vec<ConstMatchPattern>, body: ConstMatchBody) -> Self {
            Self {
                patterns,
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn patterns(&self) -> &[ConstMatchPattern] {
            &self.patterns
        }
        #[inline]
        pub fn body(&self) -> &ConstMatchBody {
            &self.body
        }
    }
    pub struct ConstMatchExpr {
        value: Box<ConstExpr>,
        branches: Vec<ConstMatchBranch>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstMatchExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ConstMatchExpr",
                "value",
                &&self.value,
                "branches",
                &&self.branches,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstMatchExpr {
        #[inline]
        fn clone(&self) -> ConstMatchExpr {
            ConstMatchExpr {
                value: ::core::clone::Clone::clone(&self.value),
                branches: ::core::clone::Clone::clone(&self.branches),
            }
        }
    }
    impl ConstMatchExpr {
        #[inline]
        pub fn new(value: ConstExpr, branches: Vec<ConstMatchBranch>) -> Self {
            Self {
                value: Box::new(value),
                branches,
            }
        }
        #[inline]
        pub fn value(&self) -> &ConstExpr {
            &self.value
        }
        #[inline]
        pub fn branches(&self) -> &[ConstMatchBranch] {
            &self.branches
        }
    }
    pub struct ConstUnaryExpr {
        inner: Box<ConstExpr>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstUnaryExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ConstUnaryExpr",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstUnaryExpr {
        #[inline]
        fn clone(&self) -> ConstUnaryExpr {
            ConstUnaryExpr {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    impl ConstUnaryExpr {
        #[inline]
        pub fn new(inner: ConstExpr) -> Self {
            Self {
                inner: Box::new(inner),
            }
        }
        #[inline]
        pub fn inner(&self) -> &ConstExpr {
            &self.inner
        }
    }
    pub struct ConstBinaryExpr {
        lhs: Box<ConstExpr>,
        rhs: Box<ConstExpr>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstBinaryExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ConstBinaryExpr",
                "lhs",
                &&self.lhs,
                "rhs",
                &&self.rhs,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstBinaryExpr {
        #[inline]
        fn clone(&self) -> ConstBinaryExpr {
            ConstBinaryExpr {
                lhs: ::core::clone::Clone::clone(&self.lhs),
                rhs: ::core::clone::Clone::clone(&self.rhs),
            }
        }
    }
    impl ConstBinaryExpr {
        #[inline]
        pub fn new(lhs: ConstExpr, rhs: ConstExpr) -> Self {
            Self {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
        }
        #[inline]
        pub fn lhs(&self) -> &ConstExpr {
            &self.lhs
        }
        #[inline]
        pub fn rhs(&self) -> &ConstExpr {
            &self.rhs
        }
    }
    pub enum ConstExpr {
        Literal(Literal),
        Ident(Ident),
        Call(ConstCallExpr),
        If(ConstIfExpr),
        Match(ConstMatchExpr),
        Block(Box<ConstBlock>),
        Neg(ConstUnaryExpr),
        Not(ConstUnaryExpr),
        Lt(ConstBinaryExpr),
        Lte(ConstBinaryExpr),
        Gt(ConstBinaryExpr),
        Gte(ConstBinaryExpr),
        Eq(ConstBinaryExpr),
        Ne(ConstBinaryExpr),
        Add(ConstBinaryExpr),
        Sub(ConstBinaryExpr),
        Mul(ConstBinaryExpr),
        Div(ConstBinaryExpr),
        Rem(ConstBinaryExpr),
        And(ConstBinaryExpr),
        Xor(ConstBinaryExpr),
        Or(ConstBinaryExpr),
        Shl(ConstBinaryExpr),
        Lsr(ConstBinaryExpr),
        Asr(ConstBinaryExpr),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ConstExpr::Literal(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Literal", &__self_0)
                }
                ConstExpr::Ident(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Ident", &__self_0)
                }
                ConstExpr::Call(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Call", &__self_0)
                }
                ConstExpr::If(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "If", &__self_0)
                }
                ConstExpr::Match(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Match", &__self_0)
                }
                ConstExpr::Block(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Block", &__self_0)
                }
                ConstExpr::Neg(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Neg", &__self_0)
                }
                ConstExpr::Not(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Not", &__self_0)
                }
                ConstExpr::Lt(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Lt", &__self_0)
                }
                ConstExpr::Lte(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Lte", &__self_0)
                }
                ConstExpr::Gt(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Gt", &__self_0)
                }
                ConstExpr::Gte(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Gte", &__self_0)
                }
                ConstExpr::Eq(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Eq", &__self_0)
                }
                ConstExpr::Ne(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Ne", &__self_0)
                }
                ConstExpr::Add(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Add", &__self_0)
                }
                ConstExpr::Sub(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Sub", &__self_0)
                }
                ConstExpr::Mul(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Mul", &__self_0)
                }
                ConstExpr::Div(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Div", &__self_0)
                }
                ConstExpr::Rem(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Rem", &__self_0)
                }
                ConstExpr::And(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "And", &__self_0)
                }
                ConstExpr::Xor(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Xor", &__self_0)
                }
                ConstExpr::Or(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Or", &__self_0)
                }
                ConstExpr::Shl(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Shl", &__self_0)
                }
                ConstExpr::Lsr(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Lsr", &__self_0)
                }
                ConstExpr::Asr(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Asr", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstExpr {
        #[inline]
        fn clone(&self) -> ConstExpr {
            match self {
                ConstExpr::Literal(__self_0) => {
                    ConstExpr::Literal(::core::clone::Clone::clone(__self_0))
                }
                ConstExpr::Ident(__self_0) => {
                    ConstExpr::Ident(::core::clone::Clone::clone(__self_0))
                }
                ConstExpr::Call(__self_0) => ConstExpr::Call(::core::clone::Clone::clone(__self_0)),
                ConstExpr::If(__self_0) => ConstExpr::If(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Match(__self_0) => {
                    ConstExpr::Match(::core::clone::Clone::clone(__self_0))
                }
                ConstExpr::Block(__self_0) => {
                    ConstExpr::Block(::core::clone::Clone::clone(__self_0))
                }
                ConstExpr::Neg(__self_0) => ConstExpr::Neg(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Not(__self_0) => ConstExpr::Not(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Lt(__self_0) => ConstExpr::Lt(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Lte(__self_0) => ConstExpr::Lte(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Gt(__self_0) => ConstExpr::Gt(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Gte(__self_0) => ConstExpr::Gte(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Eq(__self_0) => ConstExpr::Eq(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Ne(__self_0) => ConstExpr::Ne(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Add(__self_0) => ConstExpr::Add(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Sub(__self_0) => ConstExpr::Sub(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Mul(__self_0) => ConstExpr::Mul(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Div(__self_0) => ConstExpr::Div(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Rem(__self_0) => ConstExpr::Rem(::core::clone::Clone::clone(__self_0)),
                ConstExpr::And(__self_0) => ConstExpr::And(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Xor(__self_0) => ConstExpr::Xor(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Or(__self_0) => ConstExpr::Or(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Shl(__self_0) => ConstExpr::Shl(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Lsr(__self_0) => ConstExpr::Lsr(::core::clone::Clone::clone(__self_0)),
                ConstExpr::Asr(__self_0) => ConstExpr::Asr(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    pub struct ConstDeclaration {
        name: Ident,
        value: ConstExpr,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstDeclaration {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ConstDeclaration",
                "name",
                &&self.name,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstDeclaration {
        #[inline]
        fn clone(&self) -> ConstDeclaration {
            ConstDeclaration {
                name: ::core::clone::Clone::clone(&self.name),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl ConstDeclaration {
        #[inline]
        pub fn new(name: Ident, value: ConstExpr) -> Self {
            Self { name, value }
        }
        #[inline]
        pub fn name(&self) -> &Ident {
            &self.name
        }
        #[inline]
        pub fn value(&self) -> &ConstExpr {
            &self.value
        }
    }
    pub struct ConstAssignment {
        target: Ident,
        kind: AssignKind,
        value: ConstExpr,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstAssignment {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ConstAssignment",
                "target",
                &&self.target,
                "kind",
                &&self.kind,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstAssignment {
        #[inline]
        fn clone(&self) -> ConstAssignment {
            ConstAssignment {
                target: ::core::clone::Clone::clone(&self.target),
                kind: ::core::clone::Clone::clone(&self.kind),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl ConstAssignment {
        #[inline]
        pub fn new(target: Ident, kind: AssignKind, value: ConstExpr) -> Self {
            Self {
                target,
                kind,
                value,
            }
        }
        #[inline]
        pub fn target(&self) -> &Ident {
            &self.target
        }
        #[inline]
        pub fn kind(&self) -> AssignKind {
            self.kind
        }
        #[inline]
        pub fn value(&self) -> &ConstExpr {
            &self.value
        }
    }
    pub struct ConstWhileLoop {
        condition: ConstExpr,
        body: Box<ConstBlock>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstWhileLoop {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ConstWhileLoop",
                "condition",
                &&self.condition,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstWhileLoop {
        #[inline]
        fn clone(&self) -> ConstWhileLoop {
            ConstWhileLoop {
                condition: ::core::clone::Clone::clone(&self.condition),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl ConstWhileLoop {
        #[inline]
        pub fn new(condition: ConstExpr, body: ConstBlock) -> Self {
            Self {
                condition,
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn condition(&self) -> &ConstExpr {
            &self.condition
        }
        #[inline]
        pub fn body(&self) -> &ConstBlock {
            &self.body
        }
    }
    pub struct ConstForLoop {
        item_name: Ident,
        range: Range<ConstExpr>,
        body: Box<ConstBlock>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstForLoop {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ConstForLoop",
                "item_name",
                &&self.item_name,
                "range",
                &&self.range,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstForLoop {
        #[inline]
        fn clone(&self) -> ConstForLoop {
            ConstForLoop {
                item_name: ::core::clone::Clone::clone(&self.item_name),
                range: ::core::clone::Clone::clone(&self.range),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl ConstForLoop {
        #[inline]
        pub fn new(item_name: Ident, range: Range<ConstExpr>, body: ConstBlock) -> Self {
            Self {
                item_name,
                range,
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn item_name(&self) -> &Ident {
            &self.item_name
        }
        #[inline]
        pub fn range(&self) -> &Range<ConstExpr> {
            &self.range
        }
        #[inline]
        pub fn body(&self) -> &ConstBlock {
            &self.body
        }
    }
    pub enum ConstStatement {
        Expr(ConstExpr),
        Declaration(ConstDeclaration),
        Assignment(ConstAssignment),
        WhileLoop(ConstWhileLoop),
        ForLoop(ConstForLoop),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstStatement {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ConstStatement::Expr(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Expr", &__self_0)
                }
                ConstStatement::Declaration(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Declaration", &__self_0)
                }
                ConstStatement::Assignment(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Assignment", &__self_0)
                }
                ConstStatement::WhileLoop(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "WhileLoop", &__self_0)
                }
                ConstStatement::ForLoop(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ForLoop", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstStatement {
        #[inline]
        fn clone(&self) -> ConstStatement {
            match self {
                ConstStatement::Expr(__self_0) => {
                    ConstStatement::Expr(::core::clone::Clone::clone(__self_0))
                }
                ConstStatement::Declaration(__self_0) => {
                    ConstStatement::Declaration(::core::clone::Clone::clone(__self_0))
                }
                ConstStatement::Assignment(__self_0) => {
                    ConstStatement::Assignment(::core::clone::Clone::clone(__self_0))
                }
                ConstStatement::WhileLoop(__self_0) => {
                    ConstStatement::WhileLoop(::core::clone::Clone::clone(__self_0))
                }
                ConstStatement::ForLoop(__self_0) => {
                    ConstStatement::ForLoop(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    pub struct ConstBlock {
        statements: Vec<ConstStatement>,
        result: Option<ConstExpr>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstBlock {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ConstBlock",
                "statements",
                &&self.statements,
                "result",
                &&self.result,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstBlock {
        #[inline]
        fn clone(&self) -> ConstBlock {
            ConstBlock {
                statements: ::core::clone::Clone::clone(&self.statements),
                result: ::core::clone::Clone::clone(&self.result),
            }
        }
    }
    impl ConstBlock {
        #[inline]
        pub fn new(statements: Vec<ConstStatement>, result: Option<ConstExpr>) -> Self {
            Self { statements, result }
        }
        #[inline]
        pub fn statements(&self) -> &[ConstStatement] {
            &self.statements
        }
        #[inline]
        pub fn result(&self) -> Option<&ConstExpr> {
            self.result.as_ref()
        }
    }
    pub struct ConstFunc {
        args: Vec<Ident>,
        body: Box<ConstBlock>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstFunc {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ConstFunc",
                "args",
                &&self.args,
                "body",
                &&self.body,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstFunc {
        #[inline]
        fn clone(&self) -> ConstFunc {
            ConstFunc {
                args: ::core::clone::Clone::clone(&self.args),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl ConstFunc {
        #[inline]
        pub fn new(args: Vec<Ident>, body: ConstBlock) -> Self {
            Self {
                args,
                body: Box::new(body),
            }
        }
        #[inline]
        pub fn args(&self) -> &[Ident] {
            &self.args
        }
        #[inline]
        pub fn body(&self) -> &ConstBlock {
            &self.body
        }
    }
}
mod lexer {
    use crate::fmt::{DisplayScoped, ScopedFormatter};
    use crate::{default_display_impl, SharedString};
    use langbox::*;
    pub enum PunctKind {
        /// `(`
        OpenParen,
        /// `)`
        CloseParen,
        /// `[`
        OpenBracket,
        /// `]`
        CloseBracket,
        /// `{`
        OpenCurl,
        /// `}`
        CloseCurl,
        /// `,`
        Comma,
        /// `..`
        DoublePeriod,
        /// `.`
        Period,
        /// `::`
        DoubleColon,
        /// `:`
        Colon,
        /// `;`
        Semicolon,
        /// `@`
        AtSymbol,
        /// `'`
        SingleQuote,
        /// `=>`
        FatRightArrow,
        /// `==`
        Eq,
        /// `!=`
        Ne,
        /// `<=$`
        Slte,
        /// `>=$`
        Sgte,
        /// `<=`
        Lte,
        /// `>=`
        Gte,
        /// `=`
        Assign,
        /// `+=`
        AddAssign,
        /// `-=`
        SubAssign,
        /// `*=`
        MulAssign,
        /// `/=`
        DivAssign,
        /// `%=`
        RemAssign,
        /// `&=`
        AndAssign,
        /// `|=`
        OrAssign,
        /// `^=`
        XorAssign,
        /// `<<=`
        ShlAssign,
        /// `>>>=`
        AsrAssign,
        /// `>>=`
        LsrAssign,
        /// `+`
        Add,
        /// `-`
        Sub,
        /// `*`
        Mul,
        /// `/`
        Div,
        /// `%`
        Rem,
        /// `&`
        And,
        /// `|`
        Or,
        /// `^`
        Xor,
        /// `!`
        Not,
        /// `<<`
        Shl,
        /// `>>>`
        Asr,
        /// `>>`
        Lsr,
        /// `<$`
        Slt,
        /// `>$`
        Sgt,
        /// `<`
        Lt,
        /// `>`
        Gt,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PunctKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                PunctKind::OpenParen => ::core::fmt::Formatter::write_str(f, "OpenParen"),
                PunctKind::CloseParen => ::core::fmt::Formatter::write_str(f, "CloseParen"),
                PunctKind::OpenBracket => ::core::fmt::Formatter::write_str(f, "OpenBracket"),
                PunctKind::CloseBracket => ::core::fmt::Formatter::write_str(f, "CloseBracket"),
                PunctKind::OpenCurl => ::core::fmt::Formatter::write_str(f, "OpenCurl"),
                PunctKind::CloseCurl => ::core::fmt::Formatter::write_str(f, "CloseCurl"),
                PunctKind::Comma => ::core::fmt::Formatter::write_str(f, "Comma"),
                PunctKind::DoublePeriod => ::core::fmt::Formatter::write_str(f, "DoublePeriod"),
                PunctKind::Period => ::core::fmt::Formatter::write_str(f, "Period"),
                PunctKind::DoubleColon => ::core::fmt::Formatter::write_str(f, "DoubleColon"),
                PunctKind::Colon => ::core::fmt::Formatter::write_str(f, "Colon"),
                PunctKind::Semicolon => ::core::fmt::Formatter::write_str(f, "Semicolon"),
                PunctKind::AtSymbol => ::core::fmt::Formatter::write_str(f, "AtSymbol"),
                PunctKind::SingleQuote => ::core::fmt::Formatter::write_str(f, "SingleQuote"),
                PunctKind::FatRightArrow => ::core::fmt::Formatter::write_str(f, "FatRightArrow"),
                PunctKind::Eq => ::core::fmt::Formatter::write_str(f, "Eq"),
                PunctKind::Ne => ::core::fmt::Formatter::write_str(f, "Ne"),
                PunctKind::Slte => ::core::fmt::Formatter::write_str(f, "Slte"),
                PunctKind::Sgte => ::core::fmt::Formatter::write_str(f, "Sgte"),
                PunctKind::Lte => ::core::fmt::Formatter::write_str(f, "Lte"),
                PunctKind::Gte => ::core::fmt::Formatter::write_str(f, "Gte"),
                PunctKind::Assign => ::core::fmt::Formatter::write_str(f, "Assign"),
                PunctKind::AddAssign => ::core::fmt::Formatter::write_str(f, "AddAssign"),
                PunctKind::SubAssign => ::core::fmt::Formatter::write_str(f, "SubAssign"),
                PunctKind::MulAssign => ::core::fmt::Formatter::write_str(f, "MulAssign"),
                PunctKind::DivAssign => ::core::fmt::Formatter::write_str(f, "DivAssign"),
                PunctKind::RemAssign => ::core::fmt::Formatter::write_str(f, "RemAssign"),
                PunctKind::AndAssign => ::core::fmt::Formatter::write_str(f, "AndAssign"),
                PunctKind::OrAssign => ::core::fmt::Formatter::write_str(f, "OrAssign"),
                PunctKind::XorAssign => ::core::fmt::Formatter::write_str(f, "XorAssign"),
                PunctKind::ShlAssign => ::core::fmt::Formatter::write_str(f, "ShlAssign"),
                PunctKind::AsrAssign => ::core::fmt::Formatter::write_str(f, "AsrAssign"),
                PunctKind::LsrAssign => ::core::fmt::Formatter::write_str(f, "LsrAssign"),
                PunctKind::Add => ::core::fmt::Formatter::write_str(f, "Add"),
                PunctKind::Sub => ::core::fmt::Formatter::write_str(f, "Sub"),
                PunctKind::Mul => ::core::fmt::Formatter::write_str(f, "Mul"),
                PunctKind::Div => ::core::fmt::Formatter::write_str(f, "Div"),
                PunctKind::Rem => ::core::fmt::Formatter::write_str(f, "Rem"),
                PunctKind::And => ::core::fmt::Formatter::write_str(f, "And"),
                PunctKind::Or => ::core::fmt::Formatter::write_str(f, "Or"),
                PunctKind::Xor => ::core::fmt::Formatter::write_str(f, "Xor"),
                PunctKind::Not => ::core::fmt::Formatter::write_str(f, "Not"),
                PunctKind::Shl => ::core::fmt::Formatter::write_str(f, "Shl"),
                PunctKind::Asr => ::core::fmt::Formatter::write_str(f, "Asr"),
                PunctKind::Lsr => ::core::fmt::Formatter::write_str(f, "Lsr"),
                PunctKind::Slt => ::core::fmt::Formatter::write_str(f, "Slt"),
                PunctKind::Sgt => ::core::fmt::Formatter::write_str(f, "Sgt"),
                PunctKind::Lt => ::core::fmt::Formatter::write_str(f, "Lt"),
                PunctKind::Gt => ::core::fmt::Formatter::write_str(f, "Gt"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PunctKind {
        #[inline]
        fn clone(&self) -> PunctKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for PunctKind {}
    impl ::core::marker::StructuralPartialEq for PunctKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PunctKind {
        #[inline]
        fn eq(&self, other: &PunctKind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    impl ::core::marker::StructuralEq for PunctKind {}
    #[automatically_derived]
    impl ::core::cmp::Eq for PunctKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for PunctKind {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    impl DisplayScoped for PunctKind {
        fn fmt(&self, f: &mut ScopedFormatter<'_, '_>) -> std::fmt::Result {
            use std::fmt::Write;
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_display(&match self {
                    PunctKind::OpenParen => "(",
                    PunctKind::CloseParen => ")",
                    PunctKind::OpenBracket => "[",
                    PunctKind::CloseBracket => "]",
                    PunctKind::OpenCurl => "{",
                    PunctKind::CloseCurl => "}",
                    PunctKind::Comma => ",",
                    PunctKind::DoublePeriod => "..",
                    PunctKind::Period => ".",
                    PunctKind::DoubleColon => "::",
                    PunctKind::Colon => ":",
                    PunctKind::Semicolon => ";",
                    PunctKind::AtSymbol => "@",
                    PunctKind::SingleQuote => "'",
                    PunctKind::FatRightArrow => "=>",
                    PunctKind::Eq => "==",
                    PunctKind::Ne => "!=",
                    PunctKind::Slte => "<=$",
                    PunctKind::Sgte => ">=$",
                    PunctKind::Lte => "<=",
                    PunctKind::Gte => ">=",
                    PunctKind::Assign => "=",
                    PunctKind::AddAssign => "+=",
                    PunctKind::SubAssign => "-=",
                    PunctKind::MulAssign => "*=",
                    PunctKind::DivAssign => "/=",
                    PunctKind::RemAssign => "%=",
                    PunctKind::AndAssign => "&=",
                    PunctKind::OrAssign => "|=",
                    PunctKind::XorAssign => "^=",
                    PunctKind::ShlAssign => "<<=",
                    PunctKind::AsrAssign => ">>>=",
                    PunctKind::LsrAssign => ">>=",
                    PunctKind::Add => "+",
                    PunctKind::Sub => "-",
                    PunctKind::Mul => "*",
                    PunctKind::Div => "/",
                    PunctKind::Rem => "%",
                    PunctKind::And => "&",
                    PunctKind::Or => "|",
                    PunctKind::Xor => "^",
                    PunctKind::Not => "!",
                    PunctKind::Shl => "<<",
                    PunctKind::Asr => ">>>",
                    PunctKind::Lsr => ">>",
                    PunctKind::Slt => "<$",
                    PunctKind::Sgt => ">$",
                    PunctKind::Lt => "<",
                    PunctKind::Gt => ">",
                })],
            ))
        }
    }
    impl std::fmt::Display for PunctKind {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use crate::fmt::ToScoped;
            crate::fmt::DisplayScoped::fmt(self, &mut f.to_scoped())
        }
    }
    impl Into<[PunctKind; 1]> for PunctKind {
        #[inline]
        fn into(self) -> [PunctKind; 1] {
            [self]
        }
    }
    pub enum QuartzToken {
        Comment(bool),
        Punct(PunctKind),
        Ident(SharedString),
        Literal(i64),
        InvalidIdent(SharedString),
        InvalidLiteral(SharedString),
        InvalidChar(char),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for QuartzToken {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                QuartzToken::Comment(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Comment", &__self_0)
                }
                QuartzToken::Punct(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Punct", &__self_0)
                }
                QuartzToken::Ident(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Ident", &__self_0)
                }
                QuartzToken::Literal(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Literal", &__self_0)
                }
                QuartzToken::InvalidIdent(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "InvalidIdent", &__self_0)
                }
                QuartzToken::InvalidLiteral(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidLiteral",
                        &__self_0,
                    )
                }
                QuartzToken::InvalidChar(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "InvalidChar", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for QuartzToken {
        #[inline]
        fn clone(&self) -> QuartzToken {
            match self {
                QuartzToken::Comment(__self_0) => {
                    QuartzToken::Comment(::core::clone::Clone::clone(__self_0))
                }
                QuartzToken::Punct(__self_0) => {
                    QuartzToken::Punct(::core::clone::Clone::clone(__self_0))
                }
                QuartzToken::Ident(__self_0) => {
                    QuartzToken::Ident(::core::clone::Clone::clone(__self_0))
                }
                QuartzToken::Literal(__self_0) => {
                    QuartzToken::Literal(::core::clone::Clone::clone(__self_0))
                }
                QuartzToken::InvalidIdent(__self_0) => {
                    QuartzToken::InvalidIdent(::core::clone::Clone::clone(__self_0))
                }
                QuartzToken::InvalidLiteral(__self_0) => {
                    QuartzToken::InvalidLiteral(::core::clone::Clone::clone(__self_0))
                }
                QuartzToken::InvalidChar(__self_0) => {
                    QuartzToken::InvalidChar(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    # [rustfmt :: skip] const PUNCTUATION_MAP : & 'static [(& 'static str , PunctKind)] = & [("(" , PunctKind :: OpenParen) , (")" , PunctKind :: CloseParen) , ("[" , PunctKind :: OpenBracket) , ("]" , PunctKind :: CloseBracket) , ("{" , PunctKind :: OpenCurl) , ("}" , PunctKind :: CloseCurl) , ("," , PunctKind :: Comma) , (".." , PunctKind :: DoublePeriod) , ("." , PunctKind :: Period) , ("::" , PunctKind :: DoubleColon) , (":" , PunctKind :: Colon) , (";" , PunctKind :: Semicolon) , ("@" , PunctKind :: AtSymbol) , ("'" , PunctKind :: SingleQuote) , ("=>" , PunctKind :: FatRightArrow) , ("==" , PunctKind :: Eq) , ("!=" , PunctKind :: Ne) , ("<=$" , PunctKind :: Slte) , (">=$" , PunctKind :: Sgte) , ("<=" , PunctKind :: Lte) , (">=" , PunctKind :: Gte) , ("=" , PunctKind :: Assign) , ("+=" , PunctKind :: AddAssign) , ("-=" , PunctKind :: SubAssign) , ("*=" , PunctKind :: MulAssign) , ("/=" , PunctKind :: DivAssign) , ("%=" , PunctKind :: RemAssign) , ("&=" , PunctKind :: AndAssign) , ("|=" , PunctKind :: OrAssign) , ("^=" , PunctKind :: XorAssign) , ("<<=" , PunctKind :: ShlAssign) , (">>>=" , PunctKind :: AsrAssign) , (">>=" , PunctKind :: LsrAssign) , ("+" , PunctKind :: Add) , ("-" , PunctKind :: Sub) , ("*" , PunctKind :: Mul) , ("/" , PunctKind :: Div) , ("%" , PunctKind :: Rem) , ("&" , PunctKind :: And) , ("|" , PunctKind :: Or) , ("^" , PunctKind :: Xor) , ("!" , PunctKind :: Not) , ("<<" , PunctKind :: Shl) , (">>>" , PunctKind :: Asr) , (">>" , PunctKind :: Lsr) , ("<$" , PunctKind :: Slt) , (">$" , PunctKind :: Sgt) , ("<" , PunctKind :: Lt) , (">" , PunctKind :: Gt)] ;
    fn parse_comment(text: &str) -> Option<ReadTokenResult<QuartzToken>> {
        if text.starts_with("//") {
            let end_index = text
                .char_indices()
                .take_while(|(_, c)| *c != '\n')
                .last()
                .map(|(i, c)| i + c.len_utf8())
                .unwrap();
            Some(ReadTokenResult {
                token: QuartzToken::Comment(false),
                consumed_bytes: end_index,
            })
        } else if text.starts_with("/*") {
            if let Some(len) = text[2..].find("*/") {
                Some(ReadTokenResult {
                    token: QuartzToken::Comment(false),
                    consumed_bytes: len + "/**/".len(),
                })
            } else {
                Some(ReadTokenResult {
                    token: QuartzToken::Comment(true),
                    consumed_bytes: text.len(),
                })
            }
        } else {
            None
        }
    }
    fn parse_punctuation(text: &str) -> Option<ReadTokenResult<QuartzToken>> {
        for (pat, punct) in PUNCTUATION_MAP.iter() {
            if text.starts_with(pat) {
                return Some(ReadTokenResult {
                    token: QuartzToken::Punct(*punct),
                    consumed_bytes: pat.len(),
                });
            }
        }
        None
    }
    fn parse_identifier(text: &str) -> Option<ReadTokenResult<QuartzToken>> {
        if text.starts_with(|c: char| c.is_alphabetic() | (c == '_')) {
            let end_index = text
                .char_indices()
                .take_while(|(_, c)| c.is_alphanumeric() | (*c == '_'))
                .last()
                .map(|(i, c)| i + c.len_utf8())
                .unwrap();
            let ident: SharedString = text[..end_index].into();
            let token = if ident.contains(|c: char| !c.is_ascii_alphanumeric() & (c != '_')) {
                QuartzToken::InvalidIdent(ident)
            } else {
                QuartzToken::Ident(ident)
            };
            Some(ReadTokenResult {
                token,
                consumed_bytes: end_index,
            })
        } else {
            None
        }
    }
    fn parse_literal(text: &str) -> Option<ReadTokenResult<QuartzToken>> {
        if text.starts_with(|c: char| c.is_ascii_digit()) {
            let end_index = text
                .char_indices()
                .take_while(|(_, c)| c.is_alphanumeric() | (*c == '_'))
                .last()
                .map(|(i, c)| i + c.len_utf8())
                .unwrap();
            let raw_literal = &text[..end_index];
            let (literal, radix) = if raw_literal.starts_with("0x") {
                (raw_literal[2..].replace('_', ""), 16)
            } else if raw_literal.starts_with("0o") {
                (raw_literal[2..].replace('_', ""), 8)
            } else if raw_literal.starts_with("0b") {
                (raw_literal[2..].replace('_', ""), 2)
            } else {
                (raw_literal.replace('_', ""), 10)
            };
            let token = match i64::from_str_radix(&literal, radix) {
                Ok(value) => QuartzToken::Literal(value),
                Err(_) => QuartzToken::InvalidLiteral(raw_literal.into()),
            };
            Some(ReadTokenResult {
                token,
                consumed_bytes: end_index,
            })
        } else {
            None
        }
    }
    pub struct QuartzTokenReader;
    impl TokenReader for QuartzTokenReader {
        type Token = QuartzToken;
        fn read_token(text: &str) -> ReadTokenResult<Self::Token> {
            if let Some(result) = parse_comment(text) {
                return result;
            }
            if let Some(result) = parse_punctuation(text) {
                return result;
            }
            if let Some(result) = parse_identifier(text) {
                return result;
            }
            if let Some(result) = parse_literal(text) {
                return result;
            }
            let first_char = text.chars().next().unwrap();
            ReadTokenResult {
                token: QuartzToken::InvalidChar(first_char),
                consumed_bytes: first_char.len_utf8(),
            }
        }
    }
    pub type QuartzLexer<'a> = Lexer<'a, QuartzTokenReader, whitespace_mode::Remove>;
}
mod parser {
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
    const KEYWORDS: phf::Map<&'static str, KeywordKind> = phf::Map {
        key: 12913932095322966823u64,
        disps: &[
            (13u32, 12u32),
            (4u32, 0u32),
            (1u32, 14u32),
            (4u32, 18u32),
            (0u32, 11u32),
        ],
        entries: &[
            ("mod", KeywordKind::Mod),
            ("if", KeywordKind::If),
            ("comb", KeywordKind::Comb),
            ("proc", KeywordKind::Proc),
            ("else", KeywordKind::Else),
            ("fn", KeywordKind::Fn),
            ("as", KeywordKind::As),
            ("reg", KeywordKind::Reg),
            ("falling", KeywordKind::Falling),
            ("enum", KeywordKind::Enum),
            ("match", KeywordKind::Match),
            ("for", KeywordKind::For),
            ("sig", KeywordKind::Sig),
            ("out", KeywordKind::Out),
            ("struct", KeywordKind::Struct),
            ("in", KeywordKind::In),
            ("const", KeywordKind::Const),
            ("rising", KeywordKind::Rising),
            ("let", KeywordKind::Let),
            ("while", KeywordKind::While),
            ("inout", KeywordKind::InOut),
        ],
    };
    pub struct QuartzParserErrror {
        pub message: Cow<'static, str>,
        pub span: TextSpan,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for QuartzParserErrror {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "QuartzParserErrror",
                "message",
                &&self.message,
                "span",
                &&self.span,
            )
        }
    }
    pub trait QuartzParser<T> = Parser<QuartzToken, T, QuartzParserErrror>;
    fn punct<const N: usize>(punct: impl Into<[PunctKind; N]>) -> impl QuartzParser<Punct> {
        let punct = punct.into();
        ::langbox::_constrain_parse_fn(move |input| {
            if let Some(token) = input.peek() {
                if let QuartzToken :: Punct (found_punct) = & token . kind && punct . iter () . find (| p | (* p) . eq (found_punct)) . is_some () { ParseResult :: Match { value : Punct :: new (* found_punct , token . span) , span : token . span , remaining : input . advance () , } } else { ParseResult :: NoMatch }
            } else {
                ParseResult::NoMatch
            }
        })
    }
    fn ident() -> impl QuartzParser<Ident> {
        ::langbox::_constrain_parse_fn(move |input| {
            if let Some(token) = input.peek() {
                match &token.kind {
                    QuartzToken::Ident(name) => {
                        if KEYWORDS.contains_key(name) {
                            ParseResult :: Err (QuartzParserErrror { message : { let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["\'" , "\' is a reserved keyword and cannot be used as an identifier"] , & [:: core :: fmt :: ArgumentV1 :: new_display (& name)])) ; res } . into () , span : token . span , })
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
        ::langbox::_constrain_parse_fn(move |input| {
            if let Some(token) = input.peek() {
                match & token . kind { QuartzToken :: Ident (name) => { if let Some (found_kw) = KEYWORDS . get (name) && kw . eq (found_kw) { ParseResult :: Match { value : Keyword :: new (kw , token . span) , span : token . span , remaining : input . advance () , } } else { ParseResult :: NoMatch } } _ => ParseResult :: NoMatch , }
            } else {
                ParseResult::NoMatch
            }
        })
    }
    fn literal() -> impl QuartzParser<Literal> {
        ::langbox::_constrain_parse_fn(move |input| {
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
        let segment = {
            let p0 = {
                let lhs1 = {
                    let lhs2 = {
                        let p3 = punct(PunctKind::Period);
                        langbox::_map(p3, |punct| {
                            PathSeparator::new(PathSeparatorKind::Period, punct.span())
                        })
                    };
                    let rhs2 = {
                        let p3 = punct(PunctKind::DoubleColon);
                        langbox::_map(p3, |punct| {
                            PathSeparator::new(PathSeparatorKind::DoubleColon, punct.span())
                        })
                    };
                    langbox::_or_else(lhs2, rhs2)
                };
                let rhs1 = {
                    let p2 = ident();
                    langbox::_require(p2, |input| QuartzParserErrror {
                        message: "expected identifier".into(),
                        span: input.current_span(),
                    })
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |(sep, ident)| PathSegment::new(sep, ident))
        };
        {
            let p0 = {
                let lhs1 = ident();
                let rhs1 = {
                    let p2 = segment;
                    langbox::_many(p2, true)
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |(head, tail)| Path::new(head, tail))
        }
    }
    fn paren_expr(simple: bool) -> impl QuartzParser<Expr> {
        {
            let p0 = {
                let lhs1 = {
                    let lhs2 = punct(PunctKind::OpenParen);
                    let rhs2 = expr(simple);
                    langbox::_and_then(lhs2, rhs2)
                };
                let rhs1 = {
                    let p2 = punct(PunctKind::CloseParen);
                    langbox::_require(p2, |input| QuartzParserErrror {
                        message: "expected `)`".into(),
                        span: input.current_span(),
                    })
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |((open_paren, expr), close_paren)| {
                Expr::Paren(ParenExpr::new(open_paren, expr, close_paren))
            })
        }
    }
    fn call_expr(simple: bool) -> impl QuartzParser<Expr> {
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = ident();
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = punct(PunctKind::OpenParen);
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 =
                                        sep_by(expr(simple), punct(PunctKind::Comma), true, true);
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = punct(PunctKind::CloseParen);
                                                langbox::_require(p0, |input| QuartzParserErrror {
                                                    message: "expected `)`".into(),
                                                    span: input.current_span(),
                                                })
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => langbox::ParseResult::Match {
                                                    value: (v0, v1, v2, v3),
                                                    span: langbox::_join_spans(&[
                                                        input.empty_span(),
                                                        s0,
                                                        s1,
                                                        s2,
                                                        s3,
                                                    ]),
                                                    remaining,
                                                },
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(func, open_paren, args, close_paren)| {
                Expr::Call(CallExpr::new(func, open_paren, args, close_paren))
            })
        }
    }
    fn construct_expr() -> impl QuartzParser<Expr> {
        let field = {
            let p0 = {
                let lhs1 = {
                    let lhs2 = ident();
                    let rhs2 = {
                        let p3 = punct(PunctKind::Colon);
                        langbox::_require(p3, |input| QuartzParserErrror {
                            message: "expected `:`".into(),
                            span: input.current_span(),
                        })
                    };
                    langbox::_and_then(lhs2, rhs2)
                };
                let rhs1 = {
                    let p2 = expr(false);
                    langbox::_require(p2, |input| QuartzParserErrror {
                        message: "expected expression".into(),
                        span: input.current_span(),
                    })
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |((field, sep), value)| {
                FieldAssign::new(field, sep, value)
            })
        };
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = ty();
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = punct(PunctKind::OpenCurl);
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = sep_by(field, punct(PunctKind::Comma), true, true);
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = punct(PunctKind::CloseCurl);
                                                langbox::_require(p0, |input| QuartzParserErrror {
                                                    message: "expected `}`".into(),
                                                    span: input.current_span(),
                                                })
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => langbox::ParseResult::Match {
                                                    value: (v0, v1, v2, v3),
                                                    span: langbox::_join_spans(&[
                                                        input.empty_span(),
                                                        s0,
                                                        s1,
                                                        s2,
                                                        s3,
                                                    ]),
                                                    remaining,
                                                },
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(ty, open_curl, fields, close_curl)| {
                Expr::Construct(ConstructExpr::new(ty, open_curl, fields, close_curl))
            })
        }
    }
    fn if_expr() -> impl QuartzParser<Expr> {
        let else_if_block = {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = kw(KeywordKind::Else);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = kw(KeywordKind::If);
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = expr(true);
                                        langbox::_require(p0, |input| QuartzParserErrror {
                                            message: "expected expression".into(),
                                            span: input.current_span(),
                                        })
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = block();
                                                langbox::_require(p0, |input| QuartzParserErrror {
                                                    message: "expected block".into(),
                                                    span: input.current_span(),
                                                })
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => langbox::ParseResult::Match {
                                                    value: (v0, v1, v2, v3),
                                                    span: langbox::_join_spans(&[
                                                        input.empty_span(),
                                                        s0,
                                                        s1,
                                                        s2,
                                                        s3,
                                                    ]),
                                                    remaining,
                                                },
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(else_kw, if_kw, cond, body)| {
                ElseIfBlock::new(else_kw, if_kw, cond, body)
            })
        };
        let else_block = {
            let p0 = {
                let lhs1 = kw(KeywordKind::Else);
                let rhs1 = {
                    let p2 = block();
                    langbox::_require(p2, |input| QuartzParserErrror {
                        message: "expected block".into(),
                        span: input.current_span(),
                    })
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |(else_kw, body)| ElseBlock::new(else_kw, body))
        };
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = kw(KeywordKind::If);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = expr(true);
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected expression".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = block();
                                        langbox::_require(p0, |input| QuartzParserErrror {
                                            message: "expected block".into(),
                                            span: input.current_span(),
                                        })
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = else_if_block;
                                                langbox::_many(p0, true)
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = {
                                                        let p0 = else_block;
                                                        langbox::_opt(p0)
                                                    };
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => langbox::ParseResult::Match {
                                                            value: (v0, v1, v2, v3, v4),
                                                            span: langbox::_join_spans(&[
                                                                input.empty_span(),
                                                                s0,
                                                                s1,
                                                                s2,
                                                                s3,
                                                                s4,
                                                            ]),
                                                            remaining,
                                                        },
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(if_kw, cond, body, else_if_blocks, else_block)| {
                Expr::If(IfExpr::new(if_kw, cond, body, else_if_blocks, else_block))
            })
        }
    }
    fn match_expr() -> impl QuartzParser<Expr> {
        let pattern = langbox::_constrain_parse_fn(move |input| {
            let p0 = {
                let p0 = literal();
                langbox::_map(p0, MatchPattern::Literal)
            };
            match p0.run(input)? {
                langbox::InfallibleParseResult::Match {
                    value,
                    span,
                    remaining,
                } => langbox::ParseResult::Match {
                    value,
                    span,
                    remaining,
                },
                langbox::InfallibleParseResult::NoMatch => {
                    let p1 = {
                        let p0 = path();
                        langbox::_map(p0, MatchPattern::Path)
                    };
                    match p1.run(input)? {
                        langbox::InfallibleParseResult::Match {
                            value,
                            span,
                            remaining,
                        } => langbox::ParseResult::Match {
                            value,
                            span,
                            remaining,
                        },
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            }
        });
        let body = langbox::_constrain_parse_fn(move |input| {
            let p0 = {
                let p0 = expr(false);
                langbox::_map(p0, MatchBody::Expr)
            };
            match p0.run(input)? {
                langbox::InfallibleParseResult::Match {
                    value,
                    span,
                    remaining,
                } => langbox::ParseResult::Match {
                    value,
                    span,
                    remaining,
                },
                langbox::InfallibleParseResult::NoMatch => {
                    let p1 = {
                        let p0 = block();
                        langbox::_map(p0, MatchBody::Block)
                    };
                    match p1.run(input)? {
                        langbox::InfallibleParseResult::Match {
                            value,
                            span,
                            remaining,
                        } => langbox::ParseResult::Match {
                            value,
                            span,
                            remaining,
                        },
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            }
        });
        let branch = {
            let p0 = {
                let lhs1 = {
                    let lhs2 = sep_by(pattern, punct(PunctKind::Or), false, false);
                    let rhs2 = {
                        let p3 = punct(PunctKind::FatRightArrow);
                        langbox::_require(p3, |input| QuartzParserErrror {
                            message: "expected `=>`".into(),
                            span: input.current_span(),
                        })
                    };
                    langbox::_and_then(lhs2, rhs2)
                };
                let rhs1 = {
                    let p2 = body;
                    langbox::_require(p2, |input| QuartzParserErrror {
                        message: "expected expression or block".into(),
                        span: input.current_span(),
                    })
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |((patterns, arrow), body)| {
                MatchBranch::new(patterns, arrow, body)
            })
        };
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = kw(KeywordKind::Match);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = expr(true);
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected expression".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = punct(PunctKind::OpenCurl);
                                        langbox::_require(p0, |input| QuartzParserErrror {
                                            message: "expected `{`".into(),
                                            span: input.current_span(),
                                        })
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 =
                                                sep_by(branch, punct(PunctKind::Comma), true, true);
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = {
                                                        let p0 = punct(PunctKind::CloseCurl);
                                                        langbox::_require(p0, |input| {
                                                            QuartzParserErrror {
                                                                message: "expected `}`".into(),
                                                                span: input.current_span(),
                                                            }
                                                        })
                                                    };
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => langbox::ParseResult::Match {
                                                            value: (v0, v1, v2, v3, v4),
                                                            span: langbox::_join_spans(&[
                                                                input.empty_span(),
                                                                s0,
                                                                s1,
                                                                s2,
                                                                s3,
                                                                s4,
                                                            ]),
                                                            remaining,
                                                        },
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(match_kw, value, open_curl, branches, close_curl)| {
                Expr::Match(MatchExpr::new(
                    match_kw, value, open_curl, branches, close_curl,
                ))
            })
        }
    }
    fn leaf_expr() -> impl QuartzParser<Expr> {
        langbox::_constrain_parse_fn(move |input| {
            let p0 = {
                let p0 = literal();
                langbox::_map(p0, Expr::Literal)
            };
            match p0.run(input)? {
                langbox::InfallibleParseResult::Match {
                    value,
                    span,
                    remaining,
                } => langbox::ParseResult::Match {
                    value,
                    span,
                    remaining,
                },
                langbox::InfallibleParseResult::NoMatch => {
                    let p1 = if_expr();
                    match p1.run(input)? {
                        langbox::InfallibleParseResult::Match {
                            value,
                            span,
                            remaining,
                        } => langbox::ParseResult::Match {
                            value,
                            span,
                            remaining,
                        },
                        langbox::InfallibleParseResult::NoMatch => {
                            let p2 = match_expr();
                            match p2.run(input)? {
                                langbox::InfallibleParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                } => langbox::ParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                },
                                langbox::InfallibleParseResult::NoMatch => {
                                    let p3 = call_expr(false);
                                    match p3.run(input)? {
                                        langbox::InfallibleParseResult::Match {
                                            value,
                                            span,
                                            remaining,
                                        } => langbox::ParseResult::Match {
                                            value,
                                            span,
                                            remaining,
                                        },
                                        langbox::InfallibleParseResult::NoMatch => {
                                            let p4 = construct_expr();
                                            match p4.run(input)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value,
                                                    span,
                                                    remaining,
                                                } => langbox::ParseResult::Match {
                                                    value,
                                                    span,
                                                    remaining,
                                                },
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    let p5 = {
                                                        let p0 = path();
                                                        langbox::_map(p0, Expr::Path)
                                                    };
                                                    match p5.run(input)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value,
                                                            span,
                                                            remaining,
                                                        } => langbox::ParseResult::Match {
                                                            value,
                                                            span,
                                                            remaining,
                                                        },
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            let p6 = paren_expr(false);
                                                            match p6 . run (input) ? { langbox :: InfallibleParseResult :: Match { value , span , remaining } => { langbox :: ParseResult :: Match { value , span , remaining } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
    fn leaf_expr_simple() -> impl QuartzParser<Expr> {
        langbox::_constrain_parse_fn(move |input| {
            let p0 = {
                let p0 = literal();
                langbox::_map(p0, Expr::Literal)
            };
            match p0.run(input)? {
                langbox::InfallibleParseResult::Match {
                    value,
                    span,
                    remaining,
                } => langbox::ParseResult::Match {
                    value,
                    span,
                    remaining,
                },
                langbox::InfallibleParseResult::NoMatch => {
                    let p1 = call_expr(true);
                    match p1.run(input)? {
                        langbox::InfallibleParseResult::Match {
                            value,
                            span,
                            remaining,
                        } => langbox::ParseResult::Match {
                            value,
                            span,
                            remaining,
                        },
                        langbox::InfallibleParseResult::NoMatch => {
                            let p2 = {
                                let p0 = path();
                                langbox::_map(p0, Expr::Path)
                            };
                            match p2.run(input)? {
                                langbox::InfallibleParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                } => langbox::ParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                },
                                langbox::InfallibleParseResult::NoMatch => {
                                    let p3 = paren_expr(true);
                                    match p3.run(input)? {
                                        langbox::InfallibleParseResult::Match {
                                            value,
                                            span,
                                            remaining,
                                        } => langbox::ParseResult::Match {
                                            value,
                                            span,
                                            remaining,
                                        },
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
    fn indexer() -> impl QuartzParser<Indexer> {
        let range_end = {
            let lhs0 = punct(PunctKind::DoublePeriod);
            let rhs0 = {
                let p1 = expr(true);
                langbox::_require(p1, |input| QuartzParserErrror {
                    message: "expected expression".into(),
                    span: input.current_span(),
                })
            };
            langbox::_suffix(lhs0, rhs0)
        };
        {
            let p0 = {
                let lhs1 = {
                    let lhs2 = {
                        let lhs3 = punct(PunctKind::OpenBracket);
                        let rhs3 = {
                            let p4 = expr(true);
                            langbox::_require(p4, |input| QuartzParserErrror {
                                message: "expected expression".into(),
                                span: input.current_span(),
                            })
                        };
                        langbox::_and_then(lhs3, rhs3)
                    };
                    let rhs2 = {
                        let p3 = range_end;
                        langbox::_opt(p3)
                    };
                    langbox::_and_then(lhs2, rhs2)
                };
                let rhs1 = {
                    let p2 = punct(PunctKind::CloseBracket);
                    langbox::_require(p2, |input| QuartzParserErrror {
                        message: "expected `]`".into(),
                        span: input.current_span(),
                    })
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |(((open_paren, start), end), close_paren)| {
                Indexer::new(
                    open_paren,
                    if let Some(end) = end {
                        IndexKind::Range(start..end)
                    } else {
                        IndexKind::Single(start)
                    },
                    close_paren,
                )
            })
        }
    }
    fn concat_unary_suffix_exprs(base: Expr, indexers: Vec<Indexer>) -> Expr {
        let mut result = base;
        for indexer in indexers.into_iter() {
            result = Expr::Index(IndexExpr::new(result, indexer));
        }
        result
    }
    fn unary_suffix_expr(simple: bool) -> impl QuartzParser<Expr> {
        ::langbox::_constrain_parse_fn(move |input| {
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
                } => match {
                    let p0 = indexer();
                    langbox::_many(p0, true)
                }
                .run(remaining)?
                {
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
            result = match punct.kind() {
                PunctKind::Add => Expr::Pos(UnaryExpr::new(punct, result)),
                PunctKind::Sub => Expr::Neg(UnaryExpr::new(punct, result)),
                PunctKind::Not => Expr::Not(UnaryExpr::new(punct, result)),
                _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &[::core::fmt::ArgumentV1::new_display(
                        &::core::fmt::Arguments::new_v1(&["invalid unary operator"], &[]),
                    )],
                )),
            };
        }
        result
    }
    fn unary_prefix_expr(simple: bool) -> impl QuartzParser<Expr> {
        let op = punct([PunctKind::Add, PunctKind::Sub, PunctKind::Not]);
        {
            let p0 = {
                let lhs1 = {
                    let p2 = op;
                    langbox::_many(p2, true)
                };
                let rhs1 = unary_suffix_expr(simple);
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, concat_unary_prefix_exprs)
        }
    }
    fn concat_cast_exprs(value: Expr, targets: Vec<(Keyword, Type)>) -> Expr {
        let mut result = value;
        for (as_kw, target_ty) in targets.into_iter() {
            result = Expr::Cast(CastExpr::new(result, as_kw, target_ty));
        }
        result
    }
    fn cast_expr(simple: bool) -> impl QuartzParser<Expr> {
        ::langbox::_constrain_parse_fn(move |input| match unary_prefix_expr(simple).run(input)? {
            InfallibleParseResult::Match {
                value,
                span: s1,
                remaining,
            } => {
                let tail = {
                    let lhs0 = kw(KeywordKind::As);
                    let rhs0 = ty();
                    langbox::_and_then(lhs0, rhs0)
                };
                match {
                    let p0 = tail;
                    langbox::_many(p0, true)
                }
                .run(remaining)?
                {
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
        })
    }
    fn concat_binary_exprs(terms: (Expr, Vec<(Punct, Expr)>)) -> Expr {
        let mut result = terms.0;
        for (punct, expr) in terms.1.into_iter() {
            result = match punct.kind() {
                PunctKind::AtSymbol => Expr::Concat(BinaryExpr::new(result, punct, expr)),
                PunctKind::SingleQuote => ::core::panicking::panic("not yet implemented"),
                PunctKind::Lt => Expr::Lt(BinaryExpr::new(result, punct, expr)),
                PunctKind::Lte => Expr::Lte(BinaryExpr::new(result, punct, expr)),
                PunctKind::Gt => Expr::Gt(BinaryExpr::new(result, punct, expr)),
                PunctKind::Gte => Expr::Gte(BinaryExpr::new(result, punct, expr)),
                PunctKind::Slt => Expr::Slt(BinaryExpr::new(result, punct, expr)),
                PunctKind::Slte => Expr::Slte(BinaryExpr::new(result, punct, expr)),
                PunctKind::Sgt => Expr::Sgt(BinaryExpr::new(result, punct, expr)),
                PunctKind::Sgte => Expr::Sgte(BinaryExpr::new(result, punct, expr)),
                PunctKind::Eq => Expr::Eq(BinaryExpr::new(result, punct, expr)),
                PunctKind::Ne => Expr::Ne(BinaryExpr::new(result, punct, expr)),
                PunctKind::Add => Expr::Add(BinaryExpr::new(result, punct, expr)),
                PunctKind::Sub => Expr::Sub(BinaryExpr::new(result, punct, expr)),
                PunctKind::Mul => Expr::Mul(BinaryExpr::new(result, punct, expr)),
                PunctKind::Div => Expr::Div(BinaryExpr::new(result, punct, expr)),
                PunctKind::Rem => Expr::Rem(BinaryExpr::new(result, punct, expr)),
                PunctKind::And => Expr::And(BinaryExpr::new(result, punct, expr)),
                PunctKind::Or => Expr::Or(BinaryExpr::new(result, punct, expr)),
                PunctKind::Xor => Expr::Xor(BinaryExpr::new(result, punct, expr)),
                PunctKind::Shl => Expr::Shl(BinaryExpr::new(result, punct, expr)),
                PunctKind::Asr => Expr::Asr(BinaryExpr::new(result, punct, expr)),
                PunctKind::Lsr => Expr::Lsr(BinaryExpr::new(result, punct, expr)),
                _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &[::core::fmt::ArgumentV1::new_display(
                        &::core::fmt::Arguments::new_v1(&["invalid binary operator"], &[]),
                    )],
                )),
            };
        }
        result
    }
    fn binary_expr<const N: usize>(
        term: impl QuartzParser<Expr>,
        ops: impl Into<[PunctKind; N]>,
    ) -> impl QuartzParser<Expr> {
        let tail = {
            let lhs0 = punct(ops);
            let rhs0 = term;
            langbox::_and_then(lhs0, rhs0)
        };
        {
            let p0 = {
                let lhs1 = term;
                let rhs1 = {
                    let p2 = tail;
                    langbox::_many(p2, true)
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, concat_binary_exprs)
        }
    }
    #[inline]
    fn concat_expr(simple: bool) -> impl QuartzParser<Expr> {
        binary_expr(cast_expr(simple), [PunctKind::AtSymbol])
    }
    #[inline]
    fn mul_expr(simple: bool) -> impl QuartzParser<Expr> {
        binary_expr(
            concat_expr(simple),
            [PunctKind::Mul, PunctKind::Div, PunctKind::Rem],
        )
    }
    #[inline]
    fn add_expr(simple: bool) -> impl QuartzParser<Expr> {
        binary_expr(mul_expr(simple), [PunctKind::Add, PunctKind::Sub])
    }
    #[inline]
    fn shift_expr(simple: bool) -> impl QuartzParser<Expr> {
        binary_expr(
            add_expr(simple),
            [PunctKind::Shl, PunctKind::Lsr, PunctKind::Asr],
        )
    }
    #[inline]
    fn cmp_expr(simple: bool) -> impl QuartzParser<Expr> {
        binary_expr(
            shift_expr(simple),
            [
                PunctKind::Lt,
                PunctKind::Lte,
                PunctKind::Gt,
                PunctKind::Gte,
                PunctKind::Slt,
                PunctKind::Slte,
                PunctKind::Sgt,
                PunctKind::Sgte,
            ],
        )
    }
    #[inline]
    fn eq_expr(simple: bool) -> impl QuartzParser<Expr> {
        binary_expr(cmp_expr(simple), [PunctKind::Eq, PunctKind::Ne])
    }
    #[inline]
    fn and_expr(simple: bool) -> impl QuartzParser<Expr> {
        binary_expr(eq_expr(simple), [PunctKind::And])
    }
    #[inline]
    fn xor_expr(simple: bool) -> impl QuartzParser<Expr> {
        binary_expr(and_expr(simple), [PunctKind::Xor])
    }
    #[inline]
    fn or_expr(simple: bool) -> impl QuartzParser<Expr> {
        binary_expr(xor_expr(simple), [PunctKind::Or])
    }
    fn expr(simple: bool) -> impl QuartzParser<Expr> {
        or_expr(simple)
    }
    fn decl() -> impl QuartzParser<Declaration> {
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = kw(KeywordKind::Let);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = ident();
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected identifier".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = punct(PunctKind::Assign);
                                        langbox::_require(p0, |input| QuartzParserErrror {
                                            message: "expected `=`".into(),
                                            span: input.current_span(),
                                        })
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = expr(false);
                                                langbox::_require(p0, |input| QuartzParserErrror {
                                                    message: "expected expression".into(),
                                                    span: input.current_span(),
                                                })
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = {
                                                        let p0 = punct(PunctKind::Semicolon);
                                                        langbox::_require(p0, |input| {
                                                            QuartzParserErrror {
                                                                message: "expected `;`".into(),
                                                                span: input.current_span(),
                                                            }
                                                        })
                                                    };
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => langbox::ParseResult::Match {
                                                            value: (v0, v1, v2, v3, v4),
                                                            span: langbox::_join_spans(&[
                                                                input.empty_span(),
                                                                s0,
                                                                s1,
                                                                s2,
                                                                s3,
                                                                s4,
                                                            ]),
                                                            remaining,
                                                        },
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(let_kw, name, assign, value, _)| {
                Declaration::new(let_kw, name, assign, value)
            })
        }
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
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &[::core::fmt::ArgumentV1::new_display(
                    &::core::fmt::Arguments::new_v1(&["invalid assignment operator"], &[]),
                )],
            )),
        };
        AssignOp::new(kind, punct.span())
    }
    fn assign() -> impl QuartzParser<Assignment> {
        let target = {
            let p0 = {
                let lhs1 = path();
                let rhs1 = {
                    let p2 = indexer();
                    langbox::_many(p2, true)
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |(path, indexers)| AssignTarget::new(path, indexers))
        };
        let op = {
            let p0 = punct([
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
            ]);
            langbox::_map(p0, punct_to_assign_op)
        };
        {
            let p0 = {
                let lhs1 = {
                    let lhs2 = {
                        let lhs3 = target;
                        let rhs3 = op;
                        langbox::_and_then(lhs3, rhs3)
                    };
                    let rhs2 = {
                        let p3 = expr(false);
                        langbox::_require(p3, |input| QuartzParserErrror {
                            message: "expected expression".into(),
                            span: input.current_span(),
                        })
                    };
                    langbox::_and_then(lhs2, rhs2)
                };
                let rhs1 = {
                    let p2 = punct(PunctKind::Semicolon);
                    langbox::_require(p2, |input| QuartzParserErrror {
                        message: "expected `;`".into(),
                        span: input.current_span(),
                    })
                };
                langbox::_prefix(lhs1, rhs1)
            };
            langbox::_map(p0, |((target, op), value)| {
                Assignment::new(target, op, value)
            })
        }
    }
    fn while_loop() -> impl QuartzParser<WhileLoop> {
        {
            let p0 = {
                let lhs1 = {
                    let lhs2 = kw(KeywordKind::While);
                    let rhs2 = {
                        let p3 = expr(true);
                        langbox::_require(p3, |input| QuartzParserErrror {
                            message: "expected expression".into(),
                            span: input.current_span(),
                        })
                    };
                    langbox::_and_then(lhs2, rhs2)
                };
                let rhs1 = {
                    let p2 = block();
                    langbox::_require(p2, |input| QuartzParserErrror {
                        message: "expected block".into(),
                        span: input.current_span(),
                    })
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |((while_kw, condition), body)| {
                WhileLoop::new(while_kw, condition, body)
            })
        }
    }
    fn for_loop() -> impl QuartzParser<ForLoop> {
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = kw(KeywordKind::For);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = ident();
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected identifier".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = kw(KeywordKind::In);
                                        langbox::_require(p0, |input| QuartzParserErrror {
                                            message: "expected `in`".into(),
                                            span: input.current_span(),
                                        })
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = expr(true);
                                                langbox::_require(p0, |input| QuartzParserErrror {
                                                    message: "expected expression".into(),
                                                    span: input.current_span(),
                                                })
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = {
                                                        let p0 = punct(PunctKind::DoublePeriod);
                                                        langbox::_require(p0, |input| {
                                                            QuartzParserErrror {
                                                                message: "expected `..`".into(),
                                                                span: input.current_span(),
                                                            }
                                                        })
                                                    };
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => {
                                                            let p5 = {
                                                                let p0 = expr(true);
                                                                langbox::_require(p0, |input| {
                                                                    QuartzParserErrror {
                                                                        message:
                                                                            "expected expression"
                                                                                .into(),
                                                                        span: input.current_span(),
                                                                    }
                                                                })
                                                            };
                                                            match p5 . run (remaining) ? { langbox :: InfallibleParseResult :: Match { value : v5 , span : s5 , remaining } => { let p6 = { let p0 = block () ; langbox :: _require (p0 , | input | QuartzParserErrror { message : "expected block" . into () , span : input . current_span () , }) } ; match p6 . run (remaining) ? { langbox :: InfallibleParseResult :: Match { value : v6 , span : s6 , remaining } => { langbox :: ParseResult :: Match { value : (v0 , v1 , v2 , v3 , v4 , v5 , v6) , span : langbox :: _join_spans (& [input . empty_span () , s0 , s1 , s2 , s3 , s4 , s5 , s6]) , remaining , } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , }
                                                        }
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(for_kw, item_name, in_kw, start, _, end, body)| {
                ForLoop::new(for_kw, item_name, in_kw, start..end, body)
            })
        }
    }
    fn statement() -> impl QuartzParser<Statement> {
        langbox::_constrain_parse_fn(move |input| {
            let p0 = {
                let p0 = if_expr();
                langbox::_map(p0, Statement::Expr)
            };
            match p0.run(input)? {
                langbox::InfallibleParseResult::Match {
                    value,
                    span,
                    remaining,
                } => langbox::ParseResult::Match {
                    value,
                    span,
                    remaining,
                },
                langbox::InfallibleParseResult::NoMatch => {
                    let p1 = {
                        let p0 = match_expr();
                        langbox::_map(p0, Statement::Expr)
                    };
                    match p1.run(input)? {
                        langbox::InfallibleParseResult::Match {
                            value,
                            span,
                            remaining,
                        } => langbox::ParseResult::Match {
                            value,
                            span,
                            remaining,
                        },
                        langbox::InfallibleParseResult::NoMatch => {
                            let p2 = {
                                let p0 = decl();
                                langbox::_map(p0, Statement::Declaration)
                            };
                            match p2.run(input)? {
                                langbox::InfallibleParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                } => langbox::ParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                },
                                langbox::InfallibleParseResult::NoMatch => {
                                    let p3 = {
                                        let p0 = while_loop();
                                        langbox::_map(p0, Statement::WhileLoop)
                                    };
                                    match p3.run(input)? {
                                        langbox::InfallibleParseResult::Match {
                                            value,
                                            span,
                                            remaining,
                                        } => langbox::ParseResult::Match {
                                            value,
                                            span,
                                            remaining,
                                        },
                                        langbox::InfallibleParseResult::NoMatch => {
                                            let p4 = {
                                                let p0 = for_loop();
                                                langbox::_map(p0, Statement::ForLoop)
                                            };
                                            match p4.run(input)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value,
                                                    span,
                                                    remaining,
                                                } => langbox::ParseResult::Match {
                                                    value,
                                                    span,
                                                    remaining,
                                                },
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    let p5 = {
                                                        let p0 = block();
                                                        langbox::_map(p0, |block| {
                                                            Statement::Expr(Expr::Block(Box::new(
                                                                block,
                                                            )))
                                                        })
                                                    };
                                                    match p5.run(input)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value,
                                                            span,
                                                            remaining,
                                                        } => langbox::ParseResult::Match {
                                                            value,
                                                            span,
                                                            remaining,
                                                        },
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            let p6 = {
                                                                let lhs0 = {
                                                                    let p1 = expr(false);
                                                                    langbox::_map(
                                                                        p1,
                                                                        Statement::Expr,
                                                                    )
                                                                };
                                                                let rhs0 =
                                                                    punct(PunctKind::Semicolon);
                                                                langbox::_prefix(lhs0, rhs0)
                                                            };
                                                            match p6 . run (input) ? { langbox :: InfallibleParseResult :: Match { value , span , remaining } => { langbox :: ParseResult :: Match { value , span , remaining } } langbox :: InfallibleParseResult :: NoMatch => { let p7 = { let p0 = assign () ; langbox :: _map (p0 , Statement :: Assignment) } ; match p7 . run (input) ? { langbox :: InfallibleParseResult :: Match { value , span , remaining } => { langbox :: ParseResult :: Match { value , span , remaining } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , } } }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
    fn block() -> impl QuartzParser<Block> {
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = punct(PunctKind::OpenCurl);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = sep_by(
                                statement(),
                                {
                                    let p0 = punct(PunctKind::Semicolon);
                                    langbox::_many(p0, true)
                                },
                                true,
                                false,
                            );
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = punct(PunctKind::Semicolon);
                                        langbox::_many(p0, true)
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = expr(false);
                                                langbox::_opt(p0)
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = {
                                                        let p0 = punct(PunctKind::CloseCurl);
                                                        langbox::_require(p0, |input| {
                                                            QuartzParserErrror {
                                                                message: "expected `}`".into(),
                                                                span: input.current_span(),
                                                            }
                                                        })
                                                    };
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => langbox::ParseResult::Match {
                                                            value: (v0, v1, v2, v3, v4),
                                                            span: langbox::_join_spans(&[
                                                                input.empty_span(),
                                                                s0,
                                                                s1,
                                                                s2,
                                                                s3,
                                                                s4,
                                                            ]),
                                                            remaining,
                                                        },
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(
                p0,
                |(open_curl, mut statements, trailing_semis, last_expr, close_curl)| {
                    if let Some(last_expr) = last_expr {
                        Block::new(open_curl, statements, Some(last_expr), close_curl)
                    } else {
                        if let Some (Statement :: Expr (_)) = statements . last () && (trailing_semis . len () == 0) { if let Some (Statement :: Expr (last_expr)) = statements . pop () { Block :: new (open_curl , statements , Some (last_expr) , close_curl) } else { :: core :: panicking :: panic ("internal error: entered unreachable code") } } else { Block :: new (open_curl , statements , None , close_curl) }
                    }
                },
            )
        }
    }
    fn named_ty() -> impl QuartzParser<NamedType> {
        let generic_arg = {
            let lhs0 = {
                let lhs1 = {
                    let p2 = literal();
                    langbox::_map(p2, GenericTypeArg::Literal)
                };
                let rhs1 = {
                    let p2 = path();
                    langbox::_map(p2, GenericTypeArg::Path)
                };
                langbox::_or_else(lhs1, rhs1)
            };
            let rhs0 = {
                let p1 = {
                    let lhs2 = {
                        let lhs3 = punct(PunctKind::OpenCurl);
                        let rhs3 = {
                            let p4 = expr(true);
                            langbox::_require(p4, |input| QuartzParserErrror {
                                message: "expected expression".into(),
                                span: input.current_span(),
                            })
                        };
                        langbox::_suffix(lhs3, rhs3)
                    };
                    let rhs2 = {
                        let p3 = punct(PunctKind::CloseCurl);
                        langbox::_require(p3, |input| QuartzParserErrror {
                            message: "expected `}`".into(),
                            span: input.current_span(),
                        })
                    };
                    langbox::_prefix(lhs2, rhs2)
                };
                langbox::_map(p1, GenericTypeArg::Expr)
            };
            langbox::_or_else(lhs0, rhs0)
        };
        let generic_args = {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = punct(PunctKind::Lt);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = sep_by(generic_arg, punct(PunctKind::Comma), false, true);
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected generic arguments".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = punct(PunctKind::Gt);
                                        langbox::_require(p0, |input| QuartzParserErrror {
                                            message: "expected `>`".into(),
                                            span: input.current_span(),
                                        })
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => langbox::ParseResult::Match {
                                            value: (v0, v1, v2),
                                            span: langbox::_join_spans(&[
                                                input.empty_span(),
                                                s0,
                                                s1,
                                                s2,
                                            ]),
                                            remaining,
                                        },
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(open_paren, exprs, close_paren)| {
                GenericTypeArgs::new(open_paren, exprs, close_paren)
            })
        };
        {
            let p0 = {
                let lhs1 = ident();
                let rhs1 = {
                    let p2 = generic_args;
                    langbox::_opt(p2)
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |(name, generic_args)| {
                NamedType::new(name, generic_args)
            })
        }
    }
    fn array_ty() -> impl QuartzParser<ArrayType> {
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = punct(PunctKind::OpenBracket);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = ty();
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected type".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = punct(PunctKind::Semicolon);
                                        langbox::_require(p0, |input| QuartzParserErrror {
                                            message: "expected `;`".into(),
                                            span: input.current_span(),
                                        })
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = expr(true);
                                                langbox::_require(p0, |input| QuartzParserErrror {
                                                    message: "expected expression".into(),
                                                    span: input.current_span(),
                                                })
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = {
                                                        let p0 = punct(PunctKind::CloseBracket);
                                                        langbox::_require(p0, |input| {
                                                            QuartzParserErrror {
                                                                message: "expected `]`".into(),
                                                                span: input.current_span(),
                                                            }
                                                        })
                                                    };
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => langbox::ParseResult::Match {
                                                            value: (v0, v1, v2, v3, v4),
                                                            span: langbox::_join_spans(&[
                                                                input.empty_span(),
                                                                s0,
                                                                s1,
                                                                s2,
                                                                s3,
                                                                s4,
                                                            ]),
                                                            remaining,
                                                        },
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(open_bracket, ty, sep, len, close_bracket)| {
                ArrayType::new(open_bracket, ty, sep, len, close_bracket)
            })
        }
    }
    fn ty() -> impl QuartzParser<Type> {
        langbox::_constrain_parse_fn(move |input| {
            let p0 = {
                let p0 = named_ty();
                langbox::_map(p0, Type::Named)
            };
            match p0.run(input)? {
                langbox::InfallibleParseResult::Match {
                    value,
                    span,
                    remaining,
                } => langbox::ParseResult::Match {
                    value,
                    span,
                    remaining,
                },
                langbox::InfallibleParseResult::NoMatch => {
                    let p1 = {
                        let p0 = array_ty();
                        langbox::_map(p0, Type::Array)
                    };
                    match p1.run(input)? {
                        langbox::InfallibleParseResult::Match {
                            value,
                            span,
                            remaining,
                        } => langbox::ParseResult::Match {
                            value,
                            span,
                            remaining,
                        },
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            }
        })
    }
    fn generic_args() -> impl QuartzParser<GenericStructArgs> {
        {
            let p0 = {
                let lhs1 = {
                    let lhs2 = punct(PunctKind::Lt);
                    let rhs2 = {
                        let p3 = sep_by(ident(), punct(PunctKind::Comma), false, true);
                        langbox::_require(p3, |input| QuartzParserErrror {
                            message: "expected generic arguments".into(),
                            span: input.current_span(),
                        })
                    };
                    langbox::_and_then(lhs2, rhs2)
                };
                let rhs1 = {
                    let p2 = punct(PunctKind::Gt);
                    langbox::_require(p2, |input| QuartzParserErrror {
                        message: "expected `>`".into(),
                        span: input.current_span(),
                    })
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |((open_paren, args), close_paren)| {
                GenericStructArgs::new(open_paren, args, close_paren)
            })
        }
    }
    fn field() -> impl QuartzParser<Field> {
        {
            let p0 = {
                let lhs1 = {
                    let lhs2 = ident();
                    let rhs2 = {
                        let p3 = punct(PunctKind::Colon);
                        langbox::_require(p3, |input| QuartzParserErrror {
                            message: "expected `:`".into(),
                            span: input.current_span(),
                        })
                    };
                    langbox::_and_then(lhs2, rhs2)
                };
                let rhs1 = {
                    let p2 = ty();
                    langbox::_require(p2, |input| QuartzParserErrror {
                        message: "expected type".into(),
                        span: input.current_span(),
                    })
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |((name, sep), ty)| Field::new(name, sep, ty))
        }
    }
    fn struct_def() -> impl QuartzParser<Struct> {
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = kw(KeywordKind::Struct);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = ident();
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected identifier".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = generic_args();
                                        langbox::_opt(p0)
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = punct(PunctKind::OpenCurl);
                                                langbox::_require(p0, |input| QuartzParserErrror {
                                                    message: "expected `{`".into(),
                                                    span: input.current_span(),
                                                })
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = sep_by(
                                                        field(),
                                                        punct(PunctKind::Comma),
                                                        true,
                                                        true,
                                                    );
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => {
                                                            let p5 = {
                                                                let p0 =
                                                                    punct(PunctKind::CloseCurl);
                                                                langbox::_require(p0, |input| {
                                                                    QuartzParserErrror {
                                                                        message: "expected `}`"
                                                                            .into(),
                                                                        span: input.current_span(),
                                                                    }
                                                                })
                                                            };
                                                            match p5 . run (remaining) ? { langbox :: InfallibleParseResult :: Match { value : v5 , span : s5 , remaining } => { langbox :: ParseResult :: Match { value : (v0 , v1 , v2 , v3 , v4 , v5) , span : langbox :: _join_spans (& [input . empty_span () , s0 , s1 , s2 , s3 , s4 , s5]) , remaining , } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , }
                                                        }
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(
                p0,
                |(struct_kw, name, generic_args, open_curl, fields, close_curl)| {
                    Struct::new(struct_kw, name, generic_args, open_curl, fields, close_curl)
                },
            )
        }
    }
    fn variant() -> impl QuartzParser<Variant> {
        let value = {
            let lhs0 = punct(PunctKind::Assign);
            let rhs0 = {
                let p1 = expr(true);
                langbox::_require(p1, |input| QuartzParserErrror {
                    message: "expected expression".into(),
                    span: input.current_span(),
                })
            };
            langbox::_suffix(lhs0, rhs0)
        };
        {
            let p0 = {
                let lhs1 = ident();
                let rhs1 = {
                    let p2 = value;
                    langbox::_opt(p2)
                };
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |(name, value)| Variant::new(name, value))
        }
    }
    fn enum_def() -> impl QuartzParser<Enum> {
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = kw(KeywordKind::Enum);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = ident();
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected identifier".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = punct(PunctKind::Colon);
                                        langbox::_require(p0, |input| QuartzParserErrror {
                                            message: "expected `:`".into(),
                                            span: input.current_span(),
                                        })
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = ty();
                                                langbox::_require(p0, |input| QuartzParserErrror {
                                                    message: "expected type".into(),
                                                    span: input.current_span(),
                                                })
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = {
                                                        let p0 = punct(PunctKind::OpenCurl);
                                                        langbox::_require(p0, |input| {
                                                            QuartzParserErrror {
                                                                message: "expected `{`".into(),
                                                                span: input.current_span(),
                                                            }
                                                        })
                                                    };
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => {
                                                            let p5 = sep_by(
                                                                variant(),
                                                                punct(PunctKind::Comma),
                                                                true,
                                                                true,
                                                            );
                                                            match p5 . run (remaining) ? { langbox :: InfallibleParseResult :: Match { value : v5 , span : s5 , remaining } => { let p6 = { let p0 = punct (PunctKind :: CloseCurl) ; langbox :: _require (p0 , | input | QuartzParserErrror { message : "expected  `}`" . into () , span : input . current_span () , }) } ; match p6 . run (remaining) ? { langbox :: InfallibleParseResult :: Match { value : v6 , span : s6 , remaining } => { langbox :: ParseResult :: Match { value : (v0 , v1 , v2 , v3 , v4 , v5 , v6) , span : langbox :: _join_spans (& [input . empty_span () , s0 , s1 , s2 , s3 , s4 , s5 , s6]) , remaining , } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , }
                                                        }
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(
                p0,
                |(enum_kw, name, ty_sep, base_ty, open_curl, variants, close_curl)| {
                    Enum::new(
                        enum_kw, name, ty_sep, base_ty, open_curl, variants, close_curl,
                    )
                },
            )
        }
    }
    fn const_def() -> impl QuartzParser<Const> {
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = kw(KeywordKind::Const);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = ident();
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected identifier".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = punct(PunctKind::Assign);
                                        langbox::_require(p0, |input| QuartzParserErrror {
                                            message: "expected `=`".into(),
                                            span: input.current_span(),
                                        })
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = expr(false);
                                                langbox::_require(p0, |input| QuartzParserErrror {
                                                    message: "expected expression".into(),
                                                    span: input.current_span(),
                                                })
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = {
                                                        let p0 = punct(PunctKind::Semicolon);
                                                        langbox::_require(p0, |input| {
                                                            QuartzParserErrror {
                                                                message: "expected `;`".into(),
                                                                span: input.current_span(),
                                                            }
                                                        })
                                                    };
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => langbox::ParseResult::Match {
                                                            value: (v0, v1, v2, v3, v4),
                                                            span: langbox::_join_spans(&[
                                                                input.empty_span(),
                                                                s0,
                                                                s1,
                                                                s2,
                                                                s3,
                                                                s4,
                                                            ]),
                                                            remaining,
                                                        },
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(const_kw, name, assign, value, _)| {
                Const::new(const_kw, name, assign, value)
            })
        }
    }
    fn logic_mode() -> impl QuartzParser<LogicMode> {
        langbox::_constrain_parse_fn(move |input| {
            let p0 = {
                let p0 = kw(KeywordKind::Sig);
                langbox::_map(p0, |kw| LogicMode::new(LogicKind::Signal, kw.span()))
            };
            match p0.run(input)? {
                langbox::InfallibleParseResult::Match {
                    value,
                    span,
                    remaining,
                } => langbox::ParseResult::Match {
                    value,
                    span,
                    remaining,
                },
                langbox::InfallibleParseResult::NoMatch => {
                    let p1 = {
                        let p0 = kw(KeywordKind::Reg);
                        langbox::_map(p0, |kw| LogicMode::new(LogicKind::Register, kw.span()))
                    };
                    match p1.run(input)? {
                        langbox::InfallibleParseResult::Match {
                            value,
                            span,
                            remaining,
                        } => langbox::ParseResult::Match {
                            value,
                            span,
                            remaining,
                        },
                        langbox::InfallibleParseResult::NoMatch => {
                            let p2 = {
                                let p0 = kw(KeywordKind::Let);
                                langbox::_map(p0, |kw| LogicMode::new(LogicKind::Module, kw.span()))
                            };
                            match p2.run(input)? {
                                langbox::InfallibleParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                } => langbox::ParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                },
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                    }
                }
            }
        })
    }
    fn port_mode() -> impl QuartzParser<PortMode> {
        langbox::_constrain_parse_fn(move |input| {
            let p0 = {
                let p0 = kw(KeywordKind::In);
                langbox::_map(p0, |kw| PortMode::new(Direction::In, kw.span()))
            };
            match p0.run(input)? {
                langbox::InfallibleParseResult::Match {
                    value,
                    span,
                    remaining,
                } => langbox::ParseResult::Match {
                    value,
                    span,
                    remaining,
                },
                langbox::InfallibleParseResult::NoMatch => {
                    let p1 = {
                        let p0 = kw(KeywordKind::Out);
                        langbox::_map(p0, |kw| PortMode::new(Direction::Out, kw.span()))
                    };
                    match p1.run(input)? {
                        langbox::InfallibleParseResult::Match {
                            value,
                            span,
                            remaining,
                        } => langbox::ParseResult::Match {
                            value,
                            span,
                            remaining,
                        },
                        langbox::InfallibleParseResult::NoMatch => {
                            let p2 = {
                                let p0 = kw(KeywordKind::InOut);
                                langbox::_map(p0, |kw| PortMode::new(Direction::InOut, kw.span()))
                            };
                            match p2.run(input)? {
                                langbox::InfallibleParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                } => langbox::ParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                },
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                    }
                }
            }
        })
    }
    fn port() -> impl QuartzParser<Port> {
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = port_mode();
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = logic_mode();
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected `sig` or `reg`".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = ident();
                                        langbox::_require(p0, |input| QuartzParserErrror {
                                            message: "expected identifier".into(),
                                            span: input.current_span(),
                                        })
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = punct(PunctKind::Colon);
                                                langbox::_require(p0, |input| QuartzParserErrror {
                                                    message: "expected `:`".into(),
                                                    span: input.current_span(),
                                                })
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = {
                                                        let p0 = ty();
                                                        langbox::_require(p0, |input| {
                                                            QuartzParserErrror {
                                                                message: "expected type".into(),
                                                                span: input.current_span(),
                                                            }
                                                        })
                                                    };
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => langbox::ParseResult::Match {
                                                            value: (v0, v1, v2, v3, v4),
                                                            span: langbox::_join_spans(&[
                                                                input.empty_span(),
                                                                s0,
                                                                s1,
                                                                s2,
                                                                s3,
                                                                s4,
                                                            ]),
                                                            remaining,
                                                        },
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(mode, logic_mode, name, sep, ty)| {
                Port::new(mode, logic_mode, name, sep, ty)
            })
        }
    }
    fn member() -> impl QuartzParser<Member> {
        let logic = {
            let p0 = {
                let lhs1 = {
                    let lhs2 = {
                        let lhs3 = {
                            let lhs4 = logic_mode();
                            let rhs4 = {
                                let p5 = ident();
                                langbox::_require(p5, |input| QuartzParserErrror {
                                    message: "expected identifier".into(),
                                    span: input.current_span(),
                                })
                            };
                            langbox::_and_then(lhs4, rhs4)
                        };
                        let rhs3 = {
                            let p4 = punct(PunctKind::Colon);
                            langbox::_require(p4, |input| QuartzParserErrror {
                                message: "expected `:`".into(),
                                span: input.current_span(),
                            })
                        };
                        langbox::_and_then(lhs3, rhs3)
                    };
                    let rhs2 = {
                        let p3 = ty();
                        langbox::_require(p3, |input| QuartzParserErrror {
                            message: "expected type".into(),
                            span: input.current_span(),
                        })
                    };
                    langbox::_and_then(lhs2, rhs2)
                };
                let rhs1 = {
                    let p2 = punct(PunctKind::Semicolon);
                    langbox::_require(p2, |input| QuartzParserErrror {
                        message: "expected `;`".into(),
                        span: input.current_span(),
                    })
                };
                langbox::_prefix(lhs1, rhs1)
            };
            langbox::_map(p0, |(((kind, name), sep), ty)| {
                LogicMember::new(kind, name, sep, ty)
            })
        };
        let edge = langbox::_constrain_parse_fn(move |input| {
            let p0 = {
                let p0 = kw(KeywordKind::Rising);
                langbox::_map(p0, |kw| Edge::new(EdgeKind::Rising, kw.span()))
            };
            match p0.run(input)? {
                langbox::InfallibleParseResult::Match {
                    value,
                    span,
                    remaining,
                } => langbox::ParseResult::Match {
                    value,
                    span,
                    remaining,
                },
                langbox::InfallibleParseResult::NoMatch => {
                    let p1 = {
                        let p0 = kw(KeywordKind::Rising);
                        langbox::_map(p0, |kw| Edge::new(EdgeKind::Rising, kw.span()))
                    };
                    match p1.run(input)? {
                        langbox::InfallibleParseResult::Match {
                            value,
                            span,
                            remaining,
                        } => langbox::ParseResult::Match {
                            value,
                            span,
                            remaining,
                        },
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            }
        });
        let sens = {
            let p0 = {
                let lhs1 = {
                    let lhs2 = {
                        let lhs3 = edge;
                        let rhs3 = punct(PunctKind::OpenParen);
                        langbox::_and_then(lhs3, rhs3)
                    };
                    let rhs2 = path();
                    langbox::_and_then(lhs2, rhs2)
                };
                let rhs1 = punct(PunctKind::CloseParen);
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |(((edge, open_paren), sig), close_paren)| {
                Sens::new(edge, open_paren, sig, close_paren)
            })
        };
        let proc = {
            let p0 = {
                let lhs1 = {
                    let lhs2 = kw(KeywordKind::Proc);
                    let rhs2 = {
                        let p3 = sep_by(sens, punct(PunctKind::Or), false, false);
                        langbox::_require(p3, |input| QuartzParserErrror {
                            message: "expected sensitivity list".into(),
                            span: input.current_span(),
                        })
                    };
                    langbox::_and_then(lhs2, rhs2)
                };
                let rhs1 = block();
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |((proc_kw, sens), body)| {
                ProcMember::new(proc_kw, sens, body)
            })
        };
        let comb = {
            let p0 = {
                let lhs1 = kw(KeywordKind::Comb);
                let rhs1 = block();
                langbox::_and_then(lhs1, rhs1)
            };
            langbox::_map(p0, |(comb_kw, body)| CombMember::new(comb_kw, body))
        };
        langbox::_constrain_parse_fn(move |input| {
            let p0 = {
                let p0 = logic;
                langbox::_map(p0, Member::Logic)
            };
            match p0.run(input)? {
                langbox::InfallibleParseResult::Match {
                    value,
                    span,
                    remaining,
                } => langbox::ParseResult::Match {
                    value,
                    span,
                    remaining,
                },
                langbox::InfallibleParseResult::NoMatch => {
                    let p1 = {
                        let p0 = const_def();
                        langbox::_map(p0, Member::Const)
                    };
                    match p1.run(input)? {
                        langbox::InfallibleParseResult::Match {
                            value,
                            span,
                            remaining,
                        } => langbox::ParseResult::Match {
                            value,
                            span,
                            remaining,
                        },
                        langbox::InfallibleParseResult::NoMatch => {
                            let p2 = {
                                let p0 = proc;
                                langbox::_map(p0, Member::Proc)
                            };
                            match p2.run(input)? {
                                langbox::InfallibleParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                } => langbox::ParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                },
                                langbox::InfallibleParseResult::NoMatch => {
                                    let p3 = {
                                        let p0 = comb;
                                        langbox::_map(p0, Member::Comb)
                                    };
                                    match p3.run(input)? {
                                        langbox::InfallibleParseResult::Match {
                                            value,
                                            span,
                                            remaining,
                                        } => langbox::ParseResult::Match {
                                            value,
                                            span,
                                            remaining,
                                        },
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
    fn module_def() -> impl QuartzParser<Module> {
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = kw(KeywordKind::Mod);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = ident();
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected identifier".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = generic_args();
                                        langbox::_opt(p0)
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = {
                                                let p0 = punct(PunctKind::OpenParen);
                                                langbox::_require(p0, |input| QuartzParserErrror {
                                                    message: "expected `(`".into(),
                                                    span: input.current_span(),
                                                })
                                            };
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = sep_by(
                                                        port(),
                                                        punct(PunctKind::Comma),
                                                        true,
                                                        true,
                                                    );
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => {
                                                            let p5 = {
                                                                let p0 =
                                                                    punct(PunctKind::CloseParen);
                                                                langbox::_require(p0, |input| {
                                                                    QuartzParserErrror {
                                                                        message: "expected `)`"
                                                                            .into(),
                                                                        span: input.current_span(),
                                                                    }
                                                                })
                                                            };
                                                            match p5 . run (remaining) ? { langbox :: InfallibleParseResult :: Match { value : v5 , span : s5 , remaining } => { let p6 = { let p0 = punct (PunctKind :: OpenCurl) ; langbox :: _require (p0 , | input | QuartzParserErrror { message : "expected `{`" . into () , span : input . current_span () , }) } ; match p6 . run (remaining) ? { langbox :: InfallibleParseResult :: Match { value : v6 , span : s6 , remaining } => { let p7 = { let p0 = member () ; langbox :: _many (p0 , true) } ; match p7 . run (remaining) ? { langbox :: InfallibleParseResult :: Match { value : v7 , span : s7 , remaining } => { let p8 = { let p0 = punct (PunctKind :: CloseCurl) ; langbox :: _require (p0 , | input | QuartzParserErrror { message : "expected `}`" . into () , span : input . current_span () , }) } ; match p8 . run (remaining) ? { langbox :: InfallibleParseResult :: Match { value : v8 , span : s8 , remaining } => { langbox :: ParseResult :: Match { value : (v0 , v1 , v2 , v3 , v4 , v5 , v6 , v7 , v8) , span : langbox :: _join_spans (& [input . empty_span () , s0 , s1 , s2 , s3 , s4 , s5 , s6 , s7 , s8]) , remaining , } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , }
                                                        }
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(
                p0,
                |(
                    mod_kw,
                    name,
                    generic_args,
                    open_paren,
                    ports,
                    close_paren,
                    open_curl,
                    members,
                    close_curl,
                )| {
                    Module::new(
                        mod_kw,
                        name,
                        generic_args,
                        open_paren,
                        ports,
                        close_paren,
                        open_curl,
                        members,
                        close_curl,
                    )
                },
            )
        }
    }
    fn fn_def() -> impl QuartzParser<Func> {
        {
            let p0 = langbox::_constrain_parse_fn(move |input| {
                let remaining = input;
                {
                    let p0 = kw(KeywordKind::Fn);
                    match p0.run(remaining)? {
                        langbox::InfallibleParseResult::Match {
                            value: v0,
                            span: s0,
                            remaining,
                        } => {
                            let p1 = {
                                let p0 = ident();
                                langbox::_require(p0, |input| QuartzParserErrror {
                                    message: "expected identifier".into(),
                                    span: input.current_span(),
                                })
                            };
                            match p1.run(remaining)? {
                                langbox::InfallibleParseResult::Match {
                                    value: v1,
                                    span: s1,
                                    remaining,
                                } => {
                                    let p2 = {
                                        let p0 = punct(PunctKind::OpenParen);
                                        langbox::_require(p0, |input| QuartzParserErrror {
                                            message: "expected `(`".into(),
                                            span: input.current_span(),
                                        })
                                    };
                                    match p2.run(remaining)? {
                                        langbox::InfallibleParseResult::Match {
                                            value: v2,
                                            span: s2,
                                            remaining,
                                        } => {
                                            let p3 = sep_by(
                                                ident(),
                                                punct(PunctKind::Comma),
                                                true,
                                                true,
                                            );
                                            match p3.run(remaining)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value: v3,
                                                    span: s3,
                                                    remaining,
                                                } => {
                                                    let p4 = {
                                                        let p0 = punct(PunctKind::CloseParen);
                                                        langbox::_require(p0, |input| {
                                                            QuartzParserErrror {
                                                                message: "expected `)`".into(),
                                                                span: input.current_span(),
                                                            }
                                                        })
                                                    };
                                                    match p4.run(remaining)? {
                                                        langbox::InfallibleParseResult::Match {
                                                            value: v4,
                                                            span: s4,
                                                            remaining,
                                                        } => {
                                                            let p5 = {
                                                                let p0 = block();
                                                                langbox::_require(p0, |input| {
                                                                    QuartzParserErrror {
                                                                        message: "expected block"
                                                                            .into(),
                                                                        span: input.current_span(),
                                                                    }
                                                                })
                                                            };
                                                            match p5 . run (remaining) ? { langbox :: InfallibleParseResult :: Match { value : v5 , span : s5 , remaining } => { langbox :: ParseResult :: Match { value : (v0 , v1 , v2 , v3 , v4 , v5) , span : langbox :: _join_spans (& [input . empty_span () , s0 , s1 , s2 , s3 , s4 , s5]) , remaining , } } langbox :: InfallibleParseResult :: NoMatch => langbox :: ParseResult :: NoMatch , }
                                                        }
                                                        langbox::InfallibleParseResult::NoMatch => {
                                                            langbox::ParseResult::NoMatch
                                                        }
                                                    }
                                                }
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                        langbox::InfallibleParseResult::NoMatch => {
                                            langbox::ParseResult::NoMatch
                                        }
                                    }
                                }
                                langbox::InfallibleParseResult::NoMatch => {
                                    langbox::ParseResult::NoMatch
                                }
                            }
                        }
                        langbox::InfallibleParseResult::NoMatch => langbox::ParseResult::NoMatch,
                    }
                }
            });
            langbox::_map(p0, |(fn_kw, name, open_paren, args, close_paren, body)| {
                Func::new(fn_kw, name, open_paren, args, close_paren, body)
            })
        }
    }
    fn item() -> impl QuartzParser<Item> {
        langbox::_constrain_parse_fn(move |input| {
            let p0 = {
                let p0 = struct_def();
                langbox::_map(p0, Item::Struct)
            };
            match p0.run(input)? {
                langbox::InfallibleParseResult::Match {
                    value,
                    span,
                    remaining,
                } => langbox::ParseResult::Match {
                    value,
                    span,
                    remaining,
                },
                langbox::InfallibleParseResult::NoMatch => {
                    let p1 = {
                        let p0 = enum_def();
                        langbox::_map(p0, Item::Enum)
                    };
                    match p1.run(input)? {
                        langbox::InfallibleParseResult::Match {
                            value,
                            span,
                            remaining,
                        } => langbox::ParseResult::Match {
                            value,
                            span,
                            remaining,
                        },
                        langbox::InfallibleParseResult::NoMatch => {
                            let p2 = {
                                let p0 = const_def();
                                langbox::_map(p0, Item::Const)
                            };
                            match p2.run(input)? {
                                langbox::InfallibleParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                } => langbox::ParseResult::Match {
                                    value,
                                    span,
                                    remaining,
                                },
                                langbox::InfallibleParseResult::NoMatch => {
                                    let p3 = {
                                        let p0 = module_def();
                                        langbox::_map(p0, Item::Module)
                                    };
                                    match p3.run(input)? {
                                        langbox::InfallibleParseResult::Match {
                                            value,
                                            span,
                                            remaining,
                                        } => langbox::ParseResult::Match {
                                            value,
                                            span,
                                            remaining,
                                        },
                                        langbox::InfallibleParseResult::NoMatch => {
                                            let p4 = {
                                                let p0 = fn_def();
                                                langbox::_map(p0, Item::Func)
                                            };
                                            match p4.run(input)? {
                                                langbox::InfallibleParseResult::Match {
                                                    value,
                                                    span,
                                                    remaining,
                                                } => langbox::ParseResult::Match {
                                                    value,
                                                    span,
                                                    remaining,
                                                },
                                                langbox::InfallibleParseResult::NoMatch => {
                                                    langbox::ParseResult::NoMatch
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
    pub fn design() -> impl QuartzParser<Vec<Item>> {
        {
            let p0 = item();
            langbox::_many(p0, true)
        }
    }
}
mod typecheck {
    use crate::ast::*;
    use crate::ir::*;
    use crate::{SharedString, write_styled, writeln_styled};
    use std::collections::HashMap;
    use std::collections::HashSet;
    pub enum TypecheckError<'a> {
        DuplicateIdent {
            name: Ident,
        },
        InvalidConstExpr {
            expr: &'a Expr,
        },
        InvalidConstPattern {
            pattern: &'a MatchPattern,
        },
        NonExhaustiveMatch {
            match_expr: &'a MatchExpr,
        },
        InvalidConstOp {
            op: Punct,
        },
        InvalidConstAssignTarget {
            target: &'a AssignTarget,
        },
        MissingReturnValue {
            block: &'a Block,
        },
        UnexpectedReturnValue {
            value: &'a Expr,
        },
        UndefinedIdent {
            name: Ident,
        },
        TargetNotAssignable {
            name: Ident,
        },
        ValueNotConst {
            name: Ident,
        },
        InvalidValueIdent {
            name: Ident,
        },
        InvalidFuncIdent {
            name: Ident,
        },
        MissingElseBlock {
            if_expr: &'a IfExpr,
        },
        ArgumentCountMismatch {
            call_expr: &'a CallExpr,
            arg_count: usize,
        },
        UnknownType {
            ty: Type,
        },
        InvalidType {
            ty: Type,
        },
        List(Vec<TypecheckError<'a>>),
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for TypecheckError<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                TypecheckError::DuplicateIdent { name: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "DuplicateIdent",
                        "name",
                        &__self_0,
                    )
                }
                TypecheckError::InvalidConstExpr { expr: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InvalidConstExpr",
                        "expr",
                        &__self_0,
                    )
                }
                TypecheckError::InvalidConstPattern { pattern: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InvalidConstPattern",
                        "pattern",
                        &__self_0,
                    )
                }
                TypecheckError::NonExhaustiveMatch {
                    match_expr: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "NonExhaustiveMatch",
                    "match_expr",
                    &__self_0,
                ),
                TypecheckError::InvalidConstOp { op: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InvalidConstOp",
                        "op",
                        &__self_0,
                    )
                }
                TypecheckError::InvalidConstAssignTarget { target: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InvalidConstAssignTarget",
                        "target",
                        &__self_0,
                    )
                }
                TypecheckError::MissingReturnValue { block: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "MissingReturnValue",
                        "block",
                        &__self_0,
                    )
                }
                TypecheckError::UnexpectedReturnValue { value: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UnexpectedReturnValue",
                        "value",
                        &__self_0,
                    )
                }
                TypecheckError::UndefinedIdent { name: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UndefinedIdent",
                        "name",
                        &__self_0,
                    )
                }
                TypecheckError::TargetNotAssignable { name: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "TargetNotAssignable",
                        "name",
                        &__self_0,
                    )
                }
                TypecheckError::ValueNotConst { name: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "ValueNotConst",
                        "name",
                        &__self_0,
                    )
                }
                TypecheckError::InvalidValueIdent { name: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InvalidValueIdent",
                        "name",
                        &__self_0,
                    )
                }
                TypecheckError::InvalidFuncIdent { name: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InvalidFuncIdent",
                        "name",
                        &__self_0,
                    )
                }
                TypecheckError::MissingElseBlock { if_expr: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "MissingElseBlock",
                        "if_expr",
                        &__self_0,
                    )
                }
                TypecheckError::ArgumentCountMismatch {
                    call_expr: __self_0,
                    arg_count: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ArgumentCountMismatch",
                    "call_expr",
                    &__self_0,
                    "arg_count",
                    &__self_1,
                ),
                TypecheckError::UnknownType { ty: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UnknownType",
                        "ty",
                        &__self_0,
                    )
                }
                TypecheckError::InvalidType { ty: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InvalidType",
                        "ty",
                        &__self_0,
                    )
                }
                TypecheckError::List(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "List", &__self_0)
                }
            }
        }
    }
    impl TypecheckError<'_> {
        fn write_colored(&self, stream: &mut termcolor::StandardStream) -> std::io::Result<()> {
            {
                use termcolor::Color::*;
                crate::_set_style(stream, Red, true, false).and_then(|_| {
                    use std::io::Write;
                    stream.write_fmt(::core::fmt::Arguments::new_v1(&["Error"], &[]))
                })
            }?;
            {
                use termcolor::Color::*;
                crate::_set_style(stream, White, true, false).and_then(|_| {
                    use std::io::Write;
                    stream.write_fmt(::core::fmt::Arguments::new_v1(&[": "], &[]))
                })
            }?;
            let error_span = match self {
                Self::DuplicateIdent { name } => {
                    {
                        use termcolor::Color::*;
                        crate::_set_style(stream, White, true, false).and_then(|_| {
                            use std::io::Write;
                            stream.write_fmt(::core::fmt::Arguments::new_v1(
                                &["`", "` has already been defined\n"],
                                &[::core::fmt::ArgumentV1::new_display(&name)],
                            ))
                        })
                    }?;
                    name.span()
                }
                Self::InvalidConstExpr { expr } => {
                    {
                        use termcolor::Color::*;
                        crate::_set_style(stream, White, true, false).and_then(|_| {
                            use std::io::Write;
                            stream.write_fmt(::core::fmt::Arguments::new_v1(
                                &["expression is not valid in a constant context\n"],
                                &[],
                            ))
                        })
                    }?;
                    expr.span()
                }
                Self::InvalidConstPattern { pattern } => {
                    {
                        use termcolor::Color::*;
                        crate::_set_style(stream, White, true, false).and_then(|_| {
                            use std::io::Write;
                            stream.write_fmt(::core::fmt::Arguments::new_v1(
                                &["pattern is not valid in a constant context\n"],
                                &[],
                            ))
                        })
                    }?;
                    pattern.span()
                }
                Self::NonExhaustiveMatch { match_expr } => {
                    {
                        use termcolor::Color::*;
                        crate::_set_style(stream, White, true, false).and_then(|_| {
                            use std::io::Write;
                            stream.write_fmt(::core::fmt::Arguments::new_v1(
                                &["match expression is not exhaustive\n"],
                                &[],
                            ))
                        })
                    }?;
                    match_expr
                        .match_kw()
                        .span()
                        .join(&match_expr.value().span())
                }
                Self::InvalidConstOp { op } => {
                    use termcolor::Color::*;
                    crate::_set_style(stream, White, true, false).and_then(|_| {
                        use std::io::Write;
                        stream.write_fmt(::core::fmt::Arguments::new_v1(
                            &[
                                "Error: the `",
                                "` operator is not valid in a constant context.\n",
                            ],
                            &[::core::fmt::ArgumentV1::new_display(&op)],
                        ))
                    })
                }
                Self::InvalidConstAssignTarget { target } => {
                    use termcolor::Color::*;
                    crate::_set_style(stream, White, true, false).and_then(|_| {
                        use std::io::Write;
                        stream.write_fmt(::core::fmt::Arguments::new_v1(
                            &["Error: target expression is not valid in a constant context.\n"],
                            &[],
                        ))
                    })
                }
                Self::MissingReturnValue { block } => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Error: expected a return value.\n"],
                    &[],
                )),
                Self::UnexpectedReturnValue { value } => {
                    f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["Error: a return value is not expected in this context.\n"],
                        &[],
                    ))
                }
                Self::UndefinedIdent { name } => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Error: `", "` is not defined.\n"],
                    &[::core::fmt::ArgumentV1::new_display(&name)],
                )),
                Self::TargetNotAssignable { name } => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Error: `", "` is constant.\n"],
                    &[::core::fmt::ArgumentV1::new_display(&name)],
                )),
                Self::ValueNotConst { name } => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Error: `", "` is not constant.\n"],
                    &[::core::fmt::ArgumentV1::new_display(&name)],
                )),
                Self::InvalidValueIdent { name } => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Error: `", "` is not a value.\n"],
                    &[::core::fmt::ArgumentV1::new_display(&name)],
                )),
                Self::InvalidFuncIdent { name } => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Error: `", "` is not a function.\n"],
                    &[::core::fmt::ArgumentV1::new_display(&name)],
                )),
                Self::UnknownType { ty } => ::core::panicking::panic("not yet implemented"),
                Self::InvalidType { ty } => ::core::panicking::panic("not yet implemented"),
                Self::MissingElseBlock { if_expr } => {
                    ::core::panicking::panic("not yet implemented")
                }
                Self::ArgumentCountMismatch {
                    call_expr,
                    arg_count,
                } => ::core::panicking::panic("not yet implemented"),
                Self::List(list) => {
                    for (i, err) in list.iter().enumerate() {
                        if i == 0 {
                            std::fmt::Display::fmt(err, f)?;
                        } else {
                            f.write_fmt(::core::fmt::Arguments::new_v1(
                                &["\n"],
                                &[::core::fmt::ArgumentV1::new_display(&err)],
                            ))?;
                        }
                    }
                }
            };
            Ok(())
        }
    }
    pub type TypecheckResult<'a, T> = Result<T, TypecheckError<'a>>;
    pub struct Scope<'p> {
        parent: Option<&'p Scope<'p>>,
        consts: HashSet<SharedString>,
        funcs: HashMap<SharedString, usize>,
        vars: HashSet<SharedString>,
    }
    impl<'p> Scope<'p> {
        pub fn empty() -> Self {
            Self {
                parent: None,
                consts: HashSet::new(),
                funcs: HashMap::new(),
                vars: HashSet::new(),
            }
        }
        pub fn new<'pp: 'p>(parent: &'p Scope<'pp>) -> Self {
            Self {
                parent: Some(parent),
                consts: HashSet::new(),
                funcs: HashMap::new(),
                vars: HashSet::new(),
            }
        }
        pub fn add_const(&mut self, name: &Ident) {
            self.consts.insert(name.as_string());
        }
        pub fn add_func(&mut self, name: &Ident, arg_count: usize) {
            self.funcs.insert(name.as_string(), arg_count);
        }
        pub fn add_var(&mut self, name: &Ident) {
            self.vars.insert(name.as_string());
        }
        fn contains_const_inner(&self, name: &Ident) -> bool {
            self.consts.contains(name.as_ref())
                || self
                    .parent
                    .map_or(false, |parent| parent.contains_const_inner(name))
        }
        fn contains_func_inner(&self, name: &Ident) -> Option<usize> {
            self.funcs.get(name.as_ref()).copied().or_else(|| {
                self.parent
                    .and_then(|parent| parent.contains_func_inner(name))
            })
        }
        fn contains_var_inner(&self, name: &Ident) -> bool {
            self.vars.contains(name.as_ref())
                || self
                    .parent
                    .map_or(false, |parent| parent.contains_var_inner(name))
        }
        pub fn contains_func<'err>(&self, name: &Ident) -> TypecheckResult<'err, usize> {
            if let Some(arg_count) = self.contains_func_inner(name) {
                Ok(arg_count)
            } else if self.contains_const_inner(name) || self.contains_var_inner(name) {
                Err(TypecheckError::InvalidFuncIdent { name: name.clone() })
            } else {
                Err(TypecheckError::UndefinedIdent { name: name.clone() })
            }
        }
        pub fn contains_const<'err>(&self, name: &Ident) -> TypecheckResult<'err, ()> {
            if self.contains_const_inner(name) {
                Ok(())
            } else if self.contains_var_inner(name) {
                Err(TypecheckError::ValueNotConst { name: name.clone() })
            } else if self.contains_func_inner(name).is_some() {
                Err(TypecheckError::InvalidValueIdent { name: name.clone() })
            } else {
                Err(TypecheckError::UndefinedIdent { name: name.clone() })
            }
        }
        pub fn contains_var<'err>(&self, name: &Ident) -> TypecheckResult<'err, ()> {
            if self.contains_const_inner(name) || self.contains_var_inner(name) {
                Ok(())
            } else if self.contains_func_inner(name).is_some() {
                Err(TypecheckError::InvalidValueIdent { name: name.clone() })
            } else {
                Err(TypecheckError::UndefinedIdent { name: name.clone() })
            }
        }
        pub fn contains_mut_var<'err>(&self, name: &Ident) -> TypecheckResult<'err, ()> {
            if self.contains_var_inner(name) {
                Ok(())
            } else if self.contains_const_inner(name) {
                Err(TypecheckError::TargetNotAssignable { name: name.clone() })
            } else if self.contains_func_inner(name).is_some() {
                Err(TypecheckError::InvalidValueIdent { name: name.clone() })
            } else {
                Err(TypecheckError::UndefinedIdent { name: name.clone() })
            }
        }
    }
    pub fn check_for_duplicate_idents<'a>(
        items: impl Iterator<Item = &'a Item>,
    ) -> TypecheckResult<'static, ()> {
        let mut errors = Vec::new();
        let mut set = HashSet::new();
        for item in items.into_iter() {
            if set.contains(item.name().as_ref()) {
                errors.push(TypecheckError::DuplicateIdent {
                    name: item.name().clone(),
                })
            } else {
                set.insert(item.name().as_string());
            }
        }
        if errors.len() == 0 {
            let _value = ();
            #[allow(unreachable_code)]
            Ok(_value)
        } else if errors.len() == 1 {
            Err(errors.into_iter().next().unwrap())
        } else {
            Err(TypecheckError::List(errors))
        }
    }
    fn transform_const_bin_expr<'a>(
        expr: &'a BinaryExpr,
        scope: &mut Scope,
    ) -> TypecheckResult<'a, ConstBinaryExpr> {
        let lhs = transform_const_expr(expr.lhs(), scope, false);
        let rhs = transform_const_expr(expr.rhs(), scope, false);
        match lhs {
            Ok(lhs) => match rhs {
                Ok(rhs) => Ok(ConstBinaryExpr::new(lhs, rhs)),
                Err(err) => Err(err),
            },
            Err(err1) => match rhs {
                Ok(_) => Err(err1),
                Err(err2) => Err(TypecheckError::List([err1, err2].into())),
            },
        }
    }
    fn transform_const_if_expr<'a>(
        expr: &'a IfExpr,
        scope: &mut Scope,
        needs_return: bool,
    ) -> TypecheckResult<'a, ConstIfExpr> {
        let mut errors = Vec::new();
        let condition = transform_const_expr(expr.condition(), scope, false);
        let body = transform_const_block(expr.body(), scope, needs_return);
        let condition = match condition {
            Ok(condition) => Some(condition),
            Err(err) => {
                errors.push(err);
                None
            }
        };
        let body = match body {
            Ok(body) => Some(body),
            Err(err) => {
                errors.push(err);
                None
            }
        };
        let mut else_if_blocks = Vec::with_capacity(expr.else_if_blocks().len());
        for else_if_block in expr.else_if_blocks().iter() {
            let condition = transform_const_expr(else_if_block.condition(), scope, false);
            let body = transform_const_block(else_if_block.body(), scope, needs_return);
            match condition {
                Ok(condition) => match body {
                    Ok(body) => else_if_blocks.push(ConstElseIfBlock::new(condition, body)),
                    Err(err2) => errors.push(err2),
                },
                Err(err1) => {
                    errors.push(err1);
                    if let Err(err2) = body {
                        errors.push(err2);
                    }
                }
            }
        }
        let else_block = if let Some(else_block) = expr.else_block() {
            match transform_const_block(else_block.body(), scope, needs_return) {
                Ok(body) => Some(ConstElseBlock::new(body)),
                Err(err) => {
                    errors.push(err);
                    None
                }
            }
        } else {
            None
        };
        if needs_return && else_block.is_none() {
            errors.push(TypecheckError::MissingElseBlock { if_expr: expr })
        }
        if let Some (condition) = condition && let Some (body) = body { if errors . len () == 0 { let _value = ConstIfExpr :: new (condition , body , else_if_blocks , else_block) ; # [allow (unreachable_code)] Ok (_value) } else if errors . len () == 1 { Err (errors . into_iter () . next () . unwrap ()) } else { Err (TypecheckError :: List (errors)) } } else { if errors . len () == 0 { let _value = :: core :: panicking :: panic ("internal error: entered unreachable code") ; # [allow (unreachable_code)] Ok (_value) } else if errors . len () == 1 { Err (errors . into_iter () . next () . unwrap ()) } else { Err (TypecheckError :: List (errors)) } }
    }
    fn has_exhaustive_patterns(expr: &MatchExpr) -> bool {
        for branch in expr.branches().iter() {
            for pattern in branch.patterns().iter() {
                if let MatchPattern::Path(path) = pattern {
                    if (path.tail().len() == 0) && (path.head().as_ref() == "_") {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn transform_const_match_pattern<'a>(
        pattern: &'a MatchPattern,
        scope: &mut Scope,
    ) -> TypecheckResult<'a, ConstMatchPattern> {
        match pattern {
            MatchPattern::Literal(l) => Ok(ConstMatchPattern::Literal(*l)),
            MatchPattern::Path(p) => {
                if p.tail().len() == 0 {
                    if p.head().as_ref() != "_" {
                        scope.contains_const(p.head())?;
                    }
                    Ok(ConstMatchPattern::Ident(p.head().clone()))
                } else {
                    Err(TypecheckError::InvalidConstPattern { pattern })
                }
            }
        }
    }
    fn transform_const_match_expr<'a>(
        expr: &'a MatchExpr,
        scope: &mut Scope,
        needs_return: bool,
    ) -> TypecheckResult<'a, ConstMatchExpr> {
        let mut errors = Vec::new();
        let value = transform_const_expr(expr.value(), scope, false);
        let value = match value {
            Ok(value) => Some(value),
            Err(err) => {
                errors.push(err);
                None
            }
        };
        if !has_exhaustive_patterns(expr) {
            errors.push(TypecheckError::NonExhaustiveMatch { match_expr: expr });
        }
        let mut branches = Vec::with_capacity(expr.branches().len());
        for branch in expr.branches().iter() {
            let mut patterns = Vec::with_capacity(branch.patterns().len());
            for pattern in branch.patterns().iter() {
                match transform_const_match_pattern(pattern, scope) {
                    Ok(pattern) => patterns.push(pattern),
                    Err(err) => errors.push(err),
                }
            }
            match branch.body() {
                MatchBody::Expr(expr) => {
                    if !needs_return {
                        errors.push(TypecheckError::UnexpectedReturnValue { value: expr });
                    }
                    match transform_const_expr(expr, scope, false) {
                        Ok(expr) => branches
                            .push(ConstMatchBranch::new(patterns, ConstMatchBody::Expr(expr))),
                        Err(err) => errors.push(err),
                    }
                }
                MatchBody::Block(block) => {
                    match transform_const_block(block, scope, needs_return) {
                        Ok(block) => branches.push(ConstMatchBranch::new(
                            patterns,
                            ConstMatchBody::Block(block),
                        )),
                        Err(err) => errors.push(err),
                    }
                }
            }
        }
        if let Some(value) = value {
            if errors.len() == 0 {
                let _value = ConstMatchExpr::new(value, branches);
                #[allow(unreachable_code)]
                Ok(_value)
            } else if errors.len() == 1 {
                Err(errors.into_iter().next().unwrap())
            } else {
                Err(TypecheckError::List(errors))
            }
        } else {
            if errors.len() == 0 {
                let _value = ::core::panicking::panic("internal error: entered unreachable code");
                #[allow(unreachable_code)]
                Ok(_value)
            } else if errors.len() == 1 {
                Err(errors.into_iter().next().unwrap())
            } else {
                Err(TypecheckError::List(errors))
            }
        }
    }
    pub fn transform_const_expr<'a>(
        expr: &'a Expr,
        scope: &mut Scope,
        is_statement: bool,
    ) -> TypecheckResult<'a, ConstExpr> {
        match expr {
            Expr::Literal(l) => Ok(ConstExpr::Literal(l.clone())),
            Expr::Path(p) => {
                if p.tail().len() == 0 {
                    scope.contains_var(p.head())?;
                    Ok(ConstExpr::Ident(p.head().clone()))
                } else {
                    Err(TypecheckError::InvalidConstExpr { expr })
                }
            }
            Expr::Paren(expr) => transform_const_expr(expr.inner(), scope, is_statement),
            Expr::Call(expr) => {
                let mut errors = Vec::new();
                match scope.contains_func(expr.func()) {
                    Ok(arg_count) => {
                        if arg_count != expr.args().len() {
                            errors.push(TypecheckError::ArgumentCountMismatch {
                                call_expr: expr,
                                arg_count,
                            })
                        }
                    }
                    Err(err) => errors.push(err),
                }
                let mut args = Vec::with_capacity(expr.args().len());
                for arg in expr.args().iter() {
                    match transform_const_expr(arg, scope, false) {
                        Ok(arg) => args.push(arg),
                        Err(err) => errors.push(err),
                    }
                }
                if errors.len() == 0 {
                    let _value = ConstExpr::Call(ConstCallExpr::new(expr.func().clone(), args));
                    #[allow(unreachable_code)]
                    Ok(_value)
                } else if errors.len() == 1 {
                    Err(errors.into_iter().next().unwrap())
                } else {
                    Err(TypecheckError::List(errors))
                }
            }
            Expr::Construct(_) => Err(TypecheckError::InvalidConstExpr { expr }),
            Expr::If(expr) => Ok(ConstExpr::If(transform_const_if_expr(
                expr,
                scope,
                !is_statement,
            )?)),
            Expr::Match(expr) => Ok(ConstExpr::Match(transform_const_match_expr(
                expr,
                scope,
                !is_statement,
            )?)),
            Expr::Block(block) => Ok(ConstExpr::Block(Box::new(transform_const_block(
                block,
                scope,
                !is_statement,
            )?))),
            Expr::Index(_) => Err(TypecheckError::InvalidConstExpr { expr }),
            Expr::Pos(expr) => transform_const_expr(expr.inner(), scope, false),
            Expr::Neg(expr) => Ok(ConstExpr::Neg(ConstUnaryExpr::new(transform_const_expr(
                expr.inner(),
                scope,
                false,
            )?))),
            Expr::Not(expr) => Ok(ConstExpr::Not(ConstUnaryExpr::new(transform_const_expr(
                expr.inner(),
                scope,
                false,
            )?))),
            Expr::Cast(_) => Err(TypecheckError::InvalidConstExpr { expr }),
            Expr::Concat(_) => Err(TypecheckError::InvalidConstExpr { expr }),
            Expr::Lt(expr) => Ok(ConstExpr::Lt(transform_const_bin_expr(expr, scope)?)),
            Expr::Lte(expr) => Ok(ConstExpr::Lte(transform_const_bin_expr(expr, scope)?)),
            Expr::Gt(expr) => Ok(ConstExpr::Gt(transform_const_bin_expr(expr, scope)?)),
            Expr::Gte(expr) => Ok(ConstExpr::Gte(transform_const_bin_expr(expr, scope)?)),
            Expr::Slt(expr) => Err(TypecheckError::InvalidConstOp { op: *expr.op() }),
            Expr::Slte(expr) => Err(TypecheckError::InvalidConstOp { op: *expr.op() }),
            Expr::Sgt(expr) => Err(TypecheckError::InvalidConstOp { op: *expr.op() }),
            Expr::Sgte(expr) => Err(TypecheckError::InvalidConstOp { op: *expr.op() }),
            Expr::Eq(expr) => Ok(ConstExpr::Eq(transform_const_bin_expr(expr, scope)?)),
            Expr::Ne(expr) => Ok(ConstExpr::Ne(transform_const_bin_expr(expr, scope)?)),
            Expr::Add(expr) => Ok(ConstExpr::Add(transform_const_bin_expr(expr, scope)?)),
            Expr::Sub(expr) => Ok(ConstExpr::Sub(transform_const_bin_expr(expr, scope)?)),
            Expr::Mul(expr) => Ok(ConstExpr::Mul(transform_const_bin_expr(expr, scope)?)),
            Expr::Div(expr) => Ok(ConstExpr::Div(transform_const_bin_expr(expr, scope)?)),
            Expr::Rem(expr) => Ok(ConstExpr::Rem(transform_const_bin_expr(expr, scope)?)),
            Expr::And(expr) => Ok(ConstExpr::And(transform_const_bin_expr(expr, scope)?)),
            Expr::Xor(expr) => Ok(ConstExpr::Xor(transform_const_bin_expr(expr, scope)?)),
            Expr::Or(expr) => Ok(ConstExpr::Or(transform_const_bin_expr(expr, scope)?)),
            Expr::Shl(expr) => Ok(ConstExpr::Shl(transform_const_bin_expr(expr, scope)?)),
            Expr::Lsr(expr) => Ok(ConstExpr::Lsr(transform_const_bin_expr(expr, scope)?)),
            Expr::Asr(expr) => Ok(ConstExpr::Asr(transform_const_bin_expr(expr, scope)?)),
        }
    }
    fn transform_const_assignment<'a>(
        assign: &'a Assignment,
        scope: &mut Scope,
    ) -> TypecheckResult<'a, ConstAssignment> {
        if (assign.target().path().tail().len() == 0) && (assign.target().indexers().len() == 0) {
            let target = assign.target().path().head().clone();
            let err1 = match scope.contains_mut_var(&target) {
                Ok(_) => None,
                Err(err1) => Some(err1),
            };
            match transform_const_expr(assign.value(), scope, false) {
                Ok(value) => {
                    if let Some(err1) = err1 {
                        Err(err1)
                    } else {
                        Ok(ConstAssignment::new(target, assign.op().kind(), value))
                    }
                }
                Err(err2) => {
                    if let Some(err1) = err1 {
                        Err(TypecheckError::List([err1, err2].into()))
                    } else {
                        Err(err2)
                    }
                }
            }
        } else {
            Err(TypecheckError::InvalidConstAssignTarget {
                target: assign.target(),
            })
        }
    }
    fn transform_const_while_loop<'a>(
        while_loop: &'a WhileLoop,
        scope: &mut Scope,
    ) -> TypecheckResult<'a, ConstWhileLoop> {
        let condition = transform_const_expr(while_loop.condition(), scope, false);
        let body = transform_const_block(while_loop.body(), scope, false);
        match condition {
            Ok(condition) => match body {
                Ok(body) => Ok(ConstWhileLoop::new(condition, body)),
                Err(err2) => Err(err2),
            },
            Err(err1) => match body {
                Ok(_) => Err(err1),
                Err(err2) => Err(TypecheckError::List([err1, err2].into())),
            },
        }
    }
    fn transform_const_for_loop<'a>(
        for_loop: &'a ForLoop,
        scope: &mut Scope,
    ) -> TypecheckResult<'a, ConstForLoop> {
        let mut errors = Vec::new();
        let start = transform_const_expr(&for_loop.range().start, scope, false);
        let end = transform_const_expr(&for_loop.range().end, scope, false);
        let start = match start {
            Ok(start) => Some(start),
            Err(err) => {
                errors.push(err);
                None
            }
        };
        let end = match end {
            Ok(end) => Some(end),
            Err(err) => {
                errors.push(err);
                None
            }
        };
        let mut inner_scope = Scope::new(scope);
        inner_scope.add_var(for_loop.item_name());
        let body = transform_const_block(for_loop.body(), &mut inner_scope, false);
        let body = match body {
            Ok(body) => Some(body),
            Err(err) => {
                errors.push(err);
                None
            }
        };
        if errors.len() == 0 {
            let _value = ConstForLoop::new(
                for_loop.item_name().clone(),
                start.unwrap()..end.unwrap(),
                body.unwrap(),
            );
            #[allow(unreachable_code)]
            Ok(_value)
        } else if errors.len() == 1 {
            Err(errors.into_iter().next().unwrap())
        } else {
            Err(TypecheckError::List(errors))
        }
    }
    fn transform_const_statement<'a>(
        statement: &'a Statement,
        scope: &mut Scope,
    ) -> TypecheckResult<'a, ConstStatement> {
        match statement {
            Statement::Expr(expr) => Ok(ConstStatement::Expr(transform_const_expr(
                expr, scope, true,
            )?)),
            Statement::Declaration(decl) => {
                scope.add_var(decl.name());
                Ok(ConstStatement::Declaration(ConstDeclaration::new(
                    decl.name().clone(),
                    transform_const_expr(decl.value(), scope, false)?,
                )))
            }
            Statement::Assignment(assign) => Ok(ConstStatement::Assignment(
                transform_const_assignment(assign, scope)?,
            )),
            Statement::WhileLoop(while_loop) => Ok(ConstStatement::WhileLoop(
                transform_const_while_loop(while_loop, scope)?,
            )),
            Statement::ForLoop(for_loop) => Ok(ConstStatement::ForLoop(transform_const_for_loop(
                for_loop, scope,
            )?)),
        }
    }
    fn transform_const_block<'a>(
        block: &'a Block,
        parent_scope: &mut Scope,
        needs_return: bool,
    ) -> TypecheckResult<'a, ConstBlock> {
        let mut scope = Scope::new(parent_scope);
        let mut errors = Vec::new();
        let mut statements = Vec::with_capacity(block.statements().len());
        for statement in block.statements().iter() {
            match transform_const_statement(statement, &mut scope) {
                Ok(statement) => statements.push(statement),
                Err(err) => errors.push(err),
            }
        }
        let result = if let Some(result) = block.result() {
            if needs_return {
                match transform_const_expr(result, &mut scope, false) {
                    Ok(result) => Some(result),
                    Err(err) => {
                        errors.push(err);
                        None
                    }
                }
            } else {
                errors.push(TypecheckError::UnexpectedReturnValue { value: result });
                None
            }
        } else {
            if needs_return {
                errors.push(TypecheckError::MissingReturnValue { block });
            }
            None
        };
        if errors.len() == 0 {
            let _value = ConstBlock::new(statements, result);
            #[allow(unreachable_code)]
            Ok(_value)
        } else if errors.len() == 1 {
            Err(errors.into_iter().next().unwrap())
        } else {
            Err(TypecheckError::List(errors))
        }
    }
    pub fn transform_const_func<'a>(
        func: &'a Func,
        scope: &mut Scope,
    ) -> TypecheckResult<'a, ConstFunc> {
        let mut inner_scope = Scope::new(scope);
        for arg in func.args().iter() {
            inner_scope.add_var(arg);
        }
        let body = transform_const_block(func.body(), &mut inner_scope, true)?;
        Ok(ConstFunc::new(func.args().to_vec(), body))
    }
}
use const_eval::{eval, VarScope};
use langbox::*;
use lexer::*;
use std::collections::HashMap;
use std::rc::Rc;
use typecheck::*;
type SharedString = Rc<str>;
const TEST_FILE : & str = "fn clog2(bits) {\r\n    bits -= 1;\r\n    let log = 0;\r\n    while bits != 0 {\r\n        bits >>= 1;\r\n        log += 1;\r\n    }\r\n    log\r\n}\r\n\r\nconst TEST = clog2(32);\r\n\r\nstruct WritePort<ADDR_BITS, DATA_BITS> {\r\n    clk: bit,\r\n    en: bit,\r\n    addr: bits<ADDR_BITS>,\r\n    data: bits<DATA_BITS>,\r\n}\r\n\r\nstruct ReadPort<ADDR_BITS> {\r\n    clk: bit,\r\n    en: bit,\r\n    addr: bits<ADDR_BITS>,\r\n}\r\n\r\nmod Bram<ADDR_BITS, DATA_BITS>(\r\n    in sig wport: WritePort<ADDR_BITS, DATA_BITS>,\r\n    \r\n    in sig rport: ReadPort<ADDR_BITS>,\r\n    out reg rdata: bits<DATA_BITS>,\r\n) {\r\n    reg mem: [u<DATA_BITS>; 1 << ADDR_BITS];\r\n    \r\n    proc rising(wport.clk) {\r\n        if wport.en {\r\n            mem[wport.addr] = wport.data;\r\n        }\r\n    }\r\n    \r\n    proc rising(rport.clk) {\r\n        if rport.en {\r\n            rdata = mem[rport.addr];\r\n        }\r\n    }\r\n}\r\n\r\nenum AluOp: bits<4> {\r\n    Add,\r\n    AddC,\r\n    Sub,\r\n    SubB,\r\n    And,\r\n    Or,\r\n    Xor,\r\n    Shl,\r\n    Lsr,\r\n    Asr,\r\n    Mul,\r\n}\r\n\r\nstruct Flags {\r\n    c: bit,\r\n    z: bit,\r\n    s: bit,\r\n    o: bit,\r\n}\r\n\r\nmod Adder<BITS>(\r\n    in sig lhs: bits<BITS>,\r\n    in sig rhs: bits<BITS>,\r\n    out sig result: bits<BITS>,\r\n    \r\n    in sig c_in: bit,\r\n    out sig c_out: bit,\r\n) {\r\n    sig sum: bits<{BITS + 1}>;\r\n    \r\n    comb {\r\n        sum = (0 as bit)@lhs + (0 as bit)@rhs + (0 as bits<BITS>)@c_in;\r\n        result = sum[0..BITS];\r\n        c_out = sum[BITS];\r\n    }\r\n}\r\n\r\nmod Alu<BITS>(\r\n    in sig lhs: bits<BITS>,\r\n    in sig rhs: bits<BITS>,\r\n    out sig result: bits<BITS>,\r\n    \r\n    in sig flags_in: Flags,\r\n    out sig flags_out: Flags,\r\n    \r\n    in sig op: AluOp,\r\n) {\r\n    let adder: Adder<BITS>;\r\n    comb {\r\n        adder.lhs = lhs;\r\n        adder.rhs = if (op == AluOp::Sub) | (op == AluOp::SubB) {\r\n            !rhs\r\n        } else {\r\n            rhs\r\n        };\r\n        adder.c_in = match op {\r\n            AluOp::Add => 0 as bit,\r\n            AluOp::AddC => flags_in.c,\r\n            AluOp::Sub => 1 as bit,\r\n            AluOp::SubB => flags_in.c,\r\n            _ => 0 as bit,\r\n        };\r\n    }\r\n    \r\n    const SHIFT_AMT_BITS = clog2(BITS);\r\n    sig shift_amt: bits<SHIFT_AMT_BITS>;\r\n    \r\n    comb {\r\n        shift_amt = rhs[0..SHIFT_AMT_BITS];\r\n        \r\n        result = match op {\r\n            AluOp::Add => adder.result,\r\n            AluOp::AddC => adder.result,\r\n            AluOp::Sub => adder.result,\r\n            AluOp::SubB => adder.result,\r\n            AluOp::And => lhs & rhs,\r\n            AluOp::Or => lhs | rhs,\r\n            AluOp::Xor => lhs ^ rhs,\r\n            AluOp::Shl => lhs << shift_amt,\r\n            AluOp::Lsr => lhs >> shift_amt,\r\n            AluOp::Asr => lhs >>> shift_amt,\r\n            AluOp::Mul => lhs * rhs,\r\n        };\r\n    }\r\n    \r\n    sig lhs_s: bit;\r\n    sig rhs_s: bit;\r\n    \r\n    sig z: bit;\r\n    sig s: bit;\r\n    sig o: bit;\r\n    \r\n    comb {\r\n        lhs_s = lhs[BITS - 1];\r\n        rhs_s = rhs[BITS - 1];\r\n\r\n        z = result == 0 as bits<BITS>;\r\n        s = result[BITS - 1];\r\n        o = (lhs_s == rhs_s) & (lhs_s == s);\r\n\r\n        flags_out = match op {\r\n            AluOp::Add | AluOp::AddC | AluOp::Sub | AluOp::SubB => Flags {\r\n                c: adder.c_out,\r\n                z: z,\r\n                s: s,\r\n                o: o,\r\n            },\r\n            AluOp::And | AluOp::Or | AluOp::Xor | AluOp::Shl | AluOp::Lsr | AluOp::Asr => Flags {\r\n                c: flags_in.c,\r\n                z: z,\r\n                s: flags_in.s,\r\n                o: flags_in.o,\r\n            },\r\n            AluOp::Mul => Flags {\r\n                c: flags_in.c,\r\n                z: z,\r\n                s: s,\r\n                o: o,\r\n            },\r\n        };\r\n    }\r\n}\r\n" ;
#[doc(hidden)]
fn _set_style(
    stream: &mut termcolor::StandardStream,
    color: termcolor::Color,
    bold: bool,
    italic: bool,
) -> std::io::Result<()> {
    use termcolor::{ColorSpec, WriteColor};
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(color)).set_bold(bold).set_italic(italic);
    stream.set_color(&spec)
}
fn main() -> std::io::Result<()> {
    let mut stderr = termcolor::StandardStream::stderr(termcolor::ColorChoice::Auto);
    let mut file_server = FileServer::new();
    let file = file_server.register_file_memory("<test>", TEST_FILE);
    let lexer = QuartzLexer::new(file, &file_server);
    let tokens: Vec<_> = lexer
        .filter(|t| {
            if let QuartzToken::Comment(_) = &t.kind {
                false
            } else {
                true
            }
        })
        .collect();
    match parser::design().run(TokenStream::new(&tokens)) {
        ParseResult::Match { value: design, .. } => {
            let mut errors = Vec::new();
            if let Err(err) = check_for_duplicate_idents(design.iter()) {
                errors.push(err);
            }
            let mut empty_global_scope = Scope::empty();
            for item in design.iter() {
                match item {
                    ast::Item::Const(const_item) => empty_global_scope.add_const(const_item.name()),
                    ast::Item::Func(func_item) => {
                        empty_global_scope.add_func(func_item.name(), func_item.args().len())
                    }
                    _ => {}
                }
            }
            let mut consts = HashMap::new();
            let mut funcs = HashMap::new();
            for item in design.iter() {
                match item {
                    ast::Item::Const(const_item) => {
                        match transform_const_expr(
                            const_item.value(),
                            &mut empty_global_scope,
                            false,
                        ) {
                            Ok(expr) => {
                                if !consts.contains_key(const_item.name().as_ref())
                                    && !funcs.contains_key(const_item.name().as_ref())
                                {
                                    consts.insert(const_item.name().as_string(), expr);
                                }
                            }
                            Err(err) => errors.push(err),
                        }
                    }
                    ast::Item::Func(func_item) => {
                        match transform_const_func(func_item, &mut empty_global_scope) {
                            Ok(func) => {
                                if !consts.contains_key(func_item.name().as_ref())
                                    && !funcs.contains_key(func_item.name().as_ref())
                                {
                                    funcs.insert(func_item.name().as_string(), func);
                                }
                            }
                            Err(err) => errors.push(err),
                        }
                    }
                    _ => {}
                }
            }
            let mut const_values = HashMap::new();
            for (name, expr) in consts.iter() {
                const_values.insert(
                    SharedString::clone(name),
                    eval(expr, &mut VarScope::empty(), &consts, &funcs),
                );
            }
            for (name, value) in const_values.iter() {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["", " = ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&name),
                            ::core::fmt::ArgumentV1::new_display(&value),
                        ],
                    ));
                };
            }
        }
        ParseResult::NoMatch => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["input did not match"],
            &[],
        )),
        ParseResult::Err(err) => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["parse error: "],
            &[::core::fmt::ArgumentV1::new_debug(&err)],
        )),
    }
    Ok(())
}
