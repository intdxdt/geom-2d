use crate::{Point, util};
use math_util::Feq;
use bbox_2d::MBR;

//do two lines intersect line segments a && b with
//vertices sa, sb, oa, ob
pub fn intersects(sa: &Point, sb: &Point, oa: &Point, ob: &Point) -> bool {
    let mut bln = false;
    let mut a = ((ob[0] - oa[0]) * (sa[1] - oa[1])) - ((ob[1] - oa[1]) * (sa[0] - oa[0]));
    let mut b = ((sb[0] - sa[0]) * (sa[1] - oa[1])) - ((sb[1] - sa[1]) * (sa[0] - oa[0]));
    let mut d = ((ob[1] - oa[1]) * (sb[0] - sa[0])) - ((ob[0] - oa[0]) * (sb[1] - sa[1]));

    //snap to zero if near -0 or 0
    a = util::snap_to_zero(a);
    b = util::snap_to_zero(b);
    d = util::snap_to_zero(d);

    if d.feq(0.) {
        if a == 0. && b == 0. {
            bln = MBR::new_from_bounds(sa.as_array(), sb.as_array()).intersects(
				&MBR::new_from_bounds( oa.as_array(), ob.as_array())
			)
        }
        return bln;
    }

    let (mut ua, mut ub) = (a / d, b / d);
    ua = util::snap_to_zero_or_one(ua);
    ub = util::snap_to_zero_or_one(ub);

    return (0. <= ua && ua <= 1.) && (0. <= ub && ub <= 1.);
}
