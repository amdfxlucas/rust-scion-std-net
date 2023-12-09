use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};
use std::iter::*;
use std::iter;
#[macro_export]
macro_rules! bitop_impls {
    ($(
        $(#[$attr:meta])*
        impl ($BitOp:ident, $BitOpAssign:ident) for $ty:ty = ($bitop:ident, $bitop_assign:ident);
    )*) => {
        $(
            $(#[$attr])*
            impl $BitOpAssign for $ty {
                fn $bitop_assign(&mut self, rhs: $ty) {
                    for (lhs, rhs) in std::iter::zip(&mut self.octets, rhs.octets) {
                        lhs.$bitop_assign(rhs);
                    }
                }
            }

            $(#[$attr])*
            impl $BitOpAssign<&'_ $ty> for $ty {
                fn $bitop_assign(&mut self, rhs: &'_ $ty) {
                    self.$bitop_assign(*rhs);
                }
            }

            $(#[$attr])*
            impl $BitOp for $ty {
                type Output = $ty;

                #[inline]
                fn $bitop(mut self, rhs: $ty) -> $ty {
                    self.$bitop_assign(rhs);
                    self
                }
            }

            $(#[$attr])*
            impl $BitOp<&'_ $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn $bitop(mut self, rhs: &'_ $ty) -> $ty {
                    self.$bitop_assign(*rhs);
                    self
                }
            }

            $(#[$attr])*
            impl $BitOp<$ty> for &'_ $ty {
                type Output = $ty;

                #[inline]
                fn $bitop(self, rhs: $ty) -> $ty {
                    let mut lhs = *self;
                    lhs.$bitop_assign(rhs);
                    lhs
                }
            }

            $(#[$attr])*
            impl $BitOp<&'_ $ty> for &'_ $ty {
                type Output = $ty;

                #[inline]
                fn $bitop(self, rhs: &'_ $ty) -> $ty {
                    let mut lhs = *self;
                    lhs.$bitop_assign(*rhs);
                    lhs
                }
            }
        )*
    };
}
