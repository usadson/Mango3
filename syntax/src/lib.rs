// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod constituent;
mod substantive;
mod determinator;

use std::rc::Rc;

pub type Shared<T> = Rc<T>;

pub use self::{
    constituent::*,
    determinator::*,
    substantive::*,
};
