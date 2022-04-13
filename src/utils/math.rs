
use cgmath::{num_traits::{clamp, Float}, Zero, One, InnerSpace, VectorSpace};

pub fn lramp<T>(start: T, target: T, level: T) -> T
where
    T:  Float
{
    start + (target - start) * clamp(level, Zero::zero(), One::one())
}

#[inline]
pub fn clamp_magnitude<T>(v: T, max_mag: T::Scalar) -> T
where
    T: InnerSpace + VectorSpace,
    T::Scalar: Float
{
    if v.magnitude2() > max_mag { v.normalize() * max_mag } else { v }
}
