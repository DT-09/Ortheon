use std::collections::BTreeSet;
use std::fmt;

/// Error returned when a relation operation violates its carrier constraints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationError<T> {
    /// The element is not part of the finite carrier set.
    OutsideUniverse(T),
    /// Two relations have different carrier sets.
    DifferentUniverses,
}

impl<T: std::fmt::Debug> fmt::Display for RelationError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutsideUniverse(value) => write!(f, "element {value:?} is outside the relation universe"),
            Self::DifferentUniverses => write!(f, "relations have different carrier sets"),
        }
    }
}

impl<T: std::fmt::Debug> std::error::Error for RelationError<T> {}

/// A finite binary relation over an explicitly declared carrier set.
///
/// The carrier and ordered pairs are stored in sorted sets, making iteration
/// deterministic. The type does not assume that the relation is an order,
/// equivalence relation or function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteRelation<T: Ord> {
    universe: BTreeSet<T>,
    pairs: BTreeSet<(T, T)>,
}

impl<T: Ord + Clone> FiniteRelation<T> {
    /// Construct an empty relation over the supplied finite universe.
    #[must_use]
    pub fn new(universe: impl IntoIterator<Item = T>) -> Self {
        Self { universe: universe.into_iter().collect(), pairs: BTreeSet::new() }
    }

    /// Number of carrier elements.
    #[must_use]
    pub fn universe_size(&self) -> usize { self.universe.len() }

    /// Number of ordered pairs in the relation.
    #[must_use]
    pub fn pair_count(&self) -> usize { self.pairs.len() }

    /// Iterate over the carrier set.
    pub fn universe(&self) -> impl Iterator<Item = &T> { self.universe.iter() }

    /// Iterate over relation pairs in lexicographic order.
    pub fn pairs(&self) -> impl Iterator<Item = (&T, &T)> {
        self.pairs.iter().map(|(a, b)| (a, b))
    }

    /// Insert an ordered pair, rejecting elements outside the carrier.
    pub fn insert(&mut self, a: T, b: T) -> Result<bool, RelationError<T>> {
        if !self.universe.contains(&a) { return Err(RelationError::OutsideUniverse(a)); }
        if !self.universe.contains(&b) { return Err(RelationError::OutsideUniverse(b)); }
        Ok(self.pairs.insert((a, b)))
    }

    /// Test membership of an ordered pair.
    #[must_use]
    pub fn contains(&self, a: &T, b: &T) -> bool {
        self.pairs.contains(&(a.clone(), b.clone()))
    }

    /// Return the domain: all elements appearing as a first coordinate.
    #[must_use]
    pub fn domain(&self) -> BTreeSet<T> {
        self.pairs.iter().map(|(a, _)| a.clone()).collect()
    }

    /// Return the range: all elements appearing as a second coordinate.
    #[must_use]
    pub fn range(&self) -> BTreeSet<T> {
        self.pairs.iter().map(|(_, b)| b.clone()).collect()
    }

    /// Test reflexivity over the entire declared universe.
    #[must_use]
    pub fn is_reflexive(&self) -> bool { self.universe.iter().all(|x| self.contains(x, x)) }

    /// Test irreflexivity over the entire declared universe.
    #[must_use]
    pub fn is_irreflexive(&self) -> bool { self.universe.iter().all(|x| !self.contains(x, x)) }

    /// Test symmetry.
    #[must_use]
    pub fn is_symmetric(&self) -> bool {
        self.pairs.iter().all(|(a, b)| self.contains(b, a))
    }

    /// Test antisymmetry.
    #[must_use]
    pub fn is_antisymmetric(&self) -> bool {
        self.pairs.iter().all(|(a, b)| a == b || !self.contains(b, a))
    }

    /// Test asymmetry (irreflexivity plus one-way pair orientation).
    #[must_use]
    pub fn is_asymmetric(&self) -> bool {
        self.pairs.iter().all(|(a, b)| a != b && !self.contains(b, a))
    }

    /// Test transitivity.
    #[must_use]
    pub fn is_transitive(&self) -> bool {
        self.pairs.iter().all(|(a, b)| {
            self.pairs.iter().all(|(c, d)| b != c || self.contains(a, d))
        })
    }

    /// Test whether this relation is an equivalence relation.
    #[must_use]
    pub fn is_equivalence_relation(&self) -> bool {
        self.is_reflexive() && self.is_symmetric() && self.is_transitive()
    }

    /// Test whether this relation is a partial order.
    #[must_use]
    pub fn is_partial_order(&self) -> bool {
        self.is_reflexive() && self.is_antisymmetric() && self.is_transitive()
    }

