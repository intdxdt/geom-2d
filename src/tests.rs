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
    pts[1..].iter().for_each(|v| { bbox.expand_to_include_point(v.as_array()); });
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

#[test]
fn test_line_string_relate() {
    let coords = pts![[0.5, 0.5], [0.06, -0.1], [0.26, -0.61], [0, -1], [-1.5, -1.], [-0.5, -0.5]];
    let coords2 = pts![[0.64, 1.72], [1.18, 1.87], [1.68, 1.43], [0.54, 1.38]];
    let plywkt = "POLYGON (( 0.64 1.72, 1.18 1.87, 1.68 1.43, 0.54 1.38, 0.64 1.72 ), (0.9471 1.5300, 0.9471 1.7102, 1.0653 1.7102, 1.0653 1.5300, 0.9471 1.5300 ))";
    let plywktc = "POLYGON (( 0.9694190834241365 1.6351888097521738, 0.9963995357624527 1.6647388289798535, 1.013101720543315 1.6467518607543095, 1.032373472213541 1.6608844786458083, 1.0465060901050398 1.6454670773096276, 1.0278767301571547 1.6313344594181287, 1.0400821728816312 1.6152746663596074, 0.9880484433720215 1.6094931408585396, 0.9694190834241365 1.6351888097521738 ))";

    let plywktd = "POLYGON (( 1.06137745847723 1.0766292071767967, 0.9394836291630517 0.8815990802741116, 1.3140301228738902 0.7752190110544651, 1.593277804575462 1.0034929095882898, 1.2453263281695353 1.185225527838519, 1.06137745847723 1.0766292071767967 ),( 1.2364613224012313 1.0832779615030246, 1.1212162474132812 1.0012766581462138, 1.2364613224012313 0.9303566119997828, 1.3472738945050298 0.9613841321888464, 1.4093289348831568 1.0300879268932013, 1.3384088887367258 1.0877104643871764, 1.2364613224012313 1.0832779615030246 ),( 1.1721900305810282 0.850571560085048, 1.1721900305810282 0.8838153317161875, 1.3517063973891816 0.8838153317161875, 1.3517063973891816 0.850571560085048, 1.1721900305810282 0.850571560085048 ))";
    let plywkte = "POLYGON (( -0.2405548235983036 -0.1291889913629033, 0.3266242131459507 0.0813804713804726, 0.5032308593178143 -0.0442819499341227, 0.5747608247298315 -0.5006764018668276, 0.3368130581174044 -0.6318386766212843, 0.3979461279461264 -0.2718328209632546, 0.224735763431414 -0.1461703996486594, -0.2099882886839426 -0.292210510906162, -0.2405548235983036 -0.1291889913629033 ))";
    let plywktf = "POLYGON (( -0.277913921826967 -0.5367427902210501, -0.4850871029131916 -0.6997643097643087, -0.3424432733128402 -0.8152378861074503, -0.2337622602840011 -0.7303308446786697, -0.1080998389694059 -0.8661821109647186, 0.0141663006880382 -0.7371234079929722, -0.1182886839408595 -0.5842907334211672, -0.277913921826967 -0.5367427902210501 ))";
    let plywktg = "POLYGON (( 0.1161332552173457 -0.4654208754208744, 0.1161332552173457 -0.2431398008042315, 0.2824725516029848 -0.2431398008042315, 0.2824725516029848 -0.4654208754208744, 0.1161332552173457 -0.4654208754208744 ))";

    let lna = LineString::new(&coords);
    let lnb = LineString::new(&coords2);
    let plya = Polygon::from_vec(&coords2);

    let plyb = Polygon::from_wkt(plywkt);
    let plyc = Polygon::from_wkt(plywktc);
    let plyd = Polygon::from_wkt(plywktd);
    let plye = Polygon::from_wkt(plywkte);
    let plyf = Polygon::from_wkt(plywktf);
    let plyg = Polygon::from_wkt(plywktg);

    assert!(lna.bbox().equals(&lna.bbox()));
    assert!(!lna.intersects(&lnb));
    assert!(!lna.intersects(&plya));
    assert!(!lna.intersects(&plya));

    assert!(!plyb.intersects(&plyc));

    assert!(!plya.intersects(&lna));
    assert!(!plyb.intersects(&lna));

    assert!(!plyd.intersects(&plyb));
    assert!(!plyb.intersects(&plyd));

    assert!(!plyd.intersects(&plyc));
    assert!(!plyc.intersects(&plyd));

    assert!(!plye.intersects(&plyg));
    assert!(!plyg.intersects(&plye));

    assert!(!lna.intersects(&plyb));
    assert!(lna.intersects(&plye));
    assert!(!lna.intersects(&plyf));
}

#[test]
fn test_line_string_mono() {
    let pts = pts![[5.78, 8.07], [6.44, 9.09], [7.87, 9.61]];
    let ln = LineString::new(&pts);
    let n = ln.coordinates.len() as i32;

    let (a, b) = (pts[0], pts[(n - 1) as usize]);
    let bounds = MBR::new_from_bounds(a.as_array(), b.as_array());
    let mbox = MonoMBR { mbr: bounds, i: 0, j: n - 1 };
    assert_eq!(mbox.i, ln.bounds.i);
    assert_eq!(mbox.j, ln.bounds.j);
    assert_eq!(ln.bbox(), *mbox.bbox());
    assert_eq!(ln.bbox(), *mbox.bbox());
    assert_eq!(ln.bounds.mbr, mbox.mbr);
}

