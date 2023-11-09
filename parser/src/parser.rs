// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::{Catalog, WordTrait};
use mango3_syntax::{
    Article,
    AtomicDeterminator,
    Determinator,
    Substantive, NounConstituent, NounConstituentCore,
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

pub fn parse_determinator(input: &mut Input<'_>) -> ParseResult<Determinator> {
    let article = parse_article(input)?;
    Ok(Determinator::Atomic(AtomicDeterminator::Article(article)))
}

pub fn parse_noun_constituent(input: &mut Input<'_>, catalog: &Catalog) -> ParseResult<NounConstituent> {
    let determinator = input.attempt(parse_determinator).ok();

    Ok(NounConstituent {
        determinator,
        core: NounConstituentCore::Substantive(
            parse_substantive(input, catalog)?
        )
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
