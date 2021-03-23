// TODO: measure "sum of squares" for:
// Vec, VecDeque, LinkedList, HashSet, BTreeSet
/*
- RQ1: For sum of squares, how does the performance of each data structure compare against the others?
    - H1.1 Data structures stored as a contiguent array have equal performance. The compiler should be able to optimize these functionally equivalent implementations equally well.
    - H1.2 Data structures stored non-contiguently perform worse than those with contiguent storage. This is because the non-contiguent data structures cause more cache misses.
- RQ2: Does the size of the data-structure affect the performance?
    - H2.1 The size of the array should have an approximately linear relationship with processing time with some discontinuity at thresholds corresponding to the various cache sizes on the computer under testing.
*/

use float_ord::FloatOrd;
use std::iter;

/// Sum the square of each input value
pub fn sum_of_squares<T>(collection: T) -> f64
where
    T: iter::IntoIterator<Item = FloatOrd<f64>>,
{
    collection.into_iter().map(|x| x.0.powi(2)).sum::<f64>()
}
