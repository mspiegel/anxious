#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "nightly", feature(try_trait_v2))]

mod panic;
#[macro_use]
mod anxious_int;
#[macro_use]
mod anxious_bitwise_int;
#[macro_use]
mod nominal_int;
#[macro_use]
mod test;

use panic::Panic;

macro_rules! anxious_int_decl {
    ($SelfT:ident, $ActualT:ident) => {
        #[derive(Copy, Clone)]
        pub struct $SelfT(pub(crate) Result<$ActualT, Panic>);
    };
}

anxious_int_decl! {I8, i8}
anxious_int_decl! {I16, i16}
anxious_int_decl! {I32, i32}
anxious_int_decl! {I64, i64}
anxious_int_decl! {I128, i128}
anxious_int_decl! {ISize, isize}

anxious_int_impl! {I8, i8, anxious_i8, INom8}
anxious_int_impl! {I16, i16, anxious_i16, INom16}
anxious_int_impl! {I32, i32, anxious_i32, INom32}
anxious_int_impl! {I64, i64, anxious_i64, INom64}
anxious_int_impl! {I128, i128, anxious_i128, INom128}
anxious_int_impl! {ISize, isize, anxious_isize, INomSize}

macro_rules! nominal_int_decl {
    ($SelfT:ident, $ActualT:ident) => {
        #[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
        pub struct $SelfT(pub(crate) $ActualT);
    };
}

nominal_int_decl! {INom8, i8}
nominal_int_decl! {INom16, i16}
nominal_int_decl! {INom32, i32}
nominal_int_decl! {INom64, i64}
nominal_int_decl! {INom128, i128}
nominal_int_decl! {INomSize, isize}

nominal_int_impl! {INom8, i8, nominal_i8, I8}
nominal_int_impl! {INom16, i16, nominal_i16, I16}
nominal_int_impl! {INom32, i32, nominal_i32, I32}
nominal_int_impl! {INom64, i64, nominal_i64, I64}
nominal_int_impl! {INom128, i128, nominal_i128, I128}
nominal_int_impl! {INomSize, isize, nominal_isize, ISize}

anxious_int_decl! {IBit8, i8}
anxious_int_decl! {IBit16, i16}
anxious_int_decl! {IBit32, i32}
anxious_int_decl! {IBit64, i64}
anxious_int_decl! {IBit128, i128}
anxious_int_decl! {IBitSize, isize}

anxious_bitwise_int_impl! {IBit8, i8, anxious_bitwise_i8}
anxious_bitwise_int_impl! {IBit16, i16, anxious_bitwise_i16}
anxious_bitwise_int_impl! {IBit32, i32, anxious_bitwise_i32}
anxious_bitwise_int_impl! {IBit64, i64, anxious_bitwise_i64}
anxious_bitwise_int_impl! {IBit128, i128, anxious_bitwise_i128}
anxious_bitwise_int_impl! {IBitSize, isize, anxious_bitwise_isize}

#[cfg(test)]
mod total_operations {
    use crate::*;
    use std::ops::Neg;

    anxious_test_binop! {test_anxious_i8_add, test_i8_add, +}
    anxious_test_binop! {test_anxious_i8_sub, test_i8_sub, -}
    anxious_test_binop! {test_anxious_i8_mul, test_i8_mul, *}
    anxious_test_binop! {test_anxious_i8_div, test_i8_div, /}
    anxious_test_binop! {test_anxious_i8_rem, test_i8_rem, %}

    anxious_test_binfun! {test_anxious_i8_div_euclid, test_i8_div_eucvlid, div_euclid}

    anxious_test_unaryfun! {test_anxious_i8_neg, test_i8_neg, neg}
    anxious_test_unaryfun! {test_anxious_i8_abs, test_i8_abs, abs}
}
