/*
 * components/mod.rs
 *
 * wikidot-html - Library to convert Wikidot syntax into HTML
 * Copyright (c) 2019 Ammon Smith for Project Foundation
 *
 * wikidot-html is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

mod anchor;
mod component;

mod prelude {
    pub use crate::parse::build_regex;
    pub use regex::Regex;
    pub use std::fmt;
    pub use super::component::Component;
}
