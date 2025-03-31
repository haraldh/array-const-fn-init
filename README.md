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

### Advantages of `array-const-fn-init` crate:

1. **Simplified syntax**:
   The crate provides a clean macro solution that's much more concise than the manual array initialization:
   ```rust
   const ARRAY: [usize; 10] = array_const_fn_init![calculate_value; 10];
   ```
   Compared to the manual approach with blocks and loops:
   ```rust
   const ARRAY: [usize; 10] = {
       let mut arr = [0; 10];
       let mut i = 0;
       while i < 10 {
           arr[i] = calculate_value(i);
           i += 1;
       }
       arr
   };
   ```

2. **No boilerplate code**:
   - No need to manually create a mutable array
   - No need to write loop control structures
   - Eliminates potential off-by-one errors in loop conditions

3. **Implementation as a procedural macro**:
   - The crate uses a procedural macro to generate the necessary constant expressions at compile time
   - This avoids limitations with const context evaluation that might arise with manual approaches

4. **Works with complex return types**:
   - The examples show it works seamlessly with tuples and other complex types
   - For example: `const ARRAY: [(u8, u8, u8); 4] = array_const_fn_init![const_vecs; 4];`

5. **Guaranteed compatibility**:
   - Works with Rust 1.45 and newer
   - The implementation uses a well-tested approach that's focused on this specific use case

6. **Zero runtime overhead**:
   - Like manual approaches, but with more safety and less potential for errors

7. **No dependencies**:
   - The crate has no external dependencies, making it lightweight

In essence, `array-const-fn-init` provides a more ergonomic, safer, and declarative way to initialize constant arrays with values from a const function, removing the need for manual loop constructs and temporary mutable variables in const contexts.
