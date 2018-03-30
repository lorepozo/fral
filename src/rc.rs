//!  An [`Rc`]-based functional random access list.
//!
//! [`Rc`]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html

use std::iter::FromIterator;
use std::rc::Rc;

/// An immutable reference-based functional random-access list, built atop [`Rc`].
///
/// [`Rc`]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
#[derive(Hash, Debug, PartialEq, Eq)]
pub struct Fral<T> {
    size: usize,
    pair: Rc<Pair<T>>,
}
impl<T> Fral<T> {
    /// Construct an empty list.
    pub fn new() -> Fral<T> {
        Self::default()
    }
    /// Returns a reference to an element, or `None` if it is out of bounds.
    ///
    /// Time: O(log n)
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// use fral::rc::Fral;
    ///
    /// let f: Fral<_> = vec![7, 0, 17].into_iter().rev().collect();
    /// assert_eq!(f.get(2), Some(Rc::new(17)));
    /// ```
    pub fn get(&self, index: usize) -> Option<Rc<T>> {
        self.pair.get(index)
    }
    /// Insert an element at the front of the list.
    ///
    /// Time: O(1)
    pub fn cons<R>(&self, x: R) -> Fral<T>
    where
        R: AsRc<T>,
    {
        Fral {
            size: 1 + self.size,
            pair: Rc::new(self.pair.cons(x.as_arc())),
        }
    }
    /// Get the head and tail of the list.
    ///
    /// Time: O(1)
    pub fn uncons(&self) -> Option<(Rc<T>, Fral<T>)> {
        let size = self.size.wrapping_sub(1);
        self.pair.uncons().map(|(x, pair)| (x, Fral { size, pair }))
    }
    /// Returns true iff the list contains no elements.
    ///
    /// Time: O(1)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    /// Get the number of items in the list.
    ///
    /// Time: O(1)
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn iter(&self) -> Iter<T> {
        Iter { fral: self.clone() }
    }
}
impl<T> Clone for Fral<T> {
    fn clone(&self) -> Fral<T> {
        Fral {
            size: self.size,
            pair: self.pair.clone(),
        }
    }
}
impl<T> Default for Fral<T> {
    fn default() -> Fral<T> {
        Fral {
            size: 0,
            pair: Rc::new(Nil),
        }
    }
}
impl<T> IntoIterator for Fral<T> {
    type Item = Rc<T>;
    type IntoIter = Iter<T>;
    fn into_iter(self) -> Iter<T> {
        Iter { fral: self }
    }
}
/// This is done with repeated `cons`, so you may intend to reverse your iterator first.
///
/// # Examples
///
/// ```
/// # use std::rc::Rc;
/// use fral::rc::Fral;
///
/// let items = vec![1, 2, 3];
/// let f: Fral<_> = items.into_iter().collect();
///
/// // the first item in f is the last item of the iterator
/// assert_eq!(f.get(0), Some(Rc::new(3)));
/// ```
impl<T, R: AsRc<T>> FromIterator<R> for Fral<T> {
    fn from_iter<I: IntoIterator<Item = R>>(iter: I) -> Fral<T> {
        let mut f = Fral::new();
        for x in iter {
            f = f.cons(x);
        }
        f
    }
}

use self::Pair::*;
#[derive(Clone, Hash, Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Pair<T> {
    Nil,
    Cons((usize, Rc<Tree<T>>), Rc<Pair<T>>),
}
impl<T> Pair<T> {
    fn get(&self, index: usize) -> Option<Rc<T>> {
        match *self {
            Nil => None,
            Cons((size, ref tree), ref cdr) => {
                if index < size {
                    tree.lookup(size, index)
                } else {
                    cdr.get(index - size)
                }
            }
        }
    }
    fn cons(&self, x: Rc<T>) -> Self {
        match *self {
            Nil => Cons((1, Rc::new(Leaf(x))), Rc::new(Nil)),
            Cons((size1, ref t1), ref nxt) => match **nxt {
                Cons((size2, ref t2), ref rest) => {
                    if size1 == size2 {
                        Cons(
                            (1 + size1 + size2, Rc::new(Node(x, t1.clone(), t2.clone()))),
                            rest.clone(),
                        )
                    } else {
                        Cons(
                            (1, Rc::new(Leaf(x))),
                            Rc::new(Cons(
                                (size1, t1.clone()),
                                Rc::new(Cons((size2, t2.clone()), rest.clone())),
                            )),
                        )
                    }
                }
                Nil => Cons(
                    (1, Rc::new(Leaf(x))),
                    Rc::new(Cons((size1, t1.clone()), Rc::new(Nil))),
                ),
            },
        }
    }
    fn uncons(&self) -> Option<(Rc<T>, Rc<Self>)> {
        match *self {
            Nil => None,
            Cons((size, ref t), ref rest) => match **t {
                Leaf(ref x) => Some((x.clone(), rest.clone())),
                Node(ref x, ref t1, ref t2) => {
                    let half = size / 2;
                    Some((
                        x.clone(),
                        Rc::new(Cons(
                            (half, t1.clone()),
                            Rc::new(Cons((half, t2.clone()), rest.clone())),
                        )),
                    ))
                }
            },
        }
    }
}

