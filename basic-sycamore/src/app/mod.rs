mod list;
use sycamore::prelude::*;

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
  view! { cx,
    div {
      list::List()
    }
  }
}
