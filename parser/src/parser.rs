// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::{Catalog, WordTrait};
use mango3_syntax::{
    Article,
    AtomicDeterminator,
    Determinator,
    NounConstituent,
    NounConstituentCore,
    Substantive,
    Range,
    Ranged,
};

use crate::{
    Input,
    ParseErrorKind,
    ParseResult,
};

pub fn parse_article(input: &mut Input<'_>) -> ParseResult<Article> {
    let (word, range) = input.consume_until_space();

    if word.eq_ignore_ascii_case("het") {
        Ok(Article::Het)
    } else if word.eq_ignore_ascii_case("de") {
        Ok(Article::De)
    } else if word.eq_ignore_ascii_case("een") {
        Ok(Article::Een)
    } else {
        range.to_error(ParseErrorKind::ExpectedArticleDeterminator)
    }
}

pub fn parse_determinator(input: &mut Input<'_>) -> ParseResult<Ranged<Determinator>> {
    let start = input.offset();
    let article = parse_article(input)?;
    let range = Range::new(start, input.offset());
    Ok(Ranged::new(
        range,
        Determinator::Atomic(AtomicDeterminator::Article(article))
    ))
}

pub fn parse_noun_constituent(input: &mut Input<'_>, catalog: &Catalog) -> ParseResult<NounConstituent> {
    let start = input.offset();
    let determinator = input.attempt(parse_determinator).ok();
    input.trim_start();

    let core = parse_noun_constituent_core(input, catalog)?;

    let range = Range {
        start,
        end: input.offset()
    };

    Ok(NounConstituent {
        determinator,
        core,
        range,
    })
}

pub fn parse_substantive(input: &mut Input<'_>, catalog: &Catalog) -> ParseResult<Substantive> {
    let (text, range) = input.consume_until_space();

    let Some((catalog_index, word)) = catalog.find(text) else {
        return range.to_error(ParseErrorKind::ExpectedSubstantiveWasUnknownWord);
    };

    if !word.traits.iter().any(|t| matches!(t, WordTrait::Noun { .. })) {
        return range.to_error(ParseErrorKind::ExpectedSubstantiveWasNot);
    }

    Ok(Substantive {
        catalog_index,
    })
}

pub fn parse_noun_constituent_core(input: &mut Input<'_>, catalog: &Catalog) -> ParseResult<Ranged<NounConstituentCore>> {
    let start = input.offset();
    let core = NounConstituentCore::Substantive(
        parse_substantive(input, catalog)?
    );
    Ok(Ranged::new(
        Range::new(start, input.offset()),
        core,
    ))
}
