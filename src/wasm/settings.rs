/*
 * wasm/settings.rs
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

use super::error::error_to_js;
use super::prelude::*;
use crate::settings::{
    WikitextMode as RustWikitextMode, WikitextSettings as RustWikitextSettings,
};
use std::sync::Arc;

// Typescript declarations

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &str = r#"

export interface IWikitextSettings {
    mode: WikitextMode;
    enable_page_syntax: boolean;
    use_true_ids: boolean;
    allow_local_paths: boolean;
}

export type WikitextMode =
    | 'page'
    | 'draft'
    | 'forum-post'
    | 'direct-message'
    | 'list'

"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IWikitextSettings")]
    pub type IWikitextSettings;
}

// Wrapper structure

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WikitextSettings {
    inner: Arc<RustWikitextSettings>,
}

#[wasm_bindgen]
impl WikitextSettings {
    #[inline]
    pub(crate) fn get(&self) -> &RustWikitextSettings {
        &self.inner
    }

    #[wasm_bindgen]
    pub fn copy(&self) -> WikitextSettings {
        WikitextSettings {
            inner: Arc::clone(&self.inner),
        }
    }

    #[wasm_bindgen(constructor, typescript_type = "IWikitextSettings")]
    pub fn new(object: IWikitextSettings) -> Result<WikitextSettings, JsValue> {
        let rust_wikitext_settings = object.into_serde().map_err(error_to_js)?;

        Ok(WikitextSettings {
            inner: Arc::new(rust_wikitext_settings),
        })
    }

    #[wasm_bindgen]
    pub fn from_mode(mode: String) -> Result<WikitextSettings, JsValue> {
        let rust_mode = match mode.as_str() {
            "page" => RustWikitextMode::Page,
            "draft" => RustWikitextMode::Draft,
            "forum-post" => RustWikitextMode::ForumPost,
            "direct-message" => RustWikitextMode::DirectMessage,
            "list" => RustWikitextMode::List,
            _ => return Err(JsValue::from_str("Unknown mode")),
        };

        Ok(WikitextSettings {
            inner: Arc::new(RustWikitextSettings::from_mode(rust_mode)),
        })
    }
}
