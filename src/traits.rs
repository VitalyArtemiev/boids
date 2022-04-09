use std::collections::VecDeque;
use crate::formations::Goal;
use crate::ops::Vec2f;
use crate::player::PlayerAction;
use crate::world::WorldId;

//const STRUCTURE_BITS: usize = 0b0011_1111_1111_0000_0000_0000_0000_0000; this is more robust
//const CONTAINER_BITS: usize = 0b0000_0000_0000_1111_1111_1100_0000_0000;

pub trait Identifiable {
    fn get_id(&self) -> WorldId;
    fn get_member_ids(&self) -> Vec<WorldId>;
}

pub trait Clickable {
    fn is_on_screen(&self) -> bool;
    fn on_click(&self);
    
    fn is_in_bounds(&self, p: Vec2f) -> bool {
        /*let t: Triangle<f64> = [[0.; 2]; 3];
        inside_triangle(t, p.into());*/

        (p - self.center).len() < self.radius
    }

    //todo: bench normal loop; should be zero-cost
    /*pub fn get_boid_at(&self, c: Vec2f) -> Option<WorldId> {
        for (i, p) in self.ent.pos.iter().enumerate() {
            if (*p - c).man() < CLICK_PRECISION {
                return Some(self.id + i + 1);
            }
        }
        None
    }*/
}

pub trait Selectable {
    fn select(&self) {
        self.selected = true;
    }
    fn deselect(&self);
}

pub trait Controllable {
    fn new_order(&self, goal: Goal);
    fn add_order(&self, goal: Goal);
    ///maybe better to provide a getter?
    fn assign_goals(&self, mut goals: &VecDeque<Goal>, action: PlayerAction) {
        match action {
            PlayerAction::None => {}
            PlayerAction::Move(pos, dir) => {
                goals.clear();
                goals.push_back(Goal::Move(pos, dir.unwrap_or(self.direction)))
            }
            PlayerAction::AddMove(pos, dir) => {
                goals.push_back(Goal::Move(pos, dir.unwrap_or(self.direction)))
            }
            PlayerAction::FormUp(pos1, pos2) => {
                let center_front = (pos2 - pos1) * 0.5;
                let dir = center_front - self.center;

                goals.clear();
                goals.push_back(Goal::Front(pos2, pos1, dir))
            }
            PlayerAction::AddFormUp(pos1, pos2) => {
                let center_front = (pos2 - pos1) * 0.5;
                let dir = center_front - self.center;

                goals.push_back(Goal::Front(pos2, pos1, dir))
            }
        }
    }
}