use std::collections::HashMap;

fn main() {
    let mut map = HashMap::<i32, i32>::new();

    let e = map.entry(1).or_insert(0);
    *e = 10;
    // let k = map.entry(2).or_insert(0);
    // *k = 11;
    // println!("{} {}", *e, *k);
}