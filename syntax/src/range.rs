// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone)]
pub struct Ranged<T: std::fmt::Debug + Clone> {
    range: Range,
    t: T,
}

impl<T: std::fmt::Debug + Clone> Ranged<T> {
    pub fn new(range: Range, t: T) -> Self {
        Self {
            range, t,
        }
    }

    pub fn range(&self) -> Range {
        self.range
    }
}

impl<T: std::fmt::Debug + Clone> AsRef<T> for Ranged<T> {
    fn as_ref(&self) -> &T {
        &self.t
    }
}

impl<T: std::fmt::Debug + Clone> Deref for Ranged<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}
