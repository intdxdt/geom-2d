use crate::Point;

// description computes the convex hull of a point set.
// param points An array of [X, Y] coordinates
//fn convex_hull(points :&[Point]) -> Vec<Point> {
//	let pnts = points.ShallowClone();
//	//trivial case less than three coordinates
//	if points.len() < 3 {
//		return pnts
//	}
//	let N = pnts.len();
//
//	pnts.sort();
//
//	let lower = makeCoords(pnts, 0, N/2);
//	let upper = makeCoords(pnts, 0, N/2);
//
//	lower = buildHull(lower, pnts, 0, 1, N);
//	upper = buildHull(upper, pnts, N-1, -1, -1);
//
//	upper.Pop();
//	lower.Pop();
//
//	for _, v := range upper.Idxs {
//		lower.Idxs = append(lower.Idxs, v)
//	}
//
//	return lower
//}

////build boundary
//fn buildHull(hb, points Coords, start, step, stop int) Coords {
//	let pnt *Point
//	let i = start
//	let idx int
//	for i != stop {
//		idx, pnt = points.Idxs[i], points.Pt(i)
//		//pnt.CrossProduct(boundary[n - 2], boundary[n - 1])
//		for n := hb.len(); n >= 2 && pnt.SideOf(hb.Pt(n-2), hb.Pt(n-1)).IsOnOrRight(); n = hb.len() {
//			hb.Pop()
//		}
//		hb.Idxs = append(hb.Idxs, idx)
//		i += step
//	}
//	return hb
//}
//
////Coords returns a copy of linestring coordinates
//fn makeCoords(coordinates Coords, i, j int) Coords {
//	let o = Coords{Pnts: coordinates.Pnts, Idxs: make([]int, 0, j-i+1)}
//	return o
//}
//
