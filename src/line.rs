use bbox_2d::MBR;
use rtree_2d::RTree;
use crate::{Point, Coordinate, LinearRing, MonoMBR, GeomType, Geometry};
use crate::{util, segment, parse_wkt};
use rtree_2d::{RTreeObject, PointDistance, AABB, Envelope};
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct LineString {
    pub coordinates: Vec<Point>,
    pub bounds: MonoMBR,
    chains: Vec<MonoMBR>,
    index: RTree<MonoMBR>,
}

impl LineString {
    ///New LineString
    pub fn new(coords: &[Point]) -> LineString {
        let mut coordinates = Vec::with_capacity(coords.len());
        coordinates.extend_from_slice(coords);
        if coordinates.len() < 2 {
            panic!("a linestring must have at least 2 coordinates");
        }
        let (bbox, chains) = util::process_chains(&coordinates);
        let index = RTree::load(chains.clone());
        LineString { coordinates, bounds: bbox, chains, index }
    }
    ///New LineString from vector of points
    pub fn from_vec(coordinates: Vec<Point>) -> LineString {
        if coordinates.len() < 2 {
            panic!("a linestring must have at least 2 coordinates");
        }
        let (bbox, chains) = util::process_chains(&coordinates);
        let index = RTree::load(chains.clone());
        LineString { coordinates, bounds: bbox, chains, index }
    }
    ///New LineString from vector of points
    pub fn re_construct(&mut self) -> &mut LineString {
        let (bbox, chains) = util::process_chains(&self.coordinates);
        self.bounds = bbox;
        self.chains = chains;
        self.index = RTree::load(self.chains.clone());
        self
    }
    ///Linestring from point
    pub fn from_point(pt: Point) -> LineString {
        LineString::new(&[pt, pt])
    }

    ///Construct from wkt
    pub fn from_wkt(s: &str) -> LineString {
        s.into()
    }

    ///As array
    pub fn as_array(&self) -> Vec<[f64; 2]> {
        self.coordinates.iter().map(|v| v.as_array()).collect()
    }

    pub fn wkt(&self) -> String {
        format!("LINESTRING({})", self.coordinates
            .iter()
            .map(|pt| pt.fmt_xy())
            .collect::<Vec<_>>()
            .join(","))
    }

    ///Length of linestring
    pub fn length(&self) -> f64 {
        self.len(0, self.coordinates.len() - 1)
    }

    ///length of line from index i to j
    fn len(&self, i: usize, j: usize) -> f64 {
        let mut dist = 0.0;
        let (mut i, mut j) = (i, j);
        if j < i {
            std::mem::swap(&mut i, &mut j);
        }
        while i < j {
            dist += self.coordinates[i].distance(&self.coordinates[i + 1]);
            i += 1;
        }
        dist
    }
}

//linear relate
impl LineString {
    ///Checks if line intersects other{LineString}
    pub fn intersects_linestring(&self, other: &LineString) -> bool {
        let mut bln = false;
        let mut othersegs = Vec::new();
        let mut selfsegs = Vec::new();

        //var qrng *mbr.MBR
        //var qbox, ibox *mono.MBR
        let in_range = self.index.search(&other.bounds.envelope());
        let mut i = 0;
        while !bln && i < in_range.len() {
            //search ln using ibox
            let ibox = in_range[i];
            let ln_range = other.index.search(&ibox.envelope());

            let mut q = 0;
            while !bln && q < ln_range.len() {
                let qbox = ln_range[q];
                let inter = ibox.mbr.intersection(&qbox.mbr).unwrap();

                self.segs_in_range(&mut selfsegs, &inter, ibox.i, ibox.j);
                other.segs_in_range(&mut othersegs, &inter, qbox.i, qbox.j);

                if othersegs.len() > 0 && selfsegs.len() > 0 {
                    bln = self.seg_seg_intersects(&selfsegs, &othersegs)
                }
                q += 1;
            }
            i += 1;
        }
        return bln;
    }

