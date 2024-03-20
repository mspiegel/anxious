macro_rules! nominal_int_impl {
    ($SelfT:ident, $ActualT:ident, $module:ident, $AnxiousT:ident) => {
        mod $module {
            use crate::*;
            use core::fmt;
            use core::ops;

            impl From<$ActualT> for $SelfT {
                fn from(item: $ActualT) -> $SelfT {
                    $SelfT(item)
                }
            }

            impl $SelfT {
                pub const MAX: $SelfT = $SelfT($ActualT::MAX);

                pub const MIN: $SelfT = $SelfT($ActualT::MIN);

                #[inline]
                pub const fn max(self, rhs: $SelfT) -> $SelfT {
                    if self.0 >= rhs.0 {
                        self
                    } else {
                        rhs
                    }
                }

                #[inline]
                pub const fn min(self, rhs: $SelfT) -> $SelfT {
                    if self.0 <= rhs.0 {
                        self
                    } else {
                        rhs
                    }
                }

                #[must_use = "this returns the result of the operation, \
                      without modifying the original"]
                #[inline]
                pub const fn checked_add(self, rhs: $SelfT) -> $AnxiousT {
                    $AnxiousT::new(self.0).checked_add($AnxiousT::new(rhs.0))
                }

                #[must_use = "this returns the result of the operation, \
                      without modifying the original"]
                #[inline]
                pub const fn checked_sub(self, rhs: $SelfT) -> $AnxiousT {
                    $AnxiousT::new(self.0).checked_sub($AnxiousT::new(rhs.0))
                }

                #[must_use = "this returns the result of the operation, \
                    without modifying the original"]
                #[inline]
                pub const fn checked_mul(self, rhs: $SelfT) -> $AnxiousT {
                    $AnxiousT::new(self.0).checked_mul($AnxiousT::new(rhs.0))
                }

                #[must_use = "this returns the result of the operation, \
                      without modifying the original"]
                #[inline]
                pub const fn checked_div(self, rhs: $SelfT) -> $AnxiousT {
                    $AnxiousT::new(self.0).checked_div($AnxiousT::new(rhs.0))
                }

                #[must_use = "this returns the result of the operation, \
                without modifying the original"]
                #[inline]
                pub const fn checked_rem(self, rhs: $SelfT) -> $AnxiousT {
                    $AnxiousT::new(self.0).checked_rem($AnxiousT::new(rhs.0))
                }
            }

            impl ops::Add<$SelfT> for $SelfT {
                type Output = $AnxiousT;

                fn add(self, rhs: $SelfT) -> $AnxiousT {
                    self.checked_add(rhs)
                }
            }

            impl ops::Sub<$SelfT> for $SelfT {
                type Output = $AnxiousT;

                fn sub(self, rhs: $SelfT) -> $AnxiousT {
                    self.checked_sub(rhs)
                }
            }

            impl ops::Mul<$SelfT> for $SelfT {
                type Output = $AnxiousT;

                fn mul(self, rhs: $SelfT) -> $AnxiousT {
                    self.checked_mul(rhs)
                }
            }

            impl ops::Div<$SelfT> for $SelfT {
                type Output = $AnxiousT;

                fn div(self, rhs: $SelfT) -> $AnxiousT {
                    self.checked_div(rhs)
                }
            }

            impl ops::Rem<$SelfT> for $SelfT {
                type Output = $AnxiousT;

                fn rem(self, rhs: $SelfT) -> $AnxiousT {
                    self.checked_rem(rhs)
                }
            }

            impl fmt::Display for $SelfT {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", self.0)
                }
            }

            impl fmt::Debug for $SelfT {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{:?}", self.0)
                }
            }

            #[cfg(test)]
            mod test {
                extern crate alloc;
                use alloc::format;
                use std::cmp::Ordering;
                use std::collections::hash_map::DefaultHasher;
                use std::hash::Hash;
                use std::hash::Hasher;

                use super::*;

                impl $SelfT {
                    pub fn structural_eq(self, rhs: $SelfT) -> bool {
                        self.0 == rhs.0
                    }
                }

                macro_rules! structural_eq {
                    ($lhs:expr, $rhs:expr) => {
                        $lhs.structural_eq($rhs)
                    };
                }

                #[test]
                fn test_eq() {
                    assert_eq!($SelfT::from(0), $SelfT::from(0));
                }

                #[test]
                fn test_hash() {
                    let mut hasher = DefaultHasher::new();
                    $SelfT::from(0).hash(&mut hasher);
                    let h1 = hasher.finish();
                    let mut hasher = DefaultHasher::new();
                    $SelfT::from(0).hash(&mut hasher);
                    let h2 = hasher.finish();
                    assert_eq!(h1, h2);
                }

                #[test]
                fn test_partial_ord() {
                    assert_eq!(
                        $SelfT::from(0).partial_cmp(&$SelfT::from(0)),
                        Some(Ordering::Equal)
                    );
                }

                #[test]
                fn test_ord() {
                    assert_eq!($SelfT::from(0).cmp(&$SelfT::from(0)), Ordering::Equal);
                }

                #[test]
                fn test_add() {
                    assert!(structural_eq!($SelfT::from(1) + $SelfT::from(2), $AnxiousT::from(3)));
                    assert!(structural_eq!(
                        $SelfT::MAX + $SelfT::from(1),
                        $AnxiousT::from(Panic::IntegerOverflow)
                    ));
                }

                #[test]
                fn test_sub() {
                    assert!(structural_eq!($SelfT::from(3) - $SelfT::from(2), $AnxiousT::from(1)));
                    assert!(structural_eq!(
                        $SelfT::MIN - $SelfT::from(1),
                        $AnxiousT::from(Panic::IntegerOverflow)
                    ));
                }

                #[test]
                fn test_mul() {
                    assert!(structural_eq!($SelfT::from(2) * $SelfT::from(3), $AnxiousT::from(6)));
                    assert!(structural_eq!(
                        $SelfT::MAX * $SelfT::MAX,
                        $AnxiousT::from(Panic::IntegerOverflow)
                    ));
                }

                #[test]
                fn test_div() {
                    assert!(structural_eq!($SelfT::from(3) / $SelfT::from(2), $AnxiousT::from(1)));
                    assert!(structural_eq!(
                        $SelfT::from(1) / $SelfT::from(0),
                        $AnxiousT::from(Panic::IntegerDivisionByZero)
                    ));
                    assert!(structural_eq!(
                        $SelfT::MIN / $SelfT::from(-1),
                        $AnxiousT::from(Panic::IntegerOverflow)
                    ));
                }

                #[test]
                fn test_rem() {
                    assert!(structural_eq!($SelfT::from(3) % $SelfT::from(2), $AnxiousT::from(1)));
                    assert!(structural_eq!(
                        $SelfT::from(1) % $SelfT::from(0),
                        $AnxiousT::from(Panic::IntegerDivisionByZero)
                    ));
                    assert!(structural_eq!(
                        $SelfT::MIN % $SelfT::from(-1),
                        $AnxiousT::from(Panic::IntegerOverflow)
                    ));
                }

                #[test]
                fn test_max() {
                    assert!(structural_eq!($SelfT::from(2).max($SelfT::from(1)), $SelfT::from(2)));
                    assert!(structural_eq!($SelfT::from(1).max($SelfT::from(2)), $SelfT::from(2)));
                }

                #[test]
                fn test_min() {
                    assert!(structural_eq!($SelfT::from(2).min($SelfT::from(1)), $SelfT::from(1)));
                    assert!(structural_eq!($SelfT::from(1).min($SelfT::from(2)), $SelfT::from(1)));
                }

                #[test]
                fn test_clone() {
                    assert!(structural_eq!($SelfT::from(0).clone(), $SelfT::from(0).clone()));
                }

                #[test]
                fn test_debug() {
                    assert_eq!(format!("{:?}", $SelfT::from(1)), "1");
                }

                #[test]
                fn test_display() {
                    assert_eq!(format!("{}", $SelfT::from(1)), "1");
                }
            }
        }
    };
}
