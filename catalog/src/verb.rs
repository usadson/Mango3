// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct VerbId(pub(crate) usize);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ConjugationKind {
    FirstPerson,
    SecondPersonJe,
    SecondPersonU,
    SecondPersonGe,
    ThirdPerson,
    Plural,
}

impl From<u8> for ConjugationKind {
    fn from(value: u8) -> Self {
        match value {
            0 => ConjugationKind::FirstPerson,
            1 => ConjugationKind::SecondPersonJe,
            2 => ConjugationKind::SecondPersonU,
            3 => ConjugationKind::SecondPersonGe,
            4 => ConjugationKind::ThirdPerson,
            5 => ConjugationKind::Plural,
            _ => unreachable!(),
        }
    }
}
