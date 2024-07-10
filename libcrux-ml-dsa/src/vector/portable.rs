use crate::vector::traits::Operations;

mod arithmetic;
mod ntt;
mod vector_type;

pub(crate) use vector_type::PortableVector;

impl Operations for PortableVector {
    fn ZERO() -> Self {
        vector_type::ZERO()
    }

    fn from_i32_array(array: &[i32]) -> Self {
        vector_type::from_i32_array(array)
    }

    fn to_i32_array(vector: Self) -> [i32; 8] {
        vector_type::to_i32_array(vector)
    }

    fn add(lhs: &Self, rhs: &Self) -> Self {
        arithmetic::add(lhs, rhs)
    }

    fn subtract(lhs: &Self, rhs: &Self) -> Self {
        arithmetic::subtract(lhs, rhs)
    }

    fn montgomery_multiply_by_constant(vector: Self, c: i32) -> Self {
        arithmetic::montgomery_multiply_by_constant(vector, c)
    }
    fn montgomery_multiply(lhs: Self, rhs: Self) -> Self {
        arithmetic::montgomery_multiply(&lhs, &rhs)
    }

    fn ntt_at_layer_0(vector: Self, zeta0: i32, zeta1: i32, zeta2: i32, zeta3: i32) -> Self {
        ntt::ntt_at_layer_0(vector, zeta0, zeta1, zeta2, zeta3)
    }
    fn ntt_at_layer_1(vector: Self, zeta0: i32, zeta1: i32) -> Self {
        ntt::ntt_at_layer_1(vector, zeta0, zeta1)
    }
    fn ntt_at_layer_2(vector: Self, zeta: i32) -> Self {
        ntt::ntt_at_layer_2(vector, zeta)
    }

    fn invert_ntt_at_layer_0(vector: Self, zeta0: i32, zeta1: i32, zeta2: i32, zeta3: i32) -> Self {
        ntt::invert_ntt_at_layer_0(vector, zeta0, zeta1, zeta2, zeta3)
    }
    fn invert_ntt_at_layer_1(vector: Self, zeta0: i32, zeta1: i32) -> Self {
        ntt::invert_ntt_at_layer_1(vector, zeta0, zeta1)
    }
    fn invert_ntt_at_layer_2(vector: Self, zeta: i32) -> Self {
        ntt::invert_ntt_at_layer_2(vector, zeta)
    }
}
