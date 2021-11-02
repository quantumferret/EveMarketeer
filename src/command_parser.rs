use std::collections::HashMap;

pub fn parse_price_args(args: &str) -> HashMap<&str, &str> {
    let mut split_args = args.split(" /");
    let mut result: HashMap<&str, &str> = HashMap::new();
    result.insert("item", split_args.next().unwrap_or(""));
    for arg in split_args {
        let (key, value) = arg.split_once(" ").unwrap_or_else(|| (arg, ""));
        result.insert(key, value);
    }
    result.entry("market").or_insert("jita");
    result
}
