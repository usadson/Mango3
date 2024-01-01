// Copyright (C) 2023 - 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::{Catalog, WordTrait};
use mango3_syntax::{IndependentClause, Sentence, SentenceKind, Subject, VerbConstituent};

use crate::{
    Input,
    ParseResult,
    parse_noun_constituent,
};

pub fn parse_independent_clause(input: &mut Input<'_>, catalog: &Catalog) -> ParseResult<IndependentClause> {
    let mut clause = IndependentClause {
        subject: parse_subject(input, catalog)?,
        verb: VerbConstituent::default(),
    };

    while input.has_next() {
        let (text, text_range) = input.consume_until_space();

        if text.chars().next().as_ref().is_some_and(char::is_ascii_punctuation) {
            break;
        }

        let Some((word_id, word)) = catalog.find(text) else {
            break;
        };

        for word_trait in &word.traits {
            if let WordTrait::VerbConjugationIndicative { verb, kind, is_past } = word_trait {

            }
        }
    }

    Ok(clause)
}

pub fn parse_sentence(input: &mut Input<'_>, catalog: &Catalog) -> ParseResult<Sentence> {
    let noun_constituent = parse_noun_constituent(input, catalog)?;
    Ok(Sentence {
        kind: SentenceKind::NounConstituent(noun_constituent),
    })
}

pub fn parse_subject(input: &mut Input<'_>, catalog: &Catalog) -> ParseResult<Subject> {
    let noun_constituent = parse_noun_constituent(input, catalog)?;
    Ok(Subject::NounConstituent(noun_constituent))
}
