pub mod wkt;
pub mod point;
pub mod line;
pub mod ring;
pub mod polygon;
pub mod mono;
pub mod util;
pub mod chull;

pub use coordinate::Coordinate;
pub use crate::point::{Point, Points};
pub use crate::line::LineString;
pub use crate::ring::LinearRing;
pub use crate::wkt::read_wkt;
pub use crate::chull::convex_hull;
use crate::mono::MonoMBR;
use bbox_2d::MBR;

#[derive(Copy, Clone, Debug)]
pub enum GeoType {
    Point,
    Segment,
    LineString,
    Polygon,
    Unknown,
}

pub trait Geometry {
    fn bbox(&self) -> MBR;
    fn as_linear(&self) -> Vec<LineString>;
    fn wkt_string(&self) -> String;
    //fn intersects(&self, other : &dyn Geometry) -> bool;
    //	fn Intersection(Geometry) []Point
    //	fn Distance(Geometry) float64
    //	fn Type() GeoType
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
