# num_bound

Trait that adds a bound function.

Automatically implemented for anything that is `Ord`

## Usage
`bound(&self, lower: &Self, upper: &Self) -> &Self`
```rust

use num_bound::Bound;

let upper = 500;
let lower = 200;

assert_eq!(300.bound(&lower, &upper), &300);
assert_eq!(550.bound(&lower, &upper), &500);
assert_eq!(100.bound(&lower, &upper), &200);

```