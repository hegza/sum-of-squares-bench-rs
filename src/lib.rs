use float_ord::FloatOrd;
use std::{hash, iter};

/// Something float-like, but orderable. P is backing primitive.
pub trait Float<P>:
    Copy + PartialEq + PartialOrd + Ord + Eq + hash::Hash + Inner<InnerType = P>
{
}
impl Float<f64> for FloatOrd<f64> {}

pub trait Inner {
    type InnerType;

    fn inner(self) -> Self::InnerType;

    fn create(inner: Self::InnerType) -> Self;
}

impl Inner for FloatOrd<f64> {
    type InnerType = f64;

    fn inner(self) -> Self::InnerType {
        self.0
    }

    fn create(inner: f64) -> Self {
        FloatOrd::<f64>(inner)
    }
}

/// Sum the square of each input value, taking ownership of the data-structure.
///
/// Takes ownership of a collection, transforms it into an iterator and maps
/// over the iterator, squaring each input element. The subsequent iterator is
/// then accumulated to a single 'sum' value.
pub fn sum_of_squares_by_move<V, T>(collection: T) -> f64
where
    V: Float<f64>,
    T: iter::IntoIterator<Item = V>,
{
    collection
        .into_iter()
        .map(|x| x.inner().powi(2))
        .sum::<f64>()
}

/// Sum the square of each input value, referencing the data-structure
/// immutably.
///
/// Takes a reference to a collection. The reference is transformed into an
/// iterator over references to the original values in collection. This iterator
/// is mapped to produce the square of each input value. The subsequent iterator
/// is then accumulated to a single 'sum' value.
pub fn sum_of_squares_by_ref<V, T>(collection: &T) -> f64
where
    V: Float<f64>,
    for<'a> &'a T: iter::IntoIterator<Item = &'a V>,
{
    collection
        .into_iter()
        .map(|x| x.inner().powi(2))
        .sum::<f64>()
}
