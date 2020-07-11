# A tool to view every TODOs in the rust code

## Installation
```
$ cargo install cargo-todo
```

## Usage 
### Will display every line with a supported token (listed below)
#### Example
```rust
$ cargo todo
Line 39 : refactor
```
## Supported tokens
- //todo
- todo!()

## Regex
### cargo todo now support customizable regex
add all your regex in a ~/.cargo/todo_config file
run
```
$ cargo todo --regex
```
### /!\ WARNING
cargo todo will no longer use de defaults token but only the custom regex
