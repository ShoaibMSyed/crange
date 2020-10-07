# CRange

Provides types and functions to access a constant number of elements from a slice.

## Examples

### Indexing

```rust
use crange::Range;

let four_elements = [0, 1, 2, 3, 4, 5][Range::<4>];
assert_eq!([0, 1, 2, 3], four_elements);

let type_inference: [_; 2] = [0, 1, 2][Range];
assert_eq!([0, 1], type_inference);

let offset = [0, 1, 2, 3, 4, 5][1..][Range::<2>];
assert_eq!([1, 2], offset);

let mut mutable = [0, 1, 2, 3];
mutable[Range::<2>] = [1, 0];
assert_eq!([1, 0, 2, 3], mutable);
```

### RangeGet trait

```rust
use crange::RangeGet;

let elements = [1, 2, 3, 4, 5];
let two = elements.get_range::<2>();
assert_eq!(Some(&[1, 2]), two);

let none = elements.get_range_mut::<6>();
assert_eq!(None, none);
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.