use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Deserialize, Serialize};
use crate::container::{Container, ContainerTrait};
use crate::ops::Vec2f;
use crate::traits::{Clickable, Controllable, Selectable};

use rand::Rng;
use crate::container::FORMATION_SPACING;
use crate::formations::Goal::Idle;
use crate::world::{Identifiable, WorldId};

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

}

impl Controllable for Company {

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

}

impl Selectable for Battalion  {

}

impl Controllable for Battalion {

}