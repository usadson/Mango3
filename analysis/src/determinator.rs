// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_syntax::{Article, AtomicDeterminator, Determinator};

pub trait DeterminatorExtensions {
    fn is_valid_for_plural(&self) -> bool;
}

impl DeterminatorExtensions for Article {
    fn is_valid_for_plural(&self) -> bool {
        *self == Self::De
    }
}

impl DeterminatorExtensions for AtomicDeterminator {
    fn is_valid_for_plural(&self) -> bool {
        match self {
            Self::Article(art) => art.is_valid_for_plural(),
        }
    }
}

impl DeterminatorExtensions for Determinator {
    fn is_valid_for_plural(&self) -> bool {
        match self {
            Self::Atomic(atomic) => atomic.is_valid_for_plural(),
        }
    }
}
