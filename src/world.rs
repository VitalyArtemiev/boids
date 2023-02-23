use std::any::Any;
use crate::container::Container;
use crate::ops::Vec2f;
use serde::{Deserialize, Serialize};
use crate::drawable::Drawable;
use crate::units::{BasicUnit, CompositeUnit, Unit};
use crate::interaction::Interactable;

pub(crate) type WorldId = usize;

///Used to select/deselect everything in the world
pub const WORLD_ID: WorldId = 0;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub groups: Vec<Unit>,

    //pub nodes: Graph,
    //pub terrain: Array2D<i8>
}

const BOID_NUM: usize = 20;

impl Default for World {
    fn default() -> Self {
        World {
            groups: vec![]
        }
    }
}

impl World {
    pub fn single_company() -> Self {
        World {
            groups: vec![Unit::BasicUnit(BasicUnit::new(Vec2f::default(), BOID_NUM))],
        }
    }

    pub fn single_battalion(num_companies: u8, units_per_company: u8) -> Self {
        World {
            groups: vec![Unit::CompositeUnit(CompositeUnit::new(Vec2f::default(), BOID_NUM))],
        }
    }

    //maybe results should be in a hashset?
    pub fn get_ids_at(&self, pos: Vec2f) -> Vec<WorldId> {
        let mut sel = vec![];

        for group in self.groups.iter() {
            if group.is_in_bounds(pos) {
                sel.push(group.id);
                if let Some(b) = group.get_boid_at(pos) {
                    sel.push(b);
                }
            }
        }

        sel
    }

    //maybe results should be in a hashset?
    pub(crate) fn get_ids_in_rect(&self, p0: Vec2f, p1: Vec2f) -> Vec<WorldId> {
        let mut sel = vec![];

        for group in self.groups.iter() {}

        sel
    }

    pub(crate) fn process_interactions(&self) {
        for mut group in self.groups {
            for mut other in self.groups {
                group.manage_interaction(&mut other);
            }
        }
    }

    //pub fn assign
}
