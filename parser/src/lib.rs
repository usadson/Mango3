// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

extern crate mango3_syntax;

mod error;
mod input;
mod parser;

pub use self::{
    error::*,
    input::*,
    parser::*,
};
