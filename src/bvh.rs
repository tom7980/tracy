use std::str::MatchIndices;

use crate::bounding::*;
use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

pub struct BvhTree {
    hittables: Vec<Box<dyn Hittable>>,
    nodes: Vec<BvhSlab>,
    bounds: BoundingBox,
}

#[derive(Debug)]
pub enum BvhSlab {
    Leaf {
        parent_index: usize,
        shape_index: usize,
    },

    Node {
        parent_index: usize,
        bounds: BoundingBox,

        left_index: usize,

        right_index: usize,
    },
}

impl BvhSlab {
    pub fn traverse(
        nodes: &[BvhSlab],
        node_index: usize,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        objects: &[Box<dyn Hittable>],
    ) -> Option<HitRecord> {
        match &nodes[node_index] {
            BvhSlab::Node {
                parent_index,
                bounds,
                left_index,
                right_index,
            } => {
                if let Some(intersection) = bounds.intersects(r, t_min, t_max) {
                    let left_hit = BvhSlab::traverse(
                        nodes,
                        *left_index,
                        r,
                        intersection.tmin,
                        intersection.tmax,
                        objects,
                    );
                    let right_hit = BvhSlab::traverse(
                        nodes,
                        *right_index,
                        r,
                        intersection.tmin,
                        intersection.tmax,
                        objects,
                    );

                    match (left_hit, right_hit) {
                        (Some(a), Some(b)) => {
                            if a.t < b.t {
                                return Some(a);
                            } else {
                                return Some(b);
                            }
                        }
                        (Some(a), None) => return Some(a),
                        (None, Some(b)) => return Some(b),
                        (None, None) => return None,
                    }
                } else {
                    return None;
                }
            }
            BvhSlab::Leaf {
                parent_index,
                shape_index,
            } => {
                return objects[*shape_index].hit(r, t_min, t_max);
            }
        }
    }

    fn recurse_nodes(
        objs_list: &mut [Box<dyn Hittable>],
        indicies: &mut [usize],
        nodes: &mut Vec<BvhSlab>,
        index: usize,
    ) {
        let len = objs_list.len();
        let mid = len / 2;

        if len == 1 {
            nodes.insert(
                index,
                BvhSlab::Leaf {
                    parent_index: index,
                    shape_index: indicies[0],
                },
            );
            return;
        }

        let mut bbox = BoundingBox::empty();
        objs_list.iter().for_each(|obj| {
            bbox = BoundingBox::box_between(&bbox, obj.bounding_box());
        });

        let (left_objects, right_objects) = objs_list.split_at_mut(mid);
        let (left_indicies, right_indicies) = indicies.split_at_mut(mid);

        let left_len = left_indicies.len() * 2 - 1;

        nodes.insert(
            index,
            BvhSlab::Node {
                parent_index: index,
                bounds: bbox,
                left_index: index + 1,
                right_index: index + 1 + left_len,
            },
        );

        BvhSlab::recurse_nodes(left_objects, left_indicies, nodes, index + 1);
        BvhSlab::recurse_nodes(right_objects, right_indicies, nodes, index + 1 + left_len);
    }

    pub fn build_nodes(list: &mut [Box<dyn Hittable>]) -> Vec<BvhSlab> {
        let mut bbox = BoundingBox::empty();
        list.iter().for_each(|obj| {
            bbox = BoundingBox::box_between(&bbox, obj.bounding_box());
            println!("{:?}", bbox);
        });

        let axis = bbox.longest_axis();

        list.sort_by(|obj1, obj2| {
            obj1.bounding_box()
                .axis_length(axis)
                .partial_cmp(&obj2.bounding_box().axis_length(axis))
                .expect("Couldn't compare bounding boxes of objects to sort")
        });

        let mut vec: Vec<BvhSlab> = Vec::new();
        vec.reserve((list.len() * 2) - 1);

        let mut indicies: Vec<usize> = (0..list.len()).collect();

        BvhSlab::recurse_nodes(list, &mut indicies, &mut vec, 0);

        vec
    }
}

impl BvhTree {
    pub fn new() -> BvhTree {
        let bounds = BoundingBox::empty();
        BvhTree {
            hittables: Vec::new(),
            nodes: Vec::new(),
            bounds,
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        let bounds = BoundingBox::box_between(&self.bounds, object.bounding_box());
        self.bounds = bounds;
        self.hittables.push(object);

        let nodes = BvhSlab::build_nodes(&mut self.hittables);
        println!("{:?}", nodes);
        self.nodes = nodes;
    }
}

impl Hittable for BvhTree {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        BvhSlab::traverse(&self.nodes, 0, r, ray_tmin, ray_tmax, &self.hittables)
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounds
    }
}
