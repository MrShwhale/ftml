/*
 * parsing/rule/impls/block/blocks/ruby.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2022 Wikijump Team
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

use super::prelude::*;
use crate::parsing::ParserWrap;
use crate::tree::{AcceptsPartial, PartialElement, RubyText};
use std::mem;

pub const BLOCK_RUBY: BlockRule = BlockRule {
    name: "block-ruby",
    accepts_names: &["ruby"],
    accepts_star: false,
    accepts_score: false,
    accepts_newlines: true,
    parse_fn: parse_block,
};

pub const BLOCK_RT: BlockRule = BlockRule {
    name: "block-ruby-text",
    accepts_names: &["rubytext", "rt"],
    accepts_star: false,
    accepts_score: false,
    accepts_newlines: true,
    parse_fn: parse_text,
};

// Main container block

fn parse_block<'r, 't>(
    parser: &mut Parser<'r, 't>,
    name: &'t str,
    flag_star: bool,
    flag_score: bool,
    in_head: bool,
) -> ParseResult<'r, 't, Elements<'t>> {
    info!("Parsing ruby block (name '{name}', in-head {in_head})");
    assert!(!flag_star, "Ruby doesn't allow star flag");
    assert!(!flag_score, "Ruby doesn't allow score flag");
    assert_block_name(&BLOCK_RUBY, name);

    let parser = &mut ParserWrap::new(parser, AcceptsPartial::Ruby);
    let arguments = parser.get_head_map(&BLOCK_RUBY, in_head)?;

    let (mut elements, exceptions, paragraph_safe) =
        parser.get_body_elements(&BLOCK_RUBY, false)?.into();

    // Convert ruby partials to elements
    for element in &mut elements {
        let (attributes, elements) = match element {
            // Swap out so we can extract fields
            Element::Partial(PartialElement::RubyText(ref mut ruby_text)) => {
                let RubyText {
                    attributes,
                    elements,
                } = mem::take(ruby_text);

                (attributes, elements)
            }

            // Leave other elements as-is
            _ => continue,
        };

        // Replace element with container, for final AST
        *element = Element::Container(Container::new(
            ContainerType::RubyText,
            elements,
            attributes,
        ));
    }

    // Ensure it contains no partials
    cfg_if! {
        if #[cfg(debug)] {
            for element in elements {
                if let Element::Partial(_) = element {
                    panic!("Found partial after conversion");
                }
            }
        }
    }

    // Build final ruby element
    let element = Element::Container(Container::new(
        ContainerType::Ruby,
        elements,
        arguments.to_attribute_map(parser.settings()),
    ));

    ok!(paragraph_safe; element, exceptions)
}

// Label block

fn parse_text<'r, 't>(
    parser: &mut Parser<'r, 't>,
    name: &'t str,
    flag_star: bool,
    flag_score: bool,
    in_head: bool,
) -> ParseResult<'r, 't, Elements<'t>> {
    info!("Parsing ruby text block (name '{name}', in-head {in_head})");
    assert!(!flag_star, "Ruby text doesn't allow star flag");
    assert!(!flag_score, "Ruby text doesn't allow score flag");
    assert_block_name(&BLOCK_RT, name);

    let arguments = parser.get_head_map(&BLOCK_RT, in_head)?;

    let (elements, exceptions, paragraph_safe) =
        parser.get_body_elements(&BLOCK_RT, false)?.into();

    let element = Element::Partial(PartialElement::RubyText(RubyText {
        elements,
        attributes: arguments.to_attribute_map(parser.settings()),
    }));

    ok!(paragraph_safe; element, exceptions)
}
