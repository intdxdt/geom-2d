use crate::{LineString, Point, Polygon};
use bbox_2d::MBR;
use math_util::sign_of_det2;

#[derive(Clone, Debug)]
pub struct LinearRing(pub LineString);

impl LinearRing {
    ///New linear ring
    pub fn new(coords: &[Point]) -> LinearRing {
        let mut coordinates = Vec::with_capacity(coords.len());
        coordinates.extend_from_slice(coords);
        let n = coords.len();
        if n > 1 {
            if !is_ring(&coordinates) {
                coordinates.push(coordinates[0])
            }
        }
        LinearRing(LineString::new(&coordinates))
    }
    pub fn bbox(&self) -> &MBR {
        &self.0.bbox.mbr
    }

    pub fn line_string(&self) -> &LineString {
        &self.0
    }

    pub fn coordinates(&self) -> &Vec<Point> {
        &self.0.coordinates
    }

    //Contains point
    pub fn contains_point(&self, pnt: &Point) -> bool {
        return self.bbox().intersects_xy(pnt.x, pnt.y) &&
            self.point_completely_in_ring(pnt);
    }

    //Contains line
    pub fn contains_line(&self, ln: &LineString) -> bool {
        if self.bbox().disjoint(&ln.bbox.mbr) { //disjoint
            return false;
        }
        let mut bln = true;
        let mut i = 0;
        while bln && i < ln.coordinates.len() {
            bln = self.contains_point(&ln.coordinates[i]);
            i += 1
        }
        bln
    }

    //Contains polygon
    pub fn contains_polygon(&self, polygon: &Polygon) -> bool {
        return self.contains_line(polygon.shell().line_string());
    }


    //point completely in ring
    pub fn point_completely_in_ring(&self, pnt: &Point) -> bool {
        return self.bbox().intersects_xy(pnt.x, pnt.y) && self.completely_in_ring(pnt);
    }

    //Test whether a point lies inside a ring.
    //The ring may be oriented in either direction.
    //If the point lies on the ring boundary the result of this method is unspecified.
    //This algorithm does not attempt to first check the point against the envelope of the ring.
    pub fn completely_in_ring(&self, p: &Point) -> bool {

        // for each segment l = (i-1, i), see if it crosses ray from test point in positive x direction.
        let mut crossings = 0; // number of segment/ray crossings
        let coords = self.coordinates();
        for i in 1..coords.len() {
            let p1 = coords[i];
            let p2 = coords[i - 1];

            if ((p1.y > p.y) && (p2.y <= p.y)) || ((p2.y > p.y) && (p1.y <= p.y)) {
                let (x1, y1) = (p1.x - p.x, p1.y - p.y);
                let (x2, y2) = (p2.x - p.x, p2.y - p.y);
                //segment straddles x axis, so compute intersection with x-axis.
                let x_inter = (sign_of_det2(x1, y1, x2, y2) as f64) / (y2 - y1);
                //xsave = x_inter
                //crosses ray if strictly positive intersection.
                if x_inter > 0.0 {
                    crossings += 1;
                }
            }
        }
        //p is inside if number of crossings is odd.
        (crossings % 2) == 1
    }
}


///Is coordinates Po == Pn
pub  fn is_ring(coordinates: &Vec<Point>) -> bool {
    if coordinates.len() < 2 {
        false
    } else {
        coordinates.first().unwrap().equals(coordinates.last().unwrap())
    }
}




