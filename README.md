# Rust Complex

rust implement for Complex Number

install

```bash
  cargo add swnb-complex
```

usage

```rust
  use swnb_complex::*;

  fn main() {
    let a = c!(3 + i 2);
    let b = c!(2 - i 3);

    let c = a * b;
    assert_eq!(c, (12, -9 + 4).into());

    let c = a + b;
    assert_eq!(c, c!(5 - i 1));

    let c = a - b;
    assert_eq!(c, c!(1 + i 5));

    assert_eq!(b - a, -c);
  }
```
