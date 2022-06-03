use regex::Regex;
use serde_json::Value;

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.

extern crate serde_derive;

fn main() {
    let data = r#"
        {
            "<1-3>": "test",
            "<4-6>": {
                "<1-2>": "lol"
            },
            "haha": 1
        }"#;

    let mut input: Value = serde_json::from_str(data).unwrap();

    if !input.is_object() {
        panic!("only obj supported");
    }

    recurse_obj(&mut input);
}

fn recurse_obj(input_obj: &mut Value) {
    let obj = input_obj.as_object_mut().unwrap();
    let mut inserts: Vec<[i32; 2]> = Vec::new();
    for pair in obj.iter() {
        let key = pair.0;
        let value = pair.1;

        if let Some("<") = key.get(..1) {
            let re = Regex::new(r"^<(\d{1})-(\d{1})>$").unwrap();
            let nums = re.captures(key).expect("need format <x-y>");

            let from = nums
                .get(1)
                .map_or(1, |m| m.as_str().parse::<i32>().unwrap());
            let to = nums
                .get(2)
                .map_or(1, |m| m.as_str().parse::<i32>().unwrap());

            inserts.push([from, to]);
        }
    }
    println!("{:?}", inserts);
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    #[test]
    fn main() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data).unwrap();

        // Access parts of the data by indexing with square brackets.
        println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    }
}
