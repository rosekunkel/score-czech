extern crate iron;

#[macro_use(router)]
extern crate router;
extern crate rand;

#[macro_use]
extern crate lazy_static;
extern crate hyper;
extern crate serde_json;

use std::fmt;
use iron::prelude::*;
use iron::{Url, status, headers, modifiers};
use hyper::client;
use serde_json::Value;

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
            static ref NAMES: Vec<String> = get_potential_czechs();
        }
        Czech::new(rng.choose(&NAMES).unwrap())
    }
}

fn get_potential_czechs() -> Vec<String> {
    let client = client::Client::new();
    let mut potential_czechs: Vec<String> = Vec::new();
    let mut continue_string = Some("&continue=".to_owned());
    let base_query = "action=query\
                      &titles=List of Czechs\
                      &generator=links\
                      &gpllimit=max\
                      &prop=templates\
                      &tllimit=max\
                      &tltemplates=Template:Infobox person\
                      &format=json\
                      &formatversion=2\
                      &redirects";
    while continue_string.is_some() {
        let request_string = format!("https://en.wikipedia.org/w/api.php?{}{}",
                                     base_query, continue_string.unwrap());
        let response = client
            .get(&request_string)
            .header(headers::UserAgent("ScoreCzech/0.1.0 (will@wkunkel.com)".to_owned()))
            .send().unwrap();
        let data: Value = serde_json::from_reader(response).unwrap();
        let pages = data.pointer("/query/pages").and_then(Value::as_array);
        if pages.is_some() {
            let people = pages.unwrap().into_iter()
                .filter(|v| v.pointer("/templates").is_some())
                .map(|v| v.pointer("/title"))
                .map(|v| v.and_then(Value::as_str))
                .map(Option::unwrap)
                .map(Into::into);
            potential_czechs.extend(people);
        }

        continue_string = data
            .pointer("/continue")
            .and_then(Value::as_object)
            .map(|o| o.into_iter()
                 .map(|(k, v)| format!("&{}={}", k, v.as_str().unwrap()))
                 .fold(String::new(), |mut a, e| {
                     a.push_str(&e);
                     a
                 }));
    }
    potential_czechs
}

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
