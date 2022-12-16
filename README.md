<div align="center">
    <h1>ohkami</h1>
</div>

ohkami *- [狼] means wolf in Japanese -* is **simple** and **non macro-based** web framework for Rust.

<br/>

## Features
- *simple*: Less things to learn / Less code to write / Less time to hesitate.
- *non macro-based*: No need for using macros.
- async handlers
- easy error handling

<br/>

## Quick start
1. Add dependencies:

```toml
[dependencies]
ohkami = "0.1"
```

2. Write your first code with ohkami:

```rust
use ohkami::prelude::*;

fn main() -> Result<()> {
    Server::setup()
        .GET("/", |_| async {Response::OK("Hello, world!")})
        .serve_on(":3000")
}
```

3. If you're interested in ohkami, learn more by [examples](https://github.com/kana-rus/ohkami/tree/main/examples) and documentations(**WIP**)!

<br/>

## Development
ohkami is on **very early stage** now and not for producntion use.

<br/>

## License
This project is under MIT LICENSE ([LICENSE-MIT](https://github.com/kana-rus/ohkami/blob/main/LICENSE-MIT) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)).
