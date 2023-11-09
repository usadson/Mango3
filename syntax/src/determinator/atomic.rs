// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use super::Article;

#[derive(Debug, Clone, PartialEq)]
pub enum AtomicDeterminator {
    Article(Article),
}
