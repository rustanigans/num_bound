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

#[test]
fn bound_test()
{
    let l = 200;
    let u = 500;

    let in_bounds = 300;
    let out_l = 100;
    let out_u = 600;

    assert_eq!(out_l.bound(&l, &u), &l);
    assert_eq!(out_u.bound(&l, &u), &u);
    assert_eq!(in_bounds.bound(&l, &u), &in_bounds);
}
