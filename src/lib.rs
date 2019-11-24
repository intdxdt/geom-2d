pub mod wkt;
pub mod point;
pub mod line;
pub mod ring;
pub mod polygon;
pub mod mono;
pub mod util;
pub mod chull;
pub mod segment;
pub mod inter;

pub use coordinate::Coordinate;
pub use crate::point::{Point, Points};
pub use crate::line::LineString;
pub use crate::segment::Segment;
pub use crate::ring::LinearRing;
pub use crate::polygon::Polygon;
pub use crate::wkt::parse_wkt;
pub use crate::chull::convex_hull;
use crate::mono::MonoMBR;
use bbox_2d::MBR;

#[derive(Copy, Clone, Debug)]
pub enum GeomType {
    Point,
    Segment,
    LineString,
    Polygon,
    Unknown,
}

impl GeomType {
    pub fn is_point(&self) -> bool {
        match self { GeomType::Point => true, _ => false }
    }
    pub fn is_line_string(&self) -> bool {
        match self { GeomType::LineString => true, _ => false }
    }
    pub fn is_segment(&self) -> bool {
        match self { GeomType::Segment => true, _ => false }
    }
    pub fn is_polygon(&self) -> bool {
        match self { GeomType::Polygon => true, _ => false }
    }
}

impl std::fmt::Display for GeomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let type_str: String = match self {
            GeomType::Point => "Point".into(),
            GeomType::LineString => "LineString".into(),
            GeomType::Polygon => "Polygon".into(),
            _ => "Unknown".into()
        };
        write!(f, "{}", type_str)
    }
}


pub trait Geometry {
    fn bbox(&self) -> MBR;
    fn as_linear(&self) -> Vec<LineString>;
    fn wkt_string(&self) -> String;
    fn geom_type(&self) -> GeomType;
    fn intersects<T: Geometry>(&self, other: &T) -> bool;
    fn intersection<T: Geometry>(&self, other: &T) -> Vec<Point>;
    fn linear_rings(&self) -> &Vec<LinearRing> { unimplemented!(); }
    fn area(&self) -> f64 { 0f64 }
    //	fn Distance(Geometry) float64
    //	fn bbox() MBR
}

#[cfg(test)]
mod tests;