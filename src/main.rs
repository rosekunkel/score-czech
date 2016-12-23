extern crate iron;

#[macro_use(router)]
extern crate router;

use iron::prelude::*;
use iron::status;

fn main() {
    let router = router!(random: get "/random" => handle_random);

    fn handle_random(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "A random Czech.")))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
