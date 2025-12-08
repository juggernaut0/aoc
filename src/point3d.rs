use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point3D<T = i64>(pub T, pub T, pub T);

impl Point3D {
    pub fn zero() -> Self {
        Point3D(0, 0, 0)
    }

    pub fn dist(&self, other: Point3D) -> i64 {
        ((self.0 - other.0).pow(2)
            + (self.1 - other.1).pow(2)
            + (self.2 - other.2).pow(2)).isqrt()
    }
}

impl<T: Display> Display for Point3D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl<T: Display> Debug for Point3D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
