
use crate::LinearRing;


struct  Polygon {
	shell: LinearRing,
	holes: Vec<LinearRing>
}
