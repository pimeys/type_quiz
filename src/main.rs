#[macro_use]
extern crate serde_derive;

use serde_json;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Model {
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug, Clone, Deserialize)]
struct Field {
    name: String,
    is_unique: bool,
}

#[derive(Debug)]
struct RecordFinder {
    field: Field,
    value: &'static str,
}

fn main() {
    let model: Model = serde_json::from_reader(File::open("./model.json").unwrap()).unwrap();

    let field = &model.fields[0];

    let finder = RecordFinder {
        field: field.clone(),
        value: "boo",
    };

    println!("{:#?}", finder)
}
