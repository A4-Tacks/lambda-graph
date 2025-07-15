use core::fmt;
use std::{collections::BTreeMap, rc::Rc};

use char_classes::any;
use crate::Term;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Atom {
    Term(Term),
    Call(Rc<Atom>, Rc<Atom>),
    Func(Term, Rc<Atom>),
}

impl Atom {
    /// Returns `true` if the atom is [`Call`].
    ///
    /// [`Call`]: Atom::Call
    #[must_use]
    pub fn is_call(&self) -> bool {
        matches!(self, Self::Call(..))
    }

    /// Returns `true` if the atom is [`Func`].
    ///
    /// [`Func`]: Atom::Func
    #[must_use]
    pub fn is_func(&self) -> bool {
        matches!(self, Self::Func(..))
    }

    /// Returns `true` if the atom is [`Term`].
    ///
    /// [`Term`]: Atom::Term
    #[must_use]
    pub fn is_term(&self) -> bool {
        matches!(self, Self::Term(..))
    }

    fn simple(&self, level: usize) -> bool {
        if level == 0 {
            return false;
        }
        match self {
            Atom::Term(_) => true,
            Atom::Func(_, atom) => atom.simple(level-1),
            Atom::Call(a, b) => {
                let next = level.saturating_sub(2);
                a.simple(next) && b.simple(next)
            },
        }
    }

    fn indented_fmt(
        &self,
        ind: &mut String,
        level: usize,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if self.simple(level) {
            return write!(f, "{self:o}");
        }
        match self {
            Atom::Term(ch) => write!(f, "{ch}"),
            Atom::Call(fun, a) => {
                ind.push_str("  ");
                write!(f, "(\n{ind}")?;
                fun.indented_fmt(ind, level, f)?;
                write!(f, "\n{ind}")?;
                a.indented_fmt(ind, level, f)?;

                ind.pop(); ind.pop();
                write!(f, "\n{ind})")
            },
            Atom::Func(p, e) => {
                ind.push_str("  ");
                write!(f, "(λ{p}.\n{ind}")?;

                e.indented_fmt(ind, level, f)?;

                ind.pop(); ind.pop();
                write!(f, "\n{ind})")
            },
        }
    }
}

impl From<Term> for Atom {
    fn from(v: Term) -> Self {
        Self::Term(v)
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Term(ch) => write!(f, "{ch}"),
            Atom::Call(fun, a) => write!(f, "({fun}{a})"),
            Atom::Func(p, e) => write!(f, "(λ{p}.{e})"),
        }
    }
}
impl fmt::Octal for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let level = f.precision().unwrap_or(8);
            return self.indented_fmt(&mut String::new(), level, f);
        }
        match self {
            Atom::Term(ch) => write!(f, "{ch}"),
            Atom::Call(fun, a) => {
                if f.precision().is_some_and(|p| p<=2) {
                    write!(f, "{:.2o}{:.3o}", **fun, **a)
                } else {
                    write!(f, "({:.2o}{:.3o})", **fun, **a)
                }
            },
            Atom::Func(p, e) => {
                if f.precision() == Some(1) {
                    write!(f, "λ{p}.{:.1o}", **e)
                } else {
                    write!(f, "(λ{p}.{:.1o})", **e)
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Ctx {
    defines: BTreeMap<Term, Vec<Atom>>,
}

peg::parser!(grammar parser(ctx: &mut Ctx) for str {
    rule _() = quiet!{[ch if any!(" \t\r\n", ch)]*} comment()?
    rule comment() = quiet!{"#" [^'\r' | '\n']* _}
    rule term() -> Term
        = p:position!()
          t:quiet!{
            ch:[ch if any!("a-zA-Z$_Σ-κμ-ϧ+*/%!-", ch)]
            { Term(ch.encode_utf8(&mut [0, 0]).into(), p) }
            / "`" s:$(([^ch if any!(" \t\r\n`", ch)])+) "`"
            { Term(s.into(), p) }
          }
        {t}
        / expected!("term")
    rule lambda() = "λ" / "^"

    rule expr_definite() -> Atom
        = name:term() _ ":=" _ e:expr_lambda() _ ";" _
          ({ ctx.defines.entry(name.clone()).or_default().push(e); })
          e:expr_definite()
        {
            ctx.defines.get_mut(&name).unwrap().pop().unwrap();
            e
        }
        / expr_lambda()
    rule expr_lambda() -> Atom
        = (lambda() _)? names:term()++_ _ "." _ sub:expr_lambda()
        { names.into_iter().rfold(sub, |sub, name| {
            Atom::Func(name, sub.into())
        }) }
        / expr_call()
    rule expr_call() -> Atom
        = atoms:expr_atom()++_
        { atoms.into_iter().reduce(|a, b| {
            Atom::Call(a.into(), b.into())
        }).unwrap() }
    rule expr_atom() -> Atom
        = t:term()
        { ctx.defines.get(&t)
            .and_then(|vec| vec.last().cloned())
            .unwrap_or(t.into()) }
        / "(" _ e:expr_definite() _  ")" { e }
    pub rule expr() -> Atom
        = _ e:expr_definite() _ { e }
});

pub type PegResult<T> = Result<T, peg::error::ParseError<peg::str::LineCol>>;

pub fn expr(s: &str) -> PegResult<Atom> {
    parser::expr(s, &mut Default::default())
}
