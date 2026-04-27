use std::{num::Saturating, ops::Add};

// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
#[derive(Debug, Clone, Copy)]
pub struct SaturatingU16 {
    pub value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        let def_value = *value;
        SaturatingU16 { value: def_value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        let v_u16: u16 = value.into();
        SaturatingU16 { value: v_u16 }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            value: (*value).into(),
        }
    }
}

impl Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: (Saturating(self.value) + Saturating(rhs.value)).0,
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            value: (Saturating(self.value) + Saturating(rhs.value)).0,
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        Self {
            value: self.value + rhs,
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        let saturated_val = (Saturating(self.value) + Saturating(rhs)).0;
        let unsaturated_calc = self.value + rhs;
        println!(
            "saturated calc {} unsaturated calc {}",
            saturated_val, unsaturated_calc
        );
        Self {
            value: saturated_val,
        }
    }
}
impl PartialEq for SaturatingU16 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
