use crate::{GeomType, Point};

#[derive(Clone, Debug)]
pub struct WKT {
    pub geom_type: GeomType,
    pub coordinates: Vec<Vec<Point>>,
}

pub fn read_wkt(s: &str) -> WKT {
    let geom_type ;
    let mut wkt = wkt::Wkt::from_str(s).ok().unwrap();

    let extract_coordinates = |coords: &Vec<wkt::types::Coord<f64>>| {
        let mut shell = vec![];
        for c in coords.iter() {
            shell.push(Point::new(c.x, c.y));
        }
        shell
    };

    let coordinates = match wkt.items.pop().unwrap() {
        wkt::Geometry::Point(wkt::types::Point(coords)) => {
            geom_type = GeomType::Point;
            let c = coords.unwrap();
            vec![vec![Point::new(c.x, c.y)]]
        }
        wkt::Geometry::LineString(wkt::types::LineString(coords)) => {
            geom_type = GeomType::LineString;
            let shell = extract_coordinates(&coords);
            vec![shell]
        }
        wkt::Geometry::Polygon(wkt::types::Polygon(lines)) => {
            geom_type = GeomType::Polygon;
            let mut shells = vec![];
            for ln in lines {
                let shell  = extract_coordinates(&ln.0);
                shells.push(shell);
            }
            shells
        }
        _ => unreachable!(),
    };

    WKT { geom_type, coordinates }
}



