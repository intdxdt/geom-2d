use crate::{Point, util, pt, LineString, Geometry, GeomType};
use math_util::{Feq, EPSILON};
use bbox_2d::MBR;
use crate::inter::{InterPoint, SELF_A, SELF_B, OTHER_A, OTHER_B};
use crate::util::{snap_to_zero, snap_to_zero_or_one};
use side_rel::Side;

#[derive(Copy, Clone)]
pub struct Segment {
    pub coordinates: [Point; 2]
}

impl Segment {
    pub fn new(a: Point, b: Point) -> Segment {
        Segment { coordinates: [a, b] }
    }

    #[inline]
    pub fn a(&self) -> &Point {
        &self.coordinates[0]
    }

    #[inline]
    pub fn b(&self) -> &Point {
        &self.coordinates[1]
    }

    ///Segment as line string
    pub fn as_linestring(&self) -> LineString {
        (&self.coordinates[..]).into()
    }

    ///Side of pt to segement
    pub fn side_of(&self, pt: Point) -> Side {
        pt.side_of(self.a(), self.b())
    }

    //Line segments intersects
    pub fn seg_seg_intersects(&self, other: &Segment) -> bool {
        return intersects(self.a(), self.b(), other.a(), other.b());
    }

    //Line segments intersection
    pub fn seg_seg_intersection(&self, other: &Segment) -> Vec<InterPoint> {
        return intersection(self.a(), self.b(), other.a(), other.b());
    }

    ///Is segment
    pub fn is_simple(&self) -> bool {
        true
    }

    pub fn wkt(&self) -> String {
        format!("LINESTRING({})", self.coordinates
            .iter()
            .map(|pt| pt.fmt_xy())
            .collect::<Vec<_>>()
            .join(","))
    }
}

//impl GeometryClone for Segment {
//    fn clone_box(&self) -> Box<dyn Geometry> {
//        Box::new(self.clone())
//    }
//}

impl Geometry for Segment {
    fn bbox(&self) -> MBR {
        MBR::new_from_bounds(self.a().as_array(), self.b().as_array())
    }

    fn as_linear(&self) -> Vec<LineString> {
        vec![self.as_linestring()]
    }

    fn wkt_string(&self) -> String {
        self.wkt()
    }

    fn geom_type(&self) -> GeomType {
        GeomType::Segment
    }

    fn intersects(&self, other: &dyn Geometry) -> bool {
        self.as_linestring().intersects(other)
    }

    fn intersection(&self, other: &dyn Geometry) -> Vec<Point> {
        self.as_linestring().intersection(other)
    }

    fn distance(&self, other: &dyn Geometry) -> f64 {
        self.as_linestring().distance(other)
    }
}

//do two lines intersect line segments a && b with
//vertices sa, sb, oa, ob
pub fn intersects(sa: &Point, sb: &Point, oa: &Point, ob: &Point) -> bool {
    let mut bln = false;
    let mut a = ((ob[0] - oa[0]) * (sa[1] - oa[1])) - ((ob[1] - oa[1]) * (sa[0] - oa[0]));
    let mut b = ((sb[0] - sa[0]) * (sa[1] - oa[1])) - ((sb[1] - sa[1]) * (sa[0] - oa[0]));
    let mut d = ((ob[1] - oa[1]) * (sb[0] - sa[0])) - ((ob[0] - oa[0]) * (sb[1] - sa[1]));

    //snap to zero if near -0 or 0
    a = util::snap_to_zero(a);
    b = util::snap_to_zero(b);
    d = util::snap_to_zero(d);

    if d.feq(0.) {
        if a == 0. && b == 0. {
            bln = MBR::new_from_bounds(sa.as_array(), sb.as_array()).intersects(
                &MBR::new_from_bounds(oa.as_array(), ob.as_array())
            )
        }
        return bln;
    }

    let (mut ua, mut ub) = (a / d, b / d);
    ua = util::snap_to_zero_or_one(ua);
    ub = util::snap_to_zero_or_one(ub);

    return (0. <= ua && ua <= 1.) && (0. <= ub && ub <= 1.);
}

