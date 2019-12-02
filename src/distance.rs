use crate::{LineString, util, Point, segment};
use crate::mono::MonoMBR;
use rtree_2d::KObj;

///Computes the distance between geometries
pub fn dist_as_lines(lns1: Vec<LineString>, lns2: Vec<LineString>) -> f64 {
    let mut dist = std::f64::NAN;
    for i in 0..lns1.len() {
        for j in 0..lns2.len() {
            let d = line_line_dist(&lns1[i], &lns2[j]);
            if dist.is_nan() {
                dist = d
            } else {
                dist = d.min(dist)
            }
        }
    }
    return dist;
}

///Computes the distance between a linestring and another linestring
///the distance between intersecting linestrings is 0.  Otherwise, the
///distance is the Euclidean distance between the closest segments.
fn line_line_dist(self_: &LineString, other: &LineString) -> f64 {
    if self_.coordinates.len() < 16 && other.coordinates.len() < 16 {
        return min_dist_brute_force(&self_, other);
    }
    knn_min_linear_distance(&self_.coordinates, &other.coordinates)
}

// brute force distance
fn min_dist_brute_force(self_: &LineString, other: &LineString) -> f64 {
    let mut dist = std::f64::MAX;
    let mut bln = false;
    let ln = &self_.coordinates;
    let ln2 = &other.coordinates;
    let (n1, n2) = (ln.len() - 1, ln2.len() - 1);
    let (mut i, mut j) = (0usize, 0);
    while !bln && i < n1 {
        while !bln && j < n2 {
            let d = segment::seg_seg_distance(&ln[i], &ln[i + 1], &ln2[j], &ln2[j + 1], f64::hypot);
            if d < dist {
                dist = d;
            }
            bln = (dist == 0.0);
            j += 1;
        }
        i += 1;
    }
    dist
}


fn knn_min_linear_distance(a_coords: &Vec<Point>, b_coords: &Vec<Point>) -> f64 {
    let (a, b) = if a_coords.len() > b_coords.len() {
        (b_coords, a_coords)
    } else {
        (a_coords, b_coords)
    };

    let db = util::segment_db(b);
    let queries = util::query_bounds(a);

    let mut min_dist = std::f64::MAX;
    let dist_fn = |query: &MonoMBR, item: &MonoMBR| {
        segment::seg_seg_distance(
            &a[query.i as usize], &a[query.j as usize],
            &b[item.i as usize], &b[item.j as usize],
            f64::hypot,
        )
    };
    let pred_fn = |o: KObj, dist: f64| {
        o.distance > dist || dist == 0f64 //add to neibs, stop
    };

    for q in queries.iter() {
        min_dist = db.knn_min_dist(q, dist_fn, pred_fn, min_dist)
    }

    return min_dist;
}
