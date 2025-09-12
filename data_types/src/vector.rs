use std::{fmt::Display, hash::Hash, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}, path::Iter};

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;
// pub type Position2D<T> = Vector<T, 2>;
// pub type Position3D<T> = Vector<T, 3>;
// pub type Position4D<T> = Vector<T, 4>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector<T: Default + Copy, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy> Vector2<T> {
    pub fn cross(&self, other: Vector2<T>) -> T 
    where 
        T: Sub<Output = T> + Mul<Output = T> + Default + Copy
    {
        self.x() * other.y() - self.y() * other.x()
    }
}

impl<T: Default + Copy> Vector3<T> {
    pub fn cross(&self, other: Vector3<T>) -> Vector3<T> 
    where 
        T: Sub<Output = T> + Mul<Output = T> + Default + Copy
    {
        Vector3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

impl<T: Default + Copy, const N: usize> Vector<T, N> {
    pub fn as_array(&self) -> &[T; N] {
        &self.data
    }

    pub fn as_mut_array(&mut self) -> &mut [T; N] {
        &mut self.data
    }

    pub fn to_array(&self) -> [T; N] {
        self.data
    }

    pub fn dot(&self, other: &Self) -> T 
    where 
        T: Add<Output = T> + Mul<Output = T> + Default + Copy
    {
        let mut result = T::default();
        for i in 0..N {
            result = result + (self.data[i] * other.data[i]);
        }
        result
    }
}

impl<T: Default + Copy, S: Into<T>> From<(S, S)> for Vector<T, 2> {
    fn from((x,y): (S, S)) -> Self {
        Self { data: [x.into(), y.into()] }
    }
}

impl<T: Default + Copy, S: Into<T>> From<(S, S, S)> for Vector<T, 3> {
    fn from((x, y, z): (S, S, S)) -> Self {
        Self { data: [x.into(), y.into(), z.into()] }
    }
}

impl<T: Default + Copy, S: Into<T>> From<(S, S, S, S)> for Vector<T, 4> {
    fn from((x, y, z, w): (S, S, S, S)) -> Self {
        Self { data: [x.into(), y.into(), z.into(), w.into()] }
    }
}

impl<T: Default + Copy> Vector<T, 2> {
    pub fn new(x: T, y: T) -> Self {
        Self { data: [x, y] }
    }
}

impl<T: Default + Copy> Vector<T, 3> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { data: [x, y, z] }
    }
}

impl<T: Default + Copy> Vector<T, 4> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { data: [x, y, z, w] }
    }
}

impl<T: Default + Copy, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Self {
            data: [T::default(); N],
        }
    }
}

impl<T: Default + Copy, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(value: [T; N]) -> Self {
        Self { data: value }
    }
}

impl<T: Display + Default + Copy, const N: usize> std::fmt::Display for Vector<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..N {
            write!(f, "{}", self.data[i])?;
            if i < N - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T: Default + Copy, const N: usize> std::ops::Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Default + Copy, const N: usize> std::ops::IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Add<Output = T> + Default + Copy, const N: usize> Add for Vector<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..N {
            result.data[i] = self.data[i] + rhs.data[i];
        }
        result
    }
}

impl<T: AddAssign + Default + Copy, const N: usize> AddAssign for Vector<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<T: Sub<Output = T> + Default + Copy, const N: usize> Sub for Vector<T, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..N {
            result.data[i] = self.data[i] - rhs.data[i];
        }
        result
    }
}

impl<T: SubAssign + Default + Copy, const N: usize> SubAssign for Vector<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<T: Add<Output = T> + Default + Copy, const N: usize> Add<T> for Vector<T, N> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let mut result = Self::default();
        for i in 0..N {
            result.data[i] = self.data[i] + rhs;
        }
        result
    }
}

impl<T: AddAssign + Default + Copy, const N: usize> AddAssign<T> for Vector<T, N> {
    fn add_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.data[i] += rhs;
        }
    }
}

impl<T: Sub<Output = T> + Default + Copy, const N: usize> Sub<T> for Vector<T, N> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let mut result = Self::default();
        for i in 0..N {
            result.data[i] = self.data[i] - rhs;
        }
        result
    }
}

impl<T: SubAssign + Default + Copy, const N: usize> SubAssign<T> for Vector<T, N> {
    fn sub_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.data[i] -= rhs;
        }
    }
}

