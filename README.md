# A tool to view every TODOs in the rust code

## Installation
```
$ cargo install cargo-todo
```

## Usage 
### cargo todo now support customizable regex
add all your customs regex in the ~/.cargo/todo_config file (will be created at launch)
</br>all regex are non case-sensitive </br>

you can add arguments to you TODOs
```
//todo implement 18-11-2001 5 getters
```
The supported arguments are : </br>
 * Priority : A number between 1 and 9
 * Deadline : A date format x-x-x or x/x/x 
 * Content : Every text other thant the previouses willbe considered as content

</br> run
```rust
$ cargo todo
src/main.rs line: 122 //todo 
Priority: 5
Deadline: 18-11-2001
implement getters
```

### Default supported regex
 * ^s*//s*todo\b (//todo)
 * ^s*//s*fix\b (//fix)
 * ^s*//s*fixme\b (//fixme)
## Legacy mode
### Can be used for legacy code base as it's support todo!() and unimplemented!()
### Will display every line with a supported token (listed below) and the inside of the macro
### /!\ Legacy mode is way slower the the default mode and lacks a lot of cool features
#### Example
code base
```rust
todo!("implement getters");
```
run
```rust
$ cargo todo --legacy
src/main.rs TODO Line  125 : implement getters
```
## Supported tokens
- //todo
- todo!()
- unimplemented!()
- fix


### /!\ WARNING
cargo todo will no longer use regex but only the default tokens listed above
