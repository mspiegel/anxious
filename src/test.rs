#[cfg(test)]
macro_rules! anxious_test_binop {
    ($anxious_int_func:ident, $int_func:ident, $op:tt) => {
        #[test]
        fn $anxious_int_func() {
            for lhs in i8::MIN..=i8::MAX {
                for rhs in i8::MIN..=i8::MAX {
                    _ = I8::from(lhs) $op I8::from(rhs);
                }
                _ = I8::from(Panic::ThisIsFine) $op I8::from(lhs);
                _ = I8::from(lhs) $op I8::from(Panic::ThisIsFine);
            }
            _ = I8::from(Panic::ThisIsFine) $op I8::from(Panic::ThisIsFine);
        }

        #[test]
        fn $int_func() {
            let mut panics = false;
            for lhs in i8::MIN..=i8::MAX {
                for rhs in i8::MIN..=i8::MAX {
                    panics |= std::panic::catch_unwind(||
                        { _ = lhs $op rhs;}).is_err();
                }
            }
            assert!(panics);
        }
    }
}

#[cfg(test)]
macro_rules! anxious_test_binfun {
    ($anxious_int_func:ident, $int_func:ident, $fun:ident) => {
        #[test]
        fn $anxious_int_func() {
            for lhs in i8::MIN..=i8::MAX {
                for rhs in i8::MIN..=i8::MAX {
                    _ = I8::from(lhs).$fun(I8::from(rhs));
                }
                _ = I8::from(Panic::ThisIsFine).$fun(I8::from(lhs));
                _ = I8::from(lhs).$fun(I8::from(Panic::ThisIsFine));
            }
            _ = I8::from(Panic::ThisIsFine).$fun(I8::from(Panic::ThisIsFine));
        }

        #[test]
        fn $int_func() {
            let mut panics = false;
            for lhs in i8::MIN..=i8::MAX {
                for rhs in i8::MIN..=i8::MAX {
                    panics |= std::panic::catch_unwind(|| {
                        _ = lhs.$fun(rhs);
                    })
                    .is_err();
                }
            }
            assert!(panics);
        }
    };
}

#[cfg(test)]
macro_rules! anxious_test_unaryfun {
    ($anxious_int_func:ident, $int_func:ident, $fun:ident) => {
        #[test]
        fn $anxious_int_func() {
            for val in i8::MIN..=i8::MAX {
                _ = I8::from(val).$fun();
            }
            _ = I8::from(Panic::ThisIsFine).$fun();
        }

        #[test]
        fn $int_func() {
            let mut panics = false;
            for val in i8::MIN..=i8::MAX {
                panics |= std::panic::catch_unwind(|| {
                    _ = val.$fun();
                })
                .is_err();
            }
            assert!(panics);
        }
    };
}
