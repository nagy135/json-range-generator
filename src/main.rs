extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.

use std::collections::HashMap;
#[macro_use]
extern crate serde_derive;

fn main() {
    let mut prices: HashMap<String, String> = HashMap::new();

    for o in 1..20 {
        prices.insert(o.to_string(), "20.0".to_string());
    }

    let serialized = serde_json::to_string_pretty(&prices).unwrap();

    println!("serialized = {}", serialized);
}
