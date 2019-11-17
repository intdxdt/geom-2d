use rtree_2d::RTree;
use crate::{Point, Coordinate};
use crate::{
    util,
    MonoMBR,
    GeoType,
    Geometry,
};
use crate::mono::NULL_INDEX;
use bbox_2d::MBR;


#[derive(Clone, Debug)]
pub struct LineString {
    pub coordinates: Vec<Point>,
    pub bbox: MonoMBR,
    chains: Vec<MonoMBR>,
    index: RTree<MonoMBR>,
}

impl LineString {
    ///New LineString
    pub fn new(coords: &[Point]) -> LineString {
        let mut coordinates = Vec::with_capacity(coords.len());
        coordinates.extend_from_slice(coords);
        if coordinates.len() < 2 {
            panic!("a linestring must have at least 2 coordinates");
        }
        let (bbox, chains) = util::process_chains(&coordinates);
        let index = RTree::load(chains.clone());
        LineString { coordinates, bbox, chains, index }
    }
    
    ///Linestring from point
    pub fn new_from_point(pt: Point) -> LineString {
        LineString::new(&[pt, pt])
    }

    ///Geometry type
    pub fn geom_type(&self) -> GeoType {
        return GeoType::LineString;
    }

    pub fn as_linear(&self) -> Vec<LineString> {
        vec![self.clone()]
    }

    pub fn wkt(&self) -> String {
        format!("LINESTRING({})", self.coordinates
            .iter()
            .map(|pt| pt.fmt_xy())
            .collect::<Vec<_>>()
            .join(","))
    }
}


impl std::fmt::Display for LineString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.wkt())
    }
}


#[cfg(test)]
mod tests;