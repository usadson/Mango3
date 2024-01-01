// Copyright (C) 2023 - 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{VerbConstituent, Subject};

#[derive(Debug, Clone)]
pub struct IndependentClause {
    pub subject: Subject,
    pub verb: VerbConstituent,
}
