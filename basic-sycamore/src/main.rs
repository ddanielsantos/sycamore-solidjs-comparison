mod app;
use sycamore::prelude::*;

fn main() {
  sycamore::render(|cx| view! { cx, app::App() });
}
