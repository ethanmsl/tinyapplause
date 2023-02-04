# tinyapplause
CLAP learning project (Rust CLI App)

[CLAP crate](https://docs.rs/clap/latest/clap/index.html).


Notes:
CLAP v4 *no longer* supports color output. (neither by default nor specification)  
[it may, at some point, allowd color specification; but undetermined future and still won't be a default]  

Options:
 - CLAP v3 *does* support color (not the most aesthetic of choices, but it exists)
 - `bat` has a `bathelp` options (see my `hepbat` & `helpbat` shell functions in `.zshrc`) and a `batman` option
  - when these do work they work well (and provide better color parsing!) 
    - e.g. `helpbat cargo run --` will produce nice output
  - however getting them to work generally will require more work and working into binary is a tbd process
 - `PyO3` is apparently a very nice to work with Rust-Python FFI crate/package.  And `Typer` is an excellent (and much more ergonomic) CLI framework.  That may be a ridiculous, but timeworthy workaround to the under-developed (a surprise!) CLI ecosystem in rust.
  - Adding a binary dependency to python, while worth learning, may create significant distribution (multiple wheels?) and maintenance (who updates rust binaries if I'm not there) costs -- particularly if we don't have a private server to host the python packages and rust crates set-up 
