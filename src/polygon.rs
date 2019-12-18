use crate::{LinearRing, Point, Geometry, LineString, GeomType, parse_wkt};
use bbox_2d::MBR;
use std::collections::BTreeSet;
use crate::distance;

#[derive(Clone, Debug)]
pub struct Polygon(pub Vec<LinearRing>);

impl Polygon {
    ///Construct from coordinates
    pub fn new(coordinates: &[Vec<Point>]) -> Polygon {
        Polygon(lnr_rings(coordinates))
    }

    ///Construct from coordinates
    pub fn from_vec(coordinates: &Vec<Point>) -> Polygon {
        Polygon::new(&vec![coordinates.clone()])
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

    pub fn bounds(&self) -> MBR {
        self.shell().0.bounds.mbr
    }

    pub fn coordinates(&self) -> Vec<Vec<Point>> {
        self.0.iter().map(|v| v.0.coordinates.clone()).collect()
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
        self.shell().0.bounds.mbr
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

    fn intersects(&self, other: &dyn Geometry) -> bool {
        let mut bln = false;
        if other.geom_type().is_polygon() {
            if self.bbox().intersects(&other.bbox()) {
                bln = if self.bbox().area() < other.bbox().area() {
                    let ln = self.shell().line_string();
                    ln.intersects_polygon(other.linear_rings())
                } else {
                    let ln = other.linear_rings()[0].line_string();
                    ln.intersects_polygon(self.linear_rings())
                }
            }
        } else {
            let lns = other.as_linear();
            let mut i = 0;
            while !bln && i < lns.len() {
                bln = lns[i].intersects(self);
                i += 1;
            }
        }
        bln
    }

    fn intersection(&self, other: &dyn Geometry) -> Vec<Point> {
        let mut ptset = BTreeSet::new();
        if other.geom_type().is_polygon() {

            //other intersect self
            let lns = other.linear_rings();
            for ln in lns.iter() {
                ln.0.intersection(self).iter()
                    .for_each(|v| { ptset.insert(*v); });
            }

            //self intersects other
            let lns = self.linear_rings();
            for ln in lns.iter() {
                ln.0.intersection(other).iter()
                    .for_each(|v| { ptset.insert(*v); });
            }
        } else {
            let lns = other.as_linear();
            for ln in lns.iter() {
                ln.intersection(self).iter()
                    .for_each(|v| { ptset.insert(*v); });
            }
        }
        ptset.into_iter().collect()
    }

    fn linear_rings(&self) -> &Vec<LinearRing> {
        &self.0
    }

    fn area(&self) -> f64 {
        let mut a = self.shell().area();
        for rng in self.holes().iter() {
            a -= rng.area();
        }
        a
    }

    fn distance(&self,other: &dyn Geometry) -> f64 {
        if self.intersects(other) {
            0.0
        } else {
            distance::dist_as_lines(self.as_linear(), other.as_linear())
        }
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
