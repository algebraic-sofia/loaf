use core::fmt;
use std::rc::Rc;

use crate::types::Index;
use loaf_span::Span;

#[derive(Debug)]
pub struct Var {
    pub range: Span,
    pub index: Index,
}

#[derive(Debug)]
pub struct Pi {
    pub range: Span,
    pub binder: Option<String>,
    pub typ: Rc<Term>,
    pub body: Rc<Term>,
}

#[derive(Debug)]
pub struct Lambda {
    pub range: Span,
    pub binder: String,
    pub body: Rc<Term>,
}

#[derive(Debug)]
pub struct App {
    pub range: Span,
    pub head: Rc<Term>,
    pub spine: Vec<Rc<Term>>,
}

#[derive(Debug)]
pub struct Sigma {
    pub range: Span,
    pub binder: Option<String>,
    pub typ: Rc<Term>,
    pub body: Rc<Term>,
}

#[derive(Debug)]
pub struct Pair {
    pub range: Span,
    pub fst: Rc<Term>,
    pub snd: Rc<Term>,
}

#[derive(Debug)]
pub struct Universe {
    pub range: Span, // I'll add levels in the future :)
}

#[derive(Debug)]
pub struct Left {
    pub range: Span,
    pub term: Rc<Term>,
}

#[derive(Debug)]
pub struct Right {
    pub range: Span,
    pub term: Rc<Term>,
}


#[derive(Debug)]
pub struct Let {
    pub range: Span,
    pub binder: String,
    pub ty: Rc<Term>,
    pub val: Rc<Term>,
    pub then: Rc<Term>,
}

#[derive(Debug)]
pub enum Term {
    Var(Var),
    Universe(Universe),
    Let(Let),

    Pi(Pi),
    Lambda(Lambda),
    App(App),

    Pair(Pair),
    Sigma(Sigma),
    Left(Left),
    Right(Right),
}

type Env = im::Vector<Option<String>>;

pub fn add_to_env(env: &Env, name: Option<String>) -> Env {
    let mut env = env.clone();
    env.push_front(name);
    env
}

impl Universe {
    pub fn fmt_term(&self, _: &Env, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "★")
    }
}

impl Lambda {
    pub fn fmt_term(&self, ctx: &Env, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(λ {}. ", self.binder)?;
        self.body.fmt_term(&add_to_env(ctx, Some(self.binder.clone())), f)?;
        write!(f, ")")
    }
}

impl Pi {
    pub fn fmt_term(&self, ctx: &Env, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.binder {
            Some(binder) => {
                write!(f, "(Π {} : ", binder)?;
                self.typ.fmt_term(ctx, f)?;
                write!(f, " . ")?;
                self.body.fmt_term(&add_to_env(ctx, Some(binder.clone())), f)?;
                write!(f, ")")
            }
            None => {
                write!(f, "(")?;
                self.typ.fmt_term(ctx, f)?;
                write!(f, " -> ")?;
                self.body.fmt_term(&add_to_env(ctx, None), f)?;
                write!(f, ")")
            }
        }
    }
}

impl Sigma {
    pub fn fmt_term(&self, ctx: &Env, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.binder {
            Some(binder) => {
                write!(f, "(Σ {} : ", binder)?;
                self.typ.fmt_term(ctx, f)?;
                write!(f, " . ")?;
                self.body.fmt_term(&add_to_env(ctx, self.binder.clone()), f)?;
                write!(f, ")")
            }
            None => {
                write!(f, "Σ")?;
                self.typ.fmt_term(ctx, f)?;
                write!(f, " . ")?;
                self.body.fmt_term(&add_to_env(ctx, None), f)?;
                write!(f, ")")
            }
        }
    }
}

impl Pair {
    pub fn fmt_term(&self, ctx: &Env, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        self.fst.fmt_term(ctx, f)?;
        write!(f, ", ")?;
        self.snd.fmt_term(ctx, f)?;
        write!(f, ")")
    }
}

impl Left {
    pub fn fmt_term(&self, ctx: &Env, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(π1 ")?;
        self.term.fmt_term(ctx, f)?;
        write!(f, ")")
    }
}

impl Right {
    pub fn fmt_term(&self, ctx: &Env, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(π2 ")?;
        self.term.fmt_term(ctx, f)?;
        write!(f, ")")
    }
}

impl App {
    pub fn fmt_term(&self, ctx: &Env, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        self.head.fmt_term(ctx, f)?;
        for arg in &self.spine {
            arg.fmt_term(ctx, f)?;
        }
        write!(f, ")")
    }
}

impl Var {
    pub fn fmt_term(&self, ctx: &Env, f: &mut fmt::Formatter) -> fmt::Result {
        match ctx.get(self.index.0) {
            Some(Some(x)) => write!(f, "{}", x),
            Some(None) => write!(f, "_"),
            None => write!(f, "<{}>", self.index.0),
        }
    }
}

impl Let {
    pub fn fmt_term(&self, ctx: &Env, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(let {} : ", self.binder)?;
        self.ty.fmt_term(ctx, f)?;
        write!(f, " = ")?;
        self.val.fmt_term(ctx, f)?;
        write!(f, " in ")?;
        self.then.fmt_term(&add_to_env(ctx, None), f)?;
        write!(f, ")")
    }
}

impl Term {
    pub fn fmt_term(&self, ctx: &Env, f: &mut fmt::Formatter) -> fmt::Result {
        use Term::*;
        match self {
            Let(x) => x.fmt_term(ctx, f),
            Universe(x) => x.fmt_term(ctx, f),
            Lambda(x) => x.fmt_term(ctx, f),
            Pi(x) => x.fmt_term(ctx, f),
            Sigma(x) => x.fmt_term(ctx, f),
            Pair(x) => x.fmt_term(ctx, f),
            Left(x) => x.fmt_term(ctx, f),
            Right(x) => x.fmt_term(ctx, f),
            Var(x) => x.fmt_term(ctx, f),
            App(x) => x.fmt_term(ctx, f),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_term(&im::Vector::new(), f)
    }
}