#[test]
fn test_geometry_intersection() {
    let lnwkt = pts![[350, 710], [400, 770], [450, 770], [480, 810], [570, 820], [670, 730], [720, 760], [930, 800]];
    let lnwkt2 = "LINESTRING ( 620 620, 720 690, 790 680, 820 630, 870 630, 910 600, 900 530, 800 450, 730 390, 680 420, 640 460, 600 480, 650 540, 690 570, 780 570, 730 630, 680 600, 610 570, 550 610 )";

    let plywkt = "POLYGON (( 720 760, 860 770, 950 700, 930 640, 800 600, 740 580, 730 500, 760 440, 720 360, 620 390, 510 480, 460 570, 440 630, 450 740, 480 810, 570 820, 570 770, 580 740, 670 730, 720 760 ), ( 630 670, 580 650, 590 600, 650 580, 710 600, 710 670, 630 670 ), ( 780 650, 800 640, 850 710, 830 720, 780 650 ))";
    let plywkt2 = "POLYGON (( 860 920, 950 880, 860 800, 930 720, 880 690, 830 700, 810 730, 790 790, 820 840, 810 870, 860 920 ), ( 840 750, 860 750, 850 800, 830 800, 840 750 ))";

    let ptAwkt = "POINT ( 620 620 )";
    let ptBwkt = "POINT ( 710 600 )";
    let ptCwkt = "POINT ( 722.1298042987639 582.0334837046336 )";
    let ptDwkt = "POINT ( 720 360 )";
    let ptEwkt = "POINT ( 730 600 )";
    let ptFwkt = "POINT ( 680 630 )";
    let ptGwkt = "POINT ( 780 660 )";
    let ptHwkt = "POINT ( 630 570 )";

    let polyAwkt = "POLYGON ((730 410, 920 500, 930 540, 930 580, 900 640, 810 650, 750 520, 730 410))";
    let polyBwkt = "POLYGON ((630 620, 730 410, 890 410, 1040 510, 1080 620, 1020 720, 690 720, 630 620))";

    let ln = LineString::from_vec(lnwkt.clone());
    let ln2 = LineString::from_wkt(lnwkt2);
    let ply = Polygon::from_wkt(plywkt);
    let ply2 = Polygon::from_wkt(plywkt2);

    let plyA = Polygon::from_wkt(polyAwkt);
    let plyB = Polygon::from_wkt(polyBwkt);

    let ptA = Point::from_wkt(ptAwkt);
    let ptB = Point::from_wkt(ptBwkt);
    let ptC = Point::from_wkt(ptCwkt);
    let ptD = Point::from_wkt(ptDwkt);
    let ptE = Point::from_wkt(ptEwkt);
    let ptF = Point::from_wkt(ptFwkt);
    let ptG = Point::from_wkt(ptGwkt);
    let ptH = Point::from_wkt(ptHwkt);

    let segAA = Segment::new(ptA, ptA);
    let segAB = Segment::new(ptA, ptB);
    let len = |v:Vec<Point> | v.len();
    //"Intersection with pt, seg, ln, poly"
    let inters = plyA.intersection(&plyB);
    assert_eq!(inters.len(), 7);

			assert_eq!(ply.intersection(&ln)  .len(),4);
			assert_eq!(ply.intersection(&ln2) .len(),22);
			assert_eq!(ply.intersection(&ply2).len(),13);

			assert_eq!(len(ptA.intersection(&ply)),0);
			assert_eq!(len(ply.intersection(&ptA)),0);

			assert_eq!(len(ply.intersection(&ptB)),1);
			assert_eq!(len(ptB.intersection(&ply)),1);

			assert_eq!(len(segAA.intersection(&ply)),0);
			assert_eq!(len(ply.intersection(  &segAA)),0);
			assert_eq!(len(segAB.intersection(&ply)),1);

			assert_eq!(len(ptA.intersection(  &ln)),0);
			assert_eq!(len(ln.intersection(   &ptA)),0);
			assert_eq!(len(segAB.intersection(&ptA)),1);
			assert_eq!(len(ptA.intersection(  &segAB)),1);
			assert_eq!(len(ply.intersection(  &segAB)),1);

			assert_eq!(len(ply.intersection(&ptC)),1);
			assert_eq!(len(ptC.intersection(&ply)),1);

			assert_eq!(len(ply.intersection(&ptD)),1);
			assert_eq!(len(ptD.intersection(&ply)),1);

			assert_eq!(len(ptA.intersection(&ptB)),0);
			assert_eq!(len(ptA.intersection(&ptA)),1);
			assert_eq!(len(ptA.intersection(&ln2)),1);

			assert_eq!(len(ln2.intersection(&ptA)),1);
			assert_eq!(len(ln2.intersection(&ptB)),0);
			assert_eq!(len(ln2.intersection(&ptE)),0);
			assert_eq!(len(ln2.intersection(&ptF)),0);
			assert_eq!(len(ln2.intersection(&ptG)),0);
			assert_eq!(len(ln2.intersection(&ptH)),0);
}