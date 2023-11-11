// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod determinator;
mod gender;
mod noun_constituent;

use mango3_catalog::Catalog;
use mango3_syntax::{Document, Range};
use rayon::prelude::*;

pub(crate) use self::{gender::GenderAnalyzer, noun_constituent::*};

pub trait AnalysisSink {
    fn report(&mut self, range: Range, secondary_ranges: Vec<Range>, kind: &'static str, values: Vec<(&'static str, String)>);
}

pub trait Analyzer {
    fn analyze(&mut self, document: &Document, catalog: &Catalog, sink: &mut dyn AnalysisSink);
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize)]
pub struct Report {
    range: Range,
    secondary_ranges: Vec<Range>,
    kind: &'static str,
    values: Vec<(&'static str, String)>,
}

#[derive(Debug, Clone, Default)]
struct SimpleSink {
    data: Vec<Report>,
}

impl AnalysisSink for SimpleSink {
    fn report(&mut self, range: Range, secondary_ranges: Vec<Range>, kind: &'static str, values: Vec<(&'static str, String)>) {
        self.data.push(Report {
            range,
            secondary_ranges,
            kind,
            values,
        });
    }
}

pub fn analyze(document: &Document, catalog: &Catalog) -> Vec<Report> {
    let analyzers = [Box::new(DeterminatorValidator {})];

    analyzers.into_par_iter().map(|mut analyzer| {
        let mut sink = SimpleSink::default();
        analyzer
            .as_mut()
            .analyze(document, catalog, &mut sink);
        sink.data
    })
    .reduce(|| Vec::new(), |mut accumulator, data| {
        accumulator.extend(data);
        accumulator
    })
}
