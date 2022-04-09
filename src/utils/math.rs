use egui::epaint::util::FloatOrd;
use num::Float;

pub fn lramp<T>(start: T, target: T, level: T) -> T
where
    T:  Float
{
    start + (target - start) * num::clamp(level, num::zero(), num::one())
}