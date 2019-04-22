/*
 * filter/misc.rs
 *
 * wikidot-html - Convert Wikidot code to HTML
 * Copyright (C) 2019 Ammon Smith for Project Foundation
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

//! This performs the various miscellaneous substitutions that Wikidot does
//! in preparation for its parsing and handling processes. These are:
//! * Replacing DOS and legacy Mac newlines
//! * Trimming whitespace lines
//! * Concatenating lines that end with backslashes
//! * Convert tabs to four spaces
//! * Compress groups of 3+ newlines into 2 newlines

use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref TABS: Regex = Regex::new(r"\t").unwrap();
    static ref DOS_NEWLINES: Regex = Regex::new(r"\r\n").unwrap();
    static ref MAC_NEWLINES: Regex = Regex::new(r"\r").unwrap();
    static ref CONCAT_BACKSLASHES: Regex = Regex::new(r"\\\n").unwrap();

    static ref WHITESPACE: Regex = {
        RegexBuilder::new(r"^\s+$")
            .multi_line(true)
            .build()
            .unwrap()
    };

    static ref COMPRESS_NEWLINES: Regex = {
        RegexBuilder::new(r"(?:\n\s*){3,}")
            .multi_line(true)
            .build()
            .unwrap()
    };
}

pub fn substitute(text: &mut String) {
    regex_replace(text, &*DOS_NEWLINES, "\n");
    regex_replace(text, &*MAC_NEWLINES, "\n");
    regex_replace(text, &*WHITESPACE, "");
    regex_replace(text, &*CONCAT_BACKSLASHES, "");
    regex_replace(text, &*TABS, "    ");
    regex_replace(text, &*COMPRESS_NEWLINES, "\n\n");
}

fn regex_replace(text: &mut String, regex: &Regex, replacement: &str) {
    while let Some(mtch) = regex.find(text) {
        let range = mtch.start()..mtch.end();
        text.replace_range(range, replacement);
    }
}

#[test]
fn test_regexes() {
    let _ = &*TABS;
    let _ = &*DOS_NEWLINES;
    let _ = &*MAC_NEWLINES;
    let _ = &*CONCAT_BACKSLASHES;
    let _ = &*WHITESPACE;
    let _ = &*COMPRESS_NEWLINES;
}

#[test]
fn test_substitute() {
    let mut string = String::new();

    macro_rules! substitute {
        ($str:expr) => {{
            string.clear();
            string.push_str($str);
            substitute(&mut string);
        }}
    }

    substitute!("\tapple\n\tbanana\tcherry\n");
    assert_eq!(&string, "    apple\n    banana    cherry\n");

    substitute!("newlines:\r\n* apple\r* banana\r\ncherry\n\r* durian");
    assert_eq!(&string, "newlines:\n* apple\n* banana\ncherry\n\n* durian");

    substitute!("apple\nbanana\n\ncherry\n\n\npineapple\n\n\n\nstrawberry\n\n\n\n\nblueberry\n\n\n\n\n\n");
    assert_eq!(&string, "apple\nbanana\n\ncherry\n\npineapple\n\nstrawberry\n\nblueberry\n");

    substitute!("apple\rbanana\r\rcherry\r\r\rpineapple\r\r\r\rstrawberry\r\r\r\r\rblueberry\r\r\r\r\r\r");
    assert_eq!(&string, "apple\nbanana\n\ncherry\n\npineapple\n\nstrawberry\n\nblueberry\n");

    substitute!("concat:\napple banana \\\nCherry\\\nPineapple \\ grape\nblueberry\n");
    assert_eq!(&string, "concat:\napple banana CherryPineapple \\ grape\nblueberry\n");

    substitute!("<\n        \n      \n  \n      \n>");
    assert_eq!(&string, "<\n\n>");
}
