# enum_downcast

[![GitHub](https://img.shields.io/badge/GitHub-ryo33/enum_downcast-222222)](https://github.com/ryo33/enum_downcast)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/enum_downcast)](https://crates.io/crates/enum_downcast)
[![docs.rs](https://img.shields.io/docsrs/enum_downcast)](https://docs.rs/enum_downcast)

Safe downcasting for enums

## Example

```rust
#[derive(EnumDowncast)]
enum Enum {
    Player(Player),
    Enemy(Enemy),
    Items { vec: Vec<Item> },
    #[enum_downcast(skip)]
    Other,
}

let container = Enum::Player(Player { name: "Ryo".to_string() });
let _player_ref: &Player = container.downcast_ref::<Player>().unwrap();
let _player_mut: &mut Player = container.downcast_mut::<Player>().unwrap();
let player: Player = container.downcast::<Player>().unwrap();
```

You need `derive` feature to use the derive macro.

You can see more examples in
[/examples](https://github.com/ryo33/enum_downcast/tree/main/examples).

## Features

- completely `#![no_std]`
- no unsafe code (like `std::mem::transmute`)
- no dynamic dispatch
- `#[enum_downcast(skip)]` to skip some variants (especially not downcastable
  ones)
- enums with type parameters (see
  [example](https://github.com/ryo33/enum_downcast/tree/main/examples/generics.rs))
- custom downcast implementation without derive macro (see
  [example](https://github.com/ryo33/enum_downcast/tree/main/examples/custom_impl.rs))
- partial custom downcast implementation for variants with
  `#[enum_downcast(skip)]` (see
  [example](https://github.com/ryo33/enum_downcast/tree/main/examples/partial_custom_impl.rs))
- compatible with other macros, such as `serde`, `strum`, and `enum_dispatch`
  (see
  [example with serde and strum](https://github.com/ryo33/enum_downcast/tree/main/examples/other_derives.rs)
  and
  [example with enum_dispatch](https://github.com/ryo33/enum_downcast/tree/main/examples/enum_dispatch.rs))

There is one limitation: you cannot compile a code that downcasts an enum to any
type not listed in the enum definition, because of the lack of the
[specialization](https://github.com/rust-lang/rust/issues/31844) in stable Rust.
You can workaround this by using nightly compiler with `min_specialization`
feature like
[this example](https://github.com/ryo33/enum_downcast/tree/main/examples/specialization.rs).
As of specialization stabilized in the future, this limitation will be cleared,
and you don't need any boilerplate code like the example.
