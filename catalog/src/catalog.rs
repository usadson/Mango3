// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Mutex;
use std::sync::Arc;
use std::error::Error;

use crate::{ConjugationKind, Gender, Multiplicity, VerbId, Word, WordTrait};

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

    pub fn find_indicative_conjugations(&self, id: VerbId) -> Vec<Word> {
        self.data.lock().unwrap().find_indicative_conjugations(id)
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

    fn find_by_id(&mut self, id: CatalogIndex) -> Option<Word> {
        let id = id.0;

        if let Some(Some(word)) = self.cache.get(id) {
            return Some(word.clone());
        }

        let text: String = self.connection.query_row(
            "SELECT word FROM words WHERE id = (?1)",
            [id],
            |row| row.get(0),
        ).ok()?;

        if self.cache.len() < id + 1 {
            self.cache.resize_with(id * 2, || None);
        }

        let word = Word{
            text: Arc::from(text),
            traits: self.find_traits(id).unwrap().into(),
        };

        self.cache[id] = Some(word.clone());
        Some(word)
    }

    fn find_by_string(&mut self, word: &str) -> Option<(CatalogIndex, Word)> {
        let id = self.connection.query_row(
            "SELECT id FROM words WHERE word = (?1)",
            [word],
            |row| row.get(0)
        ).ok()?;

        let id = CatalogIndex(id);
        Some((id, self.find_by_id(id)?))
    }

    fn find_indicative_conjugations(&mut self, id: VerbId) -> Vec<Word> {
        let ids = self.connection.prepare("SELECT DISTINCT word_id FROM trait_verb_conjugation_indicative WHERE verb_id = (?1)")
            .unwrap()
            .query_map(
                [id.0],
                |row| row.get::<usize, usize>(0)
            )
            .unwrap()
            .map(|x| x.unwrap())
            .collect::<Vec<usize>>();

        let mut words = Vec::new();

        for id in ids {
            words.push(self.find_by_id(CatalogIndex(id)).unwrap());
        }

        words
    }

    fn find_traits(&self, id: usize) -> Result<Vec<WordTrait>, Box<dyn Error>> {
        let mut traits = self.find_noun_traits(id)?;
        traits.extend(self.find_verb_traits(id)?);
        traits.extend(self.find_verb_conjugation_traits(id)?);
        Ok(traits)
    }

    fn find_noun_traits(&self, id: usize) -> Result<Vec<WordTrait>, Box<dyn Error>> {
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

    fn find_verb_traits(&self, id: usize) -> Result<Vec<WordTrait>, Box<dyn Error>> {
        let mut stmt = self.connection.prepare("SELECT id FROM trait_verb WHERE word_id = (?1)")?;

        let iter = stmt.query_map(
            [id],
            |row| Ok(WordTrait::Verb {
                verb: VerbId(row.get(0)?),
            })
        )?;

        let values = iter
            .map(|x| x.unwrap())
            .collect::<Vec<WordTrait>>();

        Ok(values)
    }

    fn find_verb_conjugation_traits(&self, id: usize) -> Result<Vec<WordTrait>, Box<dyn Error>> {
        let mut stmt = self.connection.prepare("SELECT verb_id, conjugation_kind, is_past FROM trait_verb_conjugation_indicative WHERE word_id = (?1)")?;

        let iter = stmt.query_map(
            [id],
            |row| Ok(WordTrait::VerbConjugationIndicative {
                verb: VerbId(row.get(0)?),
                kind: ConjugationKind::from(row.get::<usize, u8>(1)?),
                is_past: row.get::<usize, bool>(2)?,
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

#[derive(Debug, Copy, Clone)]
pub struct CatalogIndex(usize);
