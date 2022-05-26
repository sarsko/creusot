use crate as creusot_contracts;
use crate::std::default::DefaultSpec;
use creusot_contracts_proc::*;

extern_spec! {
    #[ensures(^dest == src)]
    #[ensures(result == *dest)]
    fn std::mem::replace<T>(dest: &mut T, src: T) -> T
}

extern_spec! {
    #[ensures(^x == *y)]
    #[ensures(^y == *x)]
    fn std::mem::swap<T>(x: &mut T, y: &mut T)
}

extern_spec! {
    #[ensures(result == *dest)]
    #[ensures(^dest == T::default_log())]
    fn std::mem::take<T: DefaultSpec>(dest: &mut T) -> T
}
