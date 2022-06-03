#![allow(non_snake_case)]

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::suspense::Suspense;

pub const API_URL: &str = "https://jsonplaceholder.typicode.com/posts";

#[derive(Prop, Serialize, Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Post {
  userId: Option<i32>,
  id: i32,
  title: String,
  body: String,
}

async fn fetchPosts() -> Result<Vec<Post>, reqwasm::Error> {
  let url = format!("{}/", API_URL);
  let resp = Request::get(&url).send().await?;
  let body: Vec<Post> = resp.json().await?;

  Ok(body)
}

#[component]
fn PostComponent<G: Html>(cx: Scope, props: Post) -> View<G> {
  view! {
      cx, div(class="Post", key=props.id) {
      h3 {
        (props.title)
      }
      p{
        (props.body)
      }
    }
  }
}

#[component]
pub async fn List<G: Html>(cx: Scope<'_>) -> View<G> {
  let posts = fetchPosts().await.unwrap_or_default();
  let posts = create_signal(cx, posts);

  view! { cx,
    div(class="List") {
      h2 {
        "Posts: "
      }
      div(class="PostContainer") {
        Indexed {
          iterable: posts,
          view: |cx, Post { body, id, title, .. }| view! { cx,
            Suspense{
              fallback: view! {cx, "Loading..."},
              PostComponent(Post { body, id, title, userId: None})
            }
          }
        }
      }
    }
  }
}