    ///LineSting intersects polygon rings
    pub fn intersects_polygon(&self, rings: &Vec<LinearRing>) -> bool {
        let mut intersects_hole = false;
        let mut in_hole = false;
//        let rings = lns.iter().map(|ln| LinearRing(ln.clone())).collect::<Vec<LinearRing>>();
        let shell = &rings[0];

        let mut bln = self.intersects(shell.line_string());
        //if false, check if shell contains line
        if !bln {
            bln = shell.contains_line(self);
            //inside shell, does it touch hole boundary ?
            let mut i = 1;
            while bln && !intersects_hole && i < rings.len() {
                intersects_hole = self.intersects(rings[i].line_string());
                i += 1;
            }
            //inside shell but does not touch the boundary of holes
            if bln && !intersects_hole {
                //check if completely contained in hole
                let mut i = 1;
                while !in_hole && i < rings.len() {
                    in_hole = rings[i].contains_line(self);
                    i += 1;
                }
            }
            bln = bln && !in_hole
        }
        bln
    }


    pub fn linear_intersection(&self, other: &LineString) -> Vec<Point> {
        let mut ptset = BTreeSet::new();
        if self.bounds.mbr.disjoint(&other.bounds.mbr) {
            return Vec::new(); //disjoint
        }

        let mut othersegs = Vec::new();
        let mut selfsegs = Vec::new();

        let inrange = self.index.search(&other.bounds.envelope());

        for i in 0..inrange.len() {
            //cur self box
            let ibox = inrange[i];
            //search ln using ibox
            let lnrange = other.index.search(&ibox.envelope());
            for q in 0..lnrange.len() {
                let qbox = lnrange[q];
                let inter = ibox.mbr.intersection(&qbox.mbr).unwrap();

                self.segs_in_range(&mut selfsegs, &inter, ibox.i, ibox.j);
                other.segs_in_range(&mut othersegs, &inter, qbox.i, qbox.j);
                self.seg_seg_intersection(&selfsegs, &othersegs, &mut ptset)
            }
        }

        ptset.into_iter().collect()
    }


    //line intersect polygon rings
    pub fn intersection_polygon_rings(&self, rings: &Vec<LinearRing>) -> Vec<Point> {
        let mut res = Vec::new();
        let shell = &rings[0];
        let mut ptset = BTreeSet::new();
        let bln = self.bounds.mbr.intersects(&shell.bbox());

        if bln {
            let spts = self.linear_intersection(shell.line_string());
            for idx in 0..spts.len() {
                ptset.insert(spts[idx]);
            }
            //inside shell, does it touch hole boundary ?
            for hole in rings[1..].iter() {
//                let hpts = self.linear_intersection(hole.line_string());
                self.linear_intersection(hole.line_string())
                    .iter()
                    .for_each(|v| { ptset.insert(*v); });
//                for idx in 0..hpts.len() {
//                    ptset.insert(hpts[idx]);
//                }
            }

            //check for all vertices
            for idx in 0..self.coordinates.len() {
                let pt = &self.coordinates[idx];
                if shell.contains_point(pt) {
                    let mut inhole = false;
                    let mut i = 1;
                    while !inhole && i < rings.len() {
                        inhole = rings[i].contains_point(pt);
                        i += 1;
                    }
                    if !inhole {
                        ptset.insert(*pt);
                    }
                }
            }
            ptset.into_iter().for_each(|v| res.push(v));
        }
        return res;
    }


    ///Segments in range
    fn segs_in_range(&self, seglist: &mut Vec<Point>, inter: &MBR, i: i32, j: i32) {
        seglist.clear();
        for i in i..j {
            let (m, n) = (i as usize, (i + 1) as usize);
            if inter.intersects_bounds(
                &self.coordinates[m].as_array(),
                &self.coordinates[n].as_array(),
            ) {
                seglist.push(self.coordinates[m]);
                seglist.push(self.coordinates[n]);
            }
        }
    }