impl<S, R, T, const N: usize> Mul<S> for Vector<T, N> 
where 
    S: Copy,
    R: Copy + Default,
    T: Mul<S, Output = R> + Default + Copy
{
    type Output = Vector<R, N>; 

    fn mul(self, rhs: S) -> Self::Output {
        let mut result: Vector<R, N> = Vector::default();
        for i in 0..N {
            result.data[i] = self.data[i] * rhs;
        }
        result
    }
}

impl<S: Copy, T: MulAssign<S> + Default + Copy, const N: usize> MulAssign<S> for Vector<T, N> {
    fn mul_assign(&mut self, rhs: S) {
        for i in 0..N {
            self.data[i] *= rhs;
        }
    }
}

impl<S: Copy, R: Copy + Default, T: Div<S, Output = R> + Default + Copy, const N: usize> Div<S> for Vector<T, N> {
    type Output = Vector<R, N>;

    fn div(self, rhs: S) -> Self::Output {
        let mut result: Vector<R, N> = Vector::default();
        for i in 0..N {
            result.data[i] = self.data[i] / rhs;
        }
        result
    }
}

impl<S: Copy, T: DivAssign<S> + Default + Copy, const N: usize> DivAssign<S> for Vector<T, N> {
    fn div_assign(&mut self, rhs: S) {
        for i in 0..N {
            self.data[i] /= rhs;
        }
    }
}

impl<T: Default + Copy> Vector<T, 2> {
    pub fn extend(&self, value: T) -> Vector<T, 3> {
        Vector::from([self.data[0], self.data[1], value])
    }
}

impl<T: Default + Copy> Vector<T, 3> {
    pub fn extend(&self, value: T) -> Vector<T, 4> {
        Vector::from([self.data[0], self.data[1], self.data[2], value])
    }
}

macro_rules! impl_vector_accessors {
    (
        $VectorType:ident, $size:expr,
        fields: [ $( ( $field:ident , $mut_field:ident ) : $idx:expr ),+ ],
        subvectors: [ $( $return_size:expr => ($subvec:ident , $setsubvec:ident ) : [ $( $array_index:expr => $subidx:expr ),+ ] ),* ]
    ) => {
        impl<T: Default + Copy> $VectorType<T, $size> {
            $(
                pub fn $field(&self) -> T {
                    self.data[$idx]
                }
                pub fn $mut_field(&mut self) -> &mut T {
                    &mut self.data[$idx]
                }
            )+

            $(
                pub fn $subvec(&self) -> Vector<T, $return_size> {
                    Vector::from([ $( self.data[$subidx] ),+ ])
                }

                pub fn $setsubvec(&mut self, value: Vector<T, $return_size>) {
                    $(
                        self.data[$subidx] = value.data[$array_index];
                    )+
                }
                
            )*
        }
    };
}

impl<T: Default + Copy, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl_vector_accessors!(
    Vector, 2,
    fields: [ 
        (x, x_mut): 0, 
        (y, y_mut): 1 
    ],
    subvectors: []
);

impl_vector_accessors!(
    Vector, 3,
    fields: [
        (x, x_mut): 0, 
        (y, y_mut): 1, 
        (z, z_mut): 2 
    ],
    subvectors: [ 
        2 => (xy, set_xy): [0=>0, 1=>1], 
        2 => (xz, set_xz): [0=>0, 1=>2], 
        2 => (yz, set_yz): [0=>1, 1=>2] 
    ]
);

impl_vector_accessors!(
    Vector, 4,
    fields: [
        (x, x_mut): 0, 
        (y, y_mut): 1, 
        (z, z_mut): 2, 
        (w, w_mut): 3 
    ],
    subvectors: [ 
        2 => (xy, set_xy): [0=>0, 1=>1], 
        2 => (xz, set_xz): [0=>0, 1=>2], 
        2 => (yz, set_yz): [0=>1, 1=>2],
        3 => (xyz, set_xyz): [0=>0, 1=>1, 2=>2],
        3 => (xyw, set_xyw): [0=>0, 1=>1, 2=>3],
        3 => (xzw, set_xzw): [0=>0, 1=>2, 2=>3],
        3 => (yzw, set_yzw): [0=>1, 1=>2, 2=>3]
    ]
);