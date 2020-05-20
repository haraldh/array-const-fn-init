# array-const-fn-init

Initializes an array with constant values calculated by a `const fn (usize) -> T`

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

```rust
use array_const_fn_init::array_const_fn_init;

const fn const_vecs(i: usize) -> (u8, u8, u8) {
    (i as u8, i as u8, i as u8)
}
const ARRAY: [(u8, u8, u8); 4] = array_const_fn_init![const_vecs; 4];
assert_eq!(ARRAY, [(0, 0, 0), (1, 1, 1), (2, 2, 2), (3, 3, 3)]);
```
