/*
 * render/html/element/input.rs
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

use super::prelude::*;
use crate::tree::AttributeMap;

pub fn render_radio_button(
    log: &slog::Logger,
    ctx: &mut HtmlContext,
    name: &str,
    checked: bool,
    attributes: &AttributeMap,
) {
    debug!(
        log,
        "Creating radio button";
        "name" => name,
        "checked" => checked,
    );

    // Create <input> tag
    let mut tag = ctx.html().input();

    // Add attributes
    tag.attr("name", &[name]);
    tag.attr("type", &["radio"]);

    if checked {
        tag.attr("checked", &[]);
    }

    tag.attr_map(attributes);
}
