use crate::formations::{Battalion, Company};
use std::cmp::max;

pub trait Interactable {
    fn manage_interaction(&mut self, other: &mut Company);
}

//has to be this way cos a company may be shielded by another company
impl Interactable for Company {
    fn manage_interaction(&mut self, other: &mut Company) {
        if (self.center - other.center).len() > max(
                self.select_radius + other.interaction_radius,
                other.select_radius + self.interaction_radius,
            )
        { return; }

        collide(self, other);


    }
}

impl Interactable for Battalion {
    fn manage_interaction(&mut self, other: &mut Company) {
        if (self.center - other.center).len() > max(
            self.select_radius + other.interaction_radius,
            other.select_radius + self.interaction_radius,
        )
        { return; }
    }
}
