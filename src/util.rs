use bbox_2d::MBR;
use math_util::Feq;
use rtree_2d::RTree;
use crate::mono::NULL_INDEX;
use crate::{Point, Coordinate, MonoMBR};


const MINI_MONO_SIZE: usize = 8;

#[derive(Copy, Clone)]
pub enum Sign { Pos, Zero, Neg, Non }

impl Sign {
    fn is_non(&self) -> bool {
        match self {
            Sign::Non => true,
            _ => false
        }
    }
    fn is_same(&self, other: &Sign) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub fn process_chains(coordinates: &Vec<Point>) -> (MonoMBR, Vec<MonoMBR>) {
    let n = coordinates.len();
    let (i, j) = (0usize, n - 1);

    if n < MINI_MONO_SIZE {
        let mut mbr = MBR::new_from_pt(coordinates[0].as_array());
        for pt in coordinates.iter() {
            mbr.expand_to_include_xy(pt.x, pt.y);
        }
        let mut o = MonoMBR::new_mono(mbr);
        o.i = 0;
        o.j = n - 1;
        return (o, vec![o]);
    }

    let mono_limit = ((j + 1) as f64 + 1.0).log2() as i32;
    let mut prev_x = Sign::Non;
    let mut prev_y = Sign::Non;

    let mut chains = Vec::new();
    let mut root_bbox = MonoMBR::new_mono(coordinates[0].as_array().into());
    let mut mbox = root_bbox;

    xy_mono_box(coordinates, &mut mbox, &mut root_bbox, i, i);
    chains.push(mbox);

    let mut mono_size = 0;
    let mut m_index = chains.len() - 1;


    for i in (i + 1)..=j {
        let a = coordinates[(i - 1)];
        let b = coordinates[i];
        let delta = b.sub(&a);
        let cur_x = xy_sign(delta.x);
        let cur_y = xy_sign(delta.y);

        if prev_x.is_non() {
            prev_x = cur_x
        }

        if prev_y.is_non() {
            prev_y = cur_y
        }

        mono_size += 1;
        if prev_x.is_same( &cur_x) && prev_y.is_same(&cur_y) && mono_size <= mono_limit {
            xy_mono_box(coordinates, &mut chains[m_index], &mut root_bbox, i, NULL_INDEX)
        } else {
            mono_size = 1;
            prev_x = cur_x;
            prev_y = cur_y;
            mbox = MonoMBR::new(coordinates[(i - 1) as usize], coordinates[i as usize]);

            xy_mono_box(coordinates, &mut mbox, &mut root_bbox, i - 1, i);
            chains.push(mbox);

            m_index = chains.len() - 1;
        }
    }
    (root_bbox, chains)
}

//compute bbox of x or y mono chain
fn xy_mono_box(coordinates: &Vec<Point>, mbox: &mut MonoMBR, root_bbox: &mut MonoMBR, i: usize, j: usize) {
    if i != NULL_INDEX {
        let pt = coordinates[i as usize];
        mbox.mbr.expand_to_include_xy(pt.x, pt.y);
        if j == NULL_INDEX {
            mbox.j = i;
        } else {
            mbox.i = i;
            mbox.j = j;
        }

        root_bbox.mbr.expand_to_include(&mbox.mbr);
        if root_bbox.i == NULL_INDEX {
            root_bbox.i = mbox.i;
            root_bbox.j = mbox.j;
        } else {
            if mbox.j > root_bbox.j {
                root_bbox.j = mbox.j;
            }
        }
    }
}

pub fn segment_db(coords: &Vec<Point>) -> RTree<MonoMBR> {
    let n = coords.len() - 1;
    let mut items = Vec::with_capacity(n);
    for i in 0..n {
        items.push(MonoMBR::new_mono_ij(coords[i], coords[i + 1], i, i + 1));
    }
    RTree::load(items)
}

pub fn query_bounds(coords: &Vec<Point>) -> Vec<MonoMBR> {
    let n = coords.len() - 1;
    let mut items = Vec::with_capacity(n);
    for i in 0..n {
        items.push(MonoMBR::new_mono_ij(coords[i], coords[i + 1], i , i + 1 ))
    }
    items
}


///find the sign of value -1, 0 , 1
#[inline]
pub fn xy_sign(v: f64) -> Sign {
    if v > 0. { Sign::Pos } else if v < 0. { Sign::Neg } else { Sign::Zero }
}


#[inline]
///Snap value to zero
pub fn snap_to_zero(x: f64) -> f64 {
    if x.feq(0.0) { 0.0 } else { x }
}

#[inline]
///Snap value to zero or one
pub fn snap_to_zero_or_one(x: f64) -> f64 {
    if x.feq(0.0) { 0.0 } else if x.feq(1.0) { 1.0 } else { x }
}


