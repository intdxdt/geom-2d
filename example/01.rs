use geom_2d::{Polygon, Geometry, LineString, Point, PointZ};
use std::ops::Index;

fn main() {
    let a: Polygon = "POLYGON (( 450 600, 450 725, 575 725, 575 600, 450 600 ))".into();
    let b: Polygon = "POLYGON (( 575 650, 575 775, 650 775, 650 650, 575 650 ))".into();
    let c: Polygon = "POLYGON (( 675 475, 675 550, 825 550, 825 475, 675 475 ))".into();
    println!("area = {}", a.area());
    println!("a <intersects> b = {}", a.intersects(&b));
    println!("a <distance>   b = {}", a.distance(&b));
    println!("a <distance>   c = {}", a.distance(&c));
    let inters = a.intersection(&b);
    println!("a <intersection> b : ");
    inters.iter().for_each(|o| println!("{}", o));

    let ln_wkt = "LINESTRING ( 757.9565217391305 725.7391304347826, 757.9565217391305 696.608695652174, 744.0434782608696 661.8260869565217, 709.2608695652174 656.1739130434783, 714.4782608695652 621.8260869565217, 693.1739130434783 589.6521739130435, 597.5217391304348 591.3913043478261, 607.9565217391304 548.3478260869565, 608.8260869565217 519.2173913043478, 569.695652173913 510.9565217391304, 540.9999999999999 531.8260869565217, 494.91304347826076 551.3913043478261, 467.5217391304347 534.8695652173913, 436.65217391304344 555.7391304347826 )";
    let ln: LineString = ln_wkt.into();
    let pt: Point = "POINT ( 650 550 )".into();

    println!("a  <distance> ln = {}", a.distance(&ln));
    println!("a  <distance> ln = {}", a.distance(&ln));
    println!("a  <distance> pt = {}", a.distance(&pt));
    println!("pt <distance> ln = {}", pt.distance(&ln));

    let mut pts : Vec<&dyn Index<usize, Output=f64>> = vec![];
    let (a, b) = (Point::new(3., 4.), Point::new(3.2, 4.7));
    let (c, d) = (PointZ::new(31., 4., 9.7), PointZ::new(3.2, 4.7, 9.3));
    pts.push(&a);
    pts.push(&b);
    pts.push(&c);
    pts.push(&d);
    println!("{}", pts.len());
    println!("{}", pts[2][0]);

}

