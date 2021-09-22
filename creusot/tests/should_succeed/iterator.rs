// SHOULD_SUCCEED: parse-print
#![feature(register_tool, rustc_attrs)]
#![register_tool(creusot)]
#![feature(proc_macro_hygiene, stmt_expr_attributes)]

extern crate creusot_contracts;

use creusot_contracts::*;

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
impl<T> List<T> {
    logic! { fn snoc(self, v: T) -> List<T> {
      List::Nil
    }}
}

trait Iterator: Sized {
    type Item;

    logic! { fn visited(self) -> List<Self::Item>; }

    logic! { fn completed(self) -> bool; }

    #[requires(!(*self).completed())]
    #[ensures(match result {
      None => (^self).completed(),
      Some(v) => equal((^self).visited(), (*self).visited().snoc(v)) && !(^self).completed()
    })]
    fn next(&mut self) -> Option<Self::Item>;
}

#[requires(!it.completed())]
#[ensures(it.completed())]
fn sum<I: Iterator<Item = u32>>(it: I) {
    let mut x = 0;
    // Can't use the for sugar as it requires std iterators
    #[invariant(dummy, !it.completed())]
    while let Some(e) = it.next() {
        x += e;
    }
}
