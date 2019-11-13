
pub mod point;

#[cfg(test)]
mod tests {
    use super::*;
    use point::{Point};
    use math_util::SQRT_2;

    #[test]
    fn it_works() {
        let a = pt![0, 0];
        let b = pt!(4, 3);
        let z = Point{x:0., y:0.};
        let o = Point::new(1., 1.);
        assert_eq!(o.distance(&z), SQRT_2);
    }
}
