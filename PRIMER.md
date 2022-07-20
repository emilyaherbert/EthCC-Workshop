# Primer on Sway

## The development environment

[`forc`](https://fuellabs.github.io/sway/v0.18.1/introduction/sway-toolchain.html) is the Sway project and dependency management tool. It is the primary entrypoint for creating, building, testing, and deploying Sway projects.

To create a new project:

```bash
$ forc init --contract
$ tree .
.
├── Cargo.toml
├── Forc.toml
├── src
│   └── main.sw
└── tests
    └── harness.rs
```

Every `forc` project has 4 main components: the `Forc.toml` file, the `src/` directory, the `tests/` directory, and the `Cargo.toml` file.

The `Forc.toml` file is where you will defined important info about your project in order for `forc` to do it's things---this is where you will name the project, define dependencies, and more.

The `src/` directory is where all of your Sway code will go. In particular, the `src/main.sw` file is the entry point to every Sway project.

The `tests/` directory is where all of the tests for the project will live. `forc` uses `cargo` under the hood (hence the `Cargo.toml` file), which allows it to take advantage of Rust's existing testing infrastructure. The tests inside of the `tests/` directory are written in Rust and use the [Fuel Rust SDK](https://github.com/FuelLabs/fuels-rs).

To build a project:

```bash
$ forc build
  Compiled library "core".
  Compiled library "std".
  Compiled contract "tmp".
  Bytecode size is 68 bytes.
```

And to run the tests:

```bash
$ forc test
running 1 test
test can_get_contract_id ... ok
```

`forc` has lots of other awesome features (try `forc --help`, but for the purposes of this workshop these ones above are the only ones that we will be discussing).

## Setting up your contract

### The ABI

Contract ABI's in Sway are defined using the `abi` keyword:

```rust
abi SimpleAuction {
    fn bid();
}
```

then, ABI's are `impl`emented for `Contract'`s:

```rust
impl SimpleAuction for Contract {
    fn bid() {
        // where things happen
    }
}
```

### Contract Storage

Contract storage is defined using the `storage` keyword:

```rust
storage {
    my_data: u64 = 0
}
```

storage is accessed using the `storage.` syntax, and functions that access storage are annotated with corresponding annotations:

```rust
#[storage(read, write)]
fn update_data(new_data: u64) -> u64 {
    let old_data = storage.my_data;
    storage.my_data = new_data;
    old_data
}
```

Sway has special data structures that allow you use store lots of stuff in contract storage. Let's look at `StorageMap`.

`StorageMap` allows you to create a mapping in contract storage:

```rust
storage {
    my_map: StorageMap<u64, bool> = StorageMap {}
}

#[storage(write)]
fn insert_number(n: u64, on_or_off: bool) {
    storage.my_map.insert(n, on_or_off);
}

#[storage(read)]
fn get_from_number(n: u64) -> bool {
    storage.my_map.get(n)
}
```

## Standard Library

The Sway standard library provides data structures and functions that help with smart contract development, some of these include:

- `ContractId`: ID of a contract. Wraps a b256.
- `Address`: a wallet address. Wraps a b256.
- `Identity`: A wrapper type with 2 variants, `Address` and `ContractId`, that allows for handling interactions with contracts and addresses in a unified manner.
- `msg_sender()`: gets the `Identity` of the contract caller.
- `msg_asset_id()`: gets the ID of the asset forwarded with the call.
- `msg_amount()`: gets the amount of funds forwarded with the call.
- `this_balance(asset_id: ContractId)`: gets the current contract's balance of coin asset_id.
- `transfer(amount: u64, asset_id: ContractId, to: Identity)`: transfers the `amount` of coins matching the `asset_id` to the `to` `Identity`

## Writing Code

### Variable Declarations

Variables can be declared with `let` bindings:

```rust
let x = 1;
```

Mutable variables can be declared using the `mut` keyword:

```rust
let mut x = 1;
```

Type ascriptions can be added to denote type information:

```rust
let x: u64 = 1;
```

### Functions

Functions in Sway have the general form of:

```rust
fn function_name(param_1: u64, param_2: bool) -> str[4] {
    // body of the function
    "done"
}
```

Generic functions can be created by:

```rust
fn generic_function<T, F>(param_1: T, param_2: F) -> T {
    // body of the function
    param_1
}
```

### Function Calls

Functions can be called by:

```rust
let foo = function_name(4, false);
```

and generic functions can be called by:

```rust
let foo = generic_function(4, false);
let bar = generic_function::<u64, bool>(4, false);
```

the `::<u64, bool>` is called the "turbofish" syntax. Whether or not the turbofish syntax is needed depends on if the Sway compiler is able to perform type inference.

### If Expressions

If expressions can be used as statements:

```rust
let foo = 11;
if foo > 10 {
    0
} else {
    1
}
```

or as expressions:

```rust
let foo = 11;
let bar = if foo > 10 {
    0
} else {
    1
};
```

### Match Expressions

Match Expressions can also be used as either statements:

```rust
let foo = 11;
match foo {
    10 => 0,
    _ => 1
}
```

or as expressions:

```rust
let foo = 11;
let bar = match foo {
    10 => 0,
    _ => 1
};
```

### Structs (with methods)

Structs and struct methods are created as such:

```rust
struct Foo {
    bar: u64,
    baz: bool
}

impl Foo {
    fn is_baz_true(self) -> bool {
        self.baz
    }
}
```

You can also have generic structs:

```rust
struct Data<T> {
    value: T
}

impl<T> Data<T> {
    fn new(value: T) -> Data<T> {
        Data {
            value: value
        }
    }
}
```

### Enums (sum types)

Enums are created by:

```rust
enum Option<T> {
    Some: T,
    None: ()
}
```

## Modules and Dependencies

Sway's module and dependency system is similar to that of Rust's, but instead uses the `dep` keyword. For the purposes of this workshop, we will not be diving into this component of Sway.
