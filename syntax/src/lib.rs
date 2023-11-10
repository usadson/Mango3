// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod constituent;
mod document;
mod determinator;
mod range;
mod sentence;
mod substantive;

use std::rc::Rc;

pub type Shared<T> = Rc<T>;

pub use self::{
    constituent::*,
    document::*,
    determinator::*,
    range::Range,
    sentence::*,
    substantive::*,
};
