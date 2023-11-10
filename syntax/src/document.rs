// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::Sentence;

#[derive(Debug, Clone)]
pub struct Document {
    pub(crate) sentences: Vec<Sentence>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            sentences: Vec::new(),
        }
    }

    pub fn sentences(&self) -> &[Sentence] {
        &self.sentences
    }

    pub fn sentences_mut(&mut self) -> &mut Vec<Sentence> {
        &mut self.sentences
    }
}
