// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod atomic;

pub use self::atomic::*;

#[derive(Debug)]
pub enum Determinator {
    Atomic(AtomicDeterminator),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Article {
    De,
    Het,
    Een,
}
