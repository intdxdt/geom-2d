use crate::Point;

/// Computes the convex hull of a point set.
pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    let mut pnts = points.to_vec();
    //trivial case less than three coordinates
    if pnts.len() < 3 {
        return pnts;
    }
    let n = pnts.len() as i32;

    pnts.sort();

    let mut lower = Vec::with_capacity((n/2 + 1) as usize);
    let mut upper = Vec::with_capacity((n/2 + 1) as usize);

    build_hull(&mut lower, &mut pnts, 0, 1, n);
    build_hull(&mut upper, &mut pnts, n - 1, -1, -1);

    upper.pop();
    lower.pop();

    lower.extend_from_slice(&upper);
    lower
}

///build boundary
fn build_hull(hb: &mut Vec<Point>, points: &mut Vec<Point>, start: i32, step: i32, stop: i32) {
    let mut i = start;
    while i != stop {
        let pt = points[i as usize];
        let mut n = hb.len();
        while n >= 2 && pt.side_of(&hb[n - 2], &hb[n - 1]).is_on_or_right() {
            hb.pop();
            n = hb.len();
        }
        hb.push(pt);
        i += step
    }
}