use self::Tree::*;
#[derive(Clone, Hash, Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Tree<T> {
    Leaf(Rc<T>),
    Node(Rc<T>, Rc<Tree<T>>, Rc<Tree<T>>),
}
impl<T> Tree<T> {
    fn lookup(&self, size: usize, index: usize) -> Option<Rc<T>> {
        match (index, self) {
            (0, &Leaf(ref x)) | (0, &Node(ref x, _, _)) => Some(x.clone()),
            (_, &Leaf(_)) => None,
            (i, &Node(_, ref t1, ref t2)) => {
                let half = size / 2;
                if i <= half {
                    t1.lookup(half, i - 1)
                } else {
                    t2.lookup(half, i - 1 - half)
                }
            }
        }
    }
}

pub struct Iter<T> {
    fral: Fral<T>,
}
impl<T> Iterator for Iter<T> {
    type Item = Rc<T>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((item, fral)) = self.fral.uncons() {
            self.fral = fral;
            Some(item)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.fral.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        let len = self.fral.len();
        self.fral.get(len - 1)
    }
}
impl<T> ExactSizeIterator for Iter<T> {}

/// Automatic [`Rc`] wrapping.
///
/// [`Rc`]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
pub trait AsRc<T> {
    fn as_arc(self) -> Rc<T>;
}

impl<T> AsRc<T> for T {
    fn as_arc(self) -> Rc<T> {
        Rc::from(self)
    }
}

impl<T> AsRc<T> for Rc<T> {
    fn as_arc(self) -> Rc<T> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Fral;
    use std::rc::Rc;

    #[test]
    fn empty() {
        let f: Fral<u8> = Fral::new();
        assert_eq!(f, Fral::default());
        assert!(f.is_empty());
        assert_eq!(f.len(), 0);
    }
    #[test]
    fn singleton() {
        let f = Fral::new();
        let f = f.cons(42);
        assert_eq!(f.get(0), Some(Rc::new(42)));
        assert_eq!(f.get(1), None);
        if let Some((head, tail)) = f.uncons() {
            assert_eq!(*head, 42);
            assert!(tail.is_empty());
        } else {
            panic!("assertion failed: couldn't uncons");
        }
        assert_eq!(f.iter().collect::<Vec<_>>(), vec![Rc::new(42)]);
    }
    #[test]
    fn many_items() {
        let mut f = Fral::new();
        for item in vec![1, 2, 3, 4, 5] {
            f = f.cons(item);
        }
        assert_eq!(f.get(0), Some(Rc::new(5)));
        assert_eq!(f.get(1), Some(Rc::new(4)));
        assert_eq!(f.get(2), Some(Rc::new(3)));
        assert_eq!(f.get(3), Some(Rc::new(2)));
        assert_eq!(f.get(4), Some(Rc::new(1)));
        assert_eq!(f.get(5), None);
        if let Some((head, tail)) = f.uncons() {
            assert_eq!(*head, 5);
            assert_eq!(tail.len(), 4);
        } else {
            panic!("assertion failed: couldn't uncons");
        }
        assert_eq!(
            f.iter().collect::<Vec<_>>(),
            vec![Rc::new(5), Rc::new(4), Rc::new(3), Rc::new(2), Rc::new(1)]
        );
    }
}
