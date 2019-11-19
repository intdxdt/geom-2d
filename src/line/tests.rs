use super::*;
use math_util::{round, Feq, SQRT_2, FRAC_PI_4};
use rstar::Point as RStarPoint;
use crate::{
    Point, Points, pts,
    Polygon, LineString, ln,
    GeomType, Geometry, Coordinate,
    parse_wkt, convex_hull,
};

#[test]
fn test_length() {
    let pts = pts![[5.6, 7.9], [5.6, 8.9], [6.6, 8.9], [6.6, 7.9], [5.6, 7.9]];
    let pt_array = pts![[5.6, 7.9,], [5.6, 8.9,], [6.6, 8.9, ], [6.6, 7.9, ], [5.6, 7.9, ]];
    let pt_array2 = pts![[5, 7], [5, 8], [6, 8], [6, 7 ], [5, 7]];

    let ln2 = ln![[5.538, 8.467], [5.498, 8.559], [5.858, 8.987], [6.654, 8.638], [6.549, 8.024], [5.765, 8.082], [5.538, 8.467]];
    let ln3 = ln![[5.538, 8.467], [5.498, 8.559], [5.858, 8.987], [6.654, 8.638], [6.549, 8.024], [5.765, 8.082]];

    let ln: LineString = (&pts).into();
    let ply = Polygon::new(&[ln2.coordinates.clone()]);

    let cln = ln.clone();
    let pt_lnstr = LineString::from_point(pts[0].into());

    //should test length
    assert!(ln.geom_type().is_line_string());
    assert_eq!(std::mem::discriminant(&ln.geom_type()), std::mem::discriminant(&GeomType::LineString));
    assert_eq!(ln.length(), 4.0);

    assert_eq!(pt_lnstr.length(), 0.0);
    assert_eq!(ln.area(), 0.0);
    assert_eq!(ln.len(ln.coordinates.len() - 1, 0) , ln.length());
    assert_eq!(ln3.area(), 0.0);
    assert_eq!(ln2.area(), ply.area());
}