//do two lines intersect line segments a && b with
//vertices lna0, lna1 and lnb0, lnb1
pub fn intersection(sa: &Point, sb: &Point, oa: &Point, ob: &Point) -> Vec<InterPoint> {
    let mut coords = Vec::new();
    let a = ((ob[0] - oa[0]) * (sa[1] - oa[1])) - ((ob[1] - oa[1]) * (sa[0] - oa[0]));
    let b = ((sb[0] - sa[0]) * (sa[1] - oa[1])) - ((sb[1] - sa[1]) * (sa[0] - oa[0]));
    let d = ((ob[1] - oa[1]) * (sb[0] - sa[0])) - ((ob[0] - oa[0]) * (sb[1] - sa[1]));

    //snap to zero if near -0 or 0
    let (a, b, d) = (
        snap_to_zero(a),
        snap_to_zero(b),
        snap_to_zero(d)
    );

    // are the line coincident?
    if d == 0.0 {
        return coincident_segs(sa, sb, oa, ob, &mut coords, a, b);
    }

    // is the intersection along the the segments
    let (mut ua, mut ub) = (a / d, b / d);
    ua = snap_to_zero_or_one(ua);
    ub = snap_to_zero_or_one(ub);

    let ua_0_1 = 0. <= ua && ua <= 1.;
    let ub_0_1 = 0. <= ub && ub <= 1.;

    if ua_0_1 && ub_0_1 {
        coords.push(InterPoint {
            pt: pt!(sa.x + ua*(sb.x-sa.x), sa.y + ua*(sb.y-sa.y)),
            inter: inter_relation(ua, ub),
        })
    }
    coords
}

fn inter_relation(ua: f64, ub: f64) -> u8 {
    let (mut sa, mut sb, mut oa, mut ob) = (0u8, 0, 0, 0);

    if ua == 0. {
        sa = SELF_A;
    } else if ua == 1. {
        sb = SELF_B;
    }

    if ub == 0. {
        oa = OTHER_A;
    } else if ub == 1. {
        ob = OTHER_B;
    }
    sa | sb | oa | ob
}

fn coincident_segs(sa: &Point, sb: &Point, oa: &Point, ob: &Point, coords: &mut Vec<InterPoint>, a: f64, b: f64) -> Vec<InterPoint> {
    if a == 0. && b == 0. {
        let s_box = MBR::new_from_bounds(sa.as_array(), sb.as_array());
        let o_box = MBR::new_from_bounds(oa.as_array(), ob.as_array());
        if s_box.intersects(&o_box) {
            update_coords_in_bounds(o_box, sa, coords, SELF_A);
            update_coords_in_bounds(o_box, sb, coords, SELF_B);
            update_coords_in_bounds(s_box, oa, coords, OTHER_A);
            update_coords_in_bounds(s_box, ob, coords, OTHER_B);
        }
    }

    //lexical sort
    coords.sort();

    let mut points = Vec::new();
    if coords.is_empty() {
        return points;
    }

    let mut last = false;
    let n = coords.len() - 1;
    let mut idx = 0;
    while idx < n { //O(n)
        let (mut i, mut j) = (idx, idx + 1);
        let mut pt = coords[i];
        while i < n && coords[i].equals(&coords[j]) {
            coords[j].inter = coords[i].inter | coords[j].inter;
            last = j == n;
            pt = coords[j];
            i = j;
            j = i + 1;
        }
        idx = i;
        points.push(pt);
        idx += 1;
    }

    if !last {
        points.push(coords[n])
    }
    points
}

//updates Coords that are in bounds
fn update_coords_in_bounds(bbox: MBR, point: &Point, intpts: &mut Vec<InterPoint>, vbits: u8) {
    if bbox.contains_xy(point.x, point.y) {
        intpts.push(InterPoint { pt: *point, inter: vbits })
    }
}

//Distance between two segments
pub fn distance(sa: &Point, sb: &Point, oa: &Point, ob: &Point) -> f64 {
    seg_seg_distance(sa, sb, oa, ob, f64::hypot)
}

//Distance between two segments
pub fn square_distance(sa: &Point, sb: &Point, oa: &Point, ob: &Point) -> f64 {
    return seg_seg_distance(sa, sb, oa, ob, |x, y| x * x + y * y);
}

