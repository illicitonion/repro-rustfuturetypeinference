# Rust type inference issue

A match whose arms hop through a couple of trait implementations and type parameters before returning what should be unifiable/inferable types gives a type error because _ on one arm cannot be unified with the type returned by the other arm:

```
$ cargo build
   Compiling repro-rustfuturetypeinference v0.1.0 (file:///Users/dwagnerhall/src/github.com/illicitonion/repro-rustfuturetypeinference)
error[E0308]: match arms have incompatible types
  --> src/main.rs:9:17
   |
9  |       let state = match true {
   |  _________________^
10 | |         true => future::lazy(|| future::ok("foo".to_string()).to_boxed()).to_boxed(),
11 | |         false => future::err("bar".to_string()).to_boxed(), //as BoxFuture<String, String>,
12 | |     };
   | |_____^ expected struct `futures::Lazy`, found struct `futures::FutureResult`
   |
   = note: expected type `std::boxed::Box<futures::Lazy<[closure@src/main.rs:10:30: 10:73], std::boxed::Box<futures::FutureResult<std::string::String, _>>>>`
              found type `std::boxed::Box<futures::FutureResult<_, std::string::String>>`
note: match arm with an incompatible type
  --> src/main.rs:11:18
   |
11 |         false => future::err("bar".to_string()).to_boxed(), //as BoxFuture<String, String>,
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

error: Could not compile `repro-rustfuturetypeinference`.
```

If I cast result of one arm to match the type of the other arm, everything works fine:

```
$ sed -e 's#, //# #g' -i '' src/main.rs
$ cargo build
   Compiling repro-rustfuturetypeinference v0.1.0 (file:///Users/dwagnerhall/src/github.com/illicitonion/repro-rustfuturetypeinference)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63 secs
```

I shouldn't have to specify this type explicitly.
