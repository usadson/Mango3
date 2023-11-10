// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod gender;
mod noun_constituent;

use mango3_catalog::Catalog;
use mango3_syntax::Document;
use rayon::prelude::*;

pub(crate) use self::{gender::GenderAnalyzer, noun_constituent::*};

pub trait AnalysisSink {}

pub trait Analyzer {
    fn analyze(&mut self, document: &Document, catalog: &Catalog, sink: &mut dyn AnalysisSink);
}

struct SimpleSink {}
impl AnalysisSink for SimpleSink {}

pub fn analyze(document: &Document, catalog: &Catalog) {
    let analyzers = [Box::new(DeterminatorValidator {})];

    let _: usize = analyzers.into_par_iter().map(|mut analyzer| {
        analyzer
            .as_mut()
            .analyze(document, catalog, &mut SimpleSink {});
        1
    }).sum();
}
