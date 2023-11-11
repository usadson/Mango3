// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::{Catalog, Gender, WordTrait};
use mango3_syntax::{Article, AtomicDeterminator, Determinator, NounConstituentCore, Substantive};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize)]
pub struct GenderAnalysis {
    pub feminine: bool,
    pub neuter: bool,
    pub masculine: bool,
}

impl GenderAnalysis {
    pub const FEMININE: Self = Self {
        feminine: true,
        neuter: false,
        masculine: false,
    };

    pub const FEMININE_AND_MASCULINE: Self = Self {
        feminine: true,
        neuter: false,
        masculine: true,
    };

    pub const NEUTER: Self = Self {
        feminine: false,
        neuter: true,
        masculine: false,
    };

    pub const MASCULINE: Self = Self {
        feminine: false,
        neuter: false,
        masculine: true,
    };

    pub const ALL: Self = Self {
        feminine: true,
        neuter: true,
        masculine: true,
    };

    pub const fn matches(&self, other: Self) -> bool {
        if self.feminine && other.feminine {
            return true;
        }

        if self.neuter && other.neuter {
            return true;
        }

        if self.masculine && other.masculine {
            return true;
        }

        false
    }
}

pub(crate) trait GenderAnalyzer {
    /// Find the possible genders that this element can be.
    fn analyze_gender(&self, catalog: &Catalog) -> GenderAnalysis;
}

impl GenderAnalyzer for Article {
    fn analyze_gender(&self, _: &Catalog) -> GenderAnalysis {
        match self {
            Self::De => GenderAnalysis::FEMININE_AND_MASCULINE,
            Self::Een => GenderAnalysis::ALL,
            Self::Het => GenderAnalysis::NEUTER,
        }
    }
}

impl GenderAnalyzer for AtomicDeterminator {
    fn analyze_gender(&self, catalog: &Catalog) -> GenderAnalysis {
        match self {
            Self::Article(article) => article.analyze_gender(catalog),
        }
    }
}

impl GenderAnalyzer for Determinator {
    fn analyze_gender(&self, catalog: &Catalog) -> GenderAnalysis {
        match self {
            Self::Atomic(atomic) => atomic.analyze_gender(catalog),
        }
    }
}

impl GenderAnalyzer for Gender {
    fn analyze_gender(&self, _: &Catalog) -> GenderAnalysis {
        match self {
            Self::Feminine => GenderAnalysis::FEMININE,
            Self::Neuter => GenderAnalysis::NEUTER,
            Self::Masculine => GenderAnalysis::MASCULINE,
        }
    }
}

impl GenderAnalyzer for NounConstituentCore {
    fn analyze_gender(&self, catalog: &Catalog) -> GenderAnalysis {
        match self {
            Self::Substantive(substantive) => substantive.analyze_gender(catalog),
        }
    }
}

impl GenderAnalyzer for Substantive {
    fn analyze_gender(&self, catalog: &Catalog) -> GenderAnalysis {
        let word = catalog.get(&self.catalog_index);
        for trait_ in &word.traits {
            match trait_ {
                WordTrait::Noun { gender, .. } => return gender.analyze_gender(catalog),
            }
        }

        GenderAnalysis::ALL
    }
}
