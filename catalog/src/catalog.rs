// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    collections::HashMap,
    sync::Arc,
};

use unicase::Ascii;

use crate::{Word, WordTrait, Gender, Multiplicity};

#[derive(Debug, Clone)]
pub struct Catalog {
    data: Arc<CatalogData>,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            data: Arc::new(CatalogData::new()),
        }
    }

    pub fn find<'a>(&'a self, word: &'a str) -> Option<(CatalogIndex, &'a Word)> {
        let idx = self.data.map.get(&Ascii::new(word))?;
        Some((idx.clone(), &self.data.definitions[idx.0]))
    }
}

#[derive(Debug)]
struct CatalogData {
    map: HashMap<Ascii<&'static str>, CatalogIndex>,
    definitions: Vec<Word>,
}

impl CatalogData {
    fn new() -> Self {
        let mut this = Self {
            map: HashMap::new(),
            definitions: Vec::new(),
        };

        this.fill_with_test_data();
        this
    }

    fn add(&mut self, word: &'static str) -> &mut Word {
        let idx = self.map.entry(Ascii::new(word))
            .or_insert_with(|| {
                self.definitions.push(Word {
                    traits: Default::default(),
                });
                CatalogIndex(self.definitions.len() - 1)
            });

        &mut self.definitions[idx.0]
    }

    fn fill_with_test_data(&mut self) {
        self.add("man").traits.push(
            WordTrait::Noun {
                gender: Gender::Masculine,
                multiplicity: Multiplicity::Singular,
            }
        );

        self.add("mannen").traits.push(
            WordTrait::Noun {
                gender: Gender::Masculine,
                multiplicity: Multiplicity::Plural,
            }
        );

        self.add("vrouw").traits.push(
            WordTrait::Noun {
                gender: Gender::Feminine,
                multiplicity: Multiplicity::Singular,
            }
        );

        self.add("vrouwen").traits.push(
            WordTrait::Noun {
                gender: Gender::Feminine,
                multiplicity: Multiplicity::Plural,
            }
        );

        self.add("kind").traits.push(
            WordTrait::Noun {
                gender: Gender::Neuter,
                multiplicity: Multiplicity::Singular,
            }
        );

        self.add("kinderen").traits.push(
            WordTrait::Noun {
                gender: Gender::Neuter,
                multiplicity: Multiplicity::Plural,
            }
        );
    }
}

#[derive(Debug, Clone)]
pub struct CatalogIndex(usize);
