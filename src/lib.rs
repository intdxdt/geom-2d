
pub mod point;
pub mod line;
pub mod mono;


pub use point ::Point;
pub use coordinate::Coordinate;
use mono::MonoMBR;

pub enum GeoType {
    Point,
    Segment,
    LineString,
    Polygon,
    Unknown,
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
