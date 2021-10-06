# Topos-frontend
Topos started of written in Haskell because the language allow us to construct program that the data structure determines the evaluation sequence, and indeed it made it harder to craft some wasm-based software.

We are taking the opportunity to explore using wasm as our compiler frontend, so it can be run on the browsers, providing a language playground as amenities.

### Why write in Rust?
We think it already has a mature ecosystem for wasm (WebAssembly) development. For example, [pest's website](pest.rs) used wasm to provide just-in-time PEG parser demonstration.

### What are we going to do next?
We will develope an interactive web IDE for people to test use Topos programming language and provide us feedbacks.
