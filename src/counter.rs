use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::ops::Add;

pub struct Counter<T, C = u64> {
    counts: HashMap<T, C>,
}

impl<T, C> Counter<T, C> {
    pub fn new() -> Counter<T, C> {
        Counter {
            counts: HashMap::new(),
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<T, C> {
        self.counts.iter()
    }
}

impl<T, C> Default for Counter<T, C> {
    fn default() -> Self {
        Counter::new()
    }
}

impl<T: Hash + Eq, C: Add<Output = C> + Default + Copy + Eq> Counter<T, C> {
    pub fn count_n(&mut self, k: T, n: C) -> C {
        let entry = self.counts.entry(k).or_default();
        *entry = *entry + n;
        *entry
    }

    pub fn is_empty(&self) -> bool {
        self.counts.is_empty() || self.counts.values().all(|v| v == &C::default())
    }

    pub fn get(&self, k: &T) -> C {
        self.counts.get(k).copied().unwrap_or_default()
    }

    pub fn total(&self) -> C {
        self.counts.values().copied().fold(C::default(), Add::add)
    }
}

macro_rules! int_impl {
    ($ty: ty) => {
        impl<T: Hash + Eq> Counter<T, $ty> {
            pub fn count(&mut self, k: T) -> $ty {
                self.count_n(k, 1)
            }
        }

        impl<T: Hash + Eq> FromIterator<T> for Counter<T, $ty> {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                let mut counter = Counter::<_, $ty>::new();
                for item in iter {
                    counter.count(item);
                }
                counter
            }
        }
    };
}
int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
int_impl!(u64);
int_impl!(u128);
int_impl!(usize);
int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(i64);
int_impl!(i128);
int_impl!(isize);

impl<T: Debug, C: Debug> Debug for Counter<T, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.counts)
    }
}

impl<'a, T, C> IntoIterator for &'a Counter<T, C> {
    type Item = (&'a T, &'a C);
    type IntoIter = std::collections::hash_map::Iter<'a, T, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, C> IntoIterator for Counter<T, C> {
    type Item = (T, C);
    type IntoIter = std::collections::hash_map::IntoIter<T, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.into_iter()
    }
}
