/*
 * render/html/context.rs
 *
 * ftml - Convert Wikidot code to HTML
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

//! Internal state object used during rendering.

use crate::{ArticleHandle, Result};
use std::collections::HashSet;
use std::fmt::{self, Debug, Write};
use std::sync::Arc;
use super::HtmlOutput;

#[derive(Clone)]
pub struct HtmlContext {
    html: String,
    style: String,
    write_mode: WriteMode,
    footnotes: FootnoteContext,
    id: u64,
    handle: Arc<ArticleHandle>,
}

impl HtmlContext {
    pub fn new(id: u64, handle: Arc<ArticleHandle>) -> Self {
        HtmlContext {
            html: String::new(),
            style: String::new(),
            write_mode: WriteMode::Html,
            footnotes: FootnoteContext::new(),
            handle,
            id,
        }
    }

    // Field access
    #[inline]
    #[allow(dead_code)]
    pub fn id(&self) -> u64 {
        self.id
    }

    #[inline]
    pub fn handle(&self) -> Arc<ArticleHandle> {
        Arc::clone(&self.handle)
    }

    #[inline]
    pub fn footnotes(&self) -> &FootnoteContext {
        &self.footnotes
    }

    #[inline]
    pub fn footnotes_mut(&mut self) -> &mut FootnoteContext {
        &mut self.footnotes
    }

    // Operations
    pub fn substitute_footnote_block(&mut self) {
        const TOKEN: &str = "\0footnote-block\0";

        assert_eq!(self.write_mode, WriteMode::Html);
        assert_eq!(self.footnotes().needs_block(), false);

        let block = if self.footnotes.has_footnotes() {
            self.footnotes.contents()
        } else {
            ""
        };

        while let Some(idx) = self.html.find(TOKEN) {
            self.html.replace_range(idx..idx + TOKEN.len(), block);
        }
    }

    // Buffer management
    pub fn buffer(&mut self) -> &mut String {
        match self.write_mode {
            WriteMode::Html => &mut self.html,
            WriteMode::FootnoteBlock => self.footnotes.buffer(),
        }
    }

    #[inline]
    pub fn add_style(&mut self, style: &str) {
        if !self.style.is_empty() {
            self.style.push('\n');
        }

        self.style.push_str(style);
    }

    #[inline]
    pub fn insert_str(&mut self, idx: usize, s: &str) {
        self.buffer().insert_str(idx, s);
    }

    #[inline]
    pub fn push(&mut self, ch: char) {
        self.buffer().push(ch);
    }

    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.buffer().push_str(s);
    }

    pub fn write_footnote_block<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Self) -> Result<()>,
    {
        self.write_mode = WriteMode::FootnoteBlock;
        let result = f(self);
        self.write_mode = WriteMode::Html;
        result
    }

    // External calls
    #[inline]
    pub fn get_title(&self) -> Result<String> {
        self.handle.get_title(self.id)
    }

    #[inline]
    pub fn get_rating(&self) -> Result<Option<i32>> {
        self.handle.get_rating(self.id)
    }

    #[inline]
    pub fn get_tags(&mut self) -> Result<HashSet<String>> {
        self.handle.get_tags(self.id)
    }
}

impl Into<HtmlOutput> for HtmlContext {
    fn into(self) -> HtmlOutput {
        HtmlOutput {
            html: self.html,
            style: self.style,
        }
    }
}

impl Debug for HtmlContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HtmlContext")
            .field("html", &self.html)
            .field("style", &self.style)
            .field("footnotes", &self.footnotes)
            .field("id", &self.id)
            .field("handle", &"Arc<dyn ArticleHandle>")
            .finish()
    }
}

impl Write for HtmlContext {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer().write_str(s)
    }
}

// Helper structs
#[derive(Debug, Clone, PartialEq)]
pub struct FootnoteContext {
    buffer: String,
    has_block: bool,
    count: u32,
}

impl FootnoteContext {
    pub fn new() -> Self {
        FootnoteContext {
            buffer: str!("<div class=\"title\">Footnotes</div>"),
            has_block: false,
            count: 0,
        }
    }

    // Field access
    #[inline]
    pub fn set_block(&mut self, value: bool) {
        self.has_block = value;
    }

    #[inline]
    pub fn has_footnotes(&self) -> bool {
        self.count > 0
    }

    #[inline]
    pub fn incr(&mut self) -> u32 {
        self.count += 1;
        self.count
    }

    #[inline]
    pub fn needs_block(&self) -> bool {
        self.count > 0 && !self.has_block
    }

    #[inline]
    pub fn contents(&self) -> &str {
        &self.buffer
    }

    #[inline]
    fn buffer(&mut self) -> &mut String {
        &mut self.buffer
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum WriteMode {
    Html,
    FootnoteBlock,
}
