// Copyright (C) 2023 - 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::{ConjugationKind, VerbId};
use crate::Range;

#[derive(Debug, Clone)]
pub struct Verb {
    pub range: Range,
    pub id: VerbId,
    pub conjugation: VerbConjugation,
}

#[derive(Debug, Clone)]
pub struct VerbConjugation {
    pub kind: ConjugationKind,
    pub is_past: bool,
}

#[derive(Debug, Default, Clone)]
pub struct VerbConstituent {
    pub verbs: Vec<Verb>,
}
