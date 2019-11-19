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
#[should_panic]
fn test_construct_0() {
    let coords: Vec<Point> = vec![];
    let ln = LineString::new(&coords);
}

#[test]
#[should_panic]
fn test_construct_1() {
    let coords: Vec<Point> = vec![];
    let ln = LineString::from_vec(coords);
}

#[test]
fn test_to_array_and_re_construct() {
    let pts = pts![[5.6, 7.9], [5.6, 8.9], [6.6, 8.9], [6.6, 7.9], [5.6, 7.9]];
    let pt_array = vec![[5.6, 7.9, ], [5.6, 8.9, ], [6.6, 8.9, ], [6.6, 7.9, ], [5.6, 7.9, ]];
    let mut ln: LineString = (&pts).into();
    assert_eq!(ln.as_array(), pt_array);
    assert_eq!(ln.clone().as_array(), pt_array);
    ln.re_construct();
    assert_eq!(ln.as_array(), pt_array);

}

#[test]
fn test_wkt_construct() {
    let ln = ln![[5.6, 7.9], [5.6, 8.9], [6.6, 8.9], [6.6, 7.9], [5.6, 7.9]];
    let ln_str = "LINESTRING(5.6 7.9,5.6 8.9,6.6 8.9,6.6 7.9,5.6 7.9)".to_string();
    assert_eq!(ln.wkt(), ln_str);
    let ln_2: LineString = ln_str.as_str().into();
    assert_eq!(ln.wkt(), ln_2.wkt());

}

#[test]
fn test_bounds() {
    let pts = pts![[5.6, 7.9], [5.6, 8.9], [6.6, 8.9], [6.6, 7.9], [5.6, 7.9]];
    let pt_array = vec![[5.6, 7.9, ], [5.6, 8.9, ], [6.6, 8.9, ], [6.6, 7.9, ], [5.6, 7.9, ]];
    let ln: LineString = (&pts).into();
    let mut bbox = MBR::new_from_pt(pts[0].as_array());
    pts[1..].iter().for_each(|v| {bbox.expand_to_include_point(v.as_array());});
    assert!(ln.bbox().equals(&bbox));

}

#[test]
fn test_length() {
    let pts = pts![[5.6, 7.9], [5.6, 8.9], [6.6, 8.9], [6.6, 7.9], [5.6, 7.9]];
    let pt_array = vec![[5.6, 7.9, ], [5.6, 8.9, ], [6.6, 8.9, ], [6.6, 7.9, ], [5.6, 7.9, ]];
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
    assert_eq!(ln.len(ln.coordinates.len() - 1, 0), ln.length());
    assert_eq!(ln3.area(), 0.0);
    assert_eq!(ln2.area(), ply.area());
}