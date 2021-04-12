/*
 * tree/clone.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! Utilities to help convert tree structures into owned versions.
//!
//! `SyntaxTree` and its child structures use `Cow` to enable both
//! serialization and deserialization.
//!
//! However if you need to convert a referenced version into an owned
//! version, that requires traversing all the child structures and
//! using `to_owned()`.
//!
//! This module has helpers to make this process easier.

use super::element::Element;
use super::list::ListItem;
use std::borrow::Cow;

pub fn option_string_to_owned(
    option_string: &Option<Cow<'_, str>>,
) -> Option<Cow<'static, str>> {
    match option_string {
        Some(string) => Some(string_to_owned(string)),
        None => None,
    }
}

#[inline]
#[allow(clippy::ptr_arg)]
pub fn string_to_owned(string: &Cow<'_, str>) -> Cow<'static, str> {
    // Clippy complains about us using &Cow<str>, which is normally valid,
    // but here we specifically want to convert the Cow into an owned form,
    // and we get a reference to Cow that we want to turn into an owned Cow.

    Cow::Owned(string.clone().into_owned())
}

pub fn strings_to_owned(strings: &[Cow<'_, str>]) -> Vec<Cow<'static, str>> {
    strings
        .iter()
        .map(|string| string_to_owned(string))
        .collect()
}

pub fn elements_to_owned(elements: &[Element<'_>]) -> Vec<Element<'static>> {
    elements.iter().map(|element| element.to_owned()).collect()
}

pub fn list_items_to_owned(list_items: &[ListItem<'_>]) -> Vec<ListItem<'static>> {
    list_items
        .iter()
        .map(|list_item| list_item.to_owned())
        .collect()
}