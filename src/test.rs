use serde_json::json;

#[cfg(test)]
use super::*;

#[test]
fn basic() {
    let data = r#"
    {
        "<2-3>": {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }
    }"#;

    let mut v: Value = serde_json::from_str(data).unwrap();

    recurse_obj(&mut v);

    let desired = json!(
        {
            "2": {
                "name": "John Doe",
                "age":43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ]
            },
            "3": {
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ]
            }
        }
    );

    assert_eq!(format!("{}", &v), format!("{}", &desired));
}
