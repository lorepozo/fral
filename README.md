# Functional random-access lists (**`fral`**)

[![Build Status](https://travis-ci.org/lucasem/fral.svg?branch=master)](https://travis-ci.org/lucasem/fral)
[![crates.io](https://img.shields.io/crates/v/fral.svg)](https://crates.io/crates/fral)
[![docs.rs](https://docs.rs/fral/badge.svg)](https://docs.rs/fral)

```toml
[dependencies]
fral = "1.0"
```

A functional random access list is an efficient data structure with _lookup_ and _update_
operations in O(log n), and _cons_ and _uncons_ operations in O(1) while preserving the
original list. It was introduced in Chris Okasaki's 1995 _ACM FPCA_ paper [Purely Functional
Random-Access Lists].

We provide [`Arc`]-based and [`Rc`]-based implementations in [`Fral`] and [`rc::Fral`],
depending on your use-case. Because [`Arc`] is the most versatile, it is the "primary"
implementation for this crate. However, if you don't need thread-safety, [`rc::Fral`] has
less overhead and should be used instead — it is a drop-in replacement for [`Fral`].

### Usage

```rust
extern crate fral;
use fral::Fral;
use std::sync::Arc;

// cons is O(1)
let mut f = Fral::new();
for item in vec![1, 2, 3, 4, 5] {
    f = f.cons(item);
}

// lookup is O(log n)
if let Some(x) = f.get(4) {
    assert_eq!(*x, 1);
} else { unreachable!() }
// lookup out of bounds is O(1)
assert_eq!(f.get(5), None);

// uncons is O(1)
if let Some((head, tail)) = f.uncons() {
    assert_eq!(*head, 5);
    assert_eq!(tail.len(), 4);
} else { unreachable!() }

// in this scope, we want f to have an extra item in front.
// we can do this in O(1), without cloning any items.
{
    let f = f.cons(42);

    assert_eq!(*f.get(0).unwrap(), 42);
    assert_eq!(*f.get(5).unwrap(), 1);
}

// our original Fral is still here
assert_eq!(
    f.iter().take(2).collect::<Vec<_>>(),
    vec![Arc::new(5), Arc::new(4)]
);
```

### Comparison with `im`

The following are benchmark results against [`Fral`], [`im::ConsList`], and
[`im::List`] (`im` version `9.0.0`) with the `get`, `cons`, and `uncons`
operations:

```
test get_fral           ... bench:      35,129 ns/iter (+/- 162)
test get_im_conslist    ... bench:  37,545,651 ns/iter (+/- 1,089,092)
test get_im_list        ... bench: 452,968,129 ns/iter (+/- 14,544,638)
test cons_fral          ... bench:     295,407 ns/iter (+/- 414)
test cons_im_conslist   ... bench:     172,356 ns/iter (+/- 437)
test cons_im_list       ... bench:     580,119 ns/iter (+/- 14,259)
test uncons_fral        ... bench:          51 ns/iter (+/- 0)
test uncons_im_conslist ... bench:          17 ns/iter (+/- 0)
test uncons_im_list     ... bench:         438 ns/iter (+/- 1)
```

[Purely Functional Random-Access Lists]: https://www.westpoint.edu/eecs/SiteAssets/SitePages/Faculty%20Publication%20Documents/Okasaki/fpca95.pdf
[`Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
[`Rc`]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
[`Fral`]: https://docs.rs/fral/~1/fral/struct.Fral.html
[`rc::Fral`]: https://docs.rs/fral/~1/fral/rc/struct.Fral.html
[`im::ConsList`]: https://docs.rs/im/~9.0/im/conslist/struct.ConsList.html
[`im::List`]: https://docs.rs/im/~9.0/im/list/struct.List.html
