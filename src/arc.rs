use std::iter::FromIterator;
use std::sync::Arc;

/// An [`Arc`]-based functional random access list.
///
/// [`Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
#[derive(Hash, Debug, PartialEq, Eq)]
pub struct Fral<T> {
    size: usize,
    pair: Arc<Pair<T>>,
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
    /// # use fral::Fral;
    /// # use std::sync::Arc;
    /// let f: Fral<_> = vec![7, 0, 17].into_iter().rev().collect();
    /// assert_eq!(f.get(2), Some(Arc::new(17)));
    /// ```
    pub fn get(&self, index: usize) -> Option<Arc<T>> {
        self.pair.get(index)
    }
    /// Insert an element at the front of the list.
    ///
    /// Time: O(1)
    pub fn cons<R>(&self, x: R) -> Fral<T>
    where
        R: AsArc<T>,
    {
        Fral {
            size: 1 + self.size,
            pair: Arc::new(self.pair.cons(x.as_arc())),
        }
    }
    /// Get the head and tail of the list.
    ///
    /// Time: O(1)
    pub fn uncons(&self) -> Option<(Arc<T>, Fral<T>)> {
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
            pair: Arc::new(Nil),
        }
    }
}
impl<T> IntoIterator for Fral<T> {
    type Item = Arc<T>;
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
/// # use fral::Fral;
/// # use std::sync::Arc;
/// let items = vec![1, 2, 3];
/// let f: Fral<_> = items.into_iter().collect();
///
/// // the first item in f is the last item of the iterator
/// assert_eq!(f.get(0), Some(Arc::new(3)));
/// ```
impl<T, R: AsArc<T>> FromIterator<R> for Fral<T> {
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
    Cons((usize, Arc<Tree<T>>), Arc<Pair<T>>),
}
impl<T> Pair<T> {
    fn get(&self, index: usize) -> Option<Arc<T>> {
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
    fn cons(&self, x: Arc<T>) -> Self {
        match *self {
            Nil => Cons((1, Arc::new(Leaf(x))), Arc::new(Nil)),
            Cons((size1, ref t1), ref nxt) => match **nxt {
                Cons((size2, ref t2), ref rest) => {
                    if size1 == size2 {
                        Cons(
                            (1 + size1 + size2, Arc::new(Node(x, t1.clone(), t2.clone()))),
                            rest.clone(),
                        )
                    } else {
                        Cons(
                            (1, Arc::new(Leaf(x))),
                            Arc::new(Cons(
                                (size1, t1.clone()),
                                Arc::new(Cons((size2, t2.clone()), rest.clone())),
                            )),
                        )
                    }
                }
                Nil => Cons(
                    (1, Arc::new(Leaf(x))),
                    Arc::new(Cons((size1, t1.clone()), Arc::new(Nil))),
                ),
            },
        }
    }
    fn uncons(&self) -> Option<(Arc<T>, Arc<Self>)> {
        match *self {
            Nil => None,
            Cons((size, ref t), ref rest) => match **t {
                Leaf(ref x) => Some((x.clone(), rest.clone())),
                Node(ref x, ref t1, ref t2) => {
                    let half = size / 2;
                    Some((
                        x.clone(),
                        Arc::new(Cons(
                            (half, t1.clone()),
                            Arc::new(Cons((half, t2.clone()), rest.clone())),
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
    Leaf(Arc<T>),
    Node(Arc<T>, Arc<Tree<T>>, Arc<Tree<T>>),
}
impl<T> Tree<T> {
    fn lookup(&self, size: usize, index: usize) -> Option<Arc<T>> {
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
    type Item = Arc<T>;
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

/// Automatic [`Arc`] wrapping.
///
/// [`Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
pub trait AsArc<T> {
    fn as_arc(self) -> Arc<T>;
}

impl<T> AsArc<T> for T {
    fn as_arc(self) -> Arc<T> {
        Arc::from(self)
    }
}

impl<T> AsArc<T> for Arc<T> {
    fn as_arc(self) -> Arc<T> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Fral;
    use std::sync::Arc;

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
        assert_eq!(f.get(0), Some(Arc::new(42)));
        assert_eq!(f.get(1), None);
        if let Some((head, tail)) = f.uncons() {
            assert_eq!(*head, 42);
            assert!(tail.is_empty());
        } else {
            panic!("assertion failed: couldn't uncons");
        }
        assert_eq!(f.iter().collect::<Vec<_>>(), vec![Arc::new(42)]);
    }
    #[test]
    fn many_items() {
        let mut f = Fral::new();
        for item in vec![1, 2, 3, 4, 5] {
            f = f.cons(item);
        }
        assert_eq!(f.get(0), Some(Arc::new(5)));
        assert_eq!(f.get(1), Some(Arc::new(4)));
        assert_eq!(f.get(2), Some(Arc::new(3)));
        assert_eq!(f.get(3), Some(Arc::new(2)));
        assert_eq!(f.get(4), Some(Arc::new(1)));
        assert_eq!(f.get(5), None);
        if let Some((head, tail)) = f.uncons() {
            assert_eq!(*head, 5);
            assert_eq!(tail.len(), 4);
        } else {
            panic!("assertion failed: couldn't uncons");
        }
        assert_eq!(
            f.iter().collect::<Vec<_>>(),
            vec![
                Arc::new(5),
                Arc::new(4),
                Arc::new(3),
                Arc::new(2),
                Arc::new(1),
            ]
        );
    }
}
