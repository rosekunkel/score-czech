extern crate iron;

#[macro_use(router)]
extern crate router;
extern crate rand;

#[macro_use]
extern crate lazy_static;

use std::fmt;
use iron::prelude::*;
use iron::{status, headers};

#[derive(Debug)]
struct Czech<'a> {
    name: &'a str,
}

impl<'a> Czech<'a> {
    fn new(name: &str) -> Czech {
        Czech {
            name: name,
        }
    }
}

impl<'a> fmt::Display for Czech<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.name)
    }
}

impl<'a> rand::Rand for Czech<'a> {
    fn rand<R: rand::Rng>(rng: &mut R) -> Czech<'static> {
        lazy_static! {
            static ref NAMES: Vec<&'static str> =
                include_str!("../resources/czechs").lines().collect();
        }
        Czech::new(rng.choose(&NAMES).unwrap())
    }
}

fn main() {
    let router = router!(random: get "/random" => handle_random);

    fn handle_random(_: &mut Request) -> IronResult<Response> {
        let mut response =
            Response::with((status::Ok, rand::random::<Czech>().to_string()));
        response.headers.set(headers::ContentType::plaintext());
        Ok(response)
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
