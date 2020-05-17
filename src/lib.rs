extern crate iron;
use iron::prelude::*;
use iron::status;

#[macro_use]
extern crate helix;

ruby! {
    class IronRb {
        def run() {
            Iron::new(|_: &mut Request| {
                Ok(Response::with((status::Ok, "Hello world!")))
            }).http("localhost:3000");
        }
    }
}