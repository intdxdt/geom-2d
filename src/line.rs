use rtree_2d::RTree;
use crate::{Point};
use crate::MonoMBR;
use rstar::primitives::Line;

struct LineString {
    coordinates: Vec<Point>,
    bbox: MonoMBR,
    chains: Vec<MonoMBR>,
    index: RTree<MonoMBR>,
}

impl LineString {
    fn new() {

	}
}
