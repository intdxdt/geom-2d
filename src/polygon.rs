use crate::{LinearRing, Point, Geometry, LineString, GeomType, parse_wkt};
use bbox_2d::MBR;

#[derive(Clone, Debug)]
pub struct Polygon(pub Vec<LinearRing>);

impl Polygon {
    ///Construct from coordinates
    pub fn new(coordinates: &[Vec<Point>]) -> Polygon {
        Polygon(lnr_rings(coordinates))
    }

    ///Construct from wkt
    pub fn from_wkt(s: &str) -> Polygon {
        s.into()
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

    fn intersects<T>(&self, other: &T) -> bool where T: Geometry {
        unimplemented!()
    }
}

impl std::fmt::Display for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.wkt())
    }
}

impl From<&str> for Polygon {
    fn from(wkt_str: &str) -> Self {
        let o = parse_wkt(wkt_str);
        match o.geom_type {
            GeomType::Polygon => {
                Polygon::new(&o.coordinates)
            }
            _ => {
                let msg = if o.success {
                    format!("invalid wkt string, expected POLYGON, got : {}", o.geom_type)
                } else {
                    format!("parser error : {}", o.message)
                };
                panic!(msg)
            }
        }
    }
}


//polygon lnr_rings
fn lnr_rings(coordinates: &[Vec<Point>]) -> Vec<LinearRing> {
    coordinates.iter()
        .map(|coords| LinearRing::new(coords))
        .collect()
}
