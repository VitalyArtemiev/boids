use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use graphics::{Context, Graphics};
use serde::{Deserialize, Serialize};
use crate::container::Container;
use crate::ops::Vec2f;
use crate::traits::{Clickable, Controllable, Identifiable, Selectable};

use rand::Rng;
use crate::boids::BoidVec;
use crate::drawable::Drawable;
use crate::formations;
use crate::formations::{FORMATION_SPACING, FormationFunction};
use crate::units::Goal::Idle;
use crate::world::{Identifiable, WorldId};

const ACC_MAX: f64 = 1000.;
const VEL_MAX: f64 = 100.;
const DIST_REPEL: f64 = 20.;
const DIST_MARGIN: f64 = 1.;
const COLUMN_WIDTH: i32 = 4;

#[derive(Serialize, Deserialize)]
pub enum Goal {
    Idle(Vec2f),
    Hold,
    Move(Vec2f, Vec2f),
    Column(Vec2f),
    Front(Vec2f, Vec2f, Vec2f),
}

#[derive(Default, Serialize, Deserialize)]
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
pub enum Unit {
    BasicUnit(BasicUnit),
    CompositeUnit(CompositeUnit)
}

impl Identifiable for Unit {
    fn generate_id(&self) -> WorldId {
        match self {
            Unit::BasicUnit(b) => {b.generate_id()}
            Unit::CompositeUnit(c) => {c.generate_id()}
        }
    }

    fn get_id(&self) -> WorldId {
        match self {
            Unit::BasicUnit(b) => {b.get_id()}
            Unit::CompositeUnit(c) => {c.get_id()}
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BasicUnit {
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

impl BasicUnit {
    pub(crate) fn new(pos: Vec2f, num: usize) -> Self {
        BasicUnit {
            selected: false,
            fatigue: 0.0,
            morale: 1.0,
            experience: 0.0,
            avg_age: 18.0,
            center: pos,
            direction: Vec2f{x: 1.,y: 0.},
            select_radius: 0.0,
            goals: VecDeque::from([Idle(pos)]),
            formation: formations::default_formation,
            formation_positions: Vec::with_capacity(num),
            troops: None,
            interaction_radius: 0.0,
            troop_desc: TroopDesc::default(),
        }
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

const BASE_UNIT_CAPACITY: usize = 256;

pub static NUM_BASIC_UNITS: AtomicUsize = AtomicUsize::new(0);

impl Identifiable for BasicUnit {
    fn generate_id(&self) -> WorldId {
        let nc = NUM_BASIC_UNITS.fetch_add(1, Ordering::Relaxed);
        BASE_UNIT_CAPACITY * (nc + 1)
    }

    fn get_id(&self) -> WorldId {
        self.id
    }
}

impl Drawable for BasicUnit {
    fn draw<G>(&self, c: Context, g: &mut G) where G: Graphics {
        todo!()
    }
}

impl Clickable for BasicUnit {
    fn is_on_screen(&self) -> bool {
        todo!()
    }

    fn on_click(&self) {
        todo!()
    }
}

impl Selectable for BasicUnit {
    fn deselect(&self) {
        todo!()
    }
}

impl Controllable for BasicUnit {
    fn new_order(&self, goal: Goal) {
        todo!()
    }

    fn add_order(&self, goal: Goal) {
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct DrillStep {
    company_type_id: usize,
    company_formation: FormationFunction,
    pos: Vec2f,
    dir: Vec2f,
    time: f32
}

#[derive(Serialize, Deserialize)]
pub struct Drill {
    steps: Vec<DrillStep>
}

#[derive(Serialize, Deserialize)]
pub struct CompositeUnit {
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

    pub troops: Vec<BasicUnit>,
}

impl CompositeUnit {
    fn assemble() -> CompositeUnit {
        todo!()
    }

    fn attach_company(&mut self, c: &BasicUnit) {

    }

    fn detach_company(&mut self, id: WorldId) {

    }
}

impl Identifiable for CompositeUnit {
    fn get_id(&self) -> WorldId {
        self.id
    }

    fn generate_id(&self) -> WorldId {
        todo!()
    }
}

impl Clickable for CompositeUnit {
    fn is_on_screen(&self) -> bool {
        todo!()
    }

    fn on_click(&self) {
        todo!()
    }
}

impl Selectable for CompositeUnit {
    fn deselect(&self) {
        todo!()
    }
}

impl Controllable for CompositeUnit {
    fn new_order(&self, goal: Goal) {
        todo!()
    }

    fn add_order(&self, goal: Goal) {
        todo!()
    }
}
