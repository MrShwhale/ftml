/*
 * render/text/elements.rs
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

//! Module that implements text rendering for `Element` and its children.

use super::TextContext;
use crate::render::ModuleRenderMode;
use crate::tree::{ContainerType, Element, ListItem, ListType};
use crate::url::is_url;
use std::borrow::Cow;

pub fn render_elements(log: &slog::Logger, ctx: &mut TextContext, elements: &[Element]) {
    debug!(log, "Rendering elements"; "elements-len" => elements.len());

    for element in elements {
        render_element(log, ctx, element);
    }
}

pub fn render_element(log: &slog::Logger, ctx: &mut TextContext, element: &Element) {
    debug!(log, "Rendering element"; "element" => element.name());

    match element {
        Element::Container(container) => {
            let mut invisible = false;
            let (add_newlines, prefix) = match container.ctype() {
                // Don't render this at all.
                ContainerType::Hidden => return,

                // Render it, but invisibly.
                // Requires setting a special mode in the context.
                ContainerType::Invisible => {
                    ctx.enable_invisible();
                    invisible = true;

                    (false, None)
                }

                // If container is "terminating" (e.g. blockquote, p), then add newlines.
                // Also, determine if we add a prefix.
                ContainerType::Div | ContainerType::Paragraph => (true, None),
                ContainerType::Blockquote => (true, Some("    ")),
                ContainerType::Header(level) => (true, Some(level.prefix())),
                _ => (false, None),
            };

            if add_newlines {
                // Add prefix, if there's one
                if let Some(prefix) = prefix {
                    ctx.push_prefix(prefix);
                }

                ctx.add_newline();
            }

            // Render internal elements
            render_elements(log, ctx, container.elements());

            if add_newlines {
                // Pop prefix, if there's one
                if prefix.is_some() {
                    ctx.pop_prefix();
                }

                ctx.add_newline();
            }

            if invisible {
                ctx.disable_invisible();
            }
        }
        Element::Module(module) => {
            ctx.handle()
                .render_module(log, ctx.buffer(), module, ModuleRenderMode::Text)
        }
        Element::Text(text) | Element::Raw(text) | Element::Email(text) => {
            ctx.push_str(text)
        }
        Element::Anchor {
            elements,
            attributes,
            ..
        } => {
            render_elements(log, ctx, elements);

            if let Some(href) = attributes.get().get("href") {
                let url = get_full_url(log, ctx, href);
                if &url != href {
                    str_write!(ctx, " [{}]", url);
                }
            }
        }
        Element::Link { url, label, .. } => {
            ctx.handle().get_link_label(log, url, label, |label| {
                ctx.push_str(label);

                let url = get_full_url(log, ctx, url);
                if url != label {
                    str_write!(ctx, " [{}]", url);
                }
            });
        }
        Element::List { ltype, items } => {
            ctx.add_newline();

            for item in items {
                match item {
                    ListItem::Elements(elements) => {
                        // Render bullet and its depth
                        let depth = ctx.list_depth();
                        for _ in 0..depth {
                            ctx.push(' ');
                        }

                        match *ltype {
                            ListType::Bullet => ctx.push_str("* "),
                            ListType::Numbered => {
                                let index = ctx.next_list_index();
                                str_write!(ctx, "{}. ", index);
                            }
                            ListType::Generic => (),
                        }

                        // Render elements for this list item
                        render_elements(log, ctx, elements);
                        ctx.add_newline();
                    }
                    ListItem::SubList(list) => {
                        // Update bullet depth
                        ctx.incr_list_depth();
                        render_element(log, ctx, list);
                        ctx.decr_list_depth();
                    }
                }
            }
        }
        Element::RadioButton { checked, .. } => {
            str_write!(ctx, "({}) ", if *checked { '*' } else { ' ' })
        }
        Element::CheckBox { checked, .. } => {
            str_write!(ctx, "[{}] ", if *checked { 'X' } else { ' ' })
        }
        Element::Collapsible {
            elements,
            show_text,
            hide_text,
            show_top,
            show_bottom,
            ..
        } => {
            macro_rules! get_text {
                ($input:expr, $message:expr) => {
                    match $input {
                        Some(ref text) => &text,
                        None => {
                            let locale = &ctx.info().locale;

                            ctx.handle().get_message(log, locale, $message)
                        }
                    }
                };
            }

            let show_text = get_text!(show_text, "collapsible-open");
            let hide_text = get_text!(hide_text, "collapsible-hide");

            // Top of collapsible
            ctx.add_newline();
            ctx.push_str(show_text);
            ctx.add_newline();

            if *show_top {
                ctx.push_str(hide_text);
                ctx.add_newline();
            }

            // Collapsible contents
            render_elements(log, ctx, elements);

            // Bottom of collapsible
            if *show_bottom {
                ctx.add_newline();
                ctx.push_str(hide_text);
                ctx.add_newline();
            }
        }
        Element::Color { elements, .. } => render_elements(log, ctx, elements),
        Element::Code { contents, language } => {
            let language = match language {
                Some(cow) => &cow,
                None => "",
            };

            str_write!(ctx, "```{}\n{}\n```", language, contents);
        }
        Element::Html { contents } => {
            str_write!(ctx, "```html\n{}\n```", contents);
        }
        Element::Iframe { url, .. } => str_write!(ctx, "[iframe: {}]", url),
        Element::LineBreak => ctx.add_newline(),
        Element::LineBreaks(amount) => {
            for _ in 0..amount.get() {
                ctx.add_newline();
            }
        }
        Element::HorizontalRule => {
            ctx.add_newline();
            ctx.push_str("------");
            ctx.add_newline();
        }
    }
}

fn get_full_url<'a>(log: &slog::Logger, ctx: &TextContext, url: &'a str) -> Cow<'a, str> {
    // TODO: when we remove inline javascript stuff
    if url == "javascript:;" {
        return Cow::Borrowed("#");
    }

    // Anchor links should just be returned as-is
    if url.starts_with('#') {
        return Cow::Borrowed(url);
    }

    // If it's a URL with a scheme, just return this
    if is_url(url) {
        return Cow::Borrowed(url);
    }

    // Let's build a full URL:
    let site = &ctx.info().site;
    let mut full_url = ctx.handle().get_url(log, site);

    // Ensure there is exactly one slash
    if !full_url.ends_with('/') && !url.starts_with('/') {
        full_url.push('/');
    }

    // Remove duplicate slash, if present
    if full_url.ends_with('/') && url.starts_with('/') {
        full_url.pop();
    }

    full_url.push_str(url);
    Cow::Owned(full_url)
}
