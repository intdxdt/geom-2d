use crate::{LinearRing, Point, Geometry, LineString, GeomType};
use bbox_2d::MBR;

#[derive(Clone, Debug)]
pub struct Polygon(pub Vec<LinearRing>);

impl Polygon {
    pub fn new(coordinates: &[Vec<Point>]) -> Polygon {
        Polygon(lnr_rings(coordinates))
    }

    pub fn shell(&self) -> &LinearRing {
        &self.0[0]
    }

    pub fn holes(&self) -> &[LinearRing] {
            &self.0[1..]
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


impl Geometry for Polygon {
    fn bbox(&self) -> MBR {
        self.shell().0.bbox.mbr
    }

    fn as_linear(&self) -> Vec<LineString> {
        self.0.iter().map(|r| r.0.clone()).collect()
    }

    fn wkt_string(&self) -> String {
        self.wkt()
    }

    fn geom_type(&self) -> GeomType {
        GeomType::Polygon
    }

    fn intersects<T>(&self, other: T) -> bool where T: Geometry {
        unimplemented!()
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
