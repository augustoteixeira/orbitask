use crate::api::codes::FormContainer;
use maud::{html, Markup};
use rocket::request::FlashMessage;
use rocket::response::{Responder, Result};
use rocket::uri;
use rocket::Request;
use std::collections::HashMap;

use crate::api::codes::rocket_uri_macro_edit_code_submit;
use crate::api::notes::rocket_uri_macro_delete_attribute_submit;
use crate::api::notes::rocket_uri_macro_edit_note_submit;
use crate::api::notes::rocket_uri_macro_update_or_add_attribute_submit;
use crate::db_manage::codes::Code;
use crate::db_manage::Note;
use crate::frontend::codes::rocket_uri_macro_edit_code;
use crate::frontend::codes::rocket_uri_macro_list_codes;
use crate::frontend::notes::rocket_uri_macro_root_notes;
use crate::frontend::notes::rocket_uri_macro_show_note;

use super::render::render_note;

#[derive(Debug, Clone)]
pub enum MyFlashType {
    Success,
    Warning,
    Error,
    Info,
}

#[derive(Debug, Clone)]
pub struct MyFlash {
    pub flash_type: MyFlashType,
    pub message: Markup,
}

impl<'a> From<FlashMessage<'a>> for MyFlash {
    fn from(flash: FlashMessage<'a>) -> Self {
        let flash_type = match flash.kind() {
            "success" => MyFlashType::Success,
            "warning" => MyFlashType::Warning,
            "error" => MyFlashType::Error,
            _ => MyFlashType::Info,
        };
        MyFlash {
            flash_type,
            message: html! { (flash.message()) },
        }
    }
}

#[derive(Debug)]
pub enum ViewState {
    Login,
    Root(Vec<Note>),
    Note(
        Note,
        Vec<(String, String)>,
        HashMap<String, FormContainer>,
        Vec<Note>,
        Vec<(i64, String)>,
        Vec<String>,
    ),
    NoteNew(Vec<String>, Option<i64>),
    NoteEdit(i64, Note, Vec<String>, Vec<(String, String)>),
    NoteConfirmDelete(i64, String),
    Code(Code, Option<String>),
    CodeList(Vec<String>, Option<String>),
    CodeNew(),
    CodeEdit(Code, Option<String>),
}

#[derive(Debug)]
pub struct View {
    pub state: ViewState,
    pub flash: Vec<MyFlash>,
}

pub fn footer() -> Markup {
    html! {
      footer {
      small {
        a href="https://github.com/augustoteixeira/orbitasks" {
        "Source code"
        }
      }
      }
    }
}

fn render_flashes(flashes: Vec<MyFlash>) -> Markup {
    html! {
      @for flash in flashes {
          @match flash.flash_type {
              MyFlashType::Success => { p class="flash-success" { (flash.message) } },
              MyFlashType::Warning => { p class="flash-warning" { (flash.message) } },
              MyFlashType::Info => { p class="flash-info" { (flash.message) } },
              MyFlashType::Error => { p class="flash-error" { (flash.message) } },
          }
      }
    }
}

fn login() -> Markup {
    html! {
      link rel="stylesheet" href="/static/style.css";

      main.container {
        article.grid {
          div {
            hgroup {
              h1 { "Sign in" }
            }
            form method="post" action="/login?next=/" {
              input type="password" name="password" placeholder="Password"
                aria-label="Password" autocomplete="current-password"
                required;

              fieldset {
                label for="remember" {
                  input type="checkbox" role="switch"
                    id="remember" name="remember";
                  "Remember me (not implemented yet)"
                }
              }

              button type="submit" class="contrast" { "Login" }
            }
          }
        }
      }
      (footer())
    }
}

fn render_confirm_delete(id: i64, title: &String) -> Markup {
    html! {
        main class="container" {
            h1 { "Confirm Delete Note" }
            p { "Are you sure you want to delete the note: " (title) "?" }
            form method="post" action=(uri!(crate::api::notes::delete_note_submit(id))) {
                button type="submit" { "Yes, delete" }
            }
            a href=(uri!(show_note(id))) { "Cancel" }
        }
    }
}

