extern crate futures;

mod boxfuture;

use boxfuture::{BoxFuture, Boxable};
use futures::future::{self, Future};

fn main() {
    let state = match true {
        true => future::lazy(|| future::ok("foo".to_string()).to_boxed()).to_boxed(),
        false => future::err("bar".to_string()).to_boxed(), //as BoxFuture<String, String>,
    };
    println!("{:?}", state.wait());
}
