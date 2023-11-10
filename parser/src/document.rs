// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::Catalog;
use mango3_syntax::Document;

use crate::{
    Input,
    parse_sentence,
};

pub fn parse_document(input: &mut Input<'_>, catalog: &Catalog) -> Document {
    let mut doc = Document::new();

    while input.has_next() {
        let starting_offset = input.offset();

        match parse_sentence(input, catalog) {
            Ok(sentence) => doc.sentences_mut().push(sentence),
            Err(e) => {
                log::warn!("Failed to parse sentence: {}: {e:#?}", e.kind);

                if input.offset() == starting_offset {
                    log::error!("Failed to parse sentence, cannot advance.");
                    break;
                }
            }
        }
    }

    doc
}
