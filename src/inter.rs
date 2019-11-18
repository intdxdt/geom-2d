use crate::{Point, util};
use math_util::Feq;
use bbox_2d::MBR;
use std::cmp::Ordering;


// 1 << 3 == 1000
pub const SELF_A: u8 = 1 << 3;
// 1 << 2 == 0100
pub const SELF_B: u8 = 1 << 2;
// 1 << 1 == 0010
pub const OTHER_A: u8 = 1 << 1;
// 1 << 0 == 0001
pub const OTHER_B: u8 = 1 << 0;

pub const INTER_X: u8 = 0;
pub const SELF_MASK: u8 = SELF_A | SELF_B;
pub const OTHER_MASK: u8 = OTHER_A | OTHER_B;


#[derive(Copy, Clone, PartialOrd, Debug)]
pub struct InterPoint {
    pub pt: Point,
    pub inter: u8,
}

impl InterPoint {
    pub fn equals(&self, other: &InterPoint) -> bool {
        return self.pt.equals(&other.pt);
    }

    pub fn is_intersection(&self) -> bool {
        return self.inter == 0;
    }

    pub fn is_vertex(&self) -> bool {
        let mask = SELF_MASK | OTHER_MASK;
        return self.inter & mask > 0;
    }

    pub fn is_vertex_self(&self) -> bool {
        return self.inter & SELF_MASK > 0;
    }

    pub fn is_vertex_other(&self) -> bool {
        return self.inter & OTHER_MASK > 0;
    }

    pub fn is_vertex_xor(&self) -> bool {
        return (self.is_vertex_self() && !self.is_vertex_other()) ||
            (!self.is_vertex_self() && self.is_vertex_other());
    }
}
impl Eq for InterPoint{}
impl PartialEq for InterPoint{
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Ord for InterPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pt.cmp(&other.pt)
    }
}

impl std::fmt::Display for InterPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[{}, {}, {:04b}]", self.pt.x, self.pt.y, self.inter)
    }
}
