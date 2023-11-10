// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::Catalog;
use mango3_syntax::{NounConstituent, Sentence, SentenceKind, Range, NounConstituentCore};

use crate::{AnalysisSink, Analyzer, GenderAnalyzer};

pub struct DeterminatorValidator {}

impl DeterminatorValidator {
    fn analyze_sentence(
        &mut self,
        sentence: &Sentence,
        catalog: &Catalog,
        sink: &mut dyn AnalysisSink,
    ) {
        match &sentence.kind {
            SentenceKind::NounConstituent(noun) => {
                self.analyze_noun_constituent(noun, catalog, sink)
            }
        }
    }

    fn analyze_noun_constituent(
        &mut self,
        noun: &NounConstituent,
        catalog: &Catalog,
        sink: &mut dyn AnalysisSink,
    ) {
        let Some(determinator) = &noun.determinator else {
            return;
        };

        let determinator_genders = determinator.analyze_gender(catalog);
        let core_genders = noun.core.analyze_gender(catalog);

        let core = match &noun.core {
            NounConstituentCore::Substantive(substantive) => {
                catalog.get(&substantive.catalog_index).text.to_string()
            }
        };

        if !determinator_genders.matches(core_genders) {
            sink.report(
                Range {
                    start: 0,
                    end: 0,
                },
                "noun-constituent.determinator",
                vec![
                    ("core.genders", serde_json::to_string(&core_genders).unwrap()),
                    ("determinator.genders", serde_json::to_string(&determinator_genders).unwrap()),
                    ("core", core),
                ],
            );
        }
    }
}

impl Analyzer for DeterminatorValidator {
    fn analyze(
        &mut self,
        document: &mango3_syntax::Document,
        catalog: &mango3_catalog::Catalog,
        sink: &mut dyn crate::AnalysisSink,
    ) {
        for sentence in document.sentences() {
            self.analyze_sentence(sentence, catalog, sink);
        }
    }
}
