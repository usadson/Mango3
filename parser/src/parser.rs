// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_syntax::{
    Article,
    AtomicDeterminator,
    Determinator,
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
