// TODO: measure "sum of squares" for:
// Vec, VecDeque, LinkedList, HashSet, BTreeSet
/*
- RQ1: For sum of squares, how does the performance of each data structure compare against the others?
    - H1.1 Data structures stored as a contiguent array have equal performance. The compiler should be able to optimize these functionally equivalent implementations equally well.
    - H1.2 Data structures stored non-contiguently perform worse than those with contiguent storage. This is because the non-contiguent data structures cause more cache misses.
- RQ2: Does the size of the data-structure affect the performance?
    - H2.1 The size of the array should have an approximately linear relationship with processing time with some discontinuity at thresholds corresponding to the various cache sizes on the computer under testing.
- RQ3: Does it make a difference in measurement if the parameter is received by move vs. by reference.
    - H3.1 It makes no difference since input values are only read. Care must be taken to make sure that dropping the input value is not added to measurement overhead.
*/

use float_ord::FloatOrd;
use std::iter;

/// Sum the square of each input value, taking ownership of the data-structure.
///
/// Takes ownership of a collection, transforms it into an iterator and maps
/// over the iterator, squaring each input element. The subsequent iterator is
/// then accumulated to a single 'sum' value.
pub fn sum_of_squares<T>(collection: T) -> f64
where
    T: iter::IntoIterator<Item = FloatOrd<f64>>,
{
    collection.into_iter().map(|x| x.0.powi(2)).sum::<f64>()
}

/// Sum the square of each input value, referencing the data-structure
/// immutably.
///
/// Takes a reference to a collection. The reference is transformed into an
/// iterator over references to the original values in collection. This iterator
/// is mapped to produce the square of each input value. The subsequent iterator
/// is then accumulated to a single 'sum' value.
pub fn sum_of_squares_by_ref<T>(collection: &T) -> f64
where
    for<'a> &'a T: iter::IntoIterator<Item = &'a FloatOrd<f64>>,
{
    collection.into_iter().map(|x| x.0.powi(2)).sum::<f64>()
}
