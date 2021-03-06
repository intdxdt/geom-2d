pub mod wkt;
pub mod point;
pub mod pointz;
pub mod line;
pub mod ring;
pub mod polygon;
pub mod mono;
pub mod util;
pub mod chull;
pub mod segment;
pub mod inter;
pub mod distance;

pub use coordinate::Coordinate;
pub use crate::point::{
    Point, Points,
    reverse_direction,
    deflection_angle,
};
pub use crate::pointz::{PointZ, PointZs};
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
        match self {
            GeomType::Point => true,
            _ => false
        }
    }
    pub fn is_line_string(&self) -> bool {
        match self {
            GeomType::LineString => true,
            _ => false
        }
    }
    pub fn is_segment(&self) -> bool {
        match self {
            GeomType::Segment => true,
            _ => false
        }
    }
    pub fn is_polygon(&self) -> bool {
        match self {
            GeomType::Polygon => true,
            _ => false
        }
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

pub trait GeometryClone {
    fn clone_box(&self) -> Box<dyn Geometry>;
}

impl<T> GeometryClone for T where T: 'static + Geometry + Clone {
    fn clone_box(&self) -> Box<dyn Geometry> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Geometry> {
    fn clone(&self) -> Box<dyn Geometry> {
        self.clone_box()
    }
}

pub trait Geometry: GeometryClone {
    fn bbox(&self) -> MBR;
    fn as_linear(&self) -> Vec<LineString>;
    fn wkt_string(&self) -> String;
    fn geom_type(&self) -> GeomType;
    fn intersects(&self, other: &dyn Geometry) -> bool;
    fn intersection(&self, other: &dyn Geometry) -> Vec<Point>;
    fn linear_rings(&self) -> &Vec<LinearRing> { unimplemented!(); }
    fn area(&self) -> f64 { 0f64 }
    fn distance(&self, other: &dyn Geometry) -> f64;
}

#[cfg(test)]
mod tests_geom;