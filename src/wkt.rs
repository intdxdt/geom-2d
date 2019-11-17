use crate::{GeomType, Point};

#[derive(Clone, Debug)]
pub struct WKT {
    pub geom_type: GeomType,
    pub coordinates: Vec<Vec<Point>>,
    pub success: bool,
    pub message: String,
}

pub fn parse_wkt(s: &str) -> WKT {
    let geom_type;
    let mut wkt;
    let res = wkt::Wkt::from_str(s);
    match res {
        Ok(o) => wkt = o,
        Err(err) => {
            return WKT {
                geom_type: GeomType::Unknown,
                coordinates: vec![],
                success: false,
                message: err.into(),
            };
        }
    };

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
                let shell = extract_coordinates(&ln.0);
                shells.push(shell);
            }
            shells
        }
        _ => unreachable!(),
    };

    WKT {
        geom_type,
        coordinates,
        success: true,
        message: String::new(),
    }
}



