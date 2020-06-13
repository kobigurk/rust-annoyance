use num_traits::Zero;
mod t;
use t::{S2, Test2};

fn hey<T: Test2>(x: T::Two, y: T::Two) {
    let z = x + &y;
    println!("{:?}", z);
}

fn main() {
    let x = <S2 as Test2>::Two::zero();
    let y = <S2 as Test2>::Two::zero();
    hey::<S2>(x, y);
}
