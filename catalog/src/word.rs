// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Multiplicity {
    Singular,
    Plural,
}

#[derive(Debug)]
pub struct Word {
    pub text: &'static str,
    pub traits: Vec<WordTrait>,
}

#[derive(Debug)]
pub enum WordTrait {
    Noun {
        gender: Gender,
        multiplicity: Multiplicity,
    },
}
