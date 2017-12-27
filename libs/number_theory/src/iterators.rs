//! Iterators over common or useful numbers / objects.

/// A type alias for a Pythagorean triple as a 3-tuple of integers.
pub type PythagTriple = (u64, u64, u64);

/// A structure capable of iterating over all primitive Pythagorean triples.
///
/// The iteration is performed using a depth-first traversal of the tree of triples defined on
/// [Wikipedia](https://en.wikipedia.org/wiki/Tree_of_primitive_Pythagorean_triples).
pub struct PythagTripleIterator<F>
    where F: Fn(PythagTriple) -> bool
{
    stack: Vec<PythagTriple>,
    condition: F,
}

impl<F> Iterator for PythagTripleIterator<F>
    where F: Fn(PythagTriple) -> bool
{
    type Item = PythagTriple;

    fn next(&mut self) -> Option<PythagTriple> {
        while let Some(next_triple) = self.stack.pop() {
            if (self.condition)(next_triple) {
                let (a, b, c) = next_triple;
                self.stack.push((a + 2 * c - 2 * b, 2 * a + 2 * c - b, 2 * a + 3 * c - 2 * b));
                self.stack.push((2 * a + b + 2 * c, a + 2 * b + 2 * c, 2 * a + 2 * b + 3 * c));
                self.stack.push((b + 2 * c - 2 * a, 2 * b + 2 * c - a, 2 * b + 3 * c - 2 * a));
                return Some(next_triple);
            }
        }

        None
    }
}

/// An iterator over the primitive Pythagorean triples satisfying the given condition.
///
/// The condition should be such that if a given triple passes it, then so do all smaller triples.
///
/// # Examples
///
/// ```
/// use number_theory::primitive_pythag_triples;
///
/// let mut triples = primitive_pythag_triples(|(a, b, c)| a + b + c <= 100);
///
/// assert_eq!(triples.next(), Some((3, 4, 5)));
/// assert_eq!(triples.next(), Some((8, 15, 17)));
/// assert_eq!(triples.next(), Some((12, 35, 37)));
/// assert_eq!(triples.next(), Some((20, 21, 29)));
/// assert_eq!(triples.next(), Some((5, 12, 13)));
/// assert_eq!(triples.next(), Some((7, 24, 25)));
/// assert_eq!(triples.next(), Some((9, 40, 41)));
/// assert_eq!(triples.next(), None);
/// ```
pub fn primitive_pythag_triples<F>(condition: F) -> PythagTripleIterator<F>
    where F: Fn(PythagTriple) -> bool
{
    PythagTripleIterator {
        stack: vec![(3, 4, 5)],
        condition: condition,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_pythag_triples() {
        let small_triples = primitive_pythag_triples(|(a, b, c)| a + b + c <= 150);
        assert_eq!(small_triples.collect::<Vec<PythagTriple>>(),
                   vec![(3, 4, 5),
                        (8, 15, 17),
                        (12, 35, 37),
                        (16, 63, 65),
                        (20, 21, 29),
                        (5, 12, 13),
                        (28, 45, 53),
                        (7, 24, 25),
                        (9, 40, 41),
                        (11, 60, 61)]);
    }
}