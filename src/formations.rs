use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Deserialize, Serialize};
use crate::container::{Container};
use crate::ops::Vec2f;
use crate::traits::{Clickable, Controllable, Selectable};

use rand::Rng;
use crate::boids::BoidVec;
use crate::formations::Goal::Idle;
use crate::world::{Identifiable, WorldId};

const ACC_MAX: f64 = 1000.;
const VEL_MAX: f64 = 100.;
const DIST_REPEL: f64 = 20.;
const DIST_MARGIN: f64 = 1.;
pub const FORMATION_SPACING: f64 = 24.;
const COLUMN_WIDTH: i32 = 4;

pub type FormationFunction = fn(usize, usize) -> Vec2f;

pub fn default_formation(index: usize, _: usize) -> Vec2f {
    let mut rng = rand::thread_rng();
    Vec2f {
        x: rng.gen::<f64>() * FORMATION_SPACING,
        y: rng.gen::<f64>() * FORMATION_SPACING,
    }
}

pub fn crutch() -> FormationFunction {
    default_formation
}

pub fn phalanx_formation(index: usize, width: usize) -> Vec2f {
    Vec2f {
        x: index.checked_rem(width).unwrap_or_default() as f64,
        y: index.checked_div(width).unwrap_or_default() as f64,
    }
}

pub fn p_f(index: usize, width: usize, xdir_norm: Vec2f, ydir_norm: Vec2f) -> Vec2f {
    let x = index.checked_rem(width).unwrap_or_default() as f64;
    let y = index.checked_div(width).unwrap_or_default() as f64;

    xdir_norm * x + ydir_norm * y * FORMATION_SPACING
}

pub fn idle_formation(index: usize, width: usize) -> Vec2f {
    let mut rng = rand::thread_rng();
    let mut x = rng.gen::<f64>();
    let mut y = rng.gen::<f64>();

    while x * x + y * y > 1. {
        x = rng.gen::<f64>();
        y = rng.gen::<f64>();
    }

    Vec2f {
        x: x * FORMATION_SPACING,
        y: y * FORMATION_SPACING,
    }
}

#[derive(Serialize, Deserialize)]
pub enum Goal {
    Idle(Vec2f),
    Hold,
    Move(Vec2f, Vec2f),
    Column(Vec2f),
    Front(Vec2f, Vec2f, Vec2f),
}
#[derive(Default)]
struct TroopDesc {
    name: String,
    mass: f32,
    base_spd: f32,
    charge_spd: f32,

    mounted: bool,
    ranged: bool,
    ranged_ammo: u8,
    ///arrow/pilum reach on level plane
    ranged_base_reach: f32,

    ///reach of dagger
    melee_reach_cqb: f32,
    ///min usable reach of spear/sword
    melee_reach_standoff_min: f32,
    ///max usable reach of spear/sword
    melee_reach_standoff_max: f32,
    block_chance: f32,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Company  {
    pub center: Vec2f,
    pub direction: Vec2f,
    pub select_radius: f64,
    pub interaction_radius: f64,

    #[serde(skip)]
    #[serde(default = "crate::formations::crutch")]
    pub formation: FormationFunction,//remove this, change to enum maybe
    pub formation_positions: Vec<Vec2f>,

    ///.first is next goal
    pub goals: VecDeque<Goal>,

    pub selected: bool,

    pub fatigue: f32,
    pub morale: f32,
    pub experience: f32,
    pub avg_age: f32,

    pub troop_desc: TroopDesc,
    pub troops: Option<BoidVec>,
}

impl Company {
    fn new(pos: Vec2f, num: usize) -> Self {
        let mut c = Company {
            selected: false,
            fatigue: 0.0,
            morale: 1.0,
            experience: 0.0,
            avg_age: 18.0,
            center: pos,
            direction: Vec2f{x: 1.,y: 0.},
            select_radius: 0.0,
            goals: VecDeque::from([Idle(pos)]),
            formation: default_formation,
            formation_positions: Vec::with_capacity(num),
            troops: None,
            interaction_radius: 0.0,
            troop_desc: TroopDesc::default(),
        };
    }

    pub fn calculate_formation(&mut self, flen: f64) {
        let form_width = (flen / FORMATION_SPACING).round() as usize;

        for (i, pos) in self.formation_positions.iter_mut().enumerate() {
            *pos = (self.formation)(i, form_width);
        }
    }

    pub fn p_b(&mut self, dt: f64) {
        //calc cum_dist
        let mut cum_dist = 0.0;

        //get formation, rotate according to heading
        //after forming, rotate formation only within 1 quadrant, other dirs handled by rotating individual boids

        //let iter = ;

        let mut slice = self.ent.as_mut_slice();

        let slen = slice.len();


        for (i, boid) in slice.iter_mut().enumerate() {
            let d = self.formation_positions[i] - *boid.pos;

            let dist = d.len();
            cum_dist += dist;

            //let slice = self.ent.slice_mut(i+1..self.ent.len());

            /*for j in i+1..slen {
                slice.pos[j];
            }*/

            *boid.vel += d * dt;
            boid.vel.clamp(VEL_MAX.min(dist));

            *boid.pos += *boid.vel * dt;

            let heading: f64 = f64::atan2(boid.vel.y, boid.vel.x);

            if heading.is_normal() {
                *boid.r = heading;
            }
        }
        //project along a heading vector, maybe along a spline
        //the more the curve, the slower the step? or adjust speed manually
        //kinematic step
        //do collision detection, from inside out?
        if cum_dist < self.ent.len() as f64 * DIST_MARGIN {
            self.goals.pop_front();
            if self.goals.is_empty() {
                self.goals.push_back(Goal::Hold)
            }
        }
    }

}

const COMPANY_CAPACITY: usize = 256;

pub static NUM_COMPANIES: AtomicUsize = AtomicUsize::new(0);

impl Identifiable for Company {
    fn generate_id() -> WorldId {
        let nc = NUM_COMPANIES.fetch_add(1, Ordering::Relaxed);
        COMPANY_CAPACITY * (nc + 1)
    }
}

impl Clickable for Company {
    fn is_on_screen(&self) -> bool {
        todo!()
    }

    fn on_click(&self) {
        todo!()
    }
}

impl Selectable for Company {
    fn deselect(&self) {
        todo!()
    }
}

impl Controllable for Company {
    fn new_order(&self, goal: Goal) {
        todo!()
    }

    fn add_order(&self, goal: Goal) {
        todo!()
    }
}

pub struct DrillStep {
    company_type_id: usize,
    company_formation: FormationFunction,
    pos: Vec2f,
    dir: Vec2f,
    time: f32
}

pub struct Drill {
    steps: Vec<DrillStep>
}

#[derive(Serialize, Deserialize)]
pub struct Battalion {
    pub center: Vec2f,
    pub direction: Vec2f,
    pub select_radius: f64,
    pub interaction_radius: f64,

    ///.first is next goal
    pub goals: VecDeque<Goal>,

    pub selected: bool,

    pub fatigue: f32,
    pub morale: f32,

    pub formation_positions: Vec<Vec2f>,
    pub known_drills: Vec<Drill>,

    pub troops: Vec<Company>,
}

impl Clickable for Battalion {
    fn is_on_screen(&self) -> bool {
        todo!()
    }

    fn on_click(&self) {
        todo!()
    }
}

impl Selectable for Battalion  {
    fn deselect(&self) {
        todo!()
    }
}

impl Controllable for Battalion {
    fn new_order(&self, goal: Goal) {
        todo!()
    }

    fn add_order(&self, goal: Goal) {
        todo!()
    }
}