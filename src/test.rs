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

    let desired = serde_json::json!(
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

#[test]
fn prefix_suffix() {
    let data = r#"
    {
        "before_<2-3>_after": {
            "name#<88-90>": "John Doe"
        }
    }"#;

    let mut v: Value = serde_json::from_str(data).unwrap();

    recurse_obj(&mut v);

    let desired = serde_json::json!(
        {
            "before_2_after": {
                "name#88": "John Doe",
                "name#89": "John Doe",
                "name#90": "John Doe"
            },
            "before_3_after": {
                "name#88": "John Doe",
                "name#89": "John Doe",
                "name#90": "John Doe"
            }
        }
    );

    assert_eq!(format!("{}", &v), format!("{}", &desired));
}
