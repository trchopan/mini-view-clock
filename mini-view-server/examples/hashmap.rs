use std::collections::HashMap;

fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    for key in map.keys() {
        println!("{key}");
    }
}
