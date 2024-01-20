#![allow(non_snake_case)]
use test_proc_macro::view;

fn Component(_a: bool) {}

struct DOM(fn(bool));

impl DOM {
    fn build(&self, _a: bool) {}
}

fn main() {
    view! {
        <  Component />
    };

    // Recursive expansion of view! macro
    // ===================================
    DOM(Component).build()
}
