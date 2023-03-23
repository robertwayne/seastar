use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    hash::Hasher,
};

#[derive(Debug, Clone)]
pub struct Node {
    pub x: isize,
    pub y: isize,
    pub g: isize,
    pub h: isize,
    pub parent: Option<Box<Node>>,
}

impl Node {
    #[must_use]
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
            g: 0,
            h: 0,
            parent: None,
        }
    }

    #[must_use]
    pub fn f(&self) -> isize {
        self.g + self.h
    }
}

impl std::hash::Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Node {}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.f() == other.f() {
            Some(other.h.cmp(&self.h))
        } else {
            Some(other.f().cmp(&self.f()))
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.f() == other.f() {
            other.h.cmp(&self.h)
        } else {
            other.f().cmp(&self.f())
        }
    }
}
