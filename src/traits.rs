use std::collections::VecDeque;
use crate::container::Goal;
use crate::formations::Goal;
use crate::ops::Vec2f;
use crate::player::PlayerAction;

pub trait Clickable {
    fn is_on_screen(&self) -> bool;
    fn on_click(&self);
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