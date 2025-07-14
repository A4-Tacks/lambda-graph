use core::fmt;
use std::{borrow::Borrow, cmp, hash, rc::Rc};

#[derive(Clone, Eq)]
pub struct Term(pub Rc<str>, pub usize);

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.len() == 1 {
            write!(f, "{:?}@{}", self.0.chars().next().unwrap(), self.1)
        } else {
            write!(f, "{:?}@{}", self.0, self.1)
        }
    }
}
impl PartialEq<Term> for str {
    fn eq(&self, other: &Term) -> bool {
        *self == **other
    }
}
impl PartialEq<str> for Term {
    fn eq(&self, other: &str) -> bool {
        **self == *other
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Borrow<str> for Term {
    fn borrow(&self) -> &str {
        self
    }
}

impl hash::Hash for Term {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (**self).cmp(other)
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::ops::Deref for Term {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
