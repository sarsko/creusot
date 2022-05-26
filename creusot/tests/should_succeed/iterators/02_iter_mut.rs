#![feature(slice_take)]
extern crate creusot_contracts;

use creusot_contracts::*;

mod common;
use common::*;

extern_spec! {
    #[ensures(match result {
        Some(r) => {
            * r == (@**s)[0] &&
            ^ r == (@^*s)[0] &&
            (@**s).len() > 0 && // ^*s.len == **s.len ? (i dont think so)
            (@^*s).len() > 0 &&
            (@*^s).ext_eq((@**s).tail()) && (@^^s).ext_eq((@^*s).tail())
        }
        None => ^s == * s && (@**s).len() == 0
    })]
    fn <[T]>::take_first_mut<'a, T>(s: &mut &'a mut [T]) -> Option<&'a mut T>
}

struct IterMut<'a, T> {
    inner: &'a mut [T],
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[predicate]
    fn completed(self) -> bool {
        pearlite! { (@self.inner).ext_eq(Seq::EMPTY) }
    }

    #[predicate]
    fn produces(self, visited: Seq<Self::Item>, tl: Self) -> bool {
        pearlite! {
            (@*self.inner).len() == visited.len() + (@*tl.inner).len() &&
            (@^self.inner).len() == visited.len() + (@^tl.inner).len() &&
            (@(*self.inner)).subsequence(visited.len(), (@*self.inner).len()).ext_eq(@*tl.inner) &&
            (@(^self.inner)).subsequence(visited.len(), (@^self.inner).len()).ext_eq(@^tl.inner )&&
            (forall<i : Int> 0 <= i && i < visited.len() ==>
                (@*self.inner)[i] == *visited[i] && (@^self.inner)[i] == ^visited[i])
        }
    }

    #[law]
    #[ensures(a.produces(Seq::EMPTY, a))]
    fn produces_refl(a: Self) {}

    #[law]
    #[requires(a.produces(ab, b))]
    #[requires(b.produces(bc, c))]
    #[ensures(a.produces(ab.concat(bc), c))]
    fn produces_trans(a: Self, ab: Seq<Self::Item>, b: Self, bc: Seq<Self::Item>, c: Self) {}

    #[ensures(match result {
      None => (*self).completed() && self.resolve(),
      Some(v) => (*self).produces(Seq::singleton(v), ^self) && !(*self).completed()
    })]
    fn next(&mut self) -> Option<Self::Item> {
        (self.inner).take_first_mut()
    }
}

// Commented until we can get the ghost code for `produced.push` to work.

// #[trusted]
// #[ensures(@*result.inner == @*v)]
// #[ensures(@^result.inner == @^v)]
// #[ensures((@^v).len() == (@v).len())]
// fn iter_mut<'a, T>(v: &'a mut Vec<T>) -> IterMut<'a, T> {
//     // IterMut { inner : &mut v[..] }
//     panic!()
// }

// #[ensures((@^v).len() == (@v).len())]
// #[ensures(forall<i : _> 0 <= i && i < (@v).len() ==> @(@^v)[i] == 0)]
// fn all_zero(v : &mut Vec<usize>) {
//     let mut it = iter_mut(v);
//     let it_old = Ghost::record(&it);
//     let mut produced = Seq::EMPTY;

//     #[invariant(structural, (@it_old).produces(produced, it))]
//     #[invariant(user, forall<i : Int> 0 <= i && i < produced.len() ==> @^ produced[i] == 0)]
//     loop {
//         match it.next() {
//             Some(x) => {
//                 // produced = produced.push(x);
//                 *x = 0;
//             }
//             None => break,
//         }
//     }
// }
