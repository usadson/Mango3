// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

impl From<u8> for Gender {
    fn from(value: u8) -> Self {
        match value {
            0 => Gender::Masculine,
            1 => Gender::Feminine,
            2 => Gender::Neuter,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Multiplicity {
    Singular,
    Plural,
}

impl From<u8> for Multiplicity {
    fn from(value: u8) -> Self {
        match value {
            0 => Multiplicity::Singular,
            1 => Multiplicity::Plural,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Word {
    pub text: Arc<str>,
    pub traits: SharedVec<WordTrait>,
}

#[derive(Debug)]
pub enum WordTrait {
    Noun {
        gender: Gender,
        multiplicity: Multiplicity,
    },
}

#[derive(Debug)]
pub struct SharedVec<T> {
    inner: Arc<[T]>,
}

impl<T> SharedVec<T> {
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.inner.iter()
    }
}

impl<'a, T> IntoIterator for &'a SharedVec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<T> Clone for SharedVec<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

impl<T> From<Vec<T>> for SharedVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            inner: Arc::from(value)
        }
    }
}