fn root(notes: Vec<Note>) -> Markup {
    html! {
      main {
        section class="main" {
            div class="note-header" {h2 { "Home" }}
          (render_notes_grid(&notes))
          a href="/notes/new" role="button" { "Create New Root Note" }
        }
      }
    }
}

pub struct Page {
    pub title: Markup,
    pub main: Markup,
    pub flash: Markup,
}

pub fn header() -> maud::Markup {
    html! {
    header {
        div class="header-left" {
          h1 { "Orbitask" }
        }
        div class="header-right" {
          a href={(uri!(list_codes()))} role="button" {
            "Codes"
          }
          a href={(uri!(root_notes()))} role="button" {
            "Notes"
          }
          form method="post" action="/logout" {
            button type="submit" { "Logout" }
          }
        }
      }
    }
}

pub fn render(page: Page) -> Markup {
    html! {
      (maud::DOCTYPE)
      html lang="en" {
        head {
          meta charset="utf-8";
          meta name="viewport" content="width=device-width, initial-scale=1";

          (page.title)

          link rel="stylesheet" href="/static/style.css";
          link rel="icon" type="image/x-icon" href="/static/favicon.png";
        }

        body {
          main {
            (header())
            (page.flash)
            div class="page-main" { (page.main) }
            (footer())
          }
        }
      }
    }
}

pub fn render_notes_grid(notes: &Vec<Note>) -> Markup {
    html! {
      section class="note-grid" {
        @for note in notes {
          article class="note-article" {
              a href={(format!("/notes/{}", note.id))} { // TODO: use uri
                (note.title)
              }
              @if let Some(code) = &note.code_name { p class="badge" { (code) } }
          }
        }
      }
    }
}

fn render_code(code: Code, next: Option<String>) -> Markup {
    html! {
        main class="container" {
            h1 { "Code Details" }
            div class="attribute-container" {
              p class="badge" { (code.name.clone()) }
            }
            h2 { "Capabilities" }
            code { (code.capabilities.clone()) }
            h2 { "Script" }
            pre { code { (code.script.clone()) } }
            nav style="margin-top: 1rem" {
                a href=(uri!(edit_code(name = code.name.clone(), next = next)
                )) role="button" {
                    "Edit Code"
                }
            }
        }
    }
}

pub fn render_new_code() -> Markup {
    html! {
      main class="container" {
        h1 { "Create New Code" }

        form method="post" action="/codes/new"
             class="edit-code-form" {
          label for="name" { "Label (example: mark_done)" }
          input type="name" id="name" name="name" required;

          label for="capabilities" {
              r#"Capabilities (example: ["SysLog", { "GetAttribute": "Own" } ]"#
          }
          input type="capabilities" id="capabilities"
                name="capabilities" required;

          label for="script" { "Script" }
          textarea id="script" name="script" rows="30" {};

          button type="submit" class="contrast" { "Create Code" }
        }
      }
    }
}

pub fn render_edit_code(code: &Code, next_url: Option<String>) -> Markup {
    html! {
      main class="container" {
        h1 { "Edit Code" }
        form method="post" class="edit-code-form"
             action=(uri!(edit_code_submit(next=next_url))) {
          input type="hidden" name="name" value=(code.name);
          label for="capabilities" {
              r#"Capabilities (example: ["SysLog", { "GetAttribute": "Own" } ])"#
          }
          input type="capabilities" id="capabilities"
                name="capabilities" required value=(code.capabilities);
          label for="script" { "Script" }
          textarea id="script" name="script" rows="50" {
              (code.script)
          }
          button type="submit" class="contrast" { "Update Code" }
        }
      }
    }
}

