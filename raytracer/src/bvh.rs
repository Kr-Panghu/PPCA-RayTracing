//层次包围盒
use crate::scene;
use std::rc::Rc;

pub struct bvh_node {
    left: Rc<dyn scene::hittable>,
    right: Rc<dyn scene::hittable>,
    //Box: aabb,
}