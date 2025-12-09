use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point3D<T = i64>(pub T, pub T, pub T);

impl Point3D {
    pub fn zero() -> Self {
        Point3D(0, 0, 0)
    }

    pub fn sq_dist(&self, other: Point3D) -> i64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        let dz = self.2 - other.2;
        dx * dx + dy * dy + dz * dz
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
