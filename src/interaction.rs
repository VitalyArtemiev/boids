use crate::units::{CompositeUnit, BasicUnit};
use std::cmp::max;

pub trait Interactable {
    fn manage_interaction(&mut self, other: &mut BasicUnit);
}

//has to be this way cos a company may be shielded by another company
impl Interactable for BasicUnit {
    fn manage_interaction(&mut self, other: &mut BasicUnit) {
        if (self.center - other.center).len() > max(
                self.select_radius + other.interaction_radius,
                other.select_radius + self.interaction_radius,
            )
        { return; }

        //collide(self, other);


    }
}

impl Interactable for CompositeUnit {
    fn manage_interaction(&mut self, other: &mut BasicUnit) {
        if (self.center - other.center).len() > max(
            self.select_radius + other.interaction_radius,
            other.select_radius + self.interaction_radius,
        )
        { return; }
    }
}
