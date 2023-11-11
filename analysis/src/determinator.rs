// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_syntax::{Article, AtomicDeterminator, Determinator};

pub trait DeterminatorExtensions {
    fn is_valid_for_plural(&self) -> bool;

    /// Get the replacement [`String`] to convert this determinator to a
    /// plural form.
    fn to_plural_form(&self) -> String;
}

impl DeterminatorExtensions for Article {
    fn is_valid_for_plural(&self) -> bool {
        *self == Self::De
    }

    fn to_plural_form(&self) -> String {
        "de".into()
    }
}

impl DeterminatorExtensions for AtomicDeterminator {
    fn is_valid_for_plural(&self) -> bool {
        match self {
            Self::Article(art) => art.is_valid_for_plural(),
        }
    }

    fn to_plural_form(&self) -> String {
        match self {
            Self::Article(art) => art.to_plural_form(),
        }
    }
}

impl DeterminatorExtensions for Determinator {
    fn is_valid_for_plural(&self) -> bool {
        match self {
            Self::Atomic(atomic) => atomic.is_valid_for_plural(),
        }
    }

    fn to_plural_form(&self) -> String {
        match self {
            Self::Atomic(atomic) => atomic.to_plural_form(),
        }
    }
}
