use maud::{html, Markup};

pub use crate::db_manage::Tag;

pub fn render_tags(tags: Vec<Tag>) -> Markup {
    html! {
      div style="display: flex; flex-wrap: wrap; gap: 0.4rem;" {
        @for tag in tags {
          a
            href={(format!("/tags/{}", tag.id))}
            class="secondary"
          {
            (tag.name)
          }
        }
      }
    }
}
