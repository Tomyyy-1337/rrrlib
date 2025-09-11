use std::{fmt::Display, ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign, Mul, MulAssign, Div, DivAssign}};

use crate::vector::Vector;

pub type Position2D<T> = Vector<T, 2>;
pub type Position3D<T> = Vector<T, 3>;
pub type Position4D<T> = Vector<T, 4>;