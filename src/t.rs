use num_traits::Zero;
use std::ops::Add;

#[derive(Debug)]
pub struct S1 {
    pub value: u64
}

#[derive(Debug)]
pub struct S2 {
    pub value: u64
}

pub trait Test: Zero + for<'a> Add<&'a Self, Output = Self> {
}

pub trait Test2 {
    type Two: Test + std::fmt::Debug;
}

impl<'a> Add<&'a Self> for S1 {
    type Output = Self;

    fn add(self, other: &Self) -> Self{
        S1 {
            value: self.value + other.value,
        }
        
    }
}

impl Add for S1 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        S1 {
            value: self.value + other.value,
        }
    }
}

impl Zero for S1 {
    fn zero() -> S1 {
        S1 {
            value: 0,
        }
    }

    fn is_zero(&self) -> bool {
        return self.value == 0
    }
}

impl Test for S1 {}
impl Test2 for S2 {
    type Two = S1;
}


