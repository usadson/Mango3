// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{Substantive, Determinator, Range};

/// The noun constituent is a noun with optional helper words surrounding it.
///
/// ## References
/// * [ANS - Naamwoordelijke Constituent](https://e-ans.ivdnt.org/topics/pid/ans1401lingtopic)
#[derive(Debug, Clone)]
pub struct NounConstituent {
    pub range: Range,
    pub determinator: Option<Determinator>,
    pub core: NounConstituentCore,
}

#[derive(Debug, Clone)]
pub enum NounConstituentCore {
    Substantive(Substantive),
}
