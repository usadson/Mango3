// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::Range;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub range: Range,
}

#[derive(Debug, Clone)]
#[derive(thiserror::Error)]
pub enum ParseErrorKind {
    #[error("expected article determinator")]
    ExpectedArticleDeterminator,
}
