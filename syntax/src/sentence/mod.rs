// Copyright (C) 2023 - 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod actor;
mod independent_clause;

use crate::NounConstituent;

pub use self::{
    actor::Subject,
    independent_clause::IndependentClause,
};

#[derive(Debug, Clone)]
pub struct Sentence {
    pub kind: SentenceKind,
}

#[derive(Debug, Clone)]
pub enum SentenceKind {
    IndependentClause(IndependentClause),
    NounConstituent(NounConstituent),
}
