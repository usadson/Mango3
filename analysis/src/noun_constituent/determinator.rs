// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::{Catalog, Multiplicity, WordTrait};
use mango3_syntax::{NounConstituent, Sentence, SentenceKind, NounConstituentCore};

use crate::{AnalysisSink, Analyzer, GenderAnalyzer, gender::GenderAnalysis, determinator::DeterminatorExtensions};

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

        if let Some(Multiplicity::Plural) = multiplicity_for_noun_core(&noun.core, catalog) {
            if !determinator.is_valid_for_plural() {
                let secondary_ranges = vec![noun.core.range()];
                let core = match noun.core.as_ref() {
                    NounConstituentCore::Substantive(substantive) => {
                        catalog.get(&substantive.catalog_index).text.to_string()
                    }
                };

                sink.report(
                    determinator.range(),
                    secondary_ranges,
                    "noun-constituent.determinator.plurality",
                    vec![("core", core)],
                    vec![
                        (determinator.range(), determinator.to_plural_form()),
                    ],
                );
            }

            return;
        }

        let determinator_genders = match multiplicity_for_noun_core(&noun.core, catalog) {
            Some(Multiplicity::Singular) | None => determinator.analyze_gender(catalog),
            Some(Multiplicity::Plural) => GenderAnalysis::ALL,
        };
        let core_genders = noun.core.analyze_gender(catalog);
        dbg!(determinator_genders, core_genders);

        let secondary_ranges = vec![noun.core.range()];
        let core = match noun.core.as_ref() {
            NounConstituentCore::Substantive(substantive) => {
                catalog.get(&substantive.catalog_index).text.to_string()
            }
        };

        if !determinator_genders.matches(core_genders) {
            sink.report(
                determinator.range(),
                secondary_ranges,
                "noun-constituent.determinator",
                vec![
                    ("core.genders", serde_json::to_string(&core_genders).unwrap()),
                    ("determinator.genders", serde_json::to_string(&determinator_genders).unwrap()),
                    ("core", core),
                ],
                vec![],
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

fn multiplicity_for_noun_core(core: &NounConstituentCore, catalog: &Catalog) -> Option<Multiplicity> {
    match core {
        NounConstituentCore::Substantive(substantive) => {
            for trait_ in &catalog.get(&substantive.catalog_index).traits {
                match trait_ {
                    WordTrait::Noun { multiplicity , .. } => return Some(*multiplicity),
                }
            }

            None
        }
    }
}
