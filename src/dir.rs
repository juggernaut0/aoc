use crate::Point;

// set discriminant so they can be used as flags in a bitset
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Dir {
    N = 1,
    E = 2,
    S = 4,
    W = 8,
}

impl Dir {
    #[must_use]
    pub fn turn_left(self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    #[must_use]
    pub fn diff(self) -> Point {
        match self {
            Dir::N => Point(0, -1),
            Dir::E => Point(1, 0),
            Dir::S => Point(0, 1),
            Dir::W => Point(-1, 0),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DirSet(u8);

impl DirSet {
    #[must_use]
    pub fn new() -> DirSet {
        DirSet(0)
    }

    /**
    Returns true if the set changed
    */
    pub fn insert(&mut self, dir: Dir) -> bool {
        let old = self.0;
        self.0 |= dir as u8;
        self.0 != old
    }

    /**
    Returns whether the value was present
    */
    pub fn remove(&mut self, dir: Dir) -> bool {
        let old = self.0;
        self.0 &= !(dir as u8);
        self.0 != old
    }

    #[must_use]
    pub fn contains(&self, dir: Dir) -> bool {
        self.0 & (dir as u8) != 0
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl Default for DirSet {
    fn default() -> Self {
        DirSet::new()
    }
}
