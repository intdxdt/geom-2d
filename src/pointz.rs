use serde::{Serialize, Deserialize};
use coordinate::Coordinate;
use std::fmt::Debug;
use math_util::{num, Feq,  NumCast};
use std::ops::{Index, IndexMut};
use serde::export::Formatter;
use serde::export::fmt::Error;
use std::cmp::Ordering;
use crate::Point;

/// PointZ is a 3D (x:float, y:float, z:float) point type.
#[derive(Serialize, Deserialize, Copy, Clone, PartialOrd, Debug)]
pub struct PointZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PointZ {
    ///Construct new point from x and y coordinates
    pub fn new(x: f64, y: f64, z: f64) -> PointZ {
        PointZ { x, y, z }
    }

    ///Construct new point from array
    pub fn from_array(a: [f64; 3]) -> PointZ {
        a.into()
    }

    ///Operator : equals
    #[inline]
    pub fn equals(&self, other: &PointZ) -> bool {
        self.x.feq(other.x) && self.y.feq(other.y) && self.z.feq(other.z)
    }

    ///As array
    #[inline]
    pub fn as_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    ///As tuple
    #[inline]
    pub fn as_tuple(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    ///As point
    #[inline]
    pub fn as_point(&self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn fmt_xy(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }
}


pub struct PointZs {
    pub points: Vec<PointZ>
}

impl std::fmt::Display for PointZ {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "POINT M({})", self.fmt_xy())
    }
}

impl<T> From<(T, T, T)> for PointZ where T: NumCast + Copy {
    fn from(tup: (T, T, T)) -> Self {
        PointZ {
            x: num::cast(tup.0).unwrap(),
            y: num::cast(tup.1).unwrap(),
            z: num::cast(tup.2).unwrap(),
        }
    }
}

impl<T> From<[T; 3]> for PointZ where T: NumCast + Copy {
    fn from(array: [T; 3]) -> Self {
        PointZ {
            x: num::cast(array[0]).unwrap(),
            y: num::cast(array[1]).unwrap(),
            z: num::cast(array[2]).unwrap(),
        }
    }
}

impl<T> From<&[T; 3]> for PointZ where T: NumCast + Copy {
    fn from(array: &[T; 3]) -> Self {
        PointZ {
            x: num::cast(array[0]).unwrap(),
            y: num::cast(array[1]).unwrap(),
            z: num::cast(array[2]).unwrap(),
        }
    }
}

impl From<&[f64]> for PointZ {
    fn from(slice: &[f64]) -> Self {
        PointZ { x: slice[0], y: slice[1], z: slice[2] }
    }
}

impl From<&Vec<f64>> for PointZ {
    fn from(vec: &Vec<f64>) -> Self {
        PointZ { x: vec[0], y: vec[1], z: vec[2] }
    }
}

impl<T> From<Vec<[T; 3]>> for PointZs where T: NumCast + Copy {
    fn from(items: Vec<[T; 3]>) -> Self {
        let mut points = Vec::with_capacity(items.len());
        for array in items {
            points.push(array.into())
        }
        PointZs { points }
    }
}


impl Eq for PointZ {}

impl PartialEq for PointZ {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Ord for PointZ {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut d = self.x - other.x;

        if d.feq(0.0) {
            d = self.y - other.y;
        }

        if d.feq(0.0) {
            d = self.z - other.z;
        }

        if d.feq(0.0) {
            Ordering::Equal
        } else if d < 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Coordinate for PointZ {
    type Scalar = f64;
    const DIM: usize = 2;
    ///Type Constructor : Generator over dimensions
    fn gen(value: impl Fn(usize) -> Self::Scalar) -> Self {
        PointZ {
            x: value(0),
            y: value(1),
            z: value(2),
        }
    }
    ///Value in ith dimension
    fn val(&self, i: usize) -> Self::Scalar {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => unreachable!(),
        }
    }
    ///Mutable value in ith dimension
    fn val_mut(&mut self, i: usize) -> &mut Self::Scalar {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable!(),
        }
    }
}

impl rtree_2d::Point for PointZ {
    type Scalar = f64;
    const DIMENSIONS: usize = 2;

    ///Type Constructor : Generator over dimensions
    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
        PointZ::gen(generator)
    }

    ///Value in ith dimension
    fn nth(&self, index: usize) -> Self::Scalar {
        self.val(index)
    }

    ///Mutable value in ith dimension
    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        self.val_mut(index)
    }
}

impl Index<usize> for PointZ {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
}

impl IndexMut<usize> for PointZ {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.val_mut(i)
    }
}

impl Index<usize> for PointZs {
    type Output = PointZ;
    fn index(&self, i: usize) -> &Self::Output {
        &self.points[i]
    }
}

#[macro_export]
macro_rules! ptz {
    ($l :expr, $r:expr, $z:expr) => {
        PointZ{x : ($l) as f64, y : ($r) as f64, z : ($z) as f64}
    };
    [$l :expr, $r:expr, $z:expr] => {
        PointZ{x : ($l) as f64, y : ($r) as f64, z : ($z) as f64}
    };
}

#[macro_export]
macro_rules! pts_z {
    ($($x:expr),*) => {
        {
            let mut vec:Vec<PointZ> = Vec::new();
            $(
                vec.push((&$x).into());
            )*
            vec
        }
    };
}


#[cfg(test)]
mod tests;
