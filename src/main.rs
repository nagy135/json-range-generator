use std::io::stdin;
use std::io::BufRead;

use clap::Parser;

use regex::Regex;
use serde_json::{to_string_pretty, Value};

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod test;

/// Generates json from ranges in json keys
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
    head: String,
    tail: String,
}

fn main() {
    let args = Args::parse();

    let data = match args.input {
        None => {
            let lines: Vec<String> = stdin().lock().lines().map(|e| e.unwrap()).collect();
            lines.join("")
        }
        Some(input) => input,
    };

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

/// recursively mutates object, depth first
fn recurse_obj(input_obj: &mut Value) {
    let obj = input_obj.as_object_mut().unwrap();
    let mut replacements: Vec<Replacement> = Vec::new();
    let re = Regex::new(r"<\d+-\d+>").unwrap();

    for pair in obj.iter_mut() {
        let key = pair.0;
        let value = pair.1;

        if value.is_object() {
            recurse_obj(value);
        }

        match re.find(key) {
            Some(mat) => {
                let key_range_text = &key[mat.start() + 1..mat.end() - 1];
                let head = key[..mat.start()].to_string();
                let tail = key[mat.end()..].to_string();

                let mut pieces = key_range_text.split('-');

                let from = pieces.next().unwrap().parse::<i32>().unwrap();
                let to = pieces.next().unwrap().parse::<i32>().unwrap();

                replacements.push(Replacement {
                    from,
                    to,
                    key: key.clone(),
                    value: value.clone(),
                    head,
                    tail,
                });
            }
            None => {}
        };
    }
    for replacement in replacements.iter() {
        obj.remove(&replacement.key);
    }
    for Replacement {
        from,
        to,
        value,
        head,
        tail,
        ..
    } in replacements.iter()
    {
        for i in from.clone()..to.clone() + 1 {
            obj.insert(format!("{}{}{}", head, i, tail), value.clone());
        }
    }
}
