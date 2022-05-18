#[doc = include_str!("../README.md")]
pub trait Bound: Ord
{
    fn bound<'a>(&'a self, lower: &'a Self, upper: &'a Self) -> &'a Self
    {
        lower.max(self.min(upper))
    }
}

impl<T: Ord> Bound for T
{
}
