extern crate iron;

#[macro_use(router)]
extern crate router;
extern crate rand;

use iron::prelude::*;
use iron::status;

fn main() {
    let router = router!(random: get "/random" => handle_random);

    fn handle_random(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, rand::random::<u32>().to_string())))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
