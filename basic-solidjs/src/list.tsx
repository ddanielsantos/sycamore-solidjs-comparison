import { Component, createEffect, createSignal } from "solid-js";

type Post = {
  userId: number;
  id: number;
  title: string;
  body: string;
}

async function fetchPosts(): Promise<Post[]> {
  const response = await fetch("https://jsonplaceholder.typicode.com/posts");
  const posts: Post[] = await response.json()

  return posts
}

export const List: Component = () => {
  let [getPosts, setPosts] = createSignal([] as Post[])

  createEffect(async () => {
    setPosts(await fetchPosts())
  })


  return (
    <div
      class="List"
    >
      <h2>
        Posts:
      </h2>

      <div
        class="PostContainer"
      >
        {
          getPosts().map(post => {
            return (
              <div
                class="Post"
              >
                <h3>
                  {post.title}
                </h3>

                <p>
                  {post.body}
                </p>
              </div>
            )
          })
        }
      </div>

    </div>
  )
}