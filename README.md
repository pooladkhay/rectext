# rectext

⚠️ Work-in-Progress

A Terminal UI Library that uses [Rectangles](./src/elements/rectangle.rs) and [Texts](./src/elements/text.rs) to create [UIElements](./src/traits.rs) and render them to the [Terminal](./src/terminal.rs).

The Terminal type is able to set `stdin` to [non-blocking](./src/terminal.rs#L84) mode so that the reads can be performed on the same thread as the main loop, without the need to spawn a new thread or use an async runtime like tokio.

If you are curious about the story and motivations behind the creation of this library, I have summarized them in this blog post:
[Programming is modeling - An experience report](https://pky.me/blog/programming-is-modeling/)
