extern crate creusot_contracts;

use creusot_contracts::std::*;
use creusot_contracts::*;

#[requires(@i < (@a).len())]
fn read_write<T: Eq + Copy + Model>(a: &mut Vec<T>, i: usize, x: T) {
    a[i] = x; // a is slice
    assert!(a[i] == x);
}
