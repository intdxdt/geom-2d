use rstar::{AABB, RTreeObject, PointDistance};
use bbox_2d::MBR;
use crate::Point;

#[derive(Copy, Clone, Debug)]
pub struct MonoMBR {
    pub mbr: MBR,
    pub i: usize,
    pub j: usize,
}

impl MonoMBR {
    pub fn new(a: Point, b: Point, i: usize, j: usize) -> MonoMBR {
        MonoMBR { mbr: MBR::new_from_bounds(a.as_array(), b.as_array()), i, j }
    }

    pub fn new_mono(mbr: MBR) -> MonoMBR {
        MonoMBR { mbr, i: 0, j: 0 }
    }

    pub fn new_default() -> MonoMBR {
        MonoMBR { mbr: MBR::new_default(), i: 0, j: 0 }
    }

    pub fn bbox(&self, other: &Self) -> &MBR {
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

impl PartialEq for MonoMBR {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl From<AABB<Point>> for MonoMBR {
    fn from(aab: AABB<Point>) -> Self {
        MonoMBR::new(aab.lower(), aab.upper(), 0, 0)
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
        self.distance_square(&MonoMBR::new(*pt, *pt, 0, 0))
    }
}
