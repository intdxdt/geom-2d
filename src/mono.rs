use rstar::{AABB, RTreeObject, PointDistance};
use bbox_2d::MBR;
use crate::Point;
use std::fmt::{Display, Formatter, Error};

pub const NULL_INDEX: i32 = -9;

#[derive(Copy, Clone, Debug)]
pub struct MonoMBR {
    pub mbr: MBR,
    pub i: i32,
    pub j: i32,
}

impl MonoMBR {
    pub fn new(a: Point, b: Point) -> MonoMBR {
        MonoMBR::new_mono(MBR::new_from_bounds(a.as_array(), b.as_array()))
    }

    pub fn new_mono(mbr: MBR) -> MonoMBR {
        MonoMBR { mbr,i: NULL_INDEX, j: NULL_INDEX }
    }

    pub fn new_default() -> MonoMBR {
        MonoMBR { mbr: MBR::new_default(), i: 0, j: 0 }
    }

    pub fn bbox(&self) -> &MBR {
        &self.mbr
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.mbr.equals(&other.mbr)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.mbr.intersects(&other.mbr)
    }

    pub fn distance_square(&self, other: &Self) -> f64 {
        self.mbr.distance_square(&other.mbr)
    }

    pub fn wkt(&self) -> String {
        self.mbr.wkt()
    }
}

impl Display for MonoMBR{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}",  self.mbr)
    }
}

impl PartialEq for MonoMBR {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl From<AABB<Point>> for MonoMBR {
    fn from(aab: AABB<Point>) -> Self {
        MonoMBR::new(aab.lower(), aab.upper())
    }
}


impl RTreeObject for MonoMBR {
    type Envelope = AABB<Point>;
    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.mbr.ll().into(), self.mbr.ur().into())
    }
}

impl PointDistance for MonoMBR {
    fn distance_2(&self, pt: &Point) -> f64 {
        self.distance_square(&MonoMBR::new(*pt, *pt))
    }
}
