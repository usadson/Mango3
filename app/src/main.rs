// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use log::info;
use mango3_parser::{Input, parse_determinator};

fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("trace")
    ).init();

    let input = " De wijze man ";

    let mut input = Input::new(input);

    info!("Determinator: {:#?}", parse_determinator(&mut input));
}
