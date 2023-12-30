// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Mutex;
use std::sync::Arc;
use std::error::Error;

use crate::{Gender, Multiplicity, Word, WordTrait};

#[derive(Debug, Clone)]
pub struct Catalog {
    data: Arc<Mutex<CatalogData>>,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            data: Arc::new(CatalogData::new().into()),
        }
    }

    pub fn find<'a>(&'a self, word: &'a str) -> Option<(CatalogIndex, Word)> {
        self.data.lock().unwrap().find_by_string(word)
    }

    pub fn get(&self, index: &CatalogIndex) -> Word {
        self.data.lock().unwrap().get(index)
    }
}

#[derive(Debug)]
struct CatalogData {
    cache: Vec<Option<Word>>,
    connection: rusqlite::Connection,
}

impl CatalogData {
    fn new() -> Self {
        let connection = rusqlite::Connection::open("Catalog.sqlite3")
            .expect("failed to open catalog!");
        Self {
            cache: Vec::new().into(),
            connection,
        }
    }

    fn find_by_string(&mut self, word: &str) -> Option<(CatalogIndex, Word)> {
        let id = self.connection.query_row(
            "SELECT id FROM words WHERE word = (?1)",
            [word],
            |row| row.get(0)
        ).ok()?;

        if self.cache.len() < id {
            self.cache.resize_with(id, || None);
        }

        self.cache.insert(id, Some(Word{
            text: Arc::from(word),
            traits: self.find_traits(id).unwrap().into(),
        }));

        self.cache.get(id)?
            .clone()
            .map(|word| (CatalogIndex(id), word))
    }

    fn find_traits(&self, id: usize) -> Result<Vec<WordTrait>, Box<dyn Error>> {
        let mut stmt = self.connection.prepare("SELECT gender, multiplicity FROM trait_noun WHERE word_id = (?1)")?;

        let iter = stmt.query_map(
            [id],
            |row| Ok(WordTrait::Noun {
                gender: Gender::from(row.get::<usize, u8>(0)?),
                multiplicity: Multiplicity::from(row.get::<usize, u8>(1)?)
            })
        )?;

        let values = iter
            .map(|x| x.unwrap())
            .collect::<Vec<WordTrait>>();

        Ok(values)
    }

    fn get(&self, catalog_index: &CatalogIndex) -> Word {
        self.cache[catalog_index.0].clone().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct CatalogIndex(usize);
