/*
 * ffi/mod.rs
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

// Because this module is all about interfacing with C,
// which is inherently unsafe, we must permit unsafe code.
//
// This is only used for FFI, no weird memory tricks are used.
// So this is the "safe" form of unsafe within Rust.
#![allow(unsafe_code)]
// This module uses C naming for its components, mostly snake_case.
#![allow(non_camel_case_types, clippy::upper_case_acronyms)]

mod prelude {
    pub use super::pool::get_static_cstr;
    pub use super::string::*;
    pub use super::vec::*;
    pub use std::ffi::{CStr, CString};
    pub use std::os::raw::c_char;
    pub use std::{mem, ptr};
}

mod backlinks;
mod exports;
mod html;
mod misc;
mod page_info;
mod page_ref;
mod pool;
mod settings;
mod string;
mod text;
mod vec;
mod warning;
