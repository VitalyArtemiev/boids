use crate::formations::{Battalion, Company};
use std::cmp::max;

//has to be this way cos a company may be shielded by another company
impl Company {
    pub fn manage_interaction(&mut self, other: &mut Company) {
        if (self.center - other.center).len() > max(
                self.select_radius + other.interaction_radius,
                other.select_radius + self.interaction_radius,
            )
        { return; }

        collide(self, other);


    }
}

impl Battalion {
    pub fn manage_interaction(&mut self, other: &mut Company) {
        if (self.center - other.center).len() > max(
            self.select_radius + other.interaction_radius,
            other.select_radius + self.interaction_radius,
        )
        { return; }
    }
}
