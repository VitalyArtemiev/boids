use crate::ops::Vec2f;
use crate::player::PlayerAction::{Deselect, FormUp, Move, Select};
use crate::world::{World, WorldId, WORLD_ID};
use std::collections::HashSet;
use crate::app::CLICK_PRECISION;

#[derive(Default)]
pub struct PlayerState {
    pub pos: Vec2f,

    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,

    pub zoom: f32,
    pub to_zoom: f32, //Amount left to animate zooming in/out

    pub l1: Vec2f,
    pub l2: Vec2f,
    pub r1: Vec2f,
    pub r2: Vec2f,
    pub l_pressed: bool,
    pub r_pressed: bool,
    pub l_click: bool,
    pub r_click: bool,

    pub selected: HashSet<WorldId>,

    pub action: PlayerAction,
}

#[derive(Default, Copy, Clone)]
pub enum PlayerAction {
    #[default]
    None,
    Select(WorldId),
    Move(Vec2f),
    FormUp(Vec2f),
    Deselect(WorldId),
}

impl PlayerState {
    pub fn get_player_action(&mut self, world: &World) {
        if self.l_click {
            return if (self.l2 - self.l1).man() < CLICK_PRECISION {
                self.action = Deselect(WORLD_ID) //LMB click
            } else {
                let ids = world.get_ids_at(self.l2);
                self.action = Select(0) //LMB drag
            };
        }

        if self.r_click {
            return if (self.r2 - self.r1).man() < 3. {
                self.action = Move(self.r2) //RMB click
            } else {
                self.action = FormUp(self.r2 - self.r1) //LMB drag
            };
        }

        self.action = PlayerAction::None
    }
}
