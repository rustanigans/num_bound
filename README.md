# num_bound

Trait that adds a bound function enabling to restrict a number to a range.

Automatically implemented for anything that implements std trait `Ord`.

## V1 Breaking change

Removed the requirement to operate only on references.

Previously this worked:
```
let number = 19;
let upper = 60;
let low = 10;

assert_eq!(number.bound(&low, &upper), &number);
```
Now the bound method doesn't take self as ref so the equivilant is now:
`assert_eq!(number.as_ref().bound(&low, &upper), &number);`
However unless passing references is preferred, copy should suffice:
`assert_eq!(number.bound(low, upper), number);`
Only use references if type is larger than a reference.

## Usage
`bound(self, lower: Self, upper: Self) -> Self`
```rust

use num_bound::Bound;

#[test]
fn bound_test()
{
    let lower = 200;
    let upper = 500;

    let out_lower = 100;
    let out_upper = 600;
    let in_bounds = 300;
    
    assert_eq!(out_lower.bound(l, u), lower);
    assert_eq!(out_upper.bound(l, u), upper);
    assert_eq!(in_bounds.bound(l, u), in_bounds);
}

```