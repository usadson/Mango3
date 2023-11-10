// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::NounConstituent;

#[derive(Debug, Clone)]
pub struct Sentence {
    pub kind: SentenceKind,
}

#[derive(Debug, Clone)]
pub enum SentenceKind {
    NounConstituent(NounConstituent),
}
