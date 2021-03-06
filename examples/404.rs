#![feature(globs)]
extern crate iron;

use std::io::net::ip::Ipv4Addr;

use iron::prelude::*;
use iron::response::modifiers::Status;
use iron::status;

fn fourzerofour(_: &mut Request) -> IronResult<Response> {
    Ok(Response::new()
           .set(Status(status::NotFound)))
}

fn main() {
    Iron::new(fourzerofour).listen(Ipv4Addr(127, 0, 0, 1), 3000).unwrap();
    println!("On 3000");
}

