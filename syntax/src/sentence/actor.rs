// Copyright (C) 2023 - 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::ConjugationKind;

use crate::NounConstituent;

#[derive(Debug, Clone)]
pub enum Subject {
    NounConstituent(NounConstituent),
}

impl Subject {
    pub fn allows_conjugation(&self, conjugation: ConjugationKind) -> bool {
        match self {
            Self::NounConstituent(constituent) =>
                noun_constituent_allows_conjugation(constituent, conjugation),
        }
    }
}

fn noun_constituent_allows_conjugation(
    constituent: &NounConstituent,
    conjugation: ConjugationKind
) -> bool {
    // TODO
    conjugation == ConjugationKind::ThirdPerson
}
