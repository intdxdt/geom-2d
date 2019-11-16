pub mod wkt;
pub mod point;
pub mod line;
pub mod ring;
pub mod polygon;
pub mod mono;
pub mod util;

pub use coordinate::Coordinate;
pub use crate::point::{Point, Points};
pub use crate::wkt::read_wkt;
pub use crate::line::LineString;
pub use crate::ring::LinearRing;
use crate::mono::MonoMBR;
use bbox_2d::MBR;

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

    fn intersects(&self, other : &dyn Geometry) -> bool;
    //	fn Intersection(Geometry) []Point
//	fn Distance(Geometry) float64
//	fn Type() GeoType
    fn wkt(&self) -> String;
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
