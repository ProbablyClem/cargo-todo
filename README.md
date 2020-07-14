# A tool to view every TODOs in the rust code

## Installation
```
$ cargo install cargo-todo
```

## Usage 


you can add parameters to you TODOs
```
//todo 18-11-2001 5 !clement implement getters
```
The supported parameters are : </br>
 * Priority : A number between 1 and 9
 * Deadline : A date format yyyy/mm/dd
 * Member : A text that begin with '!'
 * Content : Every text other thant the previouses will be considered as content

</br>Those parameters can be added in any order as long as they follow the syntax they will be automaticaly added

### run
```rust
$cargo todo
src/main.rs line: 331 //todo 
Member: clement
Priority: 5
Deadline: 2020-08-14
implement getters
```

### Default supported regex
 * ^s*//s*todo\b (//todo)
 * ^s*//s*fix\b (//fix)
 * ^s*//s*fixme\b (//fixme)
### cargo todo now support customizable regex
add all your customs regex in the ~/.cargo/todo_config file (will be created at launch)
</br>all regex are case-insensitive </br>
## Features
 * -i, --inline : display todo in one line
 ```rust
 $cargo todo -i
 src/main.rs line: 331 //todo  Member: clement Priority: 5 Deadline: 2020-08-14 implement getters
 ```
 * -v, --verbose : Sets the level of verbosity
 </br>default or -vv 
 </br> full verbose
 -v less verbose
 ```rust
 $cargo todo -v
 src/main.rs line: 331 //todo 
implement getters
```
 * -x, --exclude <exclude>...   : Exclude some todos from the list
 ```rust
 $cargo todo -x //fix
 //wil display every todos expect those having the '//fix' keyword
 ```
 * -f, --filter <filter>... : Filter todos to show
 ```rust
 $cargo todo -f //fix
 //wil only display todos having the '//fix' keyword
 ```
 * -l, --list <list> : Number of values to display
 ```rust
 $cargo todo -l 5
 ///wil display the first 5 todos
 ```
 * m, --member <member>... : Filter from member
 ```rust
 $cargo todo -m clement
 ///wil only display todos having as member clement
 ```
 * -s, --sort <sort> : Sort todos [possible values: priority, deadline, member]
 ```rust
 $cargo todo -s priority
 ///wil display todos sorted by their priority
 ```



## Legacy mode
### Can be used for legacy code base as it's support todo!() and unimplemented!()
### Will display every line with a supported token (listed below) and the inside of the macro
### /!\ Legacy mode is way slower the the default mode and lacks a lot of cool features
#### Example
code base
```rust
todo!("implement getters");
```
### run
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
