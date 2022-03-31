use std::any::Any;
use crate::container::Container;
use crate::ops::Vec2f;
use serde::{Deserialize, Serialize};
use crate::formations::{Battalion, Company};
use crate::interaction::manage_interaction;

pub(crate) type WorldId = usize;

///Used to select/deselect everything in the world
pub const WORLD_ID: WorldId = 0;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub groups: Vec<Any>,

    //pub nodes: Graph,
    //pub terrain: Array2D<i8>
}

const BOID_NUM: usize = 20;

impl World {
    pub fn single_container() -> World {
        World {
            groups: vec![Container::new(Vec2f::default(), BOID_NUM)],
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

        let companies = self.groups.iter().filter(|g| {g.is(Company)}).map(|c| c as Company);
        let battalions = self.groups.iter().filter(|g| {g.is(Company)}).map(|b| b as Battalion);
        //self.groups.group_by(|g1, g2| g1.type_id() == g2.type_id());

        for mut company in companies {
            for mut battalion in battalions {
                battalion.manage_interaction(&mut company);
                //manage_interaction(battalion, company);
            }
        }
    }

    //pub fn assign
}

pub trait Identifiable {
    fn generate_id() -> WorldId;
}
