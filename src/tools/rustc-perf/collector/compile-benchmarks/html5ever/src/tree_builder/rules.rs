// Copyright 2014 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The tree builder rules, as a single, enormous nested match expression.

use tree_builder::types::*;
use tree_builder::tag_sets::*;
use tree_builder::actions::{NoPush, Push, TreeBuilderActions};
use tree_builder::interface::{TreeSink, Quirks, AppendNode, NextParserState};

use tokenizer::{Attribute, EndTag, StartTag, Tag};
use tokenizer::states::{Rcdata, Rawtext, ScriptData, Plaintext, Quiescent};

use util::str::is_ascii_whitespace;

use std::ascii::AsciiExt;
use std::mem::replace;
use std::borrow::Cow::Borrowed;
use std::borrow::ToOwned;

use tendril::{StrTendril, SliceExt};

fn any_not_whitespace(x: &StrTendril) -> bool {
    // FIXME: this might be much faster as a byte scan
    x.chars().any(|c| !is_ascii_whitespace(c))
}

// This goes in a trait so that we can control visibility.
pub trait TreeBuilderStep {
    fn step(&mut self, mode: InsertionMode, token: Token) -> ProcessResult;
    fn step_foreign(&mut self, token: Token) -> ProcessResult;
}

#[doc(hidden)]
impl<Handle, Sink> TreeBuilderStep
    for super::TreeBuilder<Handle, Sink>
    where Handle: Clone,
          Sink: TreeSink<Handle=Handle>,
{
    fn step(&mut self, mode: InsertionMode, token: Token) -> ProcessResult {
        self.debug_step(mode, &token);

        match mode {
            //§ the-initial-insertion-mode
            Initial => match_token!(token {
                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, _) => Done,
                CommentToken(text) => self.append_comment_to_doc(text),
                token => {
                    if !self.opts.iframe_srcdoc {
                        self.unexpected(&token);
                        self.set_quirks_mode(Quirks);
                    }
                    Reprocess(BeforeHtml, token)
                }
            }),

            //§ the-before-html-insertion-mode
            BeforeHtml => match_token!(token {
                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, _) => Done,
                CommentToken(text) => self.append_comment_to_doc(text),

                tag @ <html> => {
                    self.create_root(tag.attrs);
                    self.mode = BeforeHead;
                    Done
                }

                </head> </body> </html> </br> => else,

                tag @ </_> => self.unexpected(&tag),

                token => {
                    self.create_root(vec!());
                    Reprocess(BeforeHead, token)
                }
            }),

            //§ the-before-head-insertion-mode
            BeforeHead => match_token!(token {
                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, _) => Done,
                CommentToken(text) => self.append_comment(text),

                <html> => self.step(InBody, token),

                tag @ <head> => {
                    self.head_elem = Some(self.insert_element_for(tag));
                    self.mode = InHead;
                    Done
                }

                </head> </body> </html> </br> => else,

                tag @ </_> => self.unexpected(&tag),

                token => {
                    self.head_elem = Some(self.insert_phantom(atom!("head")));
                    Reprocess(InHead, token)
                }
            }),

            //§ parsing-main-inhead
            InHead => match_token!(token {
                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, text) => self.append_text(text),
                CommentToken(text) => self.append_comment(text),

                <html> => self.step(InBody, token),

                tag @ <base> <basefont> <bgsound> <link> <meta> => {
                    // FIXME: handle <meta charset=...> and <meta http-equiv="Content-Type">
                    self.insert_and_pop_element_for(tag);
                    DoneAckSelfClosing
                }

                tag @ <title> => {
                    self.parse_raw_data(tag, Rcdata);
                    Done
                }

                tag @ <noframes> <style> <noscript> => {
                    if (!self.opts.scripting_enabled) && (tag.name == atom!("noscript")) {
                        self.insert_element_for(tag);
                        self.mode = InHeadNoscript;
                    } else {
                        self.parse_raw_data(tag, Rawtext);
                    }
                    Done
                }

                tag @ <script> => {
                    let elem = self.sink.create_element(qualname!(html, "script"), tag.attrs);
                    if self.is_fragment() {
                        self.sink.mark_script_already_started(elem.clone());
                    }
                    self.insert_appropriately(AppendNode(elem.clone()), None);
                    self.open_elems.push(elem);
                    self.to_raw_text_mode(ScriptData);
                    Done
                }

                </head> => {
                    self.pop();
                    self.mode = AfterHead;
                    Done
                }

                </body> </html> </br> => else,

                tag @ <template> => {
                    self.insert_element_for(tag);
                    self.active_formatting.push(Marker);
                    self.frameset_ok = false;
                    self.mode = InTemplate;
                    self.template_modes.push(InTemplate);
                    Done
                }

                tag @ </template> => {
                    if !self.in_html_elem_named(atom!("template")) {
                        self.unexpected(&tag);
                    } else {
                        self.generate_implied_end(thorough_implied_end);
                        self.expect_to_close(atom!("template"));
                        self.clear_active_formatting_to_marker();
                        self.template_modes.pop();
                        self.mode = self.reset_insertion_mode();
                    }
                    Done
                }

                <head> => self.unexpected(&token),
                tag @ </_> => self.unexpected(&tag),

                token => {
                    self.pop();
                    Reprocess(AfterHead, token)
                }
            }),

            //§ parsing-main-inheadnoscript
            InHeadNoscript => match_token!(token {
                <html> => self.step(InBody, token),

                </noscript> => {
                    self.pop();
                    self.mode = InHead;
                    Done
                },

                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, _) => self.step(InHead, token),

                CommentToken(_) => self.step(InHead, token),

                <basefont> <bgsound> <link> <meta> <noframes> <style>
                    => self.step(InHead, token),

                </br> => else,

                <head> <noscript> => self.unexpected(&token),
                tag @ </_> => self.unexpected(&tag),

                token => {
                    self.unexpected(&token);
                    self.pop();
                    Reprocess(InHead, token)
                },
            }),

            //§ the-after-head-insertion-mode
            AfterHead => match_token!(token {
                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, text) => self.append_text(text),
                CommentToken(text) => self.append_comment(text),

                <html> => self.step(InBody, token),

                tag @ <body> => {
                    self.insert_element_for(tag);
                    self.frameset_ok = false;
                    self.mode = InBody;
                    Done
                }

                tag @ <frameset> => {
                    self.insert_element_for(tag);
                    self.mode = InFrameset;
                    Done
                }

                <base> <basefont> <bgsound> <link> <meta>
                      <noframes> <script> <style> <template> <title> => {
                    self.unexpected(&token);
                    let head = self.head_elem.as_ref().expect("no head element").clone();
                    self.push(&head);
                    let result = self.step(InHead, token);
                    self.remove_from_stack(&head);
                    result
                }

                </template> => self.step(InHead, token),

                </body> </html> </br> => else,

                <head> => self.unexpected(&token),
                tag @ </_> => self.unexpected(&tag),

                token => {
                    self.insert_phantom(atom!("body"));
                    Reprocess(InBody, token)
                }
            }),

            //§ parsing-main-inbody
            InBody => match_token!(token {
                NullCharacterToken => self.unexpected(&token),

                CharacterTokens(_, text) => {
                    self.reconstruct_formatting();
                    if any_not_whitespace(&text) {
                        self.frameset_ok = false;
                    }
                    self.append_text(text)
                }

                CommentToken(text) => self.append_comment(text),

                tag @ <html> => {
                    self.unexpected(&tag);
                    if !self.in_html_elem_named(atom!("template")) {
                        let top = self.html_elem();
                        self.sink.add_attrs_if_missing(top, tag.attrs);
                    }
                    Done
                }

                <base> <basefont> <bgsound> <link> <meta> <noframes>
                  <script> <style> <template> <title> </template> => {
                    self.step(InHead, token)
                }

                tag @ <body> => {
                    self.unexpected(&tag);
                    match self.body_elem() {
                        Some(ref node) if self.open_elems.len() != 1 &&
                                          !self.in_html_elem_named(atom!("template")) => {
                            self.frameset_ok = false;
                            self.sink.add_attrs_if_missing(node.clone(), tag.attrs)
                        },
                        _ => {}
                    }
                    Done
                }

                tag @ <frameset> => {
                    self.unexpected(&tag);
                    if !self.frameset_ok { return Done; }

                    // Can't use unwrap_or_return!() due to rust-lang/rust#16617.
                    let body = match self.body_elem() {
                        None => return Done,
                        Some(x) => x,
                    };
                    self.sink.remove_from_parent(body);

                    // FIXME: can we get here in the fragment case?
                    // What to do with the first element then?
                    self.open_elems.truncate(1);
                    self.insert_element_for(tag);
                    self.mode = InFrameset;
                    Done
                }

                EOFToken => {
                    if !self.template_modes.is_empty() {
                        self.step(InTemplate, token)
                    } else {
                        self.check_body_end();
                        self.stop_parsing()
                    }
                }

                </body> => {
                    if self.in_scope_named(default_scope, atom!("body")) {
                        self.check_body_end();
                        self.mode = AfterBody;
                    } else {
                        self.sink.parse_error(Borrowed("</body> with no <body> in scope"));
                    }
                    Done
                }

                </html> => {
                    if self.in_scope_named(default_scope, atom!("body")) {
                        self.check_body_end();
                        Reprocess(AfterBody, token)
                    } else {
                        self.sink.parse_error(Borrowed("</html> with no <body> in scope"));
                        Done
                    }
                }

                tag @ <address> <article> <aside> <blockquote> <center> <details> <dialog>
                  <dir> <div> <dl> <fieldset> <figcaption> <figure> <footer> <header>
                  <hgroup> <main> <menu> <nav> <ol> <p> <section> <summary> <ul> => {
                    self.close_p_element_in_button_scope();
                    self.insert_element_for(tag);
                    Done
                }

                tag @ <h1> <h2> <h3> <h4> <h5> <h6> => {
                    self.close_p_element_in_button_scope();
                    if self.current_node_in(heading_tag) {
                        self.sink.parse_error(Borrowed("nested heading tags"));
                        self.pop();
                    }
                    self.insert_element_for(tag);
                    Done
                }

                tag @ <pre> <listing> => {
                    self.close_p_element_in_button_scope();
                    self.insert_element_for(tag);
                    self.ignore_lf = true;
                    self.frameset_ok = false;
                    Done
                }

                tag @ <form> => {
                    if self.form_elem.is_some() &&
                       !self.in_html_elem_named(atom!("template")) {
                        self.sink.parse_error(Borrowed("nested forms"));
                    } else {
                        self.close_p_element_in_button_scope();
                        let elem = self.insert_element_for(tag);
                        if !self.in_html_elem_named(atom!("template")) {
                            self.form_elem = Some(elem);
                        }
                    }
                    Done
                }

                tag @ <li> <dd> <dt> => {
                    declare_tag_set!(close_list = "li");
                    declare_tag_set!(close_defn = "dd" "dt");
                    declare_tag_set!(extra_special = [special_tag] - "address" "div" "p");
                    let can_close: fn(::string_cache::QualName) -> bool = match tag.name {
                        atom!("li") => close_list,
                        atom!("dd") | atom!("dt") => close_defn,
                        _ => unreachable!(),
                    };

                    self.frameset_ok = false;

                    let mut to_close = None;
                    for node in self.open_elems.iter().rev() {
                        let name = self.sink.elem_name(node.clone());
                        if can_close(name.clone()) {
                            to_close = Some(name.local);
                            break;
                        }
                        if extra_special(name.clone()) {
                            break;
                        }
                    }

                    match to_close {
                        Some(name) => {
                            self.generate_implied_end_except(name.clone());
                            self.expect_to_close(name);
                        }
                        None => (),
                    }

                    self.close_p_element_in_button_scope();
                    self.insert_element_for(tag);
                    Done
                }

                tag @ <plaintext> => {
                    self.close_p_element_in_button_scope();
                    self.insert_element_for(tag);
                    self.next_tokenizer_state = Some(Plaintext);
                    Done
                }

                tag @ <button> => {
                    if self.in_scope_named(default_scope, atom!("button")) {
                        self.sink.parse_error(Borrowed("nested buttons"));
                        self.generate_implied_end(cursory_implied_end);
                        self.pop_until_named(atom!("button"));
                    }
                    self.reconstruct_formatting();
                    self.insert_element_for(tag);
                    self.frameset_ok = false;
                    Done
                }

                tag @ </address> </article> </aside> </blockquote> </button> </center>
                  </details> </dialog> </dir> </div> </dl> </fieldset> </figcaption>
                  </figure> </footer> </header> </hgroup> </listing> </main> </menu>
                  </nav> </ol> </pre> </section> </summary> </ul> => {
                    if !self.in_scope_named(default_scope, tag.name.clone()) {
                        self.unexpected(&tag);
                    } else {
                        self.generate_implied_end(cursory_implied_end);
                        self.expect_to_close(tag.name);
                    }
                    Done
                }

                </form> => {
                    if !self.in_html_elem_named(atom!("template")) {
                        // Can't use unwrap_or_return!() due to rust-lang/rust#16617.
                        let node = match self.form_elem.take() {
                            None => {
                                self.sink.parse_error(Borrowed("Null form element pointer on </form>"));
                                return Done;
                            }
                            Some(x) => x,
                        };
                        if !self.in_scope(default_scope, |n| self.sink.same_node(node.clone(), n)) {
                            self.sink.parse_error(Borrowed("Form element not in scope on </form>"));
                            return Done;
                        }
                        self.generate_implied_end(cursory_implied_end);
                        let current = self.current_node();
                        self.remove_from_stack(&node);
                        if !self.sink.same_node(current, node) {
                            self.sink.parse_error(Borrowed("Bad open element on </form>"));
                        }
                    } else {
                        if !self.in_scope_named(default_scope, atom!("form")) {
                            self.sink.parse_error(Borrowed("Form element not in scope on </form>"));
                            return Done;
                        }
                        self.generate_implied_end(cursory_implied_end);
                        if !self.current_node_named(atom!("form")) {
                            self.sink.parse_error(Borrowed("Bad open element on </form>"));
                        }
                        self.pop_until_named(atom!("form"));
                    }
                    Done
                }

                </p> => {
                    if !self.in_scope_named(button_scope, atom!("p")) {
                        self.sink.parse_error(Borrowed("No <p> tag to close"));
                        self.insert_phantom(atom!("p"));
                    }
                    self.close_p_element();
                    Done
                }

                tag @ </li> </dd> </dt> => {
                    let scope: fn(::string_cache::QualName) -> bool = match tag.name {
                        atom!("li") => list_item_scope,
                        _ => default_scope,
                    };
                    if self.in_scope_named(|x| scope(x), tag.name.clone()) {
                        self.generate_implied_end_except(tag.name.clone());
                        self.expect_to_close(tag.name);
                    } else {
                        self.sink.parse_error(Borrowed("No matching tag to close"));
                    }
                    Done
                }

                tag @ </h1> </h2> </h3> </h4> </h5> </h6> => {
                    if self.in_scope(default_scope, |n| self.elem_in(n.clone(), heading_tag)) {
                        self.generate_implied_end(cursory_implied_end);
                        if !self.current_node_named(tag.name) {
                            self.sink.parse_error(Borrowed("Closing wrong heading tag"));
                        }
                        self.pop_until(heading_tag);
                    } else {
                        self.sink.parse_error(Borrowed("No heading tag to close"));
                    }
                    Done
                }

                tag @ <a> => {
                    self.handle_misnested_a_tags(&tag);
                    self.reconstruct_formatting();
                    self.create_formatting_element_for(tag);
                    Done
                }

                tag @ <b> <big> <code> <em> <font> <i> <s> <small> <strike> <strong> <tt> <u> => {
                    self.reconstruct_formatting();
                    self.create_formatting_element_for(tag);
                    Done
                }

                tag @ <nobr> => {
                    self.reconstruct_formatting();
                    if self.in_scope_named(default_scope, atom!("nobr")) {
                        self.sink.parse_error(Borrowed("Nested <nobr>"));
                        self.adoption_agency(atom!("nobr"));
                        self.reconstruct_formatting();
                    }
                    self.create_formatting_element_for(tag);
                    Done
                }

                tag @ </a> </b> </big> </code> </em> </font> </i> </nobr>
                  </s> </small> </strike> </strong> </tt> </u> => {
                    self.adoption_agency(tag.name);
                    Done
                }

                tag @ <applet> <marquee> <object> => {
                    self.reconstruct_formatting();
                    self.insert_element_for(tag);
                    self.active_formatting.push(Marker);
                    self.frameset_ok = false;
                    Done
                }

                tag @ </applet> </marquee> </object> => {
                    if !self.in_scope_named(default_scope, tag.name.clone()) {
                        self.unexpected(&tag);
                    } else {
                        self.generate_implied_end(cursory_implied_end);
                        self.expect_to_close(tag.name);
                        self.clear_active_formatting_to_marker();
                    }
                    Done
                }

                tag @ <table> => {
                    if self.quirks_mode != Quirks {
                        self.close_p_element_in_button_scope();
                    }
                    self.insert_element_for(tag);
                    self.frameset_ok = false;
                    self.mode = InTable;
                    Done
                }

                tag @ </br> => {
                    self.unexpected(&tag);
                    self.step(InBody, TagToken(Tag {
                        kind: StartTag,
                        attrs: vec!(),
                        ..tag
                    }))
                }

                tag @ <area> <br> <embed> <img> <keygen> <wbr> <input> => {
                    let keep_frameset_ok = match tag.name {
                        atom!("input") => self.is_type_hidden(&tag),
                        _ => false,
                    };
                    self.reconstruct_formatting();
                    self.insert_and_pop_element_for(tag);
                    if !keep_frameset_ok {
                        self.frameset_ok = false;
                    }
                    DoneAckSelfClosing
                }

                tag @ <menuitem> <param> <source> <track> => {
                    self.insert_and_pop_element_for(tag);
                    DoneAckSelfClosing
                }

                tag @ <hr> => {
                    self.close_p_element_in_button_scope();
                    self.insert_and_pop_element_for(tag);
                    self.frameset_ok = false;
                    DoneAckSelfClosing
                }

                tag @ <image> => {
                    self.unexpected(&tag);
                    self.step(InBody, TagToken(Tag {
                        name: atom!("img"),
                        ..tag
                    }))
                }

                tag @ <isindex> => {
                    self.unexpected(&tag);
                    let in_template = self.in_html_elem_named(atom!("template"));
                    if !in_template && self.form_elem.is_some() {
                        return Done;
                    }
                    self.frameset_ok = false;
                    self.close_p_element_in_button_scope();
                    let mut form_attrs = vec![];
                    let mut prompt = None;
                    let mut input_attrs = vec![];
                    for attr in tag.attrs.into_iter() {
                        match attr.name {
                            qualname!("", "action") => form_attrs.push(attr),
                            qualname!("", "prompt") => prompt = Some(attr.value),
                            qualname!("", "name") => {},
                            _ => input_attrs.push(attr),
                        }
                    }
                    input_attrs.push(Attribute {
                        name: qualname!("", "name"),
                        value: "isindex".to_tendril(),
                    });
                    let form = self.insert_element(Push, ns!(html), atom!("form"), form_attrs);
                    if !in_template {
                        self.form_elem = Some(form.clone());
                    }
                    self.insert_element(NoPush, ns!(html), atom!("hr"), vec![]);
                    self.reconstruct_formatting();
                    self.insert_element(Push, ns!(html), atom!("label"), vec![]);
                    self.append_text(prompt.unwrap_or_else(|| {
                        "This is a searchable index. Enter search keywords: ".to_tendril()
                    }));
                    self.insert_element(NoPush, ns!(html), atom!("input"), input_attrs);
                    self.pop();
                    self.insert_element(NoPush, ns!(html), atom!("hr"), vec![]);
                    self.pop();
                    if !in_template {
                        self.form_elem = None;
                    }
                    DoneAckSelfClosing
                }

                tag @ <textarea> => {
                    self.ignore_lf = true;
                    self.frameset_ok = false;
                    self.parse_raw_data(tag, Rcdata);
                    Done
                }

                tag @ <xmp> => {
                    self.close_p_element_in_button_scope();
                    self.reconstruct_formatting();
                    self.frameset_ok = false;
                    self.parse_raw_data(tag, Rawtext);
                    Done
                }

                tag @ <iframe> => {
                    self.frameset_ok = false;
                    self.parse_raw_data(tag, Rawtext);
                    Done
                }

                tag @ <noembed> => {
                    self.parse_raw_data(tag, Rawtext);
                    Done
                }

                // <noscript> handled in wildcard case below

                tag @ <select> => {
                    self.reconstruct_formatting();
                    self.insert_element_for(tag);
                    self.frameset_ok = false;
                    // NB: mode == InBody but possibly self.mode != mode, if
                    // we're processing "as in the rules for InBody".
                    self.mode = match self.mode {
                        InTable | InCaption | InTableBody
                            | InRow | InCell => InSelectInTable,
                        _ => InSelect,
                    };
                    Done
                }

                tag @ <optgroup> <option> => {
                    if self.current_node_named(atom!("option")) {
                        self.pop();
                    }
                    self.reconstruct_formatting();
                    self.insert_element_for(tag);
                    Done
                }

                tag @ <rb> <rtc> => {
                    if self.in_scope_named(default_scope, atom!("ruby")) {
                        self.generate_implied_end(cursory_implied_end);
                    }
                    if !self.current_node_named(atom!("ruby")) {
                        self.unexpected(&tag);
                    }
                    self.insert_element_for(tag);
                    Done
                }

                tag @ <rp> <rt> => {
                    if self.in_scope_named(default_scope, atom!("ruby")) {
                        self.generate_implied_end_except(atom!("rtc"));
                    }
                    if !self.current_node_named(atom!("rtc")) && !self.current_node_named(atom!("ruby")) {
                        self.unexpected(&tag);
                    }
                    self.insert_element_for(tag);
                    Done
                }

                tag @ <math> => self.enter_foreign(tag, ns!(mathml)),

                tag @ <svg> => self.enter_foreign(tag, ns!(svg)),

                <caption> <col> <colgroup> <frame> <head>
                  <tbody> <td> <tfoot> <th> <thead> <tr> => {
                    self.unexpected(&token);
                    Done
                }

                tag @ <_> => {
                    if self.opts.scripting_enabled && tag.name == atom!("noscript") {
                        self.parse_raw_data(tag, Rawtext);
                    } else {
                        self.reconstruct_formatting();
                        self.insert_element_for(tag);
                    }
                    Done
                }

                tag @ </_> => {
                    self.process_end_tag_in_body(tag);
                    Done
                }

                // FIXME: This should be unreachable, but match_token! requires a
                // catch-all case.
                _ => panic!("impossible case in InBody mode"),
            }),

            //§ parsing-main-incdata
            Text => match_token!(token {
                CharacterTokens(_, text) => self.append_text(text),

                EOFToken => {
                    self.unexpected(&token);
                    if self.current_node_named(atom!("script")) {
                        let current = self.current_node();
                        self.sink.mark_script_already_started(current);
                    }
                    self.pop();
                    Reprocess(self.orig_mode.take().unwrap(), token)
                }

                tag @ </_> => {
                    let node = self.pop();
                    if tag.name == atom!("script") {
                        warn!("FIXME: </script> not fully implemented");
                        if self.sink.complete_script(node) == NextParserState::Suspend {
                            self.next_tokenizer_state = Some(Quiescent);
                        }
                    }
                    self.mode = self.orig_mode.take().unwrap();
                    Done
                }

                // The spec doesn't say what to do here.
                // Other tokens are impossible?
                _ => panic!("impossible case in Text mode"),
            }),

            //§ parsing-main-intable
            InTable => match_token!(token {
                // FIXME: hack, should implement pat | pat for match_token!() instead
                NullCharacterToken => self.process_chars_in_table(token),

                CharacterTokens(..) => self.process_chars_in_table(token),

                CommentToken(text) => self.append_comment(text),

                tag @ <caption> => {
                    self.pop_until_current(table_scope);
                    self.active_formatting.push(Marker);
                    self.insert_element_for(tag);
                    self.mode = InCaption;
                    Done
                }

                tag @ <colgroup> => {
                    self.pop_until_current(table_scope);
                    self.insert_element_for(tag);
                    self.mode = InColumnGroup;
                    Done
                }

                <col> => {
                    self.pop_until_current(table_scope);
                    self.insert_phantom(atom!("colgroup"));
                    Reprocess(InColumnGroup, token)
                }

                tag @ <tbody> <tfoot> <thead> => {
                    self.pop_until_current(table_scope);
                    self.insert_element_for(tag);
                    self.mode = InTableBody;
                    Done
                }

                <td> <th> <tr> => {
                    self.pop_until_current(table_scope);
                    self.insert_phantom(atom!("tbody"));
                    Reprocess(InTableBody, token)
                }

                <table> => {
                    self.unexpected(&token);
                    if self.in_scope_named(table_scope, atom!("table")) {
                        self.pop_until_named(atom!("table"));
                        Reprocess(self.reset_insertion_mode(), token)
                    } else {
                        Done
                    }
                }

                </table> => {
                    if self.in_scope_named(table_scope, atom!("table")) {
                        self.pop_until_named(atom!("table"));
                        self.mode = self.reset_insertion_mode();
                    } else {
                        self.unexpected(&token);
                    }
                    Done
                }

                </body> </caption> </col> </colgroup> </html>
                  </tbody> </td> </tfoot> </th> </thead> </tr> =>
                    self.unexpected(&token),

                <style> <script> <template> </template>
                    => self.step(InHead, token),

                tag @ <input> => {
                    self.unexpected(&tag);
                    if self.is_type_hidden(&tag) {
                        self.insert_and_pop_element_for(tag);
                        DoneAckSelfClosing
                    } else {
                        self.foster_parent_in_body(TagToken(tag))
                    }
                }

                tag @ <form> => {
                    self.unexpected(&tag);
                    if !self.in_html_elem_named(atom!("template")) && self.form_elem.is_none() {
                        self.form_elem = Some(self.insert_and_pop_element_for(tag));
                    }
                    Done
                }

                EOFToken => self.step(InBody, token),

                token => {
                    self.unexpected(&token);
                    self.foster_parent_in_body(token)
                }
            }),

            //§ parsing-main-intabletext
            InTableText => match_token!(token {
                NullCharacterToken => self.unexpected(&token),

                CharacterTokens(split, text) => {
                    self.pending_table_text.push((split, text));
                    Done
                }

                token => {
                    let pending = replace(&mut self.pending_table_text, vec!());
                    let contains_nonspace = pending.iter().any(|&(split, ref text)| {
                        match split {
                            Whitespace => false,
                            NotWhitespace => true,
                            NotSplit => any_not_whitespace(text),
                        }
                    });

                    if contains_nonspace {
                        self.sink.parse_error(Borrowed("Non-space table text"));
                        for (split, text) in pending.into_iter() {
                            match self.foster_parent_in_body(CharacterTokens(split, text)) {
                                Done => (),
                                _ => panic!("not prepared to handle this!"),
                            }
                        }
                    } else {
                        for (_, text) in pending.into_iter() {
                            self.append_text(text);
                        }
                    }

                    Reprocess(self.orig_mode.take().unwrap(), token)
                }
            }),

            //§ parsing-main-incaption
            InCaption => match_token!(token {
                tag @ <caption> <col> <colgroup> <tbody> <td> <tfoot>
                  <th> <thead> <tr> </table> </caption> => {
                    if self.in_scope_named(table_scope, atom!("caption")) {
                        self.generate_implied_end(cursory_implied_end);
                        self.expect_to_close(atom!("caption"));
                        self.clear_active_formatting_to_marker();
                        match tag {
                            Tag { kind: EndTag, name: atom!("caption"), .. } => {
                                self.mode = InTable;
                                Done
                            }
                            _ => Reprocess(InTable, TagToken(tag))
                        }
                    } else {
                        self.unexpected(&tag);
                        Done
                    }
                }

                </body> </col> </colgroup> </html> </tbody>
                  </td> </tfoot> </th> </thead> </tr> => self.unexpected(&token),

                token => self.step(InBody, token),
            }),

            //§ parsing-main-incolgroup
            InColumnGroup => match_token!(token {
                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, text) => self.append_text(text),
                CommentToken(text) => self.append_comment(text),

                <html> => self.step(InBody, token),

                tag @ <col> => {
                    self.insert_and_pop_element_for(tag);
                    DoneAckSelfClosing
                }

                </colgroup> => {
                    if self.current_node_named(atom!("colgroup")) {
                        self.pop();
                        self.mode = InTable;
                    } else {
                        self.unexpected(&token);
                    }
                    Done
                }

                </col> => self.unexpected(&token),

                <template> </template> => self.step(InHead, token),

                EOFToken => self.step(InBody, token),

                token => {
                    if self.current_node_named(atom!("colgroup")) {
                        self.pop();
                        Reprocess(InTable, token)
                    } else {
                        self.unexpected(&token)
                    }
                }
            }),

            //§ parsing-main-intbody
            InTableBody => match_token!(token {
                tag @ <tr> => {
                    self.pop_until_current(table_body_context);
                    self.insert_element_for(tag);
                    self.mode = InRow;
                    Done
                }

                <th> <td> => {
                    self.unexpected(&token);
                    self.pop_until_current(table_body_context);
                    self.insert_phantom(atom!("tr"));
                    Reprocess(InRow, token)
                }

                tag @ </tbody> </tfoot> </thead> => {
                    if self.in_scope_named(table_scope, tag.name.clone()) {
                        self.pop_until_current(table_body_context);
                        self.pop();
                        self.mode = InTable;
                    } else {
                        self.unexpected(&tag);
                    }
                    Done
                }

                <caption> <col> <colgroup> <tbody> <tfoot> <thead> </table> => {
                    declare_tag_set!(table_outer = "table" "tbody" "tfoot");
                    if self.in_scope(table_scope, |e| self.elem_in(e, table_outer)) {
                        self.pop_until_current(table_body_context);
                        self.pop();
                        Reprocess(InTable, token)
                    } else {
                        self.unexpected(&token)
                    }
                }

                </body> </caption> </col> </colgroup> </html> </td> </th> </tr>
                    => self.unexpected(&token),

                token => self.step(InTable, token),
            }),

            //§ parsing-main-intr
            InRow => match_token!(token {
                tag @ <th> <td> => {
                    self.pop_until_current(table_row_context);
                    self.insert_element_for(tag);
                    self.mode = InCell;
                    self.active_formatting.push(Marker);
                    Done
                }

                </tr> => {
                    if self.in_scope_named(table_scope, atom!("tr")) {
                        self.pop_until_current(table_row_context);
                        let node = self.pop();
                        self.assert_named(node, atom!("tr"));
                        self.mode = InTableBody;
                    } else {
                        self.unexpected(&token);
                    }
                    Done
                }

                <caption> <col> <colgroup> <tbody> <tfoot> <thead> <tr> </table> => {
                    if self.in_scope_named(table_scope, atom!("tr")) {
                        self.pop_until_current(table_row_context);
                        let node = self.pop();
                        self.assert_named(node, atom!("tr"));
                        Reprocess(InTableBody, token)
                    } else {
                        self.unexpected(&token)
                    }
                }

                tag @ </tbody> </tfoot> </thead> => {
                    if self.in_scope_named(table_scope, tag.name.clone()) {
                        if self.in_scope_named(table_scope, atom!("tr")) {
                            self.pop_until_current(table_row_context);
                            let node = self.pop();
                            self.assert_named(node, atom!("tr"));
                            Reprocess(InTableBody, TagToken(tag))
                        } else {
                            Done
                        }
                    } else {
                        self.unexpected(&tag)
                    }
                }

                </body> </caption> </col> </colgroup> </html> </td> </th>
                    => self.unexpected(&token),

                token => self.step(InTable, token),
            }),

            //§ parsing-main-intd
            InCell => match_token!(token {
                tag @ </td> </th> => {
                    if self.in_scope_named(table_scope, tag.name.clone()) {
                        self.generate_implied_end(cursory_implied_end);
                        self.expect_to_close(tag.name);
                        self.clear_active_formatting_to_marker();
                        self.mode = InRow;
                    } else {
                        self.unexpected(&tag);
                    }
                    Done
                }

                <caption> <col> <colgroup> <tbody> <td> <tfoot> <th> <thead> <tr> => {
                    if self.in_scope(table_scope, |n| self.elem_in(n.clone(), td_th)) {
                        self.close_the_cell();
                        Reprocess(InRow, token)
                    } else {
                        self.unexpected(&token)
                    }
                }

                </body> </caption> </col> </colgroup> </html>
                    => self.unexpected(&token),

                tag @ </table> </tbody> </tfoot> </thead> </tr> => {
                    if self.in_scope_named(table_scope, tag.name.clone()) {
                        self.close_the_cell();
                        Reprocess(InRow, TagToken(tag))
                    } else {
                        self.unexpected(&tag)
                    }
                }

                token => self.step(InBody, token),
            }),

            //§ parsing-main-inselect
            InSelect => match_token!(token {
                NullCharacterToken => self.unexpected(&token),
                CharacterTokens(_, text) => self.append_text(text),
                CommentToken(text) => self.append_comment(text),

                <html> => self.step(InBody, token),

                tag @ <option> => {
                    if self.current_node_named(atom!("option")) {
                        self.pop();
                    }
                    self.insert_element_for(tag);
                    Done
                }

                tag @ <optgroup> => {
                    if self.current_node_named(atom!("option")) {
                        self.pop();
                    }
                    if self.current_node_named(atom!("optgroup")) {
                        self.pop();
                    }
                    self.insert_element_for(tag);
                    Done
                }

                </optgroup> => {
                    if self.open_elems.len() >= 2
                        && self.current_node_named(atom!("option"))
                        && self.html_elem_named(self.open_elems[self.open_elems.len() - 2].clone(),
                            atom!("optgroup")) {
                        self.pop();
                    }
                    if self.current_node_named(atom!("optgroup")) {
                        self.pop();
                    } else {
                        self.unexpected(&token);
                    }
                    Done
                }

                </option> => {
                    if self.current_node_named(atom!("option")) {
                        self.pop();
                    } else {
                        self.unexpected(&token);
                    }
                    Done
                }

                tag @ <select> </select> => {
                    let in_scope = self.in_scope_named(select_scope, atom!("select"));

                    if !in_scope || tag.kind == StartTag {
                        self.unexpected(&tag);
                    }

                    if in_scope {
                        self.pop_until_named(atom!("select"));
                        self.mode = self.reset_insertion_mode();
                    }
                    Done
                }

                <input> <keygen> <textarea> => {
                    self.unexpected(&token);
                    if self.in_scope_named(select_scope, atom!("select")) {
                        self.pop_until_named(atom!("select"));
                        Reprocess(self.reset_insertion_mode(), token)
                    } else {
                        Done
                    }
                }

                <script> <template> </template> => self.step(InHead, token),

                EOFToken => self.step(InBody, token),

                token => self.unexpected(&token),
            }),

            //§ parsing-main-inselectintable
            InSelectInTable => match_token!(token {
                <caption> <table> <tbody> <tfoot> <thead> <tr> <td> <th> => {
                    self.unexpected(&token);
                    self.pop_until_named(atom!("select"));
                    Reprocess(self.reset_insertion_mode(), token)
                }

                tag @ </caption> </table> </tbody> </tfoot> </thead> </tr> </td> </th> => {
                    self.unexpected(&tag);
                    if self.in_scope_named(table_scope, tag.name.clone()) {
                        self.pop_until_named(atom!("select"));
                        Reprocess(self.reset_insertion_mode(), TagToken(tag))
                    } else {
                        Done
                    }
                }

                token => self.step(InSelect, token),
            }),

            //§ parsing-main-intemplate
            InTemplate => match_token!(token {
                CharacterTokens(_, _) => self.step(InBody, token),
                CommentToken(_) => self.step(InBody, token),

                <base> <basefont> <bgsound> <link> <meta> <noframes> <script>
                <style> <template> <title> </template> => {
                    self.step(InHead, token)
                }

                <caption> <colgroup> <tbody> <tfoot> <thead> => {
                    self.template_modes.pop();
                    self.template_modes.push(InTable);
                    Reprocess(InTable, token)
                }

                <col> => {
                    self.template_modes.pop();
                    self.template_modes.push(InColumnGroup);
                    Reprocess(InColumnGroup, token)
                }

                <tr> => {
                    self.template_modes.pop();
                    self.template_modes.push(InTableBody);
                    Reprocess(InTableBody, token)
                }

                <td> <th> => {
                    self.template_modes.pop();
                    self.template_modes.push(InRow);
                    Reprocess(InRow, token)
                }

                EOFToken => {
                    if !self.in_html_elem_named(atom!("template")) {
                        self.stop_parsing()
                    } else {
                        self.unexpected(&token);
                        self.pop_until_named(atom!("template"));
                        self.clear_active_formatting_to_marker();
                        self.template_modes.pop();
                        self.mode = self.reset_insertion_mode();
                        Reprocess(self.reset_insertion_mode(), token)
                    }
                }

                tag @ <_> => {
                    self.template_modes.pop();
                    self.template_modes.push(InBody);
                    Reprocess(InBody, TagToken(tag))
                }

                token => self.unexpected(&token),
            }),

            //§ parsing-main-afterbody
            AfterBody => match_token!(token {
                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, _) => self.step(InBody, token),
                CommentToken(text) => self.append_comment_to_html(text),

                <html> => self.step(InBody, token),

                </html> => {
                    if self.is_fragment() {
                        self.unexpected(&token);
                    } else {
                        self.mode = AfterAfterBody;
                    }
                    Done
                }

                EOFToken => self.stop_parsing(),

                token => {
                    self.unexpected(&token);
                    Reprocess(InBody, token)
                }
            }),

            //§ parsing-main-inframeset
            InFrameset => match_token!(token {
                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, text) => self.append_text(text),
                CommentToken(text) => self.append_comment(text),

                <html> => self.step(InBody, token),

                tag @ <frameset> => {
                    self.insert_element_for(tag);
                    Done
                }

                </frameset> => {
                    if self.open_elems.len() == 1 {
                        self.unexpected(&token);
                    } else {
                        self.pop();
                        if !self.is_fragment() && !self.current_node_named(atom!("frameset")) {
                            self.mode = AfterFrameset;
                        }
                    }
                    Done
                }

                tag @ <frame> => {
                    self.insert_and_pop_element_for(tag);
                    DoneAckSelfClosing
                }

                <noframes> => self.step(InHead, token),

                EOFToken => {
                    if self.open_elems.len() != 1 {
                        self.unexpected(&token);
                    }
                    self.stop_parsing()
                }

                token => self.unexpected(&token),
            }),

            //§ parsing-main-afterframeset
            AfterFrameset => match_token!(token {
                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, text) => self.append_text(text),
                CommentToken(text) => self.append_comment(text),

                <html> => self.step(InBody, token),

                </html> => {
                    self.mode = AfterAfterFrameset;
                    Done
                }

                <noframes> => self.step(InHead, token),

                EOFToken => self.stop_parsing(),

                token => self.unexpected(&token),
            }),

            //§ the-after-after-body-insertion-mode
            AfterAfterBody => match_token!(token {
                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, _) => self.step(InBody, token),
                CommentToken(text) => self.append_comment_to_doc(text),

                <html> => self.step(InBody, token),

                EOFToken => self.stop_parsing(),

                token => {
                    self.unexpected(&token);
                    Reprocess(InBody, token)
                }
            }),

            //§ the-after-after-frameset-insertion-mode
            AfterAfterFrameset => match_token!(token {
                CharacterTokens(NotSplit, text) => SplitWhitespace(text),
                CharacterTokens(Whitespace, _) => self.step(InBody, token),
                CommentToken(text) => self.append_comment_to_doc(text),

                <html> => self.step(InBody, token),

                EOFToken => self.stop_parsing(),

                <noframes> => self.step(InHead, token),

                token => self.unexpected(&token),
            }),
            //§ END
        }
    }

    fn step_foreign(&mut self, token: Token) -> ProcessResult {
        match_token!(token {
            NullCharacterToken => {
                self.unexpected(&token);
                self.append_text("\u{fffd}".to_tendril())
            }

            CharacterTokens(_, text) => {
                if any_not_whitespace(&text) {
                    self.frameset_ok = false;
                }
                self.append_text(text)
            }

            CommentToken(text) => self.append_comment(text),

            tag @ <b> <big> <blockquote> <body> <br> <center> <code> <dd> <div> <dl>
                <dt> <em> <embed> <h1> <h2> <h3> <h4> <h5> <h6> <head> <hr> <i>
                <img> <li> <listing> <menu> <meta> <nobr> <ol> <p> <pre> <ruby>
                <s> <small> <span> <strong> <strike> <sub> <sup> <table> <tt>
                <u> <ul> <var> => self.unexpected_start_tag_in_foreign_content(tag),

            tag @ <font> => {
                let unexpected = tag.attrs.iter().any(|attr| {
                    matches!(attr.name,
                             qualname!("", "color") | qualname!("", "face") | qualname!("", "size"))
                });
                if unexpected {
                    self.unexpected_start_tag_in_foreign_content(tag)
                } else {
                    self.foreign_start_tag(tag)
                }
            }

            tag @ <_> => self.foreign_start_tag(tag),

            // FIXME(#118): </script> in SVG

            tag @ </_> => {
                let mut first = true;
                let mut stack_idx = self.open_elems.len() - 1;
                loop {
                    if stack_idx == 0 {
                        return Done;
                    }

                    let node = self.open_elems[stack_idx].clone();
                    let node_name = self.sink.elem_name(node);
                    if !first && node_name.ns == ns!(html) {
                        let mode = self.mode;
                        return self.step(mode, TagToken(tag));
                    }

                    if (&*node_name.local).eq_ignore_ascii_case(&*tag.name) {
                        self.open_elems.truncate(stack_idx);
                        return Done;
                    }

                    if first {
                        self.unexpected(&tag);
                        first = false;
                    }
                    stack_idx -= 1;
                }
            }

            // FIXME: This should be unreachable, but match_token! requires a
            // catch-all case.
            _ => panic!("impossible case in foreign content"),
        })
    }
}
