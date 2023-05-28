/* #region Imports */
// 3rd Party
use serde_json::Value;
/* #endregion */

pub fn run(root: &Value, value_to_find: &Value) -> bool {
    match root {
        Value::Array(array) => {
            for value in array {
                if run(value, value_to_find) {
                    return true;
                }
            }
        }
        Value::Object(map) => {
            for (key, value) in map {
                if key == value_to_find || run(value, value_to_find) {
                    return true;
                }
            }
        }
        other => {
            if other == value_to_find {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    /* #region Imports */
    // 3rd Party
    use once_cell::sync::Lazy;
    use serde_json::{ Value, json };

    // Project
    use super::*;
    /* #endregion */

    static  MESSLY_JSON: Lazy<Value> = Lazy::new(|| json!({
        "a": {
            "b": [
                0,
                0.5,
                "shimi"
            ],
            "c": [
                null
            ]
        },
        "d": [
            [
                1,
                "hey"
            ],
            [
                "lol",
                "lol"
            ]
        ],
        "e": {
            "f": {
                "g": 2
            },
            "h": [
                3,
                true
            ]
        }
    }));

    fn expect_to_find(value_to_find: Value) {
        assert!(run(&MESSLY_JSON, &value_to_find), "Expected to find: {}", value_to_find)
    }

    #[test]
    fn should_find() {
        for letter in 'a'..='h' {
            expect_to_find(json!(letter))
        }

        for number in 0..=3 {
            expect_to_find(json!(number))
        }

        expect_to_find(json!(true));
        expect_to_find(json!(0.5));
        expect_to_find(json!("shimi"));
        expect_to_find(json!("hey"));
        expect_to_find(json!("lol"));
    }

    #[test]
    #[should_panic]
    fn should_not_find_1() {
        expect_to_find(json!(false));
    }

    #[test]
    #[should_panic]
    fn should_not_find_2() {
        expect_to_find(json!(4));
    }

    #[test]
    #[should_panic]
    fn should_not_find_3() {
        expect_to_find(json!(1.5));
    }

    #[test]
    #[should_panic]
    fn should_not_find_4() {
        expect_to_find(json!("i"));
    }

    #[test]
    #[should_panic]
    fn should_not_find_5() {
        expect_to_find(json!("Hello"));
    }
}
