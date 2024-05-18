/**
 * 
This module defines a trait `Bound` that provides a method `bound` for bounding a value between a lower and upper limit.

*/
pub trait Bound: Ord
{
    /// Bound a value between a lower and upper limit
    /// 
    /// ### Arguments
    /// 
    /// * `lower` - The lower limit
    /// * `upper` - The upper limit
    /// 
    /// ### Returns
    /// 
    /// The value bounded between the lower and upper limit
    /// 
    /// ### Example
    /// 
    /// ```
    /// use num_bound::Bound;
    ///     
    /// assert_eq!(1.bound(0, 2), 1);
    /// assert_eq!(0.bound(0, 2), 0);
    /// assert_eq!(2.bound(0, 2), 2);
    /// assert_eq!((-1).bound(0, 2), 0);
    /// assert_eq!(3.bound(0, 2), 2);
    /// ```
    fn bound(self, lower: Self, upper: Self) -> Self;    
}


impl<T: Ord> Bound for T
{
    #[inline(always)]
    fn bound(self, lower: T, upper: T) -> T
    {
        if self < lower
        {
            lower
        }
        else if self > upper
        {
            upper
        }
        else
        {
            self
        }
    }
}


#[test]
fn test_bound_ref()
{
    assert_eq!((&1).bound(&0, &2), &1);
    assert_eq!((&0).bound(&0, &2), &0);
    assert_eq!((&2).bound(&0, &2), &2);
    assert_eq!((&-1).bound(&0, &2), &0);
    assert_eq!((&3).bound(&0, &2), &2);
}

#[test]
fn test_bound()
{
    assert_eq!(1.bound(0, 2), 1);
    assert_eq!(0.bound(0, 2), 0);
    assert_eq!(2.bound(0, 2), 2);
    assert_eq!((-1).bound(0, 2), 0);
    assert_eq!(3.bound(0, 2), 2);
}
