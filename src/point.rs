use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point<T = i32>(pub T, pub T);

impl Point {
    /**
    # Panics

    Panics if input types cannot be converted into i32.
    */
    #[must_use]
    pub fn of<X: TryInto<i32>, Y: TryInto<i32>>(x: X, y: Y) -> Point
    where
        X::Error: Debug,
        Y::Error: Debug,
    {
        Point(x.try_into().unwrap(), y.try_into().unwrap())
    }

    pub fn l1dist(self, other: Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    pub fn adj(self) -> [Point; 4] {
        [
            Point(self.0 + 1, self.1),
            Point(self.0, self.1 + 1),
            Point(self.0 - 1, self.1),
            Point(self.0, self.1 - 1),
        ]
    }

    pub fn adj_diag(self) -> [Point; 8] {
        [
            Point(self.0 - 1, self.1 - 1),
            Point(self.0 - 1, self.1),
            Point(self.0 - 1, self.1 + 1),
            Point(self.0, self.1 - 1),
            Point(self.0, self.1 + 1),
            Point(self.0 + 1, self.1 - 1),
            Point(self.0 + 1, self.1),
            Point(self.0 + 1, self.1 + 1),
        ]
    }
}

impl<T> Point<T> {
    // pseudo Into impl
    pub fn into<U: From<T>>(self) -> Point<U> {
        Point(self.0.into(), self.1.into())
    }
}

impl<T: Default> Point<T> {
    pub fn zero() -> Self {
        Point(T::default(), T::default())
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Point<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T: Display> Debug for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl<T: FromStr<Err = E>, E: Debug> FromStr for Point<T> {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // strip optional parentheses
        let parts_str = s
            .strip_prefix('(')
            .unwrap_or(s)
            .strip_suffix(')')
            .unwrap_or(s);
        let parts = parts_str.split_once(',').ok_or("no comma")?;
        let x = parts
            .0
            .trim()
            .parse()
            .map_err(|e| format!("failed to parse x: {e:?}"))?;
        let y = parts
            .1
            .trim()
            .parse()
            .map_err(|e| format!("failed to parse y: {e:?}"))?;
        Ok(Point(x, y))
    }
}
