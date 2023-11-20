// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{collections::HashMap, sync::Arc};

use unicase::Ascii;

use crate::{Gender, Multiplicity, Word, WordTrait};

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

    pub fn get(&self, index: &CatalogIndex) -> &Word {
        &self.data.definitions[index.0]
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
                    text: word,
                    traits: Default::default(),
                });
                CatalogIndex(self.definitions.len() - 1)
            });

        &mut self.definitions[idx.0]
    }

    fn fill_with_test_data(&mut self) {
        self.add_noun("man", "mannen", Gender::Masculine);
        self.add_noun("vrouw", "vrouwen", Gender::Feminine);
        self.add_noun("kind", "kinderen", Gender::Neuter);
        self.add_noun("meisje", "meisjes", Gender::Neuter);
        self.add_noun("meid", "meiden", Gender::Feminine);
        self.add_noun("jongen", "jongens", Gender::Masculine);
        self.add_noun("water", "wateren", Gender::Neuter);
    }

    fn add_noun(
        &mut self,
        singular: &'static str,
        plural: &'static str,
        gender: Gender,
    ) {

        self.add(singular).traits.push(
            WordTrait::Noun {
                gender,
                multiplicity: Multiplicity::Singular,
            }
        );

        self.add(plural).traits.push(
            WordTrait::Noun {
                gender,
                multiplicity: Multiplicity::Plural,
            }
        );
    }
}

#[derive(Debug, Clone)]
pub struct CatalogIndex(usize);
