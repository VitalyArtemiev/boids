use crate::app::CLICK_PRECISION;
use crate::container::{get_boid_container, is_boid_of_container, is_container};
use crate::ops::Vec2f;
use crate::player::PlayerAction::{Deselect, FormUp, Move, Select};
use crate::world::{World, WorldId, WORLD_ID};
use std::collections::HashSet;

#[derive(Default)]
pub struct PlayerState {
    pub pos: Vec2f,

    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,

    pub ctrl_pressed: bool,
    pub shift_pressed: bool,

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
    pub fn update_player_action(&mut self, world: &World) {
        if !self.selected.is_empty() {
            println!("SELECTED: {:?}", self.selected)
        }

        if self.l_click {
            if (self.l2 - self.l1).man() < CLICK_PRECISION {
                //println!("LCLICK");
                let mut ids = world.get_ids_at(self.l2);

                if self.ctrl_pressed {
                    for id in ids {
                        if self.selected.contains(&id) {
                            self.selected.remove(&id);
                            //println!("remove selection");
                        } else {
                            /*if is_container(id) {
                                self.selected.drain_filter(|sel| is_boid_of_container(*sel, id));
                            }*/
                            self.selected.insert(id);
                            //println!("add selection");
                        }
                    }
                } else if ids.is_empty() {
                    self.selected.clear();
                    //println!("clear selection");
                } else if self.selected.is_empty() {
                    self.selected.insert(*ids.first().unwrap());
                } else {
                    println!("ids {:?}", ids);
                    let containers: Vec<WorldId> =
                        ids.drain_filter(|id| is_container(*id)).collect();
                    let boids = ids;

                    println!("c {:?}", containers);
                    println!("b {:?}", boids);

                    //select container of previously selected boid
                    for boid in &boids {
                        if self.selected.contains(boid) {
                            //println!("select cont");
                            self.selected.clear();
                            self.selected.insert(get_boid_container(*boid).unwrap());
                            println!("SELECTING: {:?}", self.selected);
                            return;
                        }
                    }


                    //or select boid of previously selected container
                    for container in containers {
                        let boid = boids
                            .iter()
                            .find(|&id| is_boid_of_container(*id, container));
                        if self.selected.contains(&container) {
                            //println!("select boid");

                            self.selected.clear();
                            if let Some(id) = boid {
                                self.selected.insert(*id);
                            }
                            return;
                        }
                    }

                    //if a boid was selected previously, allow selection of neighbor boid
                    if let Some(boid) = boids.iter().find(|&id| !self.selected.contains(id)) {
                        self.selected.clear();
                        self.selected.insert(*boid);
                    }
                }
            } else {
                //println!("LDRAG");
                let mut ids = world.get_ids_in_rect(self.l1, self.l2);
            };
        } else if self.r_click {
            if (self.r2 - self.r1).man() < CLICK_PRECISION {
                //println!("RCLICK");

                self.action = Move(self.r2) //RMB click
            } else {
                //println!("RDRAG");
                self.action = FormUp(self.r2 - self.r1) //RMB drag
            };
        } else {
            self.action = PlayerAction::None
        }
    }
}
