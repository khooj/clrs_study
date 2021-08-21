use std::collections::HashMap;
use std::convert::From;

fn check_hashmap_few_entry() {
    let mut map = HashMap::<i32, i32>::new();

    let e = map.entry(1).or_insert(0);
    *e = 10;
    // let k = map.entry(2).or_insert(0);
    // *k = 11;
    // println!("{} {}", *e, *k);
}

struct A;

impl From<A> for B {
    fn from(_: A) -> Self {
        B
    }
}

struct B;

impl From<B> for C {
    fn from(_: B) -> Self {
        C
    }
}

struct C;

fn coercion(t: &C) {
}

fn check_coercion_transitivity() {
    let a = A;
    // coercion(&a);
}

fn main() {
    check_hashmap_few_entry();
    check_coercion_transitivity();
}