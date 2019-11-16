use crate::{LinearRing, Point};

#[derive(Clone, Debug)]
pub struct Polygon(pub Vec<LinearRing>);

impl Polygon {
    pub fn new(coordinates: &[Vec<Point>]) -> Polygon {
        Polygon(lnr_rings(coordinates))
    }

    pub fn wkt(&self) -> String {
        format!("POLYGON(({}))", self.0
            .iter()
            .map(|r| {
                r.0.coordinates
                    .iter()
                    .map(|pt| pt.fmt_xy())
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .collect::<Vec<_>>()
            .join("),("))
    }
}

impl std::fmt::Display for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.wkt())
    }
}


//polygon lnr_rings
fn lnr_rings(coordinates: &[Vec<Point>]) -> Vec<LinearRing> {
    coordinates.iter()
        .map(|coords| LinearRing::new(coords))
        .collect()
}
