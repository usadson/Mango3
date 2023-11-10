// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::Catalog;
use mango3_syntax::{NounConstituent, Sentence, SentenceKind};

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

        if !determinator_genders.matches(core_genders) {
            _ = sink;
            log::warn!("Genders don't match: {noun:#?}");
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
