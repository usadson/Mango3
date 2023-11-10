// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::Catalog;
use mango3_syntax::{
    Sentence,
    SentenceKind,
};

use crate::{
    Input,
    ParseResult,
    parse_noun_constituent,
};

pub fn parse_sentence(input: &mut Input<'_>, catalog: &Catalog) -> ParseResult<Sentence> {
    let noun_constituent = parse_noun_constituent(input, catalog)?;
    Ok(Sentence {
        kind: SentenceKind::NounConstituent(noun_constituent),
    })
}