    /// Tests whether a collection of segments from line a and line b intersects
    fn seg_seg_intersects(&self, a_coords: &Vec<Point>, b_coords: &Vec<Point>) -> bool {
        // TODO:Improve O(n^2) - although expects few number of segs from index selection
        let mut bln = false;
        let na = a_coords.len();
        let nb = b_coords.len();
        let mut a = 0;
        while !bln && a < na {
            let mut b = 0;
            while !bln && b < nb {
                bln = segment::intersects(&a_coords[a], &a_coords[a + 1], &b_coords[b], &b_coords[b + 1]);
                b += 2;
            }
            a += 2;
        }
        return bln;
    }

    //Segment - Segment intersection of slice of arrays
    fn seg_seg_intersection(&self, a_coords: &Vec<Point>, b_coords: &Vec<Point>, ptset: &mut BTreeSet<Point>) {
        let (na, nb) = (a_coords.len(), b_coords.len());

        for a in (0..na).step_by(2) {
            let (a0, a1) = (&a_coords[a], &a_coords[a + 1]);
            for b in (0..nb).step_by(2) {
                let coord = segment::intersection(a0, a1, &b_coords[b], &b_coords[b + 1]);
                for idx in 0..coord.len() {
                    ptset.insert(coord[idx].pt);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! ln {
    ($($x:expr),*) => {
        {
            let mut vec:Vec<Point> = Vec::new();
            $( vec.push((&$x).into()); )*
            LineString::from_vec(vec)
        }
    };
}

impl Geometry for LineString {
    fn bbox(&self) -> MBR {
        self.bounds.mbr
    }

    fn as_linear(&self) -> Vec<LineString> {
        vec![self.clone()]
    }

    fn wkt_string(&self) -> String {
        self.wkt()
    }

    fn geom_type(&self) -> GeomType {
        GeomType::LineString
    }

    fn intersects<T: Geometry>(&self, other: &T) -> bool {
        if self.bounds.mbr.disjoint(&other.bbox()) {
            false
        } else if other.geom_type().is_polygon() {
            self.intersects_polygon(other.linear_rings())
        } else {
            //assume as_linear of other is > 1
            let mut bln = false;
            let lns = other.as_linear();
            let mut i = 0;
            while !bln && i < lns.len() {
                bln = self.intersects_linestring(&lns[i]);
                i += 1;
            }
            bln
        }
    }

    //Checks if pt intersection other geometry
    fn intersection<T: Geometry>(&self, other: &T) -> Vec<Point> {
        if other.geom_type().is_polygon() {
            self.intersection_polygon_rings(other.linear_rings())
        } else {
            //assume as_linear of other is > 1
            let mut ptset = BTreeSet::new();
            let lns = other.as_linear();
            for ln in lns.iter() {
                self.linear_intersection(ln).iter()
                    .for_each(|p| { ptset.insert(*p); });
            }
            ptset.into_iter().collect()
        }
    }
}

impl From<&str> for LineString {
    fn from(wkt_str: &str) -> Self {
        let o = parse_wkt(wkt_str);
        match o.geom_type {
            GeomType::LineString => {
                LineString::new(&o.coordinates[0])
            }
            _ => {
                let msg = if o.success {
                    format!("invalid wkt string, expected LINESTRING, got : {}", o.geom_type)
                } else {
                    format!("parser error : {}", o.message)
                };
                panic!(msg)
            }
        }
    }
}

impl From<Vec<Point>> for LineString {
    fn from(coordinates: Vec<Point>) -> Self {
        LineString::from_vec(coordinates)
    }
}

impl From<&Vec<Point>> for LineString {
    fn from(coordinates: &Vec<Point>) -> Self {
        LineString::new(coordinates)
    }
}

impl From<Vec<[f64; 2]>> for LineString {
    fn from(array: Vec<[f64; 2]>) -> Self {
        LineString::from_vec(
            array.iter()
                .map(|v| Point::new(v[0], v[1]))
                .collect::<Vec<Point>>()
        )
    }
}

impl From<&[Point]> for LineString {
    fn from(coordinates: &[Point]) -> Self {
        LineString::new(coordinates)
    }
}

impl std::fmt::Display for LineString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.wkt())
    }
}

#[cfg(test)]
mod tests;