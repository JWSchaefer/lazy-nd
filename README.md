# lazy-nd

<p align="center">
<img src="resources/logo.webp" width="160"/>
</p>

A procedural macro for lazily defined state arrays using ndarray.

> [!NOTE]
> This crate is a prototype. Functionality is not guaranteed and may change significantly.

## :rocket: Quickstart

```bash
cargo add --git https://github.com/JWSchaefer/lazy-nd
```

## :mag_right: Example

```rust
use lazy_nd::lazy_nd;
use ndarray::ArrayView2;

#[lazy_nd(dim = 3)]
struct State {
    #[vector( position : f64)]
    #[vector( velocity : f64)]
    #[vector( accaleration : f64)]
    #[scalar( mass : f64)]
    #[scalar( id : usize)]
    #[scalar( live : bool)]
    name : &str
}

fn main(){
    let state = State::new(15);
    let velocity : ArrayView2<f64> = state.velocity(); # 3 x 15
    let mass : ArrayViewMut2<f64> = state.mass_mut(); # 1 x 15
    let live : ArrayView2<bool> = state.live(); #3 x 15
}
```
