use hyper;
use serde_json;
use serde_json::Value;
use itertools::Itertools;
use std::fmt::Display;

pub struct Query {
    params: Vec<(String, String)>,
    flags: Vec<String>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            params: Vec::new(),
            flags: Vec::new(),
        }
    }

    pub fn add_param<S>(&mut self, key: S, value: S) -> &mut Query
        where S: Into<String> {
        self.params.push((key.into(), value.into()));
        self
    }

    pub fn add_flag<S>(&mut self, flag: S) -> &mut Query
        where S: Into<String> {
        self.flags.push(flag.into());
        self
    }

    fn to_query_string(&self) -> String {
        let formatted_params = self.params.iter()
            .format_with("&", |&(ref k, ref v), f|
                         f(&format_args!("{}={}", k, v)));
        let formatted_flags = self.flags.iter().format("&");
        let action = "action=query";
        let full_query: Vec<&Display> = vec![
            &action,
            &formatted_params,
            &formatted_flags,
        ];
        full_query.iter().format("&").to_string()
    }
}

pub struct Client {
    client: hyper::client::Client,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: hyper::client::Client::new(),
        }
    }

    pub fn query<F>(&self, request: &Query, mut handler: F)
        where F: FnMut(Value) {
        const API_LOCATION: &'static str = "https://en.wikipedia.org/w/api.php";
        const USER_AGENT: &'static str = "ScoreCzech/0.1.0 (will@wkunkel.com)";
        const FORMAT: &'static str = "format=json&formatversion=2";

        let base_query_string = request.to_query_string();
        let mut continue_params = Some("continue=".to_string());

        while let Some(continue_string) = continue_params {
            let request_string = format!("{}?{}&{}&{}",
                                         API_LOCATION,
                                         FORMAT,
                                         base_query_string,
                                         continue_string);
            let response = self.client
                .get(&request_string)
                .header(hyper::header::UserAgent(USER_AGENT.to_owned()))
                .send().unwrap();

            let data: Value = serde_json::from_reader(response).unwrap();
            continue_params = data
                .pointer("/continue")
                .and_then(Value::as_object)
                .map(|o| o.iter()
                     .format_with("&", |(ref k, ref v), f| {
                         let value_str = v.as_str().unwrap();
                         f(&format_args!("{}={}", k, value_str))
                     })
                     .to_string());
            handler(data);
        }
    }
}
