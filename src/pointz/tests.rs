use super::*;
use crate::{Point, pt};
use rtree_2d::Point as RStarPoint;

#[test]
fn test_pointz() {
    let a = PointZ::new(3., 4., 0.45);
    let a_wkt = format!("{}", a);
    assert_eq!(a_wkt, "POINT M(3 4 0.45)".to_string());

    assert_eq!(a.val(0), 3.);
    assert_eq!(a.val(1), 4.);
    assert_eq!(a.val(2), 0.45);
    let mut b = ptz![4, 5, 6];
    *b.nth_mut(0) = 2.0;
    *b.nth_mut(1) = 1.5;
    *b.nth_mut(2) = 4.5;
    assert_eq!(b.nth(0), 2.);
    assert_eq!(b.nth(1), 1.5);
    assert_eq!(b.nth(2), 4.5);
    assert_eq!(b.as_tuple(), (2., 1.5, 4.5));
    assert_eq!(b.as_array(), [2., 1.5, 4.5]);
    assert_eq!(b.as_point(), pt![2., 1.5]);

    let c = PointZ::generate(|_| 0.3);
    assert_eq!(c.as_tuple(), (0.3, 0.3, 0.3));


    let pa = ptz![3, 4, 7];
    let mut m_pa = PointZ::from_array([3.0, 4.0, 6.7]);
    let pb = PointZ::new(3.0, 4.0, 7.);
    let pc = PointZ::new(5.0, 4.0, 8.45);

    assert_eq!(pa.as_tuple(), (3., 4., 7.));
    assert_eq!(pa.as_array(), [3.0, 4.0, 7.]);
    assert_eq!((pa[0], pa[1], pa[2]), (3., 4., 7.));

    assert_eq!((pa.nth(0), pa.nth(1), pa.nth(2)), (3., 4., 7.));
    assert_eq!((m_pa.nth(0), m_pa.nth(1), m_pa.nth(2)), (3., 4., 6.7));
    m_pa[0] = 0.;
    m_pa[1] = 5.;
    assert_eq!((m_pa[0], m_pa[1], m_pa[2]), (0., 5., 6.7));

    assert_eq!(pa, pb);
    assert_ne!(pa, pc);
    assert_ne!(pb, pc);
    assert!(pb != pc);
    assert!(pa.equals(&pb));

    let cb = pb.comp(&pc);
    assert_eq!(cb.as_tuple(), (-2.0, 0.0, 7. - 8.45));
}

#[test]
fn test_distance_magnitude() {
    let pts: PointZs = vec![[0, 0, 0], [4, 3, 0], [0, 0, 0], [1, 1, 0], [4, 5, 6], [7, 8, 9]].into();
    let q = [4., 5., 6.];
    let zv = vec![7., 8., 9., 99.9];
    let a = ptz![0, 0, 0];
    let b: PointZ = (4, 3, 0).into();
    let z: PointZ = [0, 0, 0].into();
    let o: PointZ = (&[1, 1, 0]).into();
    let qz: PointZ = (&q[..]).into();
    let zz: PointZ = (&zv).into();
    assert_eq!(a, pts[0]);
    assert_eq!(b, pts[1]);
    assert_eq!(z, pts[2]);
    assert_eq!(o, pts[3]);
    assert_eq!(qz, pts[4]);
    assert_eq!(zz, pts[5]);

    let pts: PointZs = vec![[4, 5, 6], [0, 0, 0], [7, 8, 9],[0, 0, 0], [4, 3, 0], [1, 1, 0]].into();
    let mut ptzs = pts.points;
    ptzs.sort();
    println!("{}", ptzs[0])
}

#[test]
fn test_serialize_deserialize() {
    let point = ptz!(1, 2,6);
    let serialized = serde_json::to_string(&point).unwrap();
    assert_eq!(serialized, String::from(r#"{"x":1.0,"y":2.0,"z":6.0}"#));
    let deserialized: PointZ = serde_json::from_str(r#"{"x":1.0,"y":2.0,"z":6.0}"#).unwrap();
    assert_eq!(point, deserialized);
    let deser_array: PointZ = serde_json::from_str("[1.0,2.0,6.0]").unwrap();
    assert_eq!(point, deser_array);
}