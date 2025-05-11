use serde_json::{Map, Value};

#[test]
fn main() {
    let mut map = Map::new();
    map.insert("name".into(), "Eve".into());
    map.insert("age".into(), 30.into());

    let person = Value::Object(map);
    println!("{}", person.to_string());
}
