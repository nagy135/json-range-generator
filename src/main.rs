use clap::Parser;

use regex::Regex;
use serde_json::{to_string_pretty, Value};

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.

extern crate serde_derive;

const WRONG_RANGE_MESSAGE: &'static str = "use format <x-y>";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    input: Option<String>,

    #[clap(short, long)]
    pretty: bool,
}

struct Replacement {
    from: i32,
    to: i32,
    key: String,
    value: Value,
}

fn main() {
    let args = Args::parse();

    if args.input.is_none() {
        panic!("No input provided");
    }
    let data = args.input.unwrap();

    let mut input: Value = serde_json::from_str(&data).unwrap();

    if !input.is_object() {
        panic!("only obj supported");
    }

    recurse_obj(&mut input);

    if args.pretty {
        println!("{}", to_string_pretty(&input).unwrap());
    } else {
        println!("{}", &input);
    }
}

fn recurse_obj(input_obj: &mut Value) {
    let obj = input_obj.as_object_mut().unwrap();
    let mut replacements: Vec<Replacement> = Vec::new();
    for pair in obj.iter_mut() {
        let key = pair.0;
        let value = pair.1;

        if value.is_object() {
            recurse_obj(value);
        }

        if let Some("<") = key.get(..1) {
            let re = Regex::new(r"<\d+-\d+>").unwrap();
            let mat = re.find(key).expect(WRONG_RANGE_MESSAGE);

            let key_range_text = &key[mat.start() + 1..mat.end() - 1];

            let mut pieces = key_range_text.split('-');

            let from = pieces.next().unwrap();
            let to = pieces.next().unwrap();

            replacements.push(Replacement {
                from: from.parse::<i32>().unwrap(),
                to: to.parse::<i32>().unwrap(),
                key: key.clone(),
                value: value.clone(),
            });
        }
    }
    for replacement in replacements.iter() {
        obj.remove(&replacement.key);
    }
    for replacement in replacements.iter() {
        let from = replacement.from;
        let to = replacement.to;
        for i in from..to + 1 {
            obj.insert(format!("{}", i), replacement.value.clone());
        }
    }
}

mod test;
