use rtree_2d::RTree;
use crate::Point;
use crate::MonoMBR;
use bbox_2d::MBR;
use rstar::primitives::Line;
use serde_json::value::Value::Null;

const MINI_MONO_SIZE: usize = 8;
const NULL: i32 = -9;

#[derive(Clone, Debug)]
struct LineString {
    coordinates: Vec<Point>,
    bbox: MonoMBR,
    chains: Vec<MonoMBR>,
    index: RTree<MonoMBR>,
}

impl LineString {
    fn new(coordinates: &[Point]) -> LineString {
        let mut coords = Vec::with_capacity(coordinates.len());
        coords.extend_from_slice(coordinates);
        if coordinates.len() < 2 {
            panic!("a linestring must have at least 2 coordinates");
        }
        let mut ln = LineString {
            coordinates: coords,
            bbox: MonoMBR::new_default(),
            chains: Vec::new(),
            index: RTree::new(),
        };
        ln
    }

    fn process_chains(&mut self) -> &mut Self {
        let n = self.coordinates.len();
        let (i, j) = (0, n - 1);
        let mut a = self.coordinates[i];
        let mut bbox: MBR = a.as_array().into();

        if n < MINI_MONO_SIZE {
            for pt in self.coordinates.iter() {
                bbox.expand_to_include_xy(pt.x, pt.y);
            }
            self.bbox = MonoMBR::new_mono(bbox);
            self.bbox.i = 0;
            self.bbox.j = n - 1;
            self.chains.push(self.bbox);
            return self;
        }
        let mono_limit = ((j + 1) as f64 + 1.0).log2() as i32;

        let (prev_x, prev_y) = (NULL, NULL);
        self.bbox = MonoMBR::new_mono(bbox);


        self
    }
}
