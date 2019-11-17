use rtree_2d::RTree;
use crate::{Point, Coordinate, segment, parse_wkt};
use crate::{
    util,
    MonoMBR,
    GeomType,
    Geometry,
};
use crate::mono::NULL_INDEX;
use bbox_2d::MBR;
use rtree_2d::{RTreeObject, PointDistance, AABB, Envelope};


#[derive(Clone, Debug)]
pub struct LineString {
    pub coordinates: Vec<Point>,
    pub bbox: MonoMBR,
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
        LineString { coordinates, bbox, chains, index }
    }

    ///Linestring from point
    pub fn from_point(pt: Point) -> LineString {
        LineString::new(&[pt, pt])
    }

    ///Construct from wkt
    pub fn from_wkt(s: &str) -> LineString {
        s.into()
    }

    pub fn as_linear(&self) -> Vec<LineString> {
        vec![self.clone()]
    }

    pub fn wkt(&self) -> String {
        format!("LINESTRING({})", self.coordinates
            .iter()
            .map(|pt| pt.fmt_xy())
            .collect::<Vec<_>>()
            .join(","))
    }
}

//linear relate
impl LineString {
    ///Checks if line intersects other{LineString}
    fn intersects_linestring(&self, other: &LineString) -> bool {
        let mut bln = false;
        let mut othersegs = Vec::new();
        let mut selfsegs = Vec::new();
        let mut ln_range = Vec::new();

        //var qrng *mbr.MBR
        //var qbox, ibox *mono.MBR
        let in_range = self.index.search(&other.bbox.envelope());
        let mut i = 0;
        while !bln && i < in_range.len() {
            //search ln using ibox
            let ibox = in_range[i];
            ln_range = other.index.search(&ibox.envelope());

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
}

impl Geometry for LineString {
    fn bbox(&self) -> MBR {
        self.bbox.mbr
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
        let mut bln = false;
        let other_lns = other.as_linear();
        let shell = &other_lns[0];

        if self.bbox.mbr.disjoint(&shell.bbox.mbr) {
            bln = false
        }
//        else if other.geom_type().is_polygon() {
//            bln = self.intersects_polygon(other_lns)
//        }
        else {
            bln = self.intersects_linestring(shell)
        }
        bln
    }
}


impl From<&str> for LineString {
    fn from(wkt_str: &str) -> Self {
        let o = parse_wkt(wkt_str);
        match o.geom_type {
            GeomType::LineString => {
                LineString::new(&o.coordinates[0] )
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


impl std::fmt::Display for LineString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.wkt())
    }
}


#[cfg(test)]
mod tests;