pub fn render_edit_note(
    id: i64,
    note: &Note,
    all_codes: &Vec<String>,
    attributes: &Vec<(String, String)>,
) -> Markup {
    html! {
        main class="container" {
          a href={(uri!(show_note(id)))} role="button" {
            "Back to note"
          }
          h1 { "Edit Note" }
          form method="post" action=(uri!(edit_note_submit(id)))
               class="edit-note-form" {
            label for="title" { "Title" }
            input type="text" id="title" name="title" required value=(note.title);

            label for="description" { "Description" }
            textarea id="description" name="description" {
                (note.description)
            }

            fieldset class="code-select" {
              legend { "Code" }

              label {
                input type="radio" name="code_name" value=""
                  checked[note.code_name.is_none()];
                " No code"
              }

              @for code in all_codes {
                label {
                  input type="radio" name="code_name" value=(code)
                    checked[note.code_name.as_ref() == Some(code)];
                  " " (code)
                }
              }
            }

          button type="submit" class="contrast" { "Save Changes" }
        }


        h3 { "Attributes" }

        div class="edit-note-form"{
          @for (key, value) in attributes {
            form method="post" action=(uri!(delete_attribute_submit(id, key)))
                 class="attribute-remover" {
              label class="badge" { (format!("{}: {}", key, value)) }
              button type="submit" name="remove_attribute" value=(key) { "Remove" }
            }
          }
        }

        form method="post" action=(uri!(update_or_add_attribute_submit(id)))
             class="edit-note-form" {
          label for="new_attr_key" { "New Attribute Key" }
          input type="text" id="new_attr_key" name="key" required;

          label for="new_attr_value" { "New Attribute Value" }
          input type="text" id="new_attr_value" name="value" required;

          button type="submit" { "Add Attribute" }
        }
      }
    }
}

pub fn render_list_codes(
    codes: &Vec<String>,
    no_note: &Option<String>,
) -> Markup {
    html! {
        main class="container" {
            h1 { "All Codes" }
            ul {
                @for name in codes {
                    li {
                        a href=(uri!(crate::frontend::codes::view_code(name=&name.clone(), note=no_note.clone()))) {
                            (name)
                        }
                    }
                }
            }
            nav style="margin-top: 1rem" {
                a href=(uri!(crate::frontend::codes::new_code)) role="button" {
                    "Create New Code"
                }
            }
        }
    }
}

pub fn render_new_note(codes: Vec<String>, parent_id: Option<i64>) -> Markup {
    html! {
      main class="container" {
        h1 { "Create New Note" }

        form method="post" action="/notes/new"
             class="new-note-form" {
          label for="title" { "Title" }
          input type="text" id="title" name="title" required;

          label for="description" { "Description (Markdown)" }
          textarea id="description" name="description" {};

          label for="code_name" { "Behavior (Code)" }
          fieldset class="code-select" {
            legend { "Code" }
            // Option for "no code"
            label {
              input type="radio" name="code_name" value="__none__" required checked;
              " No code"
            }
            // Options for each code name
            @for code in codes.iter() {
              label {
                input type="radio" name="code_name" value=(code);
                (code)
              }
            }
          }

          @if let Some(pid) = parent_id {
            input type="hidden" name="parent_id" value=(pid);
          }

          button type="submit" class="contrast" { "Create Notes" }
        }
      }
    }
}

impl<'r> Responder<'r, 'static> for View {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        let main = match self.state {
            ViewState::Login => login(),
            ViewState::Root(notes) => root(notes),
            ViewState::Note(
                note,
                attributes,
                forms,
                child_notes,
                ancestors,
                logs,
            ) => render_note(
                &note,
                &attributes,
                &forms,
                &child_notes,
                &ancestors,
                &logs,
            ),
            ViewState::NoteNew(codes, parent_id) => {
                render_new_note(codes, parent_id)
            }
            ViewState::NoteEdit(id, note, all_codes, attributes) => {
                render_edit_note(id, &note, &all_codes, &attributes)
            }
            ViewState::NoteConfirmDelete(id, title) => {
                render_confirm_delete(id, &title)
            }
            ViewState::Code(code, next) => render_code(code, next),
            ViewState::CodeList(codes, no_note) => {
                render_list_codes(&codes, &no_note)
            }
            ViewState::CodeNew() => render_new_code(),
            ViewState::CodeEdit(code, next) => render_edit_code(&code, next),
        };
        let rendered_flash = render_flashes(self.flash);
        let page = Page {
            title: html! {title {"Notes"}},
            flash: rendered_flash,
            main,
        };
        render(page).respond_to(req)
    }
}
