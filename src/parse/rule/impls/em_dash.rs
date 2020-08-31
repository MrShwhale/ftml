/*
 * parse/rule/impls/em_dash.rs
 *
 * ftml - Library to parse Wikidot code
 * Copyright (C) 2019-2020 Ammon Smith
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

pub const RULE_EM_DASH: Rule = make_rule!("em-dash", try_consume);

fn try_consume<'a>(
    log: &slog::Logger,
    _extract: &ExtractedToken<'a>,
    _next: &[ExtractedToken<'a>],
) -> Option<RuleResult<'a>> {
    trace!(log, "Producing em dash as element");

    Some(RuleResult {
        offset: 1,
        element: Element::Text("\u{2014}"),
    })
}
