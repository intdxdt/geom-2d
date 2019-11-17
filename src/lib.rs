pub mod wkt;
pub mod point;
pub mod line;
pub mod ring;
pub mod polygon;
pub mod mono;
pub mod util;
pub mod chull;
pub mod segment;

pub use coordinate::Coordinate;
pub use crate::point::{Point, Points};
pub use crate::line::LineString;
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
    fn is_point(&self) -> bool {
        match self {
            GeomType::Point => true,
            _ => false
        }
    }
    fn is_line_string(&self) -> bool {
        match self {
            GeomType::LineString => true,
            _ => false
        }
    }
    fn is_polygon(&self) -> bool {
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


pub trait Geometry {
    fn bbox(&self) -> MBR;
    fn as_linear(&self) -> Vec<LineString>;
    fn wkt_string(&self) -> String;
    fn geom_type(&self) -> GeomType;
    fn intersects<T: Geometry>(&self, other: &T) -> bool;
    //fn intersects(&self, other : &dyn Geometry) -> bool;
    //	fn Intersection(Geometry) []Point
    //	fn Distance(Geometry) float64
    //	fn Geometry() Geometry
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;
    use math_util::SQRT_2;

    #[test]
    fn it_works() {
        let z = pt![0, 0];
        let o = Point::new(1., 1.);
        assert_eq!(o.distance(&z), SQRT_2);
    }
}
