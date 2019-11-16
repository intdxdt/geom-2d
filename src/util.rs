use rtree_2d::RTree;
use crate::{Point, Coordinate};
use crate::{
    MonoMBR,
    GeoType,
    Geometry,
};
use crate::mono::NULL_INDEX;
use bbox_2d::MBR;
use std::fmt::{Display, Formatter, Error};

const MINI_MONO_SIZE: i32 = 8;

pub fn process_chains(coordinates: &Vec<Point>) -> (MonoMBR, Vec<MonoMBR>) {
    let n = coordinates.len() as i32;
    let (i, j) = (0i32, n - 1);

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
    let mut prev_x = NULL_INDEX;
    let mut prev_y = NULL_INDEX;

    let mut chains = Vec::new();
    let mut root_bbox = MonoMBR::new_mono(coordinates[0].as_array().into());
    let mut mbox = root_bbox;

    xy_mono_box(coordinates, &mut mbox, &mut root_bbox, i, i);
    chains.push(mbox);

    let mut mono_size = 0;
    let mut m_index = chains.len() - 1;


    for i in (i + 1)..=j {
        let a = coordinates[(i - 1) as usize];
        let b = coordinates[i as usize];
        let delta = b.sub(&a);
        let cur_x = xy_sign(delta.x);
        let cur_y = xy_sign(delta.y);

        if prev_x == NULL_INDEX {
            prev_x = cur_x
        }

        if prev_y == NULL_INDEX {
            prev_y = cur_y
        }

        mono_size += 1;
        if prev_x == cur_x && prev_y == cur_y && mono_size <= mono_limit {
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
fn xy_mono_box(coordinates: &Vec<Point>, mbox: &mut MonoMBR, root_bbox: &mut MonoMBR, i: i32, j: i32) {
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


///find the sign of value -1, 0 , 1
#[inline]
fn xy_sign(v: f64) -> i32 {
    if v > 0. { 1 } else if v < 0. { -1 } else { 0 }
}