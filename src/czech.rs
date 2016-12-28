use std::fmt;
use rand::{Rand, Rng};
use fmt::Display;
use serde_json::Value;
use wiki_api::{Query, Client};

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
    let mut potential_czechs: Vec<String> = Vec::new();
    let mut query = Query::new();
    query
        .add_param("titles", "List of Czechs")
        .add_param("generator", "links")
        .add_param("gpllimit", "max")
        .add_param("prop", "templates")
        .add_param("tllimit", "max")
        .add_param("tltemplates", "Template:Infobox person")
        .add_flag("redirects");

    Client::new().query(&query, |data| {
        if let Some(pages) = data
            .pointer("/query/pages")
            .and_then(Value::as_array) {
            let people = pages.iter()
                .filter(|v| v.pointer("/templates").is_some())
                .map(|v| v.pointer("/title")
                     .and_then(Value::as_str)
                     .unwrap()
                     .to_string());
            potential_czechs.extend(people);
        }
    });

    potential_czechs
}