//Distance between two segments with custom hypot function
pub fn seg_seg_distance(sa: &Point, sb: &Point, oa: &Point, ob: &Point, hypot: fn(f64, f64) -> f64) -> f64 {
    let dist;
    let (x1, y1) = (sa.x, sa.y);
    let (x2, y2) = (sb.x, sb.y);

    let (x3, y3) = (oa.x, oa.y);
    let (x4, y4) = (ob.x, ob.y);


    let denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    let numera = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
    let numerb = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);

    let mut pta = sa;
    let mut ptb = sb;
    if (denom.abs()) < EPSILON {
        let is_aspt_a = sa.equals(&sb);
        let is_aspt_b = oa.equals(&ob);

        if is_aspt_a && is_aspt_b {
            dist = hypot(x1 - x4, y1 - y4)
        } else if is_aspt_a || is_aspt_b {
            let (lna, lnb) = if is_aspt_a {
                pta = sa;
                (oa, ob)
            } else if is_aspt_b {
                pta = oa;
                (sa, sb)
            } else {
                unreachable!()
            };

            dist = dist_to_point(lna, lnb, pta, hypot)
        } else {
            dist = min_dist_segment_endpoints(sa, sb, oa, ob, hypot)
        }
    } else {
        let (mut use_pta, mut use_ptb) = (false, false);
        let mua = numera / denom;
        let mua = snap_to_zero_or_one(mua);

        let mub = numerb / denom;
        let mub = snap_to_zero_or_one(mub);

        if mua < 0f64 || mua > 1f64 || mub < 0f64 || mub > 1f64 {
            //the is intersection along the the segments
            if mua < 0f64 {
                pta = sa;
                use_pta = true;
            } else if mua > 1f64 {
                pta = sb;
                use_pta = true;
            }

            if mub < 0f64 {
                ptb = oa;
                use_ptb = true;
            } else if mub > 1f64 {
                ptb = ob;
                use_ptb = true;
            }

            if use_pta && use_ptb {
                dist = f64::min(
                    dist_to_point(oa, ob, pta, hypot),
                    dist_to_point(sa, sb, ptb, hypot),
                )
            } else if use_pta {
                dist = dist_to_point(oa, ob, pta, hypot)
            } else {
                dist = dist_to_point(sa, sb, ptb, hypot)
            }
        } else {
            dist = 0f64; //lines intersect
        }
    }
    dist
}

fn min_dist_segment_endpoints(sa: &Point, sb: &Point, oa: &Point, ob: &Point, hypot: fn(f64, f64) -> f64) -> f64 {
    let o_sa = dist_to_point(oa, ob, sa, hypot);
    let o_sb = dist_to_point(oa, ob, sb, hypot);
    let s_oa = dist_to_point(sa, sb, oa, hypot);
    let s_ob = dist_to_point(sa, sb, ob, hypot);
    (o_sa.min(o_sb)).min(s_oa.min(s_ob))
}

///Distance from segment endpoints to point
pub fn distance_to_point(sa: &Point, sb: &Point, pt: &Point) -> f64 {
    dist_to_point(sa, sb, pt, f64::hypot)
}

///Square Distance from segment endpoints to point
pub fn square_distance_to_point(sa: &Point, sb: &Point, pt: &Point) -> f64 {
    dist_to_point(sa, sb, pt, |x, y| x * x + y * y)
}

///Distance from segment endpoints to point
fn dist_to_point(sa: &Point, sb: &Point, pt: &Point, hypot: fn(f64, f64) -> f64) -> f64 {
    let (ax, ay) = (sa.x, sa.y);
    let (bx, by) = (sb.x, sb.y);
    let (px, py) = (pt.x, pt.y);
    let (dx, dy) = (bx - ax, by - ay);
    let isz_x = dx.feq(0f64);
    let isz_y = dy.feq(0f64);

    if isz_x && isz_y {
        //line with zero length
        hypot(px - ax, py - ay)
    } else {
        let u = (((px - ax) * dx) + ((py - ay) * dy)) / (dx * dx + dy * dy);

        let (c_ptx, c_pty) = if u < 0.0 {
            (ax, ay)
        } else if u > 1f64 {
            (bx, by)
        } else {
            (ax + u * dx, ay + u * dy)
        };
        hypot(px - c_ptx, py - c_pty)
    }
}
