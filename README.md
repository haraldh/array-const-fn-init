# array-const-fn-init

Initializes an array with constant values calculated by a
`const fn (usize) -> usize`

Requires Rust >= 1.45

# Example

```rust
use array_const_fn_init::array_const_fn_init;

const fn const_double_it(i: usize) -> usize {
    i * 2
}
const ARRAY: [usize; 10] = array_const_fn_init![const_double_it; 10];
assert_eq!(ARRAY, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
```
