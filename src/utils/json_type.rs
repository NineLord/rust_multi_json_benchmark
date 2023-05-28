/* #region Imports */
// 3rd Party
use serde_json::{ Value, Map, json };
use once_cell::sync::Lazy;

// Project
use super::randomizer;
/* #endregion */

/* #region Enums */
enum ValueNonLeafType {
    Array,
    Object,
}

#[derive(Copy, Clone)]
enum ValueLeafType {
    Null,
    Bool,
    Number,
    String,
}

const VARIANTS_VALUE_LEAF_TYPES: [ValueLeafType; 4] = [ValueLeafType::Null, ValueLeafType::Bool, ValueLeafType::Number, ValueLeafType::String];

#[allow(unused)]
enum ValueTypes {
    Leaf(ValueLeafType),
    NoneLeaf(ValueNonLeafType)
}
/* #endregion */

static ALPHABET: Lazy<Vec<char>>  = Lazy::new(|| "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz".chars().collect());

fn get_random_none_leaf_json_type() -> ValueNonLeafType {
    if rand::random() {
        ValueNonLeafType::Array
    } else {
        ValueNonLeafType::Object
    }
}

pub fn get_random_none_leaf_json() -> Value {
    match get_random_none_leaf_json_type() {
        ValueNonLeafType::Array => Value::Array(vec!()),
        ValueNonLeafType::Object => Value::Object(Map::new())
    }
}

fn get_random_leaf_json_type() -> ValueLeafType {
    *randomizer::get_random_value_from_array(&VARIANTS_VALUE_LEAF_TYPES)
}

pub fn get_random_leaf_json() -> Value {
    match get_random_leaf_json_type() {
        ValueLeafType::Null => Value::Null,
        ValueLeafType::Bool => Value::Bool(rand::random()),
        ValueLeafType::Number => json!(randomizer::get_random_number_in_range(-1_000_000_000.0..=1_000_000_000.0)),
        ValueLeafType::String => {
            let number_of_letters = randomizer::get_random_number_in_range(0..=32);
            let mut string_builder = String::with_capacity(number_of_letters);
            for _count in 0..number_of_letters {
                string_builder.push(*randomizer::get_random_value_from_array(&ALPHABET));
            }
            Value::String(string_builder)
        }
    }
}