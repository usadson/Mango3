// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use log::info;
use mango3_catalog::Catalog;
use mango3_parser::{
    Input,
    parse_document,
};

fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("trace")
    ).init();

    let catalog = &Catalog::new();

    let input = " De vrouwen ";
    let input = &mut Input::new(input);
    let document = parse_document(input, catalog);

    info!("Result: {document:#?}");
}
