use rtree_2d::{AABB, RTreeObject, PointDistance};
use bbox_2d::MBR;
use crate::Point;
use std::fmt::{Display, Formatter, Error};

pub const NULL_INDEX: usize = std::usize::MAX;

#[derive(Copy, Clone, Debug)]
pub struct MonoMBR {
    pub mbr: MBR,
    pub i: usize,
    pub j: usize,
}

impl MonoMBR {
    pub fn new(a: Point, b: Point) -> MonoMBR {
        MonoMBR::new_mono(MBR::new_from_bounds(a.as_array(), b.as_array()))
    }

    pub fn new_mono_ij(a: Point, b: Point, i: usize, j: usize) -> MonoMBR {
        MonoMBR { mbr: MBR::new_from_bounds(a.as_array(), b.as_array()), i, j }
    }

    pub fn new_mono(mbr: MBR) -> MonoMBR {
        MonoMBR { mbr, i: NULL_INDEX, j: NULL_INDEX }
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

impl Display for MonoMBR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.mbr)
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


#[cfg(test)]
mod tests {
    use super::*;
    use rtree_2d::RTree;

    #[test]
    fn test_mono() {
        let pt_0 = Point::new(0., 0.);
        let pt_1 = Point::new(3., 4.);
        let a = MonoMBR::new_mono_ij(pt_0, pt_1, 0, 7);
        let mut b = MonoMBR::new(pt_0, pt_1);
        let c = MonoMBR::new(pt_0, pt_0);
        let d = MonoMBR::new(pt_1, pt_1);
        assert_eq!(a, b);
        b.i = 0;
        b.j = 7;
        assert_eq!(a, b);
        assert_eq!(a == b, true);
        assert_eq!(c.distance_square(&d), 25.);
        assert_eq!(a.wkt(), "POLYGON((0 0,0 4,3 4,3 0,0 0))");
        assert_eq!(format!("{}", a), "POLYGON((0 0,0 4,3 4,3 0,0 0))");
    }


    #[test]
    fn test_rtree() {
        let items = vec![
            MonoMBR::new_mono_ij(Point { x: 0., y: 0. }, Point { x: 1., y: 1. }, 0, 3),
            MonoMBR::new_mono_ij(Point { x: 1., y: 1. }, Point { x: 2., y: 2. }, 3, 7),
            MonoMBR::new_mono_ij(Point { x: 4., y: 2. }, Point { x: 7.0, y: 3.0 }, 7, 11),
        ];
        let mut tree = RTree::load(items);
        assert_eq!(tree.size(), 3);
        let neib = tree.nearest_neighbor(&Point { x: 0.5, y: 0.5 });
        assert!(neib.is_some());

        let query = MonoMBR::new_mono_ij(Point { x: 2.5, y: 0.5 }, Point { x: 4.0, y: 2.5 }, 0, 9);
        let res = tree.search(&query.envelope());
        for r in res.into_iter() {
            assert!(tree.contains(r));
        }
        let at = MonoMBR::new_mono_ij(Point { x: 1., y: 1. }, Point { x: 2., y: 2. }, 3, 7);
        let res = tree.remove(&at);
        assert_eq!(tree.size(), 2);
        assert!(!tree.contains(&at));

        match res {
            Some(v) => {
                println!("rm = {}", v)
            }
            None => println!("None!")
        }
        println!("tree size = {}", tree.size());
        tree.each(|v| println!("{}", v.wkt()));
        let rt = tree.rtree();
        assert_eq!(rt.size(), 2);
    }
}