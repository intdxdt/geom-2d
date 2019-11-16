use crate::{LinearRing, Point};

#[derive(Clone, Debug)]
pub struct Polygon {
    shell: LinearRing,
    holes: Vec<LinearRing>,
}

impl Polygon {
    pub fn new(coordinates: &[Vec<Point>]) -> Polygon {
        new_polygon_from_rings(lnr_rings(coordinates))
    }
}

//New Polygon from rings
fn new_polygon_from_rings(rings: Vec<LinearRing>) -> Polygon {
    let mut holes = vec![];
    if rings.len() > 1 {
        holes = rings[1..].to_vec();
    }
    Polygon { shell: rings[0].clone(), holes: holes }
}

//polygon lnr_rings
fn lnr_rings(coordinates: &[Vec<Point>]) -> Vec<LinearRing> {
    coordinates.iter()
        .map(|coords| LinearRing::new(coords))
        .collect()
}