    /// Construct the inverse relation over the same carrier.
    #[must_use]
    pub fn inverse(&self) -> Self {
        Self {
            universe: self.universe.clone(),
            pairs: self.pairs.iter().map(|(a, b)| (b.clone(), a.clone())).collect(),
        }
    }

    /// Compose this relation with `after`: (a,c) exists when some b has
    /// (a,b) in this relation and (b,c) in `after`.
    ///
    /// Composition is defined only when both relations have identical carriers.
    pub fn compose(&self, after: &Self) -> Result<Self, RelationError<T>> {
        if self.universe != after.universe {
            return Err(RelationError::DifferentUniverses);
        }
        let mut result = Self::new(self.universe.iter().cloned());
        for (a, b) in &self.pairs {
            for (next, c) in &after.pairs {
                if b == next { result.pairs.insert((a.clone(), c.clone())); }
            }
        }
        Ok(result)
    }

    /// Compute the transitive closure using a finite Warshall algorithm.
    ///
    /// Unlike reflexive_transitive_closure, this does not add identity pairs
    /// unless they are implied by a non-empty cycle.
    #[must_use]
    pub fn transitive_closure(&self) -> Self {
        let elements: Vec<T> = self.universe.iter().cloned().collect();
        let n = elements.len();
        let indices: std::collections::BTreeMap<T, usize> = elements.iter().cloned()
            .enumerate().map(|(index, value)| (value, index)).collect();
        let mut reach = vec![vec![false; n]; n];
        for (a, b) in &self.pairs { reach[indices[a]][indices[b]] = true; }
        for k in 0..n {
            let pivot = reach[k].clone();
            for row in &mut reach {
                if row[k] {
                    for (cell, reachable) in row.iter_mut().zip(&pivot) { *cell |= *reachable; }
                }
            }
        }
        let mut closure = Self::new(elements.clone());
        for (i, row) in reach.iter().enumerate() {
            for (j, reachable) in row.iter().enumerate() {
                if *reachable { closure.pairs.insert((elements[i].clone(), elements[j].clone())); }
            }
        }
        closure
    }

    /// Compute the reflexive transitive closure using a finite Warshall algorithm.
    ///
    /// The result contains every pair connected by a path, including all
    /// identity pairs. Runtime is O(n^3) for n carrier elements.
    #[must_use]
    pub fn reflexive_transitive_closure(&self) -> Self {
        let mut closure = self.transitive_closure();
        for value in &self.universe { closure.pairs.insert((value.clone(), value.clone())); }
        closure
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_elements_outside_declared_universe() {
        let mut relation = FiniteRelation::new([1, 2, 3]);
        assert_eq!(relation.insert(1, 4), Err(RelationError::OutsideUniverse(4)));
    }

    #[test]
    fn checks_equivalence_relation_properties() {
        let mut relation = FiniteRelation::new([1, 2]);
        relation.insert(1, 1).unwrap(); relation.insert(2, 2).unwrap();
        relation.insert(1, 2).unwrap(); relation.insert(2, 1).unwrap();
        assert!(relation.is_equivalence_relation());
    }

    #[test]
    fn detects_partial_order() {
        let mut relation = FiniteRelation::new([1, 2, 3]);
        relation.insert(1, 1).unwrap(); relation.insert(2, 2).unwrap();
        relation.insert(3, 3).unwrap(); relation.insert(1, 2).unwrap();
        relation.insert(2, 3).unwrap(); relation.insert(1, 3).unwrap();
        assert!(relation.is_partial_order());
        assert!(!relation.is_symmetric());
    }

    #[test]
    fn composition_and_inverse_are_correct() {
        let mut first = FiniteRelation::new([1, 2, 3]);
        let mut second = FiniteRelation::new([1, 2, 3]);
        first.insert(1, 2).unwrap(); second.insert(2, 3).unwrap();
        let composed = first.compose(&second).unwrap();
        assert!(composed.contains(&1, &3));
        assert!(first.inverse().contains(&2, &1));
    }

    #[test]
    fn composition_rejects_different_carriers() {
        let first = FiniteRelation::new([1, 2]);
        let second = FiniteRelation::new([1, 2, 3]);
        assert_eq!(first.compose(&second), Err(RelationError::DifferentUniverses));
    }

    #[test]
    fn computes_both_closures() {
        let mut relation = FiniteRelation::new([1, 2, 3]);
        relation.insert(1, 2).unwrap(); relation.insert(2, 3).unwrap();
        let transitive = relation.transitive_closure();
        assert_eq!(transitive.pair_count(), 3);
        assert!(transitive.contains(&1, &3));
        let reflexive = relation.reflexive_transitive_closure();
        assert_eq!(reflexive.pair_count(), 6);
        assert!(reflexive.contains(&3, &3));
    }
}
