use crate::vector::traits::FIELD_ELEMENTS_IN_VECTOR;
use hax_lib::*;
use hax_bounded_integers::*;

/// Values having this type hold a representative 'x' of the Kyber field.
/// We use 'fe' as a shorthand for this type.
pub(crate) type FieldElement = i16;

pub(crate) type MaxFieldElement <const MIN:i16, const MAX:i16> = BoundedI16<MIN, MAX>;

#[derive(Clone, Copy)]
pub struct PortableVector {
    pub(crate) elements: [FieldElement; FIELD_ELEMENTS_IN_VECTOR],
}

#[derive(Clone, Copy)]
pub struct MaxPortableVector <const MIN:i16, const MAX: i16> {
    pub(crate) elements: [MaxFieldElement<MIN, MAX>; FIELD_ELEMENTS_IN_VECTOR],
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn zero() -> PortableVector {
    PortableVector {
        elements: [0i16; FIELD_ELEMENTS_IN_VECTOR],
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn max_zero<const MIN:i16, const MAX: i16>() -> MaxPortableVector<MIN, MAX> {
    MaxPortableVector {
        elements: [BoundedI16::new(0); FIELD_ELEMENTS_IN_VECTOR],
    }
}

#[inline(always)]
pub fn from_i16_array(array: &[i16]) -> PortableVector {
    PortableVector {
        elements: array[0..16].try_into().unwrap(),
    }
}

#[inline(always)]
pub fn to_i16_array(x: PortableVector) -> [i16; 16] {
    x.elements
}
