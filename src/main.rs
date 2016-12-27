extern crate iron;

#[macro_use(router)]
extern crate router;
extern crate rand;

#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate hyper;
extern crate serde_json;

mod czech;

use std::fmt;
use iron::prelude::*;
use iron::{Url, status, headers, modifiers};
use czech::Czech;

fn main() {
    let router = router!(random: get "/random" => handle_random);

    fn handle_random(_: &mut Request) -> IronResult<Response> {
        let czech = rand::random::<Czech>();
        let url = Url::parse(&format!("https://en.wikipedia.org/wiki/{}",
                                     czech)).unwrap();
        let mut response =
            Response::with((status::SeeOther, modifiers::Redirect(url)));
        response.headers.set(headers::ContentType::plaintext());
        Ok(response)
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
