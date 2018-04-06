//! # Functional random-access lists
//!
//! A functional random access list is an efficient data structure with _lookup_ and _update_
//! operations in O(log n), and _cons_ and _uncons_ operations in O(1) while preserving the
//! original list. It was introduced in Chris Okasaki's 1995 _ACM FPCA_ paper [Purely Functional
//! Random-Access Lists].
//!
//! We provide [`Arc`]-based and [`Rc`]-based implementations in [`Fral`] and [`rc::Fral`],
//! depending on your use-case. Because [`Arc`] is the most versatile, it is the "primary"
//! implementation for this crate. However, if you don't need thread-safety, [`rc::Fral`] has
//! less overhead and should be used instead — it is a drop-in replacement for [`Fral`].
//!
//! ### Comparison with `im`
//!
//! The following are benchmark results against [`Fral`], [`im::Vector`], [`im::CatList`], and
//! [`im::ConsList`] (`im` version `10.0.0`) with the `get`, `cons`, and `uncons` operations (which
//! are `push_front` and `pop_front` for `im::Vector`). Results are sorted by efficiency:
//!
//! ```plain
//! test get_im_vector      ... bench:      25,968 ns/iter (+/- 113)
//! test get_fral           ... bench:      37,356 ns/iter (+/- 124)
//! test get_im_catlist     ... bench:  15,397,279 ns/iter (+/- 375,877)
//! test get_im_conslist    ... bench:  36,834,300 ns/iter (+/- 1,073,303)
//!
//! test cons_im_conslist   ... bench:     170,603 ns/iter (+/- 461)
//! test cons_fral          ... bench:     294,475 ns/iter (+/- 195)
//! test cons_im_catlist    ... bench:     641,423 ns/iter (+/- 2,625)
//! test cons_im_vector     ... bench:     949,886 ns/iter (+/- 6,663)
//!
//! test uncons_im_conslist ... bench:          17 ns/iter (+/- 0)
//! test uncons_fral        ... bench:          52 ns/iter (+/- 0)
//! test uncons_im_catlist  ... bench:         149 ns/iter (+/- 0)
//! test uncons_im_vector   ... bench:         454 ns/iter (+/- 2)
//! ```
//!
//! # Examples
//!
//! ```
//! # use std::sync::Arc;
//! use fral::Fral;
//!
//! // cons is O(1)
//! let mut f = Fral::new();
//! for item in vec![1, 2, 3, 4, 5] {
//!     f = f.cons(item);
//! }
//!
//! // lookup is O(log n)
//! if let Some(x) = f.get(4) {
//!     assert_eq!(*x, 1);
//! } else { unreachable!() }
//! // lookup out of bounds is O(1)
//! assert_eq!(f.get(5), None);
//!
//! // uncons is O(1)
//! if let Some((head, tail)) = f.uncons() {
//!     assert_eq!(*head, 5);
//!     assert_eq!(tail.len(), 4);
//! } else { unreachable!() }
//!
//! // in this scope, we want f to have an extra item in front.
//! // we can do this in O(1), without cloning any items.
//! {
//!     let f = f.cons(42);
//!
//!     assert_eq!(*f.get(0).unwrap(), 42);
//!     assert_eq!(*f.get(5).unwrap(), 1);
//! }
//!
//! // our original Fral is still here
//! assert_eq!(
//!     f.iter().take(2).collect::<Vec<_>>(),
//!     vec![Arc::new(5), Arc::new(4)]
//! );
//! ```
//!
//! [Purely Functional Random-Access Lists]: https://www.westpoint.edu/eecs/SiteAssets/SitePages/Faculty%20Publication%20Documents/Okasaki/fpca95.pdf
//! [`Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
//! [`Rc`]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
//! [`Fral`]: struct.Fral.html
//! [`rc::Fral`]: rc/struct.Fral.html
//! [`im::Vector`]: https://docs.rs/im/~10.0/im/vector/struct.Vector.html
//! [`im::CatList`]: https://docs.rs/im/~10.0/im/catlist/struct.CatList.html
//! [`im::ConsList`]: https://docs.rs/im/~10.0/im/conslist/struct.ConsList.html

mod arc;
pub mod rc;

pub use arc::*;
