use std::fmt;
use hyper::{client, header};
use rand::{Rand, Rng};
use fmt::Display;
use serde_json;
use serde_json::Value;

#[derive(Debug)]
pub struct Czech<'a> {
    name: &'a str,
}

impl<'a> Czech<'a> {
    fn new(name: &str) -> Czech {
        Czech {
            name: name,
        }
    }
}

impl<'a> Display for Czech<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.name)
    }
}

impl<'a> Rand for Czech<'a> {
    fn rand<R: Rng>(rng: &mut R) -> Czech<'static> {
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
            .header(header::UserAgent("ScoreCzech/0.1.0 (will@wkunkel.com)".to_owned()))
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
