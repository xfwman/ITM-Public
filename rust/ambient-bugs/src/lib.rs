#![allow(deprecated)]

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const TAU: f64 = std::f64::consts::PI * 2.0;
const RESPAWN_MARGIN: f64 = 64.0;
const DRAW_MARGIN: f64 = 48.0;
const COVER_REGION_STRIDE: usize = 5;
const MIN_SPLAT_HIT_RADIUS: f64 = 18.0;
const PREDATOR_DISPERSAL_MARGIN: f64 = 140.0;
const GROUND_BEETLE_STARVE_TIME: f64 = 520.0;
const CENTIPEDE_STARVE_TIME: f64 = 620.0;
const LADYBUG_STARVE_TIME: f64 = 460.0;
const OBJECT_TARGET_COUNT: usize = 8;
const MAX_OBJECT_TARGET: usize = 80;
const DEFAULT_DANDELION_COUNT: usize = 8;
const DEFAULT_STRAWBERRY_PLANT_COUNT: usize = 6;
const DEFAULT_GRASS_CLUMP_COUNT: usize = 12;
const DEFAULT_APHID_COUNT: usize = 8;
const DEFAULT_LEAF_BEETLE_COUNT: usize = 8;
const DEFAULT_GROUND_BEETLE_COUNT: usize = 3;
const DEFAULT_CENTIPEDE_COUNT: usize = 1;
const DEFAULT_LADYBUG_COUNT: usize = 3;

#[wasm_bindgen]
pub struct BugField {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    viewport: Viewport,
    world: CoverWorld,
    dpr: f64,
    objects: Vec<Box<dyn SceneObject>>,
    rng: Lcg,
    last_time: Option<f64>,
    dark_mode: bool,
    object_targets: ObjectCountTargets,
}

#[derive(Clone, Copy)]
struct ObjectCountTargets {
    dandelion: usize,
    strawberry_plant: usize,
    grass_clump: usize,
    aphid: usize,
    leaf_beetle: usize,
    ground_beetle: usize,
    centipede: usize,
    ladybug: usize,
}

impl Default for ObjectCountTargets {
    fn default() -> Self {
        Self {
            dandelion: DEFAULT_DANDELION_COUNT,
            strawberry_plant: DEFAULT_STRAWBERRY_PLANT_COUNT,
            grass_clump: DEFAULT_GRASS_CLUMP_COUNT,
            aphid: DEFAULT_APHID_COUNT,
            leaf_beetle: DEFAULT_LEAF_BEETLE_COUNT,
            ground_beetle: DEFAULT_GROUND_BEETLE_COUNT,
            centipede: DEFAULT_CENTIPEDE_COUNT,
            ladybug: DEFAULT_LADYBUG_COUNT,
        }
    }
}

impl ObjectCountTargets {
    fn from_values(values: &[f32]) -> Self {
        let defaults = Self::default();

        Self {
            dandelion: Self::value_at(values, 0, defaults.dandelion),
            strawberry_plant: Self::value_at(values, 1, defaults.strawberry_plant),
            grass_clump: Self::value_at(values, 2, defaults.grass_clump),
            aphid: Self::value_at(values, 3, defaults.aphid),
            leaf_beetle: Self::value_at(values, 4, defaults.leaf_beetle),
            ground_beetle: Self::value_at(values, 5, defaults.ground_beetle),
            centipede: Self::value_at(values, 6, defaults.centipede),
            ladybug: Self::value_at(values, 7, defaults.ladybug),
        }
    }

    fn value_at(values: &[f32], index: usize, fallback: usize) -> usize {
        values
            .get(index)
            .copied()
            .filter(|value| value.is_finite())
            .unwrap_or(fallback as f32)
            .round()
            .clamp(0.0, MAX_OBJECT_TARGET as f32) as usize
    }

    fn total_flora(self) -> usize {
        self.dandelion + self.strawberry_plant + self.grass_clump
    }
}

#[derive(Clone, Copy)]
struct Viewport {
    width: f64,
    height: f64,
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Default)]
struct WorldBounds {
    width: f64,
    height: f64,
}

#[derive(Default)]
struct CoverWorld {
    bounds: WorldBounds,
    regions: Vec<CoverRegion>,
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, PartialEq)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Clone, Copy)]
struct CoverRegion {
    kind: CoverRegionKind,
    rect: Rect,
}

#[derive(Clone, Copy)]
struct FloraSpawnRequest {
    kind: FloraKind,
    landing: Point,
}

#[derive(Clone, Copy)]
struct FloraBiteRequest {
    point: Point,
    radius: f64,
    damage: f64,
    kind: FloraDamageKind,
}

#[derive(Clone, Copy)]
struct PlantTarget {
    position: Point,
    quality: f64,
}

#[derive(Clone, Copy)]
struct SplatPalette {
    fill_light: &'static str,
    fill_dark: &'static str,
    edge_light: &'static str,
    edge_dark: &'static str,
    speck_light: &'static str,
    speck_dark: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SceneRole {
    Flora,
    Fauna,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FloraKind {
    Dandelion,
    Strawberry,
    Grass,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FaunaKind {
    Aphid,
    LeafBeetle,
    GroundBeetle,
    Centipede,
    Ladybug,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SmallPredatorPrey {
    Aphid,
    LeafBeetle,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CentipedePrey {
    GroundBeetle,
    LeafBeetle,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FloraDamageKind {
    SapDrain,
    LeafChew,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CoverRegionKind {
    Shelter,
    Barrier,
    Canopy,
}

#[derive(Clone, Copy)]
struct MotionState {
    position: Point,
    heading: f64,
    base_speed: f64,
    size: f64,
    stride_phase: f64,
    wander_phase: f64,
}

#[derive(Clone, Copy)]
struct MotionProfile {
    size_min: f64,
    size_max: f64,
    speed_min: f64,
    speed_max: f64,
    obstacle_padding: f64,
    obstacle_push: f64,
    edge_margin: f64,
    wander_weight: f64,
    turn_rate: f64,
    gait_frequency: f64,
    gait_influence: f64,
    avoidance_speed_gain: f64,
    step_scale: f64,
}

#[derive(Clone, Copy)]
struct RenderProfile {
    leg_pairs: usize,
    leg_anchor_x: f64,
    leg_offset_start: f64,
    leg_offset_step: f64,
    leg_sweep_base: f64,
    leg_sweep_step: f64,
    leg_reach_base: f64,
    leg_reach_step: f64,
    front_leg_factor: f64,
    antenna_start_x: f64,
    antenna_start_y: f64,
    antenna_end_x: f64,
    antenna_end_y: f64,
    antenna_spread: f64,
    limb_width: f64,
}

impl SplatPalette {
    fn fill(self, dark_mode: bool) -> &'static str {
        if dark_mode {
            self.fill_dark
        } else {
            self.fill_light
        }
    }

    fn edge(self, dark_mode: bool) -> &'static str {
        if dark_mode {
            self.edge_dark
        } else {
            self.edge_light
        }
    }

    fn speck(self, dark_mode: bool) -> &'static str {
        if dark_mode {
            self.speck_dark
        } else {
            self.speck_light
        }
    }
}

impl Point {
    fn distance_to(self, other: Point) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn angle_to(self, other: Point) -> f64 {
        (other.y - self.y).atan2(other.x - self.x)
    }
}

impl Rect {
    fn contains(self, point: Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    fn expanded(self, amount: f64) -> Rect {
        Rect {
            x: self.x - amount,
            y: self.y - amount,
            width: self.width + amount * 2.0,
            height: self.height + amount * 2.0,
        }
    }
}

fn point_visible_in_viewport(point: Point, viewport: Viewport, margin: f64) -> bool {
    point.x >= viewport.x - margin
        && point.x <= viewport.x + viewport.width + margin
        && point.y >= viewport.y - margin
        && point.y <= viewport.y + viewport.height + margin
}

fn circular_splat_candidate(
    point: Point,
    center: Point,
    radius: f64,
    viewport: Viewport,
    heading: f64,
    size: f64,
    palette: SplatPalette,
    seed: f64,
) -> Option<(f64, Box<dyn SceneObject>)> {
    let hit_radius = radius.max(MIN_SPLAT_HIT_RADIUS);

    if !point_visible_in_viewport(center, viewport, DRAW_MARGIN + hit_radius) {
        return None;
    }

    let distance = center.distance_to(point);
    if distance > hit_radius {
        return None;
    }

    Some((
        distance,
        Box::new(InsectSplat::new(center, heading, size, palette, seed)) as Box<dyn SceneObject>,
    ))
}

impl CoverWorld {
    fn has_shelter_regions(&self) -> bool {
        self.regions
            .iter()
            .any(|region| region.kind == CoverRegionKind::Shelter)
    }

    fn set_cover_regions(&mut self, regions: Vec<f32>) {
        self.regions.clear();

        for region in regions.chunks_exact(COVER_REGION_STRIDE) {
            let Some(kind) = CoverRegionKind::from_code(region[0]) else {
                continue;
            };

            let width = region[3] as f64;
            let height = region[4] as f64;

            if width <= 0.0 || height <= 0.0 {
                continue;
            }

            self.regions.push(CoverRegion {
                kind,
                rect: Rect {
                    x: region[1] as f64,
                    y: region[2] as f64,
                    width,
                    height,
                },
            });
        }
    }

    fn set_obstacles(&mut self, rects: Vec<f32>) {
        self.regions.clear();

        for rect in rects.chunks_exact(4) {
            let width = rect[2] as f64;
            let height = rect[3] as f64;

            if width <= 0.0 || height <= 0.0 {
                continue;
            }

            self.regions.push(CoverRegion {
                kind: CoverRegionKind::Shelter,
                rect: Rect {
                    x: rect[0] as f64,
                    y: rect[1] as f64,
                    width,
                    height,
                },
            });
        }
    }

    fn sample_open_point(&self, rng: &mut Lcg, clearance: f64) -> Option<Point> {
        sample_open_point_in_rect(
            Rect {
                x: 0.0,
                y: 0.0,
                width: self.bounds.width,
                height: self.bounds.height,
            },
            self.bounds,
            &self.regions,
            rng,
            clearance,
            72,
        )
    }

    fn nearest_shelter_home(
        &self,
        origin: Point,
        inset: f64,
        peek_clearance: f64,
    ) -> Option<(Rect, Point, Point)> {
        let rect = nearest_shelter_rect(origin, &self.regions, inset)?;
        let (hide_anchor, peek_point) =
            shelter_anchor_from_hint(rect, origin, inset, peek_clearance);
        Some((rect, hide_anchor, peek_point))
    }

    fn is_open_ground(&self, point: Point, clearance: f64) -> bool {
        is_open_ground_in_regions(point, self.bounds, &self.regions, clearance)
    }
}

impl CoverRegionKind {
    fn from_code(code: f32) -> Option<Self> {
        match code.round() as i32 {
            0 => Some(Self::Shelter),
            1 => Some(Self::Barrier),
            2 => Some(Self::Canopy),
            _ => None,
        }
    }

    fn movement_profile(self, motion: MotionProfile) -> Option<(f64, f64)> {
        match self {
            Self::Shelter => Some((motion.obstacle_padding, motion.obstacle_push)),
            Self::Barrier => Some((motion.obstacle_padding * 1.35, motion.obstacle_push * 1.25)),
            Self::Canopy => None,
        }
    }

    fn blocks_ground(self) -> bool {
        matches!(self, Self::Shelter | Self::Barrier | Self::Canopy)
    }

    fn blocks_visibility(self) -> bool {
        matches!(self, Self::Shelter | Self::Barrier | Self::Canopy)
    }
}

struct UpdateContext<'a> {
    bounds: WorldBounds,
    cover_regions: &'a [CoverRegion],
    flora_targets: &'a [Point],
    aphid_host_targets: &'a [PlantTarget],
    aphid_fallback_hosts: &'a [PlantTarget],
    leaf_beetle_food_targets: &'a [PlantTarget],
    leaf_beetle_fallback_food: &'a [PlantTarget],
    aphid_targets: &'a [Point],
    leaf_beetle_targets: &'a [Point],
    ground_beetle_targets: &'a [Point],
    aphid_threats: &'a [Point],
    leaf_beetle_threats: &'a [Point],
    ground_beetle_threats: &'a [Point],
    time: f64,
    dt: f64,
}

struct RenderContext<'a> {
    context: &'a CanvasRenderingContext2d,
    viewport: Viewport,
    dark_mode: bool,
    time: f64,
}

trait SceneObject {
    fn role(&self) -> SceneRole {
        SceneRole::Fauna
    }

    fn update(&mut self, update: &UpdateContext);
    fn draw(&self, render: &RenderContext) -> Result<(), JsValue>;
    fn position(&self) -> Point;

    fn object_name(&self) -> &'static str {
        "SceneObject"
    }

    fn should_respawn(&self, bounds: WorldBounds) -> bool {
        let position = self.position();
        position.x < -RESPAWN_MARGIN
            || position.x > bounds.width + RESPAWN_MARGIN
            || position.y < -RESPAWN_MARGIN
            || position.y > bounds.height + RESPAWN_MARGIN
    }

    fn should_rehome(&self, bounds: WorldBounds) -> bool {
        let position = self.position();
        position.x > bounds.width + 80.0 || position.y > bounds.height + 80.0
    }

    fn should_rehome_in_cover_world(&self, _world: &CoverWorld) -> bool {
        false
    }

    fn flora_bite_request(&self) -> Option<FloraBiteRequest> {
        None
    }

    fn on_flora_bitten(&mut self, _point: Point) {}

    fn receive_flora_damage(&mut self, _kind: FloraDamageKind, _damage: f64) {}

    fn aphid_host_position(&self) -> Option<PlantTarget> {
        None
    }

    fn aphid_fallback_host_position(&self) -> Option<PlantTarget> {
        None
    }

    fn leaf_beetle_food_position(&self) -> Option<PlantTarget> {
        None
    }

    fn leaf_beetle_fallback_food_position(&self) -> Option<PlantTarget> {
        None
    }

    fn aphid_bite_request(&self) -> Option<(Point, f64)> {
        None
    }

    fn on_aphid_eaten(&mut self, _point: Point) {}

    fn aphid_prey_position(&self) -> Option<Point> {
        None
    }

    fn leaf_beetle_bite_request(&self) -> Option<(Point, f64)> {
        None
    }

    fn on_leaf_beetle_eaten(&mut self, _point: Point) {}

    fn leaf_beetle_prey_position(&self) -> Option<Point> {
        None
    }

    fn ground_beetle_bite_request(&self) -> Option<(Point, f64)> {
        None
    }

    fn on_ground_beetle_eaten(&mut self, _point: Point) {}

    fn ground_beetle_prey_position(&self) -> Option<Point> {
        None
    }

    fn aphid_threat_position(&self) -> Option<Point> {
        None
    }

    fn leaf_beetle_threat_position(&self) -> Option<Point> {
        None
    }

    fn ground_beetle_threat_position(&self) -> Option<Point> {
        None
    }

    fn splat_candidate(
        &self,
        _point: Point,
        _viewport: Viewport,
    ) -> Option<(f64, Box<dyn SceneObject>)> {
        None
    }

    fn take_flora_spawn_requests(&mut self) -> Vec<FloraSpawnRequest> {
        Vec::new()
    }

    fn emergence_request(&self) -> Option<(FaunaKind, Point)> {
        None
    }

    fn emerge_at(&mut self, _point: Point, _world: &CoverWorld, _viewport: Viewport) {}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LeafBeetleState {
    Hidden,
    Peeking,
    Foraging,
    Eating,
    Returning,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AphidState {
    Feeding,
    Wandering,
    Dropping,
}

struct Aphid {
    motion: MotionState,
    host_flora: Option<Point>,
    host_offset: Point,
    drop_target: Point,
    state: AphidState,
    state_time: f64,
    body_phase: f64,
    antenna_phase: f64,
}

struct LeafBeetle {
    motion: MotionState,
    home: Rect,
    hide_anchor: Point,
    peek_point: Point,
    target_flora: Option<Point>,
    state: LeafBeetleState,
    state_time: f64,
    shell_phase: f64,
    antenna_phase: f64,
    peek_phase: f64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GroundBeetleState {
    Sheltering,
    Patrolling,
    Hunting,
    Feeding,
    Dispersing,
}

struct GroundBeetle {
    motion: MotionState,
    rest_home: Option<Rect>,
    rest_anchor: Point,
    patrol_target: Point,
    target_prey: Option<Point>,
    target_prey_kind: Option<SmallPredatorPrey>,
    state: GroundBeetleState,
    state_time: f64,
    shell_phase: f64,
    antenna_phase: f64,
    patrol_cursor: u32,
    hunger: f64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CentipedeState {
    Sheltering,
    Patrolling,
    Hunting,
    Feeding,
    Dispersing,
}

struct Centipede {
    motion: MotionState,
    rest_home: Option<Rect>,
    rest_anchor: Point,
    patrol_target: Point,
    target_prey: Option<Point>,
    target_prey_kind: Option<CentipedePrey>,
    state: CentipedeState,
    state_time: f64,
    trail: Vec<Point>,
    body_phase: f64,
    antenna_phase: f64,
    patrol_cursor: u32,
    hunger: f64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LadybugState {
    Walking,
    Feeding,
    Flying,
    Dispersing,
}

struct Ladybug {
    motion: MotionState,
    walk_target: Point,
    flight_target: Point,
    target_aphid: Option<Point>,
    state: LadybugState,
    state_time: f64,
    flight_delay: f64,
    walk_speed: f64,
    flight_speed: f64,
    shell_phase: f64,
    wing_phase: f64,
    flight_phase: f64,
    patrol_cursor: u32,
    hunger: f64,
}

struct Dandelion {
    root: Point,
    life: f64,
    health: f64,
    leaf_health: f64,
    sap_health: f64,
    life_rate: f64,
    leaf_span: f64,
    max_height: f64,
    bloom_radius: f64,
    stem_curve: f64,
    sway_phase: f64,
    leaf_phase: f64,
    seed_phase: f64,
    seed_count: usize,
    germinated_seed_count: usize,
}

struct StrawberryPlant {
    root: Point,
    life: f64,
    health: f64,
    leaf_health: f64,
    sap_health: f64,
    life_rate: f64,
    leaf_span: f64,
    flower_count: usize,
    berry_count: usize,
    runner_count: usize,
    propagated_runner_count: usize,
    sway_phase: f64,
    leaf_phase: f64,
    bloom_phase: f64,
    fruit_phase: f64,
    runner_phase: f64,
}

struct GrassClump {
    root: Point,
    life: f64,
    health: f64,
    leaf_health: f64,
    sap_health: f64,
    life_rate: f64,
    blade_count: usize,
    spread: f64,
    max_height: f64,
    sway_phase: f64,
    blade_phase: f64,
    seed: f64,
}

struct InsectSplat {
    position: Point,
    heading: f64,
    size: f64,
    age: f64,
    life_span: f64,
    spread: f64,
    seed: f64,
    palette: SplatPalette,
}

#[allow(dead_code)]
struct MaturingFauna {
    kind: FaunaKind,
    position: Point,
    age: f64,
    emerge_after: f64,
    size: f64,
    phase: f64,
}

struct Lcg {
    state: u64,
}

const LEAF_BEETLE_MOTION: MotionProfile = MotionProfile {
    size_min: 3.1,
    size_max: 4.6,
    speed_min: 0.54,
    speed_max: 0.92,
    obstacle_padding: 13.0,
    obstacle_push: 0.98,
    edge_margin: 24.0,
    wander_weight: 0.07,
    turn_rate: 0.24,
    gait_frequency: 10.8,
    gait_influence: 0.22,
    avoidance_speed_gain: 0.08,
    step_scale: 2.7,
};

const GROUND_BEETLE_MOTION: MotionProfile = MotionProfile {
    size_min: 6.2,
    size_max: 9.0,
    speed_min: 0.72,
    speed_max: 1.18,
    obstacle_padding: 18.0,
    obstacle_push: 1.38,
    edge_margin: 34.0,
    wander_weight: 0.06,
    turn_rate: 0.24,
    gait_frequency: 7.8,
    gait_influence: 0.28,
    avoidance_speed_gain: 0.09,
    step_scale: 3.55,
};

const CENTIPEDE_MOTION: MotionProfile = MotionProfile {
    size_min: 7.0,
    size_max: 10.8,
    speed_min: 0.78,
    speed_max: 1.24,
    obstacle_padding: 18.0,
    obstacle_push: 1.3,
    edge_margin: 36.0,
    wander_weight: 0.07,
    turn_rate: 0.18,
    gait_frequency: 10.8,
    gait_influence: 0.34,
    avoidance_speed_gain: 0.1,
    step_scale: 3.9,
};

const LADYBUG_WALK_MOTION: MotionProfile = MotionProfile {
    size_min: 4.8,
    size_max: 6.8,
    speed_min: 0.32,
    speed_max: 0.56,
    obstacle_padding: 12.0,
    obstacle_push: 0.92,
    edge_margin: 26.0,
    wander_weight: 0.12,
    turn_rate: 0.24,
    gait_frequency: 11.2,
    gait_influence: 0.24,
    avoidance_speed_gain: 0.06,
    step_scale: 2.9,
};

const LADYBUG_FLY_MOTION: MotionProfile = MotionProfile {
    size_min: 4.8,
    size_max: 6.8,
    speed_min: 0.92,
    speed_max: 1.38,
    obstacle_padding: 8.0,
    obstacle_push: 0.42,
    edge_margin: 24.0,
    wander_weight: 0.04,
    turn_rate: 0.2,
    gait_frequency: 14.8,
    gait_influence: 0.08,
    avoidance_speed_gain: 0.03,
    step_scale: 4.15,
};

const APHID_MOTION: MotionProfile = MotionProfile {
    size_min: 2.1,
    size_max: 3.2,
    speed_min: 0.18,
    speed_max: 0.34,
    obstacle_padding: 8.0,
    obstacle_push: 0.64,
    edge_margin: 18.0,
    wander_weight: 0.11,
    turn_rate: 0.28,
    gait_frequency: 13.4,
    gait_influence: 0.18,
    avoidance_speed_gain: 0.05,
    step_scale: 2.3,
};

const LEAF_BEETLE_RENDER: RenderProfile = RenderProfile {
    leg_pairs: 3,
    leg_anchor_x: -0.02,
    leg_offset_start: -0.5,
    leg_offset_step: 0.44,
    leg_sweep_base: 0.13,
    leg_sweep_step: 0.04,
    leg_reach_base: 0.84,
    leg_reach_step: 0.08,
    front_leg_factor: 0.52,
    antenna_start_x: 0.52,
    antenna_start_y: 0.08,
    antenna_end_x: 0.98,
    antenna_end_y: 0.34,
    antenna_spread: 0.86,
    limb_width: 0.95,
};

const GROUND_BEETLE_RENDER: RenderProfile = RenderProfile {
    leg_pairs: 3,
    leg_anchor_x: -0.02,
    leg_offset_start: -0.56,
    leg_offset_step: 0.48,
    leg_sweep_base: 0.12,
    leg_sweep_step: 0.05,
    leg_reach_base: 1.08,
    leg_reach_step: 0.1,
    front_leg_factor: 0.62,
    antenna_start_x: 0.72,
    antenna_start_y: 0.08,
    antenna_end_x: 1.44,
    antenna_end_y: 0.34,
    antenna_spread: 0.96,
    limb_width: 0.95,
};

const APHID_SPLAT_PALETTE: SplatPalette = SplatPalette {
    fill_light: "rgba(118, 176, 76, 0.84)",
    fill_dark: "rgba(112, 186, 72, 0.88)",
    edge_light: "rgba(56, 90, 34, 0.62)",
    edge_dark: "rgba(32, 58, 20, 0.72)",
    speck_light: "rgba(236, 255, 210, 0.42)",
    speck_dark: "rgba(210, 245, 182, 0.46)",
};

const LEAF_BEETLE_SPLAT_PALETTE: SplatPalette = SplatPalette {
    fill_light: "rgba(110, 172, 68, 0.84)",
    fill_dark: "rgba(122, 190, 76, 0.88)",
    edge_light: "rgba(44, 70, 28, 0.64)",
    edge_dark: "rgba(24, 42, 16, 0.74)",
    speck_light: "rgba(242, 255, 198, 0.36)",
    speck_dark: "rgba(224, 248, 178, 0.42)",
};

const GROUND_BEETLE_SPLAT_PALETTE: SplatPalette = SplatPalette {
    fill_light: "rgba(56, 88, 56, 0.82)",
    fill_dark: "rgba(52, 104, 74, 0.88)",
    edge_light: "rgba(24, 42, 26, 0.62)",
    edge_dark: "rgba(10, 20, 14, 0.72)",
    speck_light: "rgba(168, 224, 176, 0.32)",
    speck_dark: "rgba(156, 232, 182, 0.38)",
};

const CENTIPEDE_SPLAT_PALETTE: SplatPalette = SplatPalette {
    fill_light: "rgba(108, 74, 52, 0.82)",
    fill_dark: "rgba(142, 84, 54, 0.88)",
    edge_light: "rgba(54, 28, 18, 0.64)",
    edge_dark: "rgba(28, 12, 8, 0.74)",
    speck_light: "rgba(238, 170, 110, 0.34)",
    speck_dark: "rgba(250, 182, 124, 0.38)",
};

const LADYBUG_SPLAT_PALETTE: SplatPalette = SplatPalette {
    fill_light: "rgba(190, 42, 26, 0.86)",
    fill_dark: "rgba(214, 56, 40, 0.9)",
    edge_light: "rgba(70, 16, 12, 0.64)",
    edge_dark: "rgba(34, 8, 8, 0.74)",
    speck_light: "rgba(255, 206, 188, 0.34)",
    speck_dark: "rgba(255, 214, 196, 0.38)",
};

impl Lcg {
    fn new(seed: u32) -> Self {
        Self {
            state: seed as u64 | 1,
        }
    }

    fn next_f64(&mut self) -> f64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.state >> 11) as f64) / ((1_u64 << 53) as f64)
    }

    fn range(&mut self, min: f64, max: f64) -> f64 {
        min + self.next_f64() * (max - min)
    }

    fn bool(&mut self, chance: f64) -> bool {
        self.next_f64() < chance
    }
}

impl InsectSplat {
    fn new(position: Point, heading: f64, size: f64, palette: SplatPalette, seed: f64) -> Self {
        Self {
            position,
            heading,
            size,
            age: 0.0,
            life_span: 36.0,
            spread: (0.82 + seed.sin().abs() * 0.42) * size,
            seed,
            palette,
        }
    }
}

#[allow(dead_code)]
impl MaturingFauna {
    fn spawn(kind: FaunaKind, position: Point, rng: &mut Lcg) -> Self {
        let (min_delay, max_delay, min_size, max_size) = match kind {
            FaunaKind::Aphid => (72.0, 138.0, 1.8, 2.8),
            FaunaKind::LeafBeetle => (150.0, 250.0, 2.6, 4.0),
            FaunaKind::GroundBeetle => (230.0, 360.0, 3.8, 5.8),
            FaunaKind::Centipede => (330.0, 520.0, 4.4, 6.6),
            FaunaKind::Ladybug => (190.0, 320.0, 3.0, 4.8),
        };

        Self {
            kind,
            position,
            age: 0.0,
            emerge_after: rng.range(min_delay, max_delay),
            size: rng.range(min_size, max_size),
            phase: rng.range(0.0, TAU),
        }
    }
}

impl SceneObject for MaturingFauna {
    fn update(&mut self, update: &UpdateContext) {
        self.age += update.dt;
    }

    fn draw(&self, render: &RenderContext) -> Result<(), JsValue> {
        draw_maturing_fauna(self, render)
    }

    fn position(&self) -> Point {
        self.position
    }

    fn object_name(&self) -> &'static str {
        "MaturingFauna"
    }

    fn emergence_request(&self) -> Option<(FaunaKind, Point)> {
        (self.age >= self.emerge_after).then_some((self.kind, self.position))
    }
}

impl SceneObject for InsectSplat {
    fn update(&mut self, update: &UpdateContext) {
        self.age += update.dt;
    }

    fn draw(&self, render: &RenderContext) -> Result<(), JsValue> {
        draw_insect_splat(self, render)
    }

    fn position(&self) -> Point {
        self.position
    }

    fn object_name(&self) -> &'static str {
        "InsectSplat"
    }

    fn should_respawn(&self, bounds: WorldBounds) -> bool {
        self.age >= self.life_span
            || self.position.x < -RESPAWN_MARGIN
            || self.position.x > bounds.width + RESPAWN_MARGIN
            || self.position.y < -RESPAWN_MARGIN
            || self.position.y > bounds.height + RESPAWN_MARGIN
    }
}

#[wasm_bindgen]
impl BugField {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, seed: u32) -> Result<BugField, JsValue> {
        let context = canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("2d canvas context unavailable"))?
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Self {
            canvas,
            context,
            viewport: Viewport {
                width: 1.0,
                height: 1.0,
                x: 0.0,
                y: 0.0,
            },
            world: CoverWorld {
                bounds: WorldBounds {
                    width: 1.0,
                    height: 1.0,
                },
                regions: Vec::new(),
            },
            dpr: 1.0,
            objects: Vec::new(),
            rng: Lcg::new(seed),
            last_time: None,
            dark_mode: false,
            object_targets: ObjectCountTargets::default(),
        })
    }

    pub fn resize(
        &mut self,
        viewport_width: f64,
        viewport_height: f64,
        page_width: f64,
        page_height: f64,
        dpr: f64,
    ) -> Result<(), JsValue> {
        self.viewport.width = viewport_width.max(1.0);
        self.viewport.height = viewport_height.max(1.0);
        self.world.bounds.width = page_width.max(self.viewport.width);
        self.world.bounds.height = page_height.max(self.viewport.height);
        self.dpr = dpr.clamp(1.0, 1.5);
        self.clamp_viewport();

        self.canvas
            .set_width((self.viewport.width * self.dpr).round().max(1.0) as u32);
        self.canvas
            .set_height((self.viewport.height * self.dpr).round().max(1.0) as u32);

        self.context
            .set_transform(self.dpr, 0.0, 0.0, self.dpr, 0.0, 0.0)?;
        self.sync_object_count();
        Ok(())
    }

    pub fn set_viewport(&mut self, viewport_x: f64, viewport_y: f64) {
        self.viewport.x = viewport_x.max(0.0);
        self.viewport.y = viewport_y.max(0.0);
        self.clamp_viewport();
    }

    pub fn set_dark_mode(&mut self, dark_mode: bool) {
        self.dark_mode = dark_mode;
    }

    pub fn set_object_targets(&mut self, targets: Vec<f32>) {
        self.object_targets =
            ObjectCountTargets::from_values(&targets[..OBJECT_TARGET_COUNT.min(targets.len())]);
        self.sync_object_count();
    }

    pub fn object_counts_json(&self) -> String {
        let mut dandelion = 0;
        let mut strawberry_plant = 0;
        let mut grass_clump = 0;
        let mut aphid = 0;
        let mut leaf_beetle = 0;
        let mut ground_beetle = 0;
        let mut centipede = 0;
        let mut ladybug = 0;
        let mut maturing_fauna = 0;
        let mut insect_splat = 0;
        let mut scene_object = 0;

        for object in &self.objects {
            match object.object_name() {
                "Dandelion" => dandelion += 1,
                "StrawberryPlant" => strawberry_plant += 1,
                "GrassClump" => grass_clump += 1,
                "Aphid" => aphid += 1,
                "LeafBeetle" => leaf_beetle += 1,
                "GroundBeetle" => ground_beetle += 1,
                "Centipede" => centipede += 1,
                "Ladybug" => ladybug += 1,
                "MaturingFauna" => maturing_fauna += 1,
                "InsectSplat" => insect_splat += 1,
                _ => scene_object += 1,
            }
        }

        format!(
            "[{{\"name\":\"Dandelion\",\"count\":{}}},\
{{\"name\":\"StrawberryPlant\",\"count\":{}}},\
{{\"name\":\"GrassClump\",\"count\":{}}},\
{{\"name\":\"Aphid\",\"count\":{}}},\
{{\"name\":\"LeafBeetle\",\"count\":{}}},\
{{\"name\":\"GroundBeetle\",\"count\":{}}},\
{{\"name\":\"Centipede\",\"count\":{}}},\
{{\"name\":\"Ladybug\",\"count\":{}}},\
{{\"name\":\"MaturingFauna\",\"count\":{}}},\
{{\"name\":\"InsectSplat\",\"count\":{}}},\
{{\"name\":\"SceneObject\",\"count\":{}}}]",
            dandelion,
            strawberry_plant,
            grass_clump,
            aphid,
            leaf_beetle,
            ground_beetle,
            centipede,
            ladybug,
            maturing_fauna,
            insect_splat,
            scene_object
        )
    }

    pub fn splat_at(&mut self, page_x: f64, page_y: f64) -> bool {
        let point = Point {
            x: page_x.clamp(0.0, self.world.bounds.width.max(1.0)),
            y: page_y.clamp(0.0, self.world.bounds.height.max(1.0)),
        };
        let mut hit: Option<(usize, f64, Box<dyn SceneObject>)> = None;

        for (index, object) in self.objects.iter().enumerate() {
            if object.role() != SceneRole::Fauna {
                continue;
            }

            let Some((distance, replacement)) = object.splat_candidate(point, self.viewport) else {
                continue;
            };

            if hit
                .as_ref()
                .is_none_or(|(_, best_distance, _)| distance < *best_distance)
            {
                hit = Some((index, distance, replacement));
            }
        }

        if let Some((index, _, replacement)) = hit {
            self.objects[index] = replacement;
            true
        } else {
            false
        }
    }

    pub fn set_cover_regions(&mut self, regions: Vec<f32>) {
        let had_shelters = self.world.has_shelter_regions();
        self.world.set_cover_regions(regions);
        if !had_shelters && self.world.has_shelter_regions() {
            self.redistribute_fauna();
        }
        self.sync_object_count();
    }

    pub fn set_obstacles(&mut self, rects: Vec<f32>) {
        self.world.set_obstacles(rects);
        self.sync_object_count();
    }

    pub fn frame(&mut self, timestamp: f64) -> Result<(), JsValue> {
        let dt = if let Some(last_time) = self.last_time {
            ((timestamp - last_time) / 16.666_7).clamp(0.45, 1.8)
        } else {
            1.0
        };
        self.last_time = Some(timestamp);

        self.context
            .clear_rect(0.0, 0.0, self.viewport.width, self.viewport.height);

        let time_seconds = timestamp * 0.001;
        let object_capacity = self.objects.len();
        let mut flora_targets: Vec<Point> = Vec::with_capacity(self.object_targets.total_flora());
        let mut aphid_host_targets: Vec<PlantTarget> =
            Vec::with_capacity(self.object_targets.total_flora());
        let mut aphid_fallback_hosts: Vec<PlantTarget> =
            Vec::with_capacity(self.object_targets.total_flora());
        let mut leaf_beetle_food_targets: Vec<PlantTarget> =
            Vec::with_capacity(self.object_targets.total_flora());
        let mut leaf_beetle_fallback_food: Vec<PlantTarget> =
            Vec::with_capacity(self.object_targets.total_flora());
        let mut aphid_targets: Vec<Point> = Vec::with_capacity(self.object_targets.aphid);
        let mut leaf_beetle_targets: Vec<Point> =
            Vec::with_capacity(self.object_targets.leaf_beetle);
        let mut ground_beetle_targets: Vec<Point> =
            Vec::with_capacity(self.object_targets.ground_beetle);
        let mut aphid_threats: Vec<Point> = Vec::with_capacity(object_capacity);
        let mut leaf_beetle_threats: Vec<Point> = Vec::with_capacity(object_capacity);
        let mut ground_beetle_threats: Vec<Point> = Vec::with_capacity(object_capacity);

        for object in &self.objects {
            if object.role() == SceneRole::Flora {
                flora_targets.push(object.position());
                if let Some(target) = object.aphid_host_position() {
                    aphid_host_targets.push(target);
                }
                if let Some(target) = object.aphid_fallback_host_position() {
                    aphid_fallback_hosts.push(target);
                }
                if let Some(target) = object.leaf_beetle_food_position() {
                    leaf_beetle_food_targets.push(target);
                }
                if let Some(target) = object.leaf_beetle_fallback_food_position() {
                    leaf_beetle_fallback_food.push(target);
                }
            } else {
                if let Some(target) = object.aphid_prey_position() {
                    aphid_targets.push(target);
                }
                if let Some(target) = object.leaf_beetle_prey_position() {
                    leaf_beetle_targets.push(target);
                }
                if let Some(target) = object.ground_beetle_prey_position() {
                    ground_beetle_targets.push(target);
                }
                if let Some(target) = object.aphid_threat_position() {
                    aphid_threats.push(target);
                }
                if let Some(target) = object.leaf_beetle_threat_position() {
                    leaf_beetle_threats.push(target);
                }
                if let Some(target) = object.ground_beetle_threat_position() {
                    ground_beetle_threats.push(target);
                }
            }
        }
        let mut respawn_indices = Vec::new();
        let mut rehome_flora_indices = Vec::new();
        let mut germination_requests = Vec::new();
        let mut emergence_requests = Vec::new();
        {
            let update = UpdateContext {
                bounds: self.world.bounds,
                cover_regions: &self.world.regions,
                flora_targets: &flora_targets,
                aphid_host_targets: &aphid_host_targets,
                aphid_fallback_hosts: &aphid_fallback_hosts,
                leaf_beetle_food_targets: &leaf_beetle_food_targets,
                leaf_beetle_fallback_food: &leaf_beetle_fallback_food,
                aphid_targets: &aphid_targets,
                leaf_beetle_targets: &leaf_beetle_targets,
                ground_beetle_targets: &ground_beetle_targets,
                aphid_threats: &aphid_threats,
                leaf_beetle_threats: &leaf_beetle_threats,
                ground_beetle_threats: &ground_beetle_threats,
                time: time_seconds,
                dt,
            };

            for index in 0..self.objects.len() {
                let (role, should_cycle) = {
                    let object = &mut self.objects[index];
                    object.update(&update);
                    germination_requests.extend(object.take_flora_spawn_requests());
                    if let Some(request) = object.emergence_request() {
                        emergence_requests.push((index, request));
                    }
                    let role = object.role();
                    let should_cycle = match role {
                        SceneRole::Fauna => object.should_respawn(self.world.bounds),
                        SceneRole::Flora => object.should_rehome_in_cover_world(&self.world),
                    };
                    (role, should_cycle)
                };

                if should_cycle {
                    match role {
                        SceneRole::Fauna => respawn_indices.push(index),
                        SceneRole::Flora => rehome_flora_indices.push(index),
                    }
                }
            }
        }

        let mut bite_events = Vec::new();
        for fauna_index in 0..self.objects.len() {
            let Some(bite) = self.objects[fauna_index].flora_bite_request() else {
                continue;
            };

            let mut nearest_flora: Option<(usize, Point, f64)> = None;
            for flora_index in 0..self.objects.len() {
                if self.objects[flora_index].role() != SceneRole::Flora
                    || rehome_flora_indices.contains(&flora_index)
                {
                    continue;
                }

                let flora_point = self.objects[flora_index].position();
                let distance = bite.point.distance_to(flora_point);
                if distance > bite.radius {
                    continue;
                }

                if nearest_flora
                    .as_ref()
                    .is_none_or(|(_, _, best_distance)| distance < *best_distance)
                {
                    nearest_flora = Some((flora_index, flora_point, distance));
                }
            }

            if let Some((flora_index, flora_point, _)) = nearest_flora {
                bite_events.push((
                    fauna_index,
                    flora_index,
                    flora_point,
                    bite.kind,
                    bite.damage,
                ));
            }
        }

        for (fauna_index, flora_index, flora_point, kind, damage) in bite_events {
            self.objects[flora_index].receive_flora_damage(kind, damage);
            self.objects[fauna_index].on_flora_bitten(flora_point);
        }

        let mut consumed_fauna_indices = Vec::new();
        let mut aphid_bite_events = Vec::new();
        let mut consumed_aphid_indices = Vec::new();
        for hunter_index in 0..self.objects.len() {
            let Some((bite_point, bite_radius)) = self.objects[hunter_index].aphid_bite_request()
            else {
                continue;
            };

            let mut nearest_prey: Option<(usize, Point, f64)> = None;
            for prey_index in 0..self.objects.len() {
                if prey_index == hunter_index
                    || consumed_aphid_indices.contains(&prey_index)
                    || respawn_indices.contains(&prey_index)
                {
                    continue;
                }

                let Some(prey_point) = self.objects[prey_index].aphid_prey_position() else {
                    continue;
                };
                let distance = bite_point.distance_to(prey_point);
                if distance > bite_radius {
                    continue;
                }

                if nearest_prey
                    .as_ref()
                    .is_none_or(|(_, _, best_distance)| distance < *best_distance)
                {
                    nearest_prey = Some((prey_index, prey_point, distance));
                }
            }

            if let Some((prey_index, prey_point, _)) = nearest_prey {
                consumed_aphid_indices.push(prey_index);
                aphid_bite_events.push((hunter_index, prey_index, prey_point));
            }
        }

        for (hunter_index, prey_index, prey_point) in aphid_bite_events {
            consumed_fauna_indices.push(prey_index);
            self.objects[hunter_index].on_aphid_eaten(prey_point);
        }

        let mut leaf_beetle_bite_events = Vec::new();
        let mut consumed_leaf_beetle_indices = Vec::new();
        for hunter_index in 0..self.objects.len() {
            let Some((bite_point, bite_radius)) =
                self.objects[hunter_index].leaf_beetle_bite_request()
            else {
                continue;
            };

            let mut nearest_prey: Option<(usize, Point, f64)> = None;
            for prey_index in 0..self.objects.len() {
                if prey_index == hunter_index
                    || consumed_leaf_beetle_indices.contains(&prey_index)
                    || respawn_indices.contains(&prey_index)
                {
                    continue;
                }

                let Some(prey_point) = self.objects[prey_index].leaf_beetle_prey_position() else {
                    continue;
                };
                let distance = bite_point.distance_to(prey_point);
                if distance > bite_radius {
                    continue;
                }

                if nearest_prey
                    .as_ref()
                    .is_none_or(|(_, _, best_distance)| distance < *best_distance)
                {
                    nearest_prey = Some((prey_index, prey_point, distance));
                }
            }

            if let Some((prey_index, prey_point, _)) = nearest_prey {
                consumed_leaf_beetle_indices.push(prey_index);
                leaf_beetle_bite_events.push((hunter_index, prey_index, prey_point));
            }
        }

        for (hunter_index, prey_index, prey_point) in leaf_beetle_bite_events {
            consumed_fauna_indices.push(prey_index);
            self.objects[hunter_index].on_leaf_beetle_eaten(prey_point);
        }

        let mut ground_beetle_bite_events = Vec::new();
        let mut consumed_ground_beetle_indices = Vec::new();
        for hunter_index in 0..self.objects.len() {
            let Some((bite_point, bite_radius)) =
                self.objects[hunter_index].ground_beetle_bite_request()
            else {
                continue;
            };

            let mut nearest_prey: Option<(usize, Point, f64)> = None;
            for prey_index in 0..self.objects.len() {
                if prey_index == hunter_index
                    || consumed_ground_beetle_indices.contains(&prey_index)
                    || respawn_indices.contains(&prey_index)
                {
                    continue;
                }

                let Some(prey_point) = self.objects[prey_index].ground_beetle_prey_position()
                else {
                    continue;
                };
                let distance = bite_point.distance_to(prey_point);
                if distance > bite_radius {
                    continue;
                }

                if nearest_prey
                    .as_ref()
                    .is_none_or(|(_, _, best_distance)| distance < *best_distance)
                {
                    nearest_prey = Some((prey_index, prey_point, distance));
                }
            }

            if let Some((prey_index, prey_point, _)) = nearest_prey {
                consumed_ground_beetle_indices.push(prey_index);
                ground_beetle_bite_events.push((hunter_index, prey_index, prey_point));
            }
        }

        for (hunter_index, prey_index, prey_point) in ground_beetle_bite_events {
            consumed_fauna_indices.push(prey_index);
            self.objects[hunter_index].on_ground_beetle_eaten(prey_point);
        }

        let mut occupied_flora_points: Vec<Point> = self
            .objects
            .iter()
            .enumerate()
            .filter_map(|(index, object)| {
                (object.role() == SceneRole::Flora && !rehome_flora_indices.contains(&index))
                    .then_some(object.position())
            })
            .collect();
        let flora_max = self.object_targets.total_flora();
        let mut flora_count = occupied_flora_points.len();
        for request in germination_requests {
            if flora_count >= flora_max {
                break;
            }

            let Some(root) = self.resolve_flora_root(request.landing, &occupied_flora_points)
            else {
                continue;
            };
            let Some(seedling) = self.spawn_flora_of_kind_at(request.kind, root) else {
                continue;
            };

            occupied_flora_points.push(root);
            flora_count += 1;
            self.objects.push(seedling);
        }

        consumed_fauna_indices.sort_unstable();
        consumed_fauna_indices.dedup();

        let mut emerged_indices = Vec::new();
        for (index, (kind, point)) in emergence_requests {
            if index >= self.objects.len()
                || consumed_fauna_indices.contains(&index)
                || respawn_indices.contains(&index)
            {
                continue;
            }

            self.objects[index] = self.spawn_emerged_fauna(kind, point);
            emerged_indices.push(index);
        }

        let mut removed_object_indices = consumed_fauna_indices.clone();
        for index in respawn_indices {
            if consumed_fauna_indices.contains(&index) || emerged_indices.contains(&index) {
                continue;
            }
            removed_object_indices.push(index);
        }

        for index in rehome_flora_indices {
            removed_object_indices.push(index);
        }

        removed_object_indices.sort_unstable();
        removed_object_indices.dedup();
        self.remove_indices(removed_object_indices);
        self.sync_object_count();

        let render = RenderContext {
            context: &self.context,
            viewport: self.viewport,
            dark_mode: self.dark_mode,
            time: time_seconds,
        };

        for object in &self.objects {
            if object.role() == SceneRole::Flora {
                object.draw(&render)?;
            }
        }

        for object in &self.objects {
            if object.role() == SceneRole::Fauna {
                object.draw(&render)?;
            }
        }

        Ok(())
    }
}

impl BugField {
    fn sync_object_count(&mut self) {
        self.sync_flora_count();
        self.sync_fauna_count();
    }

    fn sync_fauna_count(&mut self) {
        self.sync_fauna_kind_count(FaunaKind::Aphid, "Aphid", self.object_targets.aphid);
        self.sync_fauna_kind_count(
            FaunaKind::LeafBeetle,
            "LeafBeetle",
            self.object_targets.leaf_beetle,
        );
        self.sync_fauna_kind_count(
            FaunaKind::GroundBeetle,
            "GroundBeetle",
            self.object_targets.ground_beetle,
        );
        self.sync_fauna_kind_count(
            FaunaKind::Centipede,
            "Centipede",
            self.object_targets.centipede,
        );
        self.sync_fauna_kind_count(FaunaKind::Ladybug, "Ladybug", self.object_targets.ladybug);
    }

    fn sync_flora_count(&mut self) {
        self.sync_flora_kind_count(
            FloraKind::Dandelion,
            "Dandelion",
            self.object_targets.dandelion,
        );
        self.sync_flora_kind_count(
            FloraKind::Strawberry,
            "StrawberryPlant",
            self.object_targets.strawberry_plant,
        );
        self.sync_flora_kind_count(
            FloraKind::Grass,
            "GrassClump",
            self.object_targets.grass_clump,
        );
    }

    fn sync_fauna_kind_count(&mut self, kind: FaunaKind, name: &'static str, desired: usize) {
        self.trim_object_count(name, desired);

        let rehome_indices: Vec<usize> = self
            .indices_for_object_name(name)
            .into_iter()
            .filter(|index| self.objects[*index].should_rehome(self.world.bounds))
            .collect();
        for index in rehome_indices {
            self.objects[index] = self.spawn_fauna_kind(kind, false);
        }

        let current = self.indices_for_object_name(name).len();
        if current < desired {
            for _ in 0..(desired - current) {
                let object = self.spawn_fauna_kind(kind, true);
                self.objects.push(object);
            }
        }
    }

    fn sync_flora_kind_count(&mut self, kind: FloraKind, name: &'static str, desired: usize) {
        self.trim_object_count(name, desired);

        let rehome_indices: Vec<usize> = self
            .indices_for_object_name(name)
            .into_iter()
            .filter(|index| self.objects[*index].should_rehome_in_cover_world(&self.world))
            .collect();
        let mut remove_indices = Vec::new();
        for index in rehome_indices {
            if let Some(replacement) = self.spawn_flora_kind(kind) {
                self.objects[index] = replacement;
            } else {
                remove_indices.push(index);
            }
        }
        self.remove_indices(remove_indices);

        let current = self.indices_for_object_name(name).len();
        if current < desired {
            for _ in 0..(desired - current) {
                if let Some(object) = self.spawn_flora_kind(kind) {
                    self.objects.push(object);
                }
            }
        }
    }

    fn trim_object_count(&mut self, name: &'static str, desired: usize) {
        let indices = self.indices_for_object_name(name);
        if indices.len() > desired {
            self.remove_indices(indices.into_iter().skip(desired).collect());
        }
    }

    fn indices_for_object_name(&self, name: &'static str) -> Vec<usize> {
        self.objects
            .iter()
            .enumerate()
            .filter_map(|(index, object)| (object.object_name() == name).then_some(index))
            .collect()
    }

    fn clamp_viewport(&mut self) {
        self.viewport.x = self.viewport.x.clamp(
            0.0,
            (self.world.bounds.width - self.viewport.width).max(0.0),
        );
        self.viewport.y = self.viewport.y.clamp(
            0.0,
            (self.world.bounds.height - self.viewport.height).max(0.0),
        );
    }

    fn spawn_fauna_kind(&mut self, kind: FaunaKind, edge_only: bool) -> Box<dyn SceneObject> {
        match kind {
            FaunaKind::Aphid => self.spawn_aphid(edge_only),
            FaunaKind::LeafBeetle => self.spawn_leaf_beetle(edge_only),
            FaunaKind::GroundBeetle => self.spawn_ground_beetle(edge_only),
            FaunaKind::Centipede => self.spawn_centipede(edge_only),
            FaunaKind::Ladybug => self.spawn_ladybug(edge_only),
        }
    }

    fn spawn_emerged_fauna(&mut self, kind: FaunaKind, point: Point) -> Box<dyn SceneObject> {
        let emergence_point = self
            .open_point_near(point, fauna_emergence_clearance(kind))
            .unwrap_or(point);
        let mut object = self.spawn_fauna_kind(kind, false);
        object.emerge_at(emergence_point, &self.world, self.viewport);
        object
    }

    fn open_point_near(&mut self, center: Point, clearance: f64) -> Option<Point> {
        if self.world.is_open_ground(center, clearance) {
            return Some(center);
        }

        for _ in 0..18 {
            let angle = self.rng.range(0.0, TAU);
            let distance = self.rng.range(clearance * 0.8, clearance * 4.4 + 24.0);
            let candidate = Point {
                x: center.x + angle.cos() * distance,
                y: center.y + angle.sin() * distance,
            };

            if self.world.is_open_ground(candidate, clearance) {
                return Some(candidate);
            }
        }

        None
    }

    fn spawn_flora_kind(&mut self, kind: FloraKind) -> Option<Box<dyn SceneObject>> {
        let root = self.world.sample_open_point(&mut self.rng, 10.0)?;
        self.spawn_flora_of_kind_at(kind, root)
    }

    fn indices_for_role(&self, role: SceneRole) -> Vec<usize> {
        self.objects
            .iter()
            .enumerate()
            .filter_map(|(index, object)| (object.role() == role).then_some(index))
            .collect()
    }

    fn remove_indices(&mut self, mut indices: Vec<usize>) {
        indices.sort_unstable();

        for index in indices.into_iter().rev() {
            self.objects.remove(index);
        }
    }

    fn redistribute_fauna(&mut self) {
        for index in self.indices_for_role(SceneRole::Fauna) {
            let replacement = match self.objects[index].object_name() {
                "Aphid" => Some(self.spawn_fauna_kind(FaunaKind::Aphid, false)),
                "LeafBeetle" => Some(self.spawn_fauna_kind(FaunaKind::LeafBeetle, false)),
                "GroundBeetle" => Some(self.spawn_fauna_kind(FaunaKind::GroundBeetle, false)),
                "Centipede" => Some(self.spawn_fauna_kind(FaunaKind::Centipede, false)),
                "Ladybug" => Some(self.spawn_fauna_kind(FaunaKind::Ladybug, false)),
                _ => None,
            };

            if let Some(replacement) = replacement {
                self.objects[index] = replacement;
            }
        }
    }

    fn spawn_flora_of_kind_at(
        &mut self,
        kind: FloraKind,
        root: Point,
    ) -> Option<Box<dyn SceneObject>> {
        if !self.world.is_open_ground(root, 10.0) {
            return None;
        }

        Some(match kind {
            FloraKind::Dandelion => {
                Box::new(Dandelion::spawn(root, &mut self.rng)) as Box<dyn SceneObject>
            }
            FloraKind::Strawberry => {
                Box::new(StrawberryPlant::spawn(root, &mut self.rng)) as Box<dyn SceneObject>
            }
            FloraKind::Grass => {
                Box::new(GrassClump::spawn(root, &mut self.rng)) as Box<dyn SceneObject>
            }
        })
    }

    fn resolve_flora_root(&mut self, landing: Point, occupied: &[Point]) -> Option<Point> {
        if self.can_spawn_flora_at(landing, occupied) {
            return Some(landing);
        }

        for _ in 0..12 {
            let angle = self.rng.range(0.0, TAU);
            let distance = self.rng.range(6.0, 24.0);
            let candidate = Point {
                x: landing.x + angle.cos() * distance,
                y: landing.y + angle.sin() * distance,
            };

            if self.can_spawn_flora_at(candidate, occupied) {
                return Some(candidate);
            }
        }

        None
    }

    fn can_spawn_flora_at(&self, root: Point, occupied: &[Point]) -> bool {
        self.world.is_open_ground(root, 10.0)
            && occupied.iter().all(|point| point.distance_to(root) >= 24.0)
    }

    fn spawn_leaf_beetle(&mut self, edge_only: bool) -> Box<dyn SceneObject> {
        Box::new(LeafBeetle::spawn(&mut self.rng, &self.world, edge_only))
    }

    fn spawn_aphid(&mut self, edge_only: bool) -> Box<dyn SceneObject> {
        let aphid_host_targets: Vec<PlantTarget> = self
            .objects
            .iter()
            .filter_map(|object| object.aphid_host_position())
            .collect();
        let aphid_fallback_hosts: Vec<PlantTarget> = self
            .objects
            .iter()
            .filter_map(|object| object.aphid_fallback_host_position())
            .collect();

        Box::new(Aphid::spawn(
            &mut self.rng,
            &self.world,
            edge_only,
            &aphid_host_targets,
            &aphid_fallback_hosts,
        ))
    }

    fn spawn_ground_beetle(&mut self, edge_only: bool) -> Box<dyn SceneObject> {
        Box::new(GroundBeetle::spawn(&mut self.rng, &self.world, edge_only))
    }

    fn spawn_centipede(&mut self, edge_only: bool) -> Box<dyn SceneObject> {
        Box::new(Centipede::spawn(&mut self.rng, &self.world, edge_only))
    }

    fn spawn_ladybug(&mut self, edge_only: bool) -> Box<dyn SceneObject> {
        Box::new(Ladybug::spawn(&mut self.rng, &self.world, edge_only))
    }
}

impl Aphid {
    fn spawn(
        rng: &mut Lcg,
        world: &CoverWorld,
        edge_only: bool,
        preferred_hosts: &[PlantTarget],
        fallback_hosts: &[PlantTarget],
    ) -> Self {
        let host_flora = random_preferred_target(rng, preferred_hosts, fallback_hosts, 0.82);
        let host_offset = aphid_host_offset(rng);
        let position = host_flora
            .map(|host| Point {
                x: host.x + host_offset.x,
                y: host.y + host_offset.y,
            })
            .unwrap_or_else(|| {
                world.sample_open_point(rng, 8.0).unwrap_or_else(|| {
                    fallback_ground_beetle_position(rng, world.bounds, edge_only)
                })
            });

        Self {
            motion: MotionState {
                position,
                heading: rng.range(0.0, TAU),
                base_speed: rng.range(APHID_MOTION.speed_min, APHID_MOTION.speed_max),
                size: rng.range(APHID_MOTION.size_min, APHID_MOTION.size_max),
                stride_phase: rng.range(0.0, TAU),
                wander_phase: rng.range(0.0, TAU),
            },
            host_flora,
            host_offset,
            drop_target: position,
            state: if host_flora.is_some() {
                AphidState::Feeding
            } else {
                AphidState::Wandering
            },
            state_time: rng.range(0.0, 80.0),
            body_phase: rng.range(0.0, TAU),
            antenna_phase: rng.range(0.0, TAU),
        }
    }

    fn live_host(&self, update: &UpdateContext) -> Option<Point> {
        self.host_flora
            .and_then(|host| {
                nearest_flora_target(host, update.aphid_host_targets, 34.0)
                    .or_else(|| nearest_flora_target(host, update.aphid_fallback_hosts, 34.0))
            })
            .or_else(|| {
                nearest_preferred_target(
                    self.motion.position,
                    update.aphid_host_targets,
                    update.aphid_fallback_hosts,
                    230.0,
                    150.0,
                )
            })
    }

    fn feeding_point(&self, host: Point, time: f64) -> Point {
        let crawl = (time * 0.34 + self.body_phase).sin();
        Point {
            x: host.x + self.host_offset.x + crawl * 1.6,
            y: host.y + self.host_offset.y + (time * 0.27 + self.antenna_phase).cos() * 1.1,
        }
    }

    fn nearest_threat(&self, update: &UpdateContext) -> Option<Point> {
        nearest_visible_target(
            self.motion.position,
            update.aphid_threats,
            46.0,
            update.cover_regions,
            5.0,
        )
    }

    fn drop_from_threat(&mut self, threat: Point, update: &UpdateContext) {
        self.host_flora = None;
        self.drop_target = flee_point_from(self.motion.position, threat, update, 38.0, 7.0);
        self.state = AphidState::Dropping;
        self.state_time = 0.0;
    }
}

impl SceneObject for Aphid {
    fn object_name(&self) -> &'static str {
        "Aphid"
    }

    fn update(&mut self, update: &UpdateContext) {
        self.state_time += update.dt;

        if let Some(threat) = self.nearest_threat(update) {
            if self.state != AphidState::Dropping || self.state_time > 18.0 {
                self.drop_from_threat(threat, update);
            }
        }

        match self.state {
            AphidState::Feeding => {
                let Some(host) = self.live_host(update) else {
                    self.state = AphidState::Wandering;
                    self.state_time = 0.0;
                    return;
                };

                self.host_flora = Some(host);
                let target = self.feeding_point(host, update.time);
                move_motion_toward(
                    &mut self.motion,
                    target,
                    update,
                    APHID_MOTION,
                    None,
                    0.56,
                    (update.time * 0.72 + self.body_phase).sin() * 0.03,
                );
            }
            AphidState::Wandering => {
                if let Some(host) = nearest_preferred_target(
                    self.motion.position,
                    update.aphid_host_targets,
                    update.aphid_fallback_hosts,
                    260.0,
                    150.0,
                ) {
                    self.host_flora = Some(host);
                    self.state = AphidState::Feeding;
                    self.state_time = 0.0;
                    return;
                }

                let wander = Point {
                    x: self.motion.position.x + (update.time * 0.34 + self.body_phase).cos() * 18.0,
                    y: self.motion.position.y
                        + (update.time * 0.29 + self.antenna_phase).sin() * 14.0,
                };
                move_motion_toward(
                    &mut self.motion,
                    wander,
                    update,
                    APHID_MOTION,
                    None,
                    0.42,
                    0.08,
                );
            }
            AphidState::Dropping => {
                let distance = move_motion_toward(
                    &mut self.motion,
                    self.drop_target,
                    update,
                    APHID_MOTION,
                    None,
                    1.35,
                    (update.time * 1.8 + self.body_phase).sin() * 0.06,
                );

                if self.nearest_threat(update).is_none()
                    && (distance < self.motion.size + 3.0 || self.state_time > 42.0)
                {
                    if let Some(host) = nearest_preferred_target(
                        self.motion.position,
                        update.aphid_host_targets,
                        update.aphid_fallback_hosts,
                        210.0,
                        120.0,
                    ) {
                        self.host_flora = Some(host);
                        self.state = AphidState::Feeding;
                    } else {
                        self.state = AphidState::Wandering;
                    }
                    self.state_time = 0.0;
                }
            }
        }
    }

    fn draw(&self, render: &RenderContext) -> Result<(), JsValue> {
        draw_aphid(self, render)
    }

    fn position(&self) -> Point {
        self.motion.position
    }

    fn flora_bite_request(&self) -> Option<FloraBiteRequest> {
        (self.state == AphidState::Feeding).then_some(FloraBiteRequest {
            point: self.motion.position,
            radius: self.motion.size + 6.0,
            damage: 0.00055,
            kind: FloraDamageKind::SapDrain,
        })
    }

    fn aphid_prey_position(&self) -> Option<Point> {
        (self.state != AphidState::Dropping || self.state_time < 18.0)
            .then_some(self.motion.position)
    }

    fn emerge_at(&mut self, point: Point, _world: &CoverWorld, _viewport: Viewport) {
        self.motion.position = point;
        self.host_flora = None;
        self.drop_target = point;
        self.state = AphidState::Wandering;
        self.state_time = 0.0;
    }

    fn splat_candidate(
        &self,
        point: Point,
        viewport: Viewport,
    ) -> Option<(f64, Box<dyn SceneObject>)> {
        circular_splat_candidate(
            point,
            self.motion.position,
            self.motion.size * 1.9,
            viewport,
            self.motion.heading,
            self.motion.size * 1.05,
            APHID_SPLAT_PALETTE,
            self.body_phase + self.antenna_phase,
        )
    }
}

impl LeafBeetle {
    fn spawn(rng: &mut Lcg, world: &CoverWorld, _edge_only: bool) -> Self {
        let shelter_origin = Point {
            x: rng.range(0.0, world.bounds.width.max(1.0)),
            y: rng.range(0.0, world.bounds.height.max(1.0)),
        };
        let (home, hide_anchor, peek_point) = world
            .nearest_shelter_home(shelter_origin, 10.0, 14.0)
            .unwrap_or_else(|| fallback_leaf_beetle_home(rng, world.bounds));

        Self {
            motion: MotionState {
                position: hide_anchor,
                heading: hide_anchor.angle_to(peek_point),
                base_speed: rng.range(LEAF_BEETLE_MOTION.speed_min, LEAF_BEETLE_MOTION.speed_max),
                size: rng.range(LEAF_BEETLE_MOTION.size_min, LEAF_BEETLE_MOTION.size_max),
                stride_phase: rng.range(0.0, TAU),
                wander_phase: rng.range(0.0, TAU),
            },
            home,
            hide_anchor,
            peek_point,
            target_flora: None,
            state: LeafBeetleState::Hidden,
            state_time: rng.range(0.0, 160.0),
            shell_phase: rng.range(0.0, TAU),
            antenna_phase: rng.range(0.0, TAU),
            peek_phase: rng.range(0.0, TAU),
        }
    }

    fn sync_home(&mut self, cover_regions: &[CoverRegion]) {
        let Some(nearest_home) = nearest_shelter_rect(self.motion.position, cover_regions, 10.0)
        else {
            return;
        };

        let current_is_real = cover_regions
            .iter()
            .any(|region| region.kind == CoverRegionKind::Shelter && region.rect == self.home);
        let current_distance = if current_is_real {
            distance_to_rect(self.motion.position, self.home)
        } else {
            f64::INFINITY
        };
        let next_distance = distance_to_rect(self.motion.position, nearest_home);

        if !current_is_real || (nearest_home != self.home && next_distance + 8.0 < current_distance)
        {
            self.home = nearest_home;
            let (hide_anchor, peek_point) =
                shelter_anchor_from_hint(nearest_home, self.motion.position, 10.0, 14.0);
            self.hide_anchor = hide_anchor;
            self.peek_point = peek_point;
        }
    }
}

impl SceneObject for LeafBeetle {
    fn object_name(&self) -> &'static str {
        "LeafBeetle"
    }

    fn update(&mut self, update: &UpdateContext) {
        self.state_time += update.dt;
        self.sync_home(update.cover_regions);

        if self.state != LeafBeetleState::Hidden
            && nearest_visible_target(
                self.motion.position,
                update.leaf_beetle_threats,
                118.0,
                update.cover_regions,
                6.0,
            )
            .is_some()
        {
            self.target_flora = None;
            self.state = LeafBeetleState::Returning;
            self.state_time = 0.0;
        }

        match self.state {
            LeafBeetleState::Hidden => {
                move_motion_toward(
                    &mut self.motion,
                    self.hide_anchor,
                    update,
                    LEAF_BEETLE_MOTION,
                    Some(self.home),
                    0.74,
                    -0.02,
                );

                let peek_signal = (update.time * 0.58 + self.peek_phase).sin();
                if self.state_time > 72.0 && peek_signal > 0.95 {
                    self.state = LeafBeetleState::Peeking;
                    self.state_time = 0.0;
                }
            }
            LeafBeetleState::Peeking => {
                let distance = move_motion_toward(
                    &mut self.motion,
                    self.peek_point,
                    update,
                    LEAF_BEETLE_MOTION,
                    Some(self.home),
                    1.02,
                    0.04,
                );

                if let Some(target) = (distance < 5.0 || self.state_time > 24.0)
                    .then(|| {
                        nearest_preferred_visible_target(
                            self.peek_point,
                            update.leaf_beetle_food_targets,
                            update.leaf_beetle_fallback_food,
                            250.0,
                            150.0,
                            update.cover_regions,
                            5.0,
                        )
                    })
                    .flatten()
                {
                    self.target_flora = Some(target);
                    self.state = LeafBeetleState::Foraging;
                    self.state_time = 0.0;
                    return;
                }

                if self.state_time > 92.0 || (distance < 4.2 && self.state_time > 28.0) {
                    self.state = LeafBeetleState::Returning;
                    self.state_time = 0.0;
                }
            }
            LeafBeetleState::Foraging => {
                self.target_flora = nearest_preferred_visible_target(
                    self.motion.position,
                    update.leaf_beetle_food_targets,
                    update.leaf_beetle_fallback_food,
                    290.0,
                    170.0,
                    update.cover_regions,
                    5.0,
                )
                .or_else(|| {
                    self.target_flora.filter(|target| {
                        visible_target_between(
                            self.motion.position,
                            *target,
                            update.cover_regions,
                            5.0,
                        )
                    })
                });

                let Some(target) = self.target_flora else {
                    self.state = LeafBeetleState::Returning;
                    self.state_time = 0.0;
                    return;
                };

                let chase_wander =
                    (update.time * 1.8 + self.shell_phase + self.motion.wander_phase).sin() * 0.02;
                let distance = move_motion_toward(
                    &mut self.motion,
                    target,
                    update,
                    LEAF_BEETLE_MOTION,
                    Some(self.home),
                    1.38,
                    chase_wander,
                );

                if distance < self.motion.size + 6.0 {
                    self.motion.position = target;
                    self.state = LeafBeetleState::Eating;
                    self.state_time = 0.0;
                } else if self.state_time > 280.0 {
                    self.target_flora = None;
                    self.state = LeafBeetleState::Returning;
                    self.state_time = 0.0;
                }
            }
            LeafBeetleState::Eating => {
                if let Some(target) = self.target_flora {
                    move_motion_toward(
                        &mut self.motion,
                        target,
                        update,
                        LEAF_BEETLE_MOTION,
                        Some(self.home),
                        0.32,
                        0.0,
                    );
                }

                if self.state_time > 18.0 {
                    self.target_flora = None;
                    self.state = LeafBeetleState::Returning;
                    self.state_time = 0.0;
                }
            }
            LeafBeetleState::Returning => {
                let distance = move_motion_toward(
                    &mut self.motion,
                    self.hide_anchor,
                    update,
                    LEAF_BEETLE_MOTION,
                    Some(self.home),
                    1.04,
                    -0.04,
                );

                if distance < 3.2 {
                    self.motion.position = self.hide_anchor;
                    self.target_flora = None;
                    self.state = LeafBeetleState::Hidden;
                    self.state_time = 0.0;
                }
            }
        }
    }

    fn draw(&self, render: &RenderContext) -> Result<(), JsValue> {
        draw_leaf_beetle(self, render)
    }

    fn position(&self) -> Point {
        self.motion.position
    }

    fn flora_bite_request(&self) -> Option<FloraBiteRequest> {
        (self.state == LeafBeetleState::Eating).then_some(FloraBiteRequest {
            point: self.motion.position,
            radius: self.motion.size + 7.0,
            damage: 0.012,
            kind: FloraDamageKind::LeafChew,
        })
    }

    fn on_flora_bitten(&mut self, point: Point) {
        self.motion.position = point;
    }

    fn leaf_beetle_prey_position(&self) -> Option<Point> {
        (self.state != LeafBeetleState::Hidden).then_some(self.motion.position)
    }

    fn emerge_at(&mut self, point: Point, world: &CoverWorld, _viewport: Viewport) {
        if let Some((home, hide_anchor, peek_point)) = world.nearest_shelter_home(point, 10.0, 14.0)
        {
            self.home = home;
            self.hide_anchor = hide_anchor;
            self.peek_point = peek_point;
            self.motion.position = hide_anchor;
            self.motion.heading = hide_anchor.angle_to(peek_point);
        } else {
            self.home = Rect {
                x: point.x - 8.0,
                y: point.y - 8.0,
                width: 16.0,
                height: 16.0,
            };
            self.hide_anchor = point;
            self.peek_point = point;
            self.motion.position = point;
        }
        self.target_flora = None;
        self.state = LeafBeetleState::Hidden;
        self.state_time = 0.0;
    }

    fn splat_candidate(
        &self,
        point: Point,
        viewport: Viewport,
    ) -> Option<(f64, Box<dyn SceneObject>)> {
        if self.state == LeafBeetleState::Hidden {
            return None;
        }

        circular_splat_candidate(
            point,
            self.motion.position,
            self.motion.size * 1.85,
            viewport,
            self.motion.heading,
            self.motion.size * 1.05,
            LEAF_BEETLE_SPLAT_PALETTE,
            self.shell_phase + self.antenna_phase,
        )
    }
}

impl GroundBeetle {
    fn spawn(rng: &mut Lcg, world: &CoverWorld, edge_only: bool) -> Self {
        let position = world
            .sample_open_point(rng, 12.0)
            .unwrap_or_else(|| fallback_ground_beetle_position(rng, world.bounds, edge_only));
        let patrol_target = world.sample_open_point(rng, 14.0).unwrap_or(position);
        let (rest_home, rest_anchor) = shelter_rest_home(position, &world.regions, 12.0);

        Self {
            motion: MotionState {
                position,
                heading: position.angle_to(patrol_target),
                base_speed: rng.range(
                    GROUND_BEETLE_MOTION.speed_min,
                    GROUND_BEETLE_MOTION.speed_max,
                ),
                size: rng.range(GROUND_BEETLE_MOTION.size_min, GROUND_BEETLE_MOTION.size_max),
                stride_phase: rng.range(0.0, TAU),
                wander_phase: rng.range(0.0, TAU),
            },
            rest_home,
            rest_anchor,
            patrol_target,
            target_prey: None,
            target_prey_kind: None,
            state: if rest_home.is_some() && rng.bool(0.42) {
                GroundBeetleState::Sheltering
            } else {
                GroundBeetleState::Patrolling
            },
            state_time: rng.range(0.0, 120.0),
            shell_phase: rng.range(0.0, TAU),
            antenna_phase: rng.range(0.0, TAU),
            patrol_cursor: (rng.next_f64() * 32.0).floor() as u32,
            hunger: rng.range(0.0, GROUND_BEETLE_STARVE_TIME * 0.24),
        }
    }

    fn retarget_patrol(&mut self, update: &UpdateContext) {
        self.patrol_target = ground_beetle_patrol_target(self, update);
        self.patrol_cursor = self.patrol_cursor.wrapping_add(1);
    }

    fn sync_rest_home(&mut self, cover_regions: &[CoverRegion]) {
        let (home, anchor) = shelter_rest_home(self.motion.position, cover_regions, 12.0);
        if home.is_some() {
            self.rest_home = home;
            self.rest_anchor = anchor;
        }
    }

    fn start_dispersal(&mut self, update: &UpdateContext) {
        self.target_prey = None;
        self.target_prey_kind = None;
        self.patrol_target = dispersal_target_from(
            self.motion.position,
            update.bounds,
            self.shell_phase + self.antenna_phase,
            PREDATOR_DISPERSAL_MARGIN,
        );
        self.state = GroundBeetleState::Dispersing;
        self.state_time = 0.0;
    }
}

impl SceneObject for GroundBeetle {
    fn object_name(&self) -> &'static str {
        "GroundBeetle"
    }

    fn update(&mut self, update: &UpdateContext) {
        self.state_time += update.dt;
        self.sync_rest_home(update.cover_regions);

        let food_available = ground_beetle_food_available(update);
        if self.state == GroundBeetleState::Feeding {
            self.hunger = (self.hunger - update.dt * 14.0).max(0.0);
        } else if food_available {
            self.hunger = (self.hunger - update.dt * 0.54).max(0.0);
        } else {
            self.hunger += update.dt;
        }

        if self.state != GroundBeetleState::Dispersing && self.hunger > GROUND_BEETLE_STARVE_TIME {
            self.start_dispersal(update);
        }

        if self.state != GroundBeetleState::Dispersing {
            if let Some(threat) = nearest_visible_target(
                self.motion.position,
                update.ground_beetle_threats,
                156.0,
                update.cover_regions,
                8.0,
            ) {
                self.target_prey = None;
                self.target_prey_kind = None;
                self.patrol_target =
                    flee_point_from(self.motion.position, threat, update, 92.0, 14.0);
                self.state = if self.rest_home.is_some() {
                    GroundBeetleState::Sheltering
                } else {
                    GroundBeetleState::Patrolling
                };
                self.state_time = 0.0;
            }
        }

        match self.state {
            GroundBeetleState::Sheltering => {
                if self.state_time > 36.0 {
                    if let Some((target, prey_kind)) =
                        ground_beetle_prey_target(self.motion.position, update, 190.0, 130.0)
                    {
                        self.target_prey = Some(target);
                        self.target_prey_kind = Some(prey_kind);
                        self.state = GroundBeetleState::Hunting;
                        self.state_time = 0.0;
                        return;
                    }
                }

                let distance = move_motion_toward(
                    &mut self.motion,
                    self.rest_anchor,
                    update,
                    GROUND_BEETLE_MOTION,
                    self.rest_home,
                    0.82,
                    -0.03,
                );

                if (distance < 5.0 && self.state_time > 170.0) || self.state_time > 260.0 {
                    self.state = GroundBeetleState::Patrolling;
                    self.state_time = 0.0;
                    self.retarget_patrol(update);
                }
            }
            GroundBeetleState::Patrolling => {
                if let Some((target, prey_kind)) =
                    ground_beetle_prey_target(self.motion.position, update, 280.0, 180.0)
                {
                    self.target_prey = Some(target);
                    self.target_prey_kind = Some(prey_kind);
                    self.state = GroundBeetleState::Hunting;
                    self.state_time = 0.0;
                    return;
                }

                let rest_signal = (update.time * 0.19 + self.shell_phase).sin();
                if self.rest_home.is_some() && self.state_time > 145.0 && rest_signal > 0.52 {
                    self.state = GroundBeetleState::Sheltering;
                    self.state_time = 0.0;
                    return;
                }

                let patrol_wander =
                    (update.time * 0.84 + self.motion.wander_phase + self.shell_phase).sin() * 0.04;
                let distance = move_motion_toward(
                    &mut self.motion,
                    self.patrol_target,
                    update,
                    GROUND_BEETLE_MOTION,
                    None,
                    0.92,
                    patrol_wander,
                );

                if distance < 10.0 || self.state_time > 180.0 {
                    self.retarget_patrol(update);
                    self.state_time = 0.0;
                }
            }
            GroundBeetleState::Hunting => {
                if let Some((target, prey_kind)) =
                    ground_beetle_prey_target(self.motion.position, update, 360.0, 230.0)
                {
                    self.target_prey = Some(target);
                    self.target_prey_kind = Some(prey_kind);
                } else {
                    self.target_prey = self.target_prey.filter(|target| {
                        visible_target_between(
                            self.motion.position,
                            *target,
                            update.cover_regions,
                            8.0,
                        )
                    });
                }

                let Some(target) = self.target_prey else {
                    self.target_prey_kind = None;
                    self.state = GroundBeetleState::Patrolling;
                    self.state_time = 0.0;
                    self.retarget_patrol(update);
                    return;
                };

                let chase_wander =
                    (update.time * 1.2 + self.shell_phase + self.motion.wander_phase).sin() * 0.015;
                let distance = move_motion_toward(
                    &mut self.motion,
                    target,
                    update,
                    GROUND_BEETLE_MOTION,
                    None,
                    1.5,
                    chase_wander,
                );

                if distance < self.motion.size + 8.0 {
                    self.motion.position = target;
                    self.state = GroundBeetleState::Feeding;
                    self.state_time = 0.0;
                } else if self.state_time > 240.0 {
                    self.target_prey = None;
                    self.target_prey_kind = None;
                    self.state = GroundBeetleState::Patrolling;
                    self.state_time = 0.0;
                    self.retarget_patrol(update);
                }
            }
            GroundBeetleState::Feeding => {
                if let Some(target) = self.target_prey {
                    move_motion_toward(
                        &mut self.motion,
                        target,
                        update,
                        GROUND_BEETLE_MOTION,
                        None,
                        0.28,
                        0.0,
                    );
                }

                if self.state_time > 18.0 {
                    self.target_prey = None;
                    self.target_prey_kind = None;
                    self.state = GroundBeetleState::Patrolling;
                    self.state_time = 0.0;
                    self.retarget_patrol(update);
                }
            }
            GroundBeetleState::Dispersing => {
                move_motion_toward(
                    &mut self.motion,
                    self.patrol_target,
                    update,
                    GROUND_BEETLE_MOTION,
                    None,
                    1.24,
                    (update.time * 0.74 + self.shell_phase).sin() * 0.03,
                );
            }
        }
    }

    fn draw(&self, render: &RenderContext) -> Result<(), JsValue> {
        draw_ground_beetle(self, render)
    }

    fn position(&self) -> Point {
        self.motion.position
    }

    fn should_respawn(&self, bounds: WorldBounds) -> bool {
        let position = self.position();
        self.state == GroundBeetleState::Dispersing && self.state_time > 320.0
            || position.x < -RESPAWN_MARGIN
            || position.x > bounds.width + RESPAWN_MARGIN
            || position.y < -RESPAWN_MARGIN
            || position.y > bounds.height + RESPAWN_MARGIN
    }

    fn aphid_bite_request(&self) -> Option<(Point, f64)> {
        (self.state == GroundBeetleState::Feeding
            && self.target_prey_kind == Some(SmallPredatorPrey::Aphid))
        .then_some((self.motion.position, self.motion.size + 7.0))
    }

    fn on_aphid_eaten(&mut self, point: Point) {
        self.motion.position = point;
        self.target_prey = None;
        self.target_prey_kind = None;
        self.hunger = (self.hunger - GROUND_BEETLE_STARVE_TIME * 0.42).max(0.0);
        self.state = GroundBeetleState::Feeding;
        self.state_time = 0.0;
    }

    fn leaf_beetle_bite_request(&self) -> Option<(Point, f64)> {
        (self.state == GroundBeetleState::Feeding
            && self.target_prey_kind == Some(SmallPredatorPrey::LeafBeetle))
        .then_some((self.motion.position, self.motion.size + 8.0))
    }

    fn on_leaf_beetle_eaten(&mut self, point: Point) {
        self.motion.position = point;
        self.target_prey = None;
        self.target_prey_kind = None;
        self.hunger = (self.hunger - GROUND_BEETLE_STARVE_TIME * 0.52).max(0.0);
        self.state = GroundBeetleState::Feeding;
        self.state_time = 0.0;
    }

    fn ground_beetle_prey_position(&self) -> Option<Point> {
        (self.state != GroundBeetleState::Sheltering).then_some(self.motion.position)
    }

    fn aphid_threat_position(&self) -> Option<Point> {
        (self.state != GroundBeetleState::Sheltering).then_some(self.motion.position)
    }

    fn leaf_beetle_threat_position(&self) -> Option<Point> {
        (self.state != GroundBeetleState::Sheltering).then_some(self.motion.position)
    }

    fn emerge_at(&mut self, point: Point, world: &CoverWorld, _viewport: Viewport) {
        self.motion.position = point;
        self.patrol_target = point;
        let (home, anchor) = shelter_rest_home(point, &world.regions, 12.0);
        self.rest_home = home;
        self.rest_anchor = anchor;
        self.target_prey = None;
        self.target_prey_kind = None;
        self.state = if home.is_some() {
            GroundBeetleState::Sheltering
        } else {
            GroundBeetleState::Patrolling
        };
        self.state_time = 0.0;
        self.hunger = GROUND_BEETLE_STARVE_TIME * 0.22;
    }

    fn splat_candidate(
        &self,
        point: Point,
        viewport: Viewport,
    ) -> Option<(f64, Box<dyn SceneObject>)> {
        if self.state == GroundBeetleState::Sheltering {
            return None;
        }

        circular_splat_candidate(
            point,
            self.motion.position,
            self.motion.size * 1.52,
            viewport,
            self.motion.heading,
            self.motion.size * 1.14,
            GROUND_BEETLE_SPLAT_PALETTE,
            self.shell_phase + self.antenna_phase,
        )
    }
}

impl Centipede {
    fn spawn(rng: &mut Lcg, world: &CoverWorld, edge_only: bool) -> Self {
        let position = world
            .sample_open_point(rng, 16.0)
            .unwrap_or_else(|| fallback_ground_beetle_position(rng, world.bounds, edge_only));
        let patrol_target = world.sample_open_point(rng, 18.0).unwrap_or(position);
        let heading = position.angle_to(patrol_target);
        let size = rng.range(CENTIPEDE_MOTION.size_min, CENTIPEDE_MOTION.size_max);
        let (rest_home, rest_anchor) = shelter_rest_home(position, &world.regions, 16.0);

        Self {
            motion: MotionState {
                position,
                heading,
                base_speed: rng.range(CENTIPEDE_MOTION.speed_min, CENTIPEDE_MOTION.speed_max),
                size,
                stride_phase: rng.range(0.0, TAU),
                wander_phase: rng.range(0.0, TAU),
            },
            rest_home,
            rest_anchor,
            patrol_target,
            target_prey: None,
            target_prey_kind: None,
            state: if rest_home.is_some() && rng.bool(0.52) {
                CentipedeState::Sheltering
            } else {
                CentipedeState::Patrolling
            },
            state_time: rng.range(0.0, 160.0),
            trail: seed_centipede_trail(position, heading, size),
            body_phase: rng.range(0.0, TAU),
            antenna_phase: rng.range(0.0, TAU),
            patrol_cursor: (rng.next_f64() * 48.0).floor() as u32,
            hunger: rng.range(0.0, CENTIPEDE_STARVE_TIME * 0.24),
        }
    }

    fn retarget_patrol(&mut self, update: &UpdateContext) {
        self.patrol_target = centipede_patrol_target(self, update);
        self.patrol_cursor = self.patrol_cursor.wrapping_add(1);
    }

    fn sync_rest_home(&mut self, cover_regions: &[CoverRegion]) {
        let (home, anchor) = shelter_rest_home(self.motion.position, cover_regions, 16.0);
        if home.is_some() {
            self.rest_home = home;
            self.rest_anchor = anchor;
        }
    }

    fn sync_trail(&mut self, previous_position: Point) {
        extend_centipede_trail(
            &mut self.trail,
            previous_position,
            self.motion.position,
            self.motion.heading,
            self.motion.size,
        );
    }

    fn reset_trail(&mut self) {
        self.trail =
            seed_centipede_trail(self.motion.position, self.motion.heading, self.motion.size);
    }

    fn start_dispersal(&mut self, update: &UpdateContext) {
        self.target_prey = None;
        self.target_prey_kind = None;
        self.patrol_target = dispersal_target_from(
            self.motion.position,
            update.bounds,
            self.body_phase + self.antenna_phase,
            PREDATOR_DISPERSAL_MARGIN * 1.25,
        );
        self.state = CentipedeState::Dispersing;
        self.state_time = 0.0;
    }
}

impl SceneObject for Centipede {
    fn object_name(&self) -> &'static str {
        "Centipede"
    }

    fn update(&mut self, update: &UpdateContext) {
        self.state_time += update.dt;
        let previous_position = self.motion.position;
        self.sync_rest_home(update.cover_regions);

        let food_available = centipede_food_available(update);
        if self.state == CentipedeState::Feeding {
            self.hunger = (self.hunger - update.dt * 12.0).max(0.0);
        } else if food_available {
            self.hunger = (self.hunger - update.dt * 0.42).max(0.0);
        } else {
            self.hunger += update.dt;
        }

        if self.state != CentipedeState::Dispersing && self.hunger > CENTIPEDE_STARVE_TIME {
            self.start_dispersal(update);
        }

        match self.state {
            CentipedeState::Sheltering => {
                if self.state_time > 44.0 {
                    if let Some((target, prey_kind)) =
                        centipede_prey_target(self.motion.position, update, 230.0, 150.0)
                    {
                        self.target_prey = Some(target);
                        self.target_prey_kind = Some(prey_kind);
                        self.state = CentipedeState::Hunting;
                        self.state_time = 0.0;
                    }
                }

                if self.state == CentipedeState::Sheltering {
                    let distance = move_motion_toward(
                        &mut self.motion,
                        self.rest_anchor,
                        update,
                        CENTIPEDE_MOTION,
                        self.rest_home,
                        0.72,
                        -0.04,
                    );

                    if (distance < 6.0 && self.state_time > 210.0) || self.state_time > 330.0 {
                        self.state = CentipedeState::Patrolling;
                        self.state_time = 0.0;
                        self.retarget_patrol(update);
                    }
                }
            }
            CentipedeState::Patrolling => {
                if let Some((target, prey_kind)) =
                    centipede_prey_target(self.motion.position, update, 360.0, 230.0)
                {
                    self.target_prey = Some(target);
                    self.target_prey_kind = Some(prey_kind);
                    self.state = CentipedeState::Hunting;
                    self.state_time = 0.0;
                } else {
                    let rest_signal = (update.time * 0.16 + self.body_phase).sin();
                    if self.rest_home.is_some() && self.state_time > 165.0 && rest_signal > 0.44 {
                        self.state = CentipedeState::Sheltering;
                        self.state_time = 0.0;
                        self.sync_trail(previous_position);
                        return;
                    }

                    let patrol_wander =
                        (update.time * 0.92 + self.motion.wander_phase + self.body_phase).sin()
                            * 0.05;
                    let distance = move_motion_toward(
                        &mut self.motion,
                        self.patrol_target,
                        update,
                        CENTIPEDE_MOTION,
                        None,
                        0.96,
                        patrol_wander,
                    );

                    if distance < 12.0 || self.state_time > 220.0 {
                        self.retarget_patrol(update);
                        self.state_time = 0.0;
                    }
                }
            }
            CentipedeState::Hunting => {
                if let Some((target, prey_kind)) =
                    centipede_prey_target(self.motion.position, update, 460.0, 300.0)
                {
                    self.target_prey = Some(target);
                    self.target_prey_kind = Some(prey_kind);
                } else {
                    self.target_prey = self.target_prey.filter(|target| {
                        visible_target_between(
                            self.motion.position,
                            *target,
                            update.cover_regions,
                            10.0,
                        )
                    });
                }

                if let Some(target) = self.target_prey {
                    let chase_wander =
                        (update.time * 1.46 + self.body_phase + self.motion.wander_phase).sin()
                            * 0.02;
                    let distance = move_motion_toward(
                        &mut self.motion,
                        target,
                        update,
                        CENTIPEDE_MOTION,
                        None,
                        1.68,
                        chase_wander,
                    );

                    if distance < self.motion.size + 10.0 {
                        self.motion.position = target;
                        self.state = CentipedeState::Feeding;
                        self.state_time = 0.0;
                    } else if self.state_time > 300.0 {
                        self.target_prey = None;
                        self.target_prey_kind = None;
                        self.state = CentipedeState::Patrolling;
                        self.state_time = 0.0;
                        self.retarget_patrol(update);
                    }
                } else {
                    self.target_prey_kind = None;
                    self.state = CentipedeState::Patrolling;
                    self.state_time = 0.0;
                    self.retarget_patrol(update);
                }
            }
            CentipedeState::Feeding => {
                if let Some(target) = self.target_prey {
                    move_motion_toward(
                        &mut self.motion,
                        target,
                        update,
                        CENTIPEDE_MOTION,
                        None,
                        0.24,
                        0.0,
                    );
                }

                if self.state_time > 24.0 {
                    self.target_prey = None;
                    self.target_prey_kind = None;
                    self.state = CentipedeState::Patrolling;
                    self.state_time = 0.0;
                    self.retarget_patrol(update);
                }
            }
            CentipedeState::Dispersing => {
                move_motion_toward(
                    &mut self.motion,
                    self.patrol_target,
                    update,
                    CENTIPEDE_MOTION,
                    None,
                    1.18,
                    (update.time * 0.62 + self.body_phase).sin() * 0.04,
                );
            }
        }

        self.sync_trail(previous_position);
    }

    fn draw(&self, render: &RenderContext) -> Result<(), JsValue> {
        draw_centipede(self, render)
    }

    fn position(&self) -> Point {
        self.motion.position
    }

    fn should_respawn(&self, bounds: WorldBounds) -> bool {
        let position = self.position();
        self.state == CentipedeState::Dispersing && self.state_time > 400.0
            || position.x < -RESPAWN_MARGIN
            || position.x > bounds.width + RESPAWN_MARGIN
            || position.y < -RESPAWN_MARGIN
            || position.y > bounds.height + RESPAWN_MARGIN
    }

    fn leaf_beetle_bite_request(&self) -> Option<(Point, f64)> {
        (self.state == CentipedeState::Feeding
            && self.target_prey_kind == Some(CentipedePrey::LeafBeetle))
        .then_some((self.motion.position, self.motion.size + 10.0))
    }

    fn on_leaf_beetle_eaten(&mut self, point: Point) {
        self.motion.position = point;
        self.target_prey = None;
        self.target_prey_kind = None;
        self.hunger = (self.hunger - CENTIPEDE_STARVE_TIME * 0.34).max(0.0);
        self.state = CentipedeState::Feeding;
        self.state_time = 0.0;
        self.reset_trail();
    }

    fn ground_beetle_bite_request(&self) -> Option<(Point, f64)> {
        (self.state == CentipedeState::Feeding
            && self.target_prey_kind == Some(CentipedePrey::GroundBeetle))
        .then_some((self.motion.position, self.motion.size + 12.0))
    }

    fn on_ground_beetle_eaten(&mut self, point: Point) {
        self.motion.position = point;
        self.target_prey = None;
        self.target_prey_kind = None;
        self.hunger = (self.hunger - CENTIPEDE_STARVE_TIME * 0.52).max(0.0);
        self.state = CentipedeState::Feeding;
        self.state_time = 0.0;
        self.reset_trail();
    }

    fn leaf_beetle_threat_position(&self) -> Option<Point> {
        (self.state != CentipedeState::Sheltering).then_some(self.motion.position)
    }

    fn ground_beetle_threat_position(&self) -> Option<Point> {
        (self.state != CentipedeState::Sheltering).then_some(self.motion.position)
    }

    fn emerge_at(&mut self, point: Point, world: &CoverWorld, _viewport: Viewport) {
        self.motion.position = point;
        self.patrol_target = point;
        let (home, anchor) = shelter_rest_home(point, &world.regions, 16.0);
        self.rest_home = home;
        self.rest_anchor = anchor;
        self.target_prey = None;
        self.target_prey_kind = None;
        self.state = if home.is_some() {
            CentipedeState::Sheltering
        } else {
            CentipedeState::Patrolling
        };
        self.state_time = 0.0;
        self.hunger = CENTIPEDE_STARVE_TIME * 0.24;
        self.reset_trail();
    }

    fn splat_candidate(
        &self,
        point: Point,
        viewport: Viewport,
    ) -> Option<(f64, Box<dyn SceneObject>)> {
        if self.state == CentipedeState::Sheltering {
            return None;
        }

        centipede_splat_candidate(self, point, viewport)
    }
}

impl Ladybug {
    fn spawn(rng: &mut Lcg, world: &CoverWorld, edge_only: bool) -> Self {
        let position = world
            .sample_open_point(rng, 10.0)
            .unwrap_or_else(|| fallback_ground_beetle_position(rng, world.bounds, edge_only));
        let walk_target = world.sample_open_point(rng, 10.0).unwrap_or(position);
        let walk_speed = rng.range(LADYBUG_WALK_MOTION.speed_min, LADYBUG_WALK_MOTION.speed_max);
        let flight_speed = rng.range(LADYBUG_FLY_MOTION.speed_min, LADYBUG_FLY_MOTION.speed_max);

        Self {
            motion: MotionState {
                position,
                heading: if position == walk_target {
                    rng.range(0.0, TAU)
                } else {
                    position.angle_to(walk_target)
                },
                base_speed: walk_speed,
                size: rng.range(LADYBUG_WALK_MOTION.size_min, LADYBUG_WALK_MOTION.size_max),
                stride_phase: rng.range(0.0, TAU),
                wander_phase: rng.range(0.0, TAU),
            },
            walk_target,
            flight_target: position,
            target_aphid: None,
            state: LadybugState::Walking,
            state_time: rng.range(0.0, 96.0),
            flight_delay: rng.range(48.0, 108.0),
            walk_speed,
            flight_speed,
            shell_phase: rng.range(0.0, TAU),
            wing_phase: rng.range(0.0, TAU),
            flight_phase: rng.range(0.0, TAU),
            patrol_cursor: (rng.next_f64() * 36.0).floor() as u32,
            hunger: rng.range(0.0, LADYBUG_STARVE_TIME * 0.2),
        }
    }

    fn retarget_walk(&mut self, update: &UpdateContext) {
        self.walk_target = ladybug_ground_target(self, update);
        self.patrol_cursor = self.patrol_cursor.wrapping_add(1);
    }

    fn refresh_flight_delay(&mut self) {
        let phase_mix = self.flight_phase.sin().abs() * 22.0 + self.shell_phase.cos().abs() * 14.0;
        self.flight_delay = 56.0 + (self.patrol_cursor % 5) as f64 * 14.0 + phase_mix;
    }

    fn start_flight(&mut self, update: &UpdateContext) {
        self.flight_target = ladybug_flight_target(self, update);
        self.state = LadybugState::Flying;
        self.state_time = 0.0;
        self.motion.base_speed = self.flight_speed;
        self.motion.heading = self.motion.position.angle_to(self.flight_target);
        self.patrol_cursor = self.patrol_cursor.wrapping_add(1);
    }

    fn start_dispersal(&mut self, update: &UpdateContext) {
        self.flight_target = dispersal_target_from(
            self.motion.position,
            update.bounds,
            self.wing_phase + self.flight_phase,
            PREDATOR_DISPERSAL_MARGIN,
        );
        self.target_aphid = None;
        self.state = LadybugState::Dispersing;
        self.state_time = 0.0;
        self.motion.base_speed = self.flight_speed;
        self.motion.heading = self.motion.position.angle_to(self.flight_target);
    }

    fn land(&mut self, update: &UpdateContext) {
        self.state = LadybugState::Walking;
        self.state_time = 0.0;
        self.motion.base_speed = self.walk_speed;
        self.target_aphid = None;
        self.walk_target = ladybug_ground_target(self, update);
        self.refresh_flight_delay();
    }
}

impl SceneObject for Ladybug {
    fn object_name(&self) -> &'static str {
        "Ladybug"
    }

    fn update(&mut self, update: &UpdateContext) {
        self.state_time += update.dt;

        if self.state == LadybugState::Feeding {
            self.hunger = (self.hunger - update.dt * 15.0).max(0.0);
        } else if update.aphid_targets.is_empty() {
            self.hunger += update.dt;
        } else {
            self.hunger = (self.hunger - update.dt * 0.62).max(0.0);
        }

        if self.state != LadybugState::Dispersing && self.hunger > LADYBUG_STARVE_TIME {
            self.start_dispersal(update);
        }

        match self.state {
            LadybugState::Walking => {
                let previous_target = self.target_aphid;
                self.target_aphid = tracked_visible_target(
                    self.motion.position,
                    previous_target,
                    update.aphid_targets,
                    42.0,
                    244.0,
                    update.cover_regions,
                    5.0,
                )
                .or_else(|| {
                    nearest_visible_target(
                        self.motion.position,
                        update.aphid_targets,
                        190.0,
                        update.cover_regions,
                        5.0,
                    )
                });
                if target_switched(previous_target, self.target_aphid, 18.0) {
                    self.state_time = 0.0;
                }
                let pursuing_aphid = self.target_aphid.is_some();
                let target = self.target_aphid.unwrap_or(self.walk_target);
                let wander = (update.time * 1.08 + self.shell_phase + self.motion.wander_phase)
                    .sin()
                    * if pursuing_aphid { 0.015 } else { 0.05 };
                let distance = move_motion_toward(
                    &mut self.motion,
                    target,
                    update,
                    LADYBUG_WALK_MOTION,
                    None,
                    if pursuing_aphid { 1.28 } else { 0.94 },
                    wander,
                );

                if pursuing_aphid && distance < self.motion.size + 5.6 {
                    self.motion.position = target;
                    self.state = LadybugState::Feeding;
                    self.state_time = 0.0;
                    return;
                }

                if !pursuing_aphid && (distance < self.motion.size + 5.0 || self.state_time > 132.0)
                {
                    self.retarget_walk(update);
                    if self.state_time > 132.0 {
                        self.state_time = 0.0;
                    }
                }

                if pursuing_aphid && self.state_time > 220.0 {
                    self.target_aphid = None;
                    self.retarget_walk(update);
                    self.state_time = 0.0;
                    return;
                }

                if !pursuing_aphid && self.state_time > self.flight_delay {
                    let candidate = ladybug_flight_target(self, update);
                    if candidate.distance_to(self.motion.position) > 52.0 {
                        self.flight_target = candidate;
                        self.start_flight(update);
                    } else {
                        self.refresh_flight_delay();
                        self.state_time = 0.0;
                    }
                }
            }
            LadybugState::Feeding => {
                if let Some(target) = tracked_visible_target(
                    self.motion.position,
                    self.target_aphid,
                    update.aphid_targets,
                    28.0,
                    34.0,
                    update.cover_regions,
                    5.0,
                )
                .or_else(|| {
                    nearest_visible_target(
                        self.motion.position,
                        update.aphid_targets,
                        22.0,
                        update.cover_regions,
                        5.0,
                    )
                }) {
                    self.target_aphid = Some(target);
                    move_motion_toward(
                        &mut self.motion,
                        target,
                        update,
                        LADYBUG_WALK_MOTION,
                        None,
                        0.34,
                        0.0,
                    );
                }

                if self.state_time > 26.0 {
                    self.target_aphid = None;
                    self.state = LadybugState::Walking;
                    self.state_time = 0.0;
                    self.retarget_walk(update);
                }
            }
            LadybugState::Flying => {
                let flutter = (update.time * 2.4 + self.wing_phase).sin() * 0.018;
                let distance = move_motion_toward(
                    &mut self.motion,
                    self.flight_target,
                    update,
                    LADYBUG_FLY_MOTION,
                    None,
                    1.74,
                    flutter,
                );

                if (self.state_time > 18.0 && distance < self.motion.size + 10.0)
                    || self.state_time > 82.0
                {
                    if distance < self.motion.size + 18.0 {
                        self.motion.position = self.flight_target;
                    }
                    self.land(update);
                }
            }
            LadybugState::Dispersing => {
                let flutter = (update.time * 2.7 + self.wing_phase).sin() * 0.018;
                move_motion_toward(
                    &mut self.motion,
                    self.flight_target,
                    update,
                    LADYBUG_FLY_MOTION,
                    None,
                    1.86,
                    flutter,
                );
            }
        }
    }

    fn draw(&self, render: &RenderContext) -> Result<(), JsValue> {
        draw_ladybug(self, render)
    }

    fn position(&self) -> Point {
        self.motion.position
    }

    fn should_respawn(&self, bounds: WorldBounds) -> bool {
        let position = self.position();
        self.state == LadybugState::Dispersing && self.state_time > 260.0
            || position.x < -RESPAWN_MARGIN
            || position.x > bounds.width + RESPAWN_MARGIN
            || position.y < -RESPAWN_MARGIN
            || position.y > bounds.height + RESPAWN_MARGIN
    }

    fn aphid_bite_request(&self) -> Option<(Point, f64)> {
        (self.state == LadybugState::Feeding)
            .then_some((self.motion.position, self.motion.size + 7.0))
    }

    fn on_aphid_eaten(&mut self, point: Point) {
        self.motion.position = point;
        self.motion.base_speed = self.walk_speed;
        self.target_aphid = None;
        self.walk_target = point;
        self.state = LadybugState::Walking;
        self.state_time = 0.0;
        self.hunger = (self.hunger - LADYBUG_STARVE_TIME * 0.56).max(0.0);
        self.refresh_flight_delay();
    }

    fn aphid_threat_position(&self) -> Option<Point> {
        Some(self.motion.position)
    }

    fn emerge_at(&mut self, point: Point, _world: &CoverWorld, _viewport: Viewport) {
        self.motion.position = point;
        self.motion.base_speed = self.walk_speed;
        self.walk_target = point;
        self.flight_target = point;
        self.target_aphid = None;
        self.state = LadybugState::Walking;
        self.state_time = 0.0;
        self.hunger = LADYBUG_STARVE_TIME * 0.18;
        self.refresh_flight_delay();
    }

    fn splat_candidate(
        &self,
        point: Point,
        viewport: Viewport,
    ) -> Option<(f64, Box<dyn SceneObject>)> {
        let display_position = Point {
            x: self.motion.position.x,
            y: self.motion.position.y
                - if ladybug_is_airborne(self.state) {
                    self.motion.size * 2.6
                } else {
                    0.0
                },
        };

        circular_splat_candidate(
            point,
            display_position,
            self.motion.size
                * if ladybug_is_airborne(self.state) {
                    1.68
                } else {
                    1.42
                },
            viewport,
            self.motion.heading,
            self.motion.size * 1.1,
            LADYBUG_SPLAT_PALETTE,
            self.shell_phase + self.wing_phase + self.flight_phase,
        )
    }
}

impl Dandelion {
    fn spawn(root: Point, rng: &mut Lcg) -> Self {
        Self {
            root,
            life: rng.range(0.0, 0.08),
            health: 1.0,
            leaf_health: 1.0,
            sap_health: 1.0,
            life_rate: rng.range(0.78, 1.26),
            leaf_span: rng.range(8.0, 14.0),
            max_height: rng.range(18.0, 30.0),
            bloom_radius: rng.range(4.6, 7.6),
            stem_curve: rng.range(-1.0, 1.0),
            sway_phase: rng.range(0.0, TAU),
            leaf_phase: rng.range(0.0, TAU),
            seed_phase: rng.range(0.0, TAU),
            seed_count: 8 + (rng.next_f64() * 6.0).floor() as usize,
            germinated_seed_count: 0,
        }
    }
}

impl StrawberryPlant {
    fn spawn(root: Point, rng: &mut Lcg) -> Self {
        Self {
            root,
            life: rng.range(0.0, 0.1),
            health: 1.0,
            leaf_health: 1.0,
            sap_health: 1.0,
            life_rate: rng.range(0.82, 1.18),
            leaf_span: rng.range(9.0, 14.0),
            flower_count: 2 + (rng.next_f64() * 3.0).floor() as usize,
            berry_count: 1 + (rng.next_f64() * 3.0).floor() as usize,
            runner_count: 1 + (rng.next_f64() * 3.0).floor() as usize,
            propagated_runner_count: 0,
            sway_phase: rng.range(0.0, TAU),
            leaf_phase: rng.range(0.0, TAU),
            bloom_phase: rng.range(0.0, TAU),
            fruit_phase: rng.range(0.0, TAU),
            runner_phase: rng.range(0.0, TAU),
        }
    }
}

impl GrassClump {
    fn spawn(root: Point, rng: &mut Lcg) -> Self {
        Self {
            root,
            life: rng.range(0.08, 0.22),
            health: 1.0,
            leaf_health: 1.0,
            sap_health: 1.0,
            life_rate: rng.range(0.72, 1.18),
            blade_count: 22 + (rng.next_f64() * 22.0).floor() as usize,
            spread: rng.range(30.0, 58.0),
            max_height: rng.range(14.0, 30.0),
            sway_phase: rng.range(0.0, TAU),
            blade_phase: rng.range(0.0, TAU),
            seed: rng.range(0.0, 10_000.0),
        }
    }
}

impl SceneObject for Dandelion {
    fn object_name(&self) -> &'static str {
        "Dandelion"
    }

    fn role(&self) -> SceneRole {
        SceneRole::Flora
    }

    fn update(&mut self, update: &UpdateContext) {
        self.leaf_health = (self.leaf_health + 0.00048 * update.dt).min(1.0);
        self.sap_health = (self.sap_health + 0.00034 * update.dt).min(1.0);
        self.health = combined_plant_health(self.leaf_health, self.sap_health);
        let life_step = 0.00085 + self.life_rate * 0.00095;
        self.life = (self.life + life_step * update.dt * plant_vigor(self.health)).min(1.16);
    }

    fn draw(&self, render: &RenderContext) -> Result<(), JsValue> {
        draw_dandelion(self, render)
    }

    fn position(&self) -> Point {
        self.root
    }

    fn aphid_fallback_host_position(&self) -> Option<PlantTarget> {
        Some(PlantTarget {
            position: self.root,
            quality: dandelion_aphid_host_quality(self),
        })
    }

    fn leaf_beetle_food_position(&self) -> Option<PlantTarget> {
        Some(PlantTarget {
            position: self.root,
            quality: dandelion_leaf_food_quality(self),
        })
    }

    fn should_respawn(&self, _bounds: WorldBounds) -> bool {
        false
    }

    fn should_rehome(&self, _bounds: WorldBounds) -> bool {
        false
    }

    fn should_rehome_in_cover_world(&self, world: &CoverWorld) -> bool {
        self.health <= 0.08 || !world.is_open_ground(self.root, 10.0)
    }

    fn take_flora_spawn_requests(&mut self) -> Vec<FloraSpawnRequest> {
        let released_count = dandelion_released_seed_count(self.life, self.seed_count);
        if released_count <= self.germinated_seed_count {
            return Vec::new();
        }

        let mut requests = Vec::with_capacity(released_count - self.germinated_seed_count);
        for index in self.germinated_seed_count..released_count {
            requests.push(FloraSpawnRequest {
                kind: FloraKind::Dandelion,
                landing: dandelion_seed_landing_point(self, index),
            });
        }
        self.germinated_seed_count = released_count;
        requests
    }

    fn receive_flora_damage(&mut self, kind: FloraDamageKind, damage: f64) {
        match kind {
            FloraDamageKind::SapDrain => {
                self.sap_health = (self.sap_health - damage).max(0.0);
                self.leaf_health = (self.leaf_health - damage * 0.18).max(0.0);
            }
            FloraDamageKind::LeafChew => {
                self.leaf_health = (self.leaf_health - damage).max(0.0);
                self.sap_health = (self.sap_health - damage * 0.12).max(0.0);
            }
        }
        self.health = combined_plant_health(self.leaf_health, self.sap_health);
    }
}

impl SceneObject for StrawberryPlant {
    fn object_name(&self) -> &'static str {
        "StrawberryPlant"
    }

    fn role(&self) -> SceneRole {
        SceneRole::Flora
    }

    fn update(&mut self, update: &UpdateContext) {
        self.leaf_health = (self.leaf_health + 0.00052 * update.dt).min(1.0);
        self.sap_health = (self.sap_health + 0.00038 * update.dt).min(1.0);
        self.health = combined_plant_health(self.leaf_health, self.sap_health);
        let life_step = 0.00062 + self.life_rate * 0.00078;
        self.life = (self.life + life_step * update.dt * plant_vigor(self.health)).min(1.2);
    }

    fn draw(&self, render: &RenderContext) -> Result<(), JsValue> {
        draw_strawberry_plant(self, render)
    }

    fn position(&self) -> Point {
        self.root
    }

    fn aphid_host_position(&self) -> Option<PlantTarget> {
        Some(PlantTarget {
            position: self.root,
            quality: strawberry_aphid_host_quality(self),
        })
    }

    fn leaf_beetle_fallback_food_position(&self) -> Option<PlantTarget> {
        Some(PlantTarget {
            position: self.root,
            quality: strawberry_leaf_food_quality(self),
        })
    }

    fn should_respawn(&self, _bounds: WorldBounds) -> bool {
        false
    }

    fn should_rehome(&self, _bounds: WorldBounds) -> bool {
        false
    }

    fn should_rehome_in_cover_world(&self, world: &CoverWorld) -> bool {
        self.health <= 0.08 || !world.is_open_ground(self.root, 11.0)
    }

    fn take_flora_spawn_requests(&mut self) -> Vec<FloraSpawnRequest> {
        let released_count = strawberry_released_runner_count(self.life, self.runner_count);
        if released_count <= self.propagated_runner_count {
            return Vec::new();
        }

        let mut requests = Vec::with_capacity(released_count - self.propagated_runner_count);
        for index in self.propagated_runner_count..released_count {
            requests.push(FloraSpawnRequest {
                kind: FloraKind::Strawberry,
                landing: strawberry_runner_landing_point(self, index),
            });
        }
        self.propagated_runner_count = released_count;
        requests
    }

    fn receive_flora_damage(&mut self, kind: FloraDamageKind, damage: f64) {
        match kind {
            FloraDamageKind::SapDrain => {
                self.sap_health = (self.sap_health - damage * 1.18).max(0.0);
                self.leaf_health = (self.leaf_health - damage * 0.12).max(0.0);
            }
            FloraDamageKind::LeafChew => {
                self.leaf_health = (self.leaf_health - damage * 0.86).max(0.0);
                self.sap_health = (self.sap_health - damage * 0.08).max(0.0);
            }
        }
        self.health = combined_plant_health(self.leaf_health, self.sap_health);
    }
}

impl SceneObject for GrassClump {
    fn object_name(&self) -> &'static str {
        "GrassClump"
    }

    fn role(&self) -> SceneRole {
        SceneRole::Flora
    }

    fn update(&mut self, update: &UpdateContext) {
        self.leaf_health = (self.leaf_health + 0.00062 * update.dt).min(1.0);
        self.sap_health = (self.sap_health + 0.00024 * update.dt).min(1.0);
        self.health = combined_plant_health(self.leaf_health, self.sap_health);
        let life_step = 0.00044 + self.life_rate * 0.00058;
        self.life = (self.life + life_step * update.dt * plant_vigor(self.health)).min(1.08);
    }

    fn draw(&self, render: &RenderContext) -> Result<(), JsValue> {
        draw_grass_clump(self, render)
    }

    fn position(&self) -> Point {
        self.root
    }

    fn aphid_fallback_host_position(&self) -> Option<PlantTarget> {
        Some(PlantTarget {
            position: self.root,
            quality: grass_aphid_host_quality(self),
        })
    }

    fn leaf_beetle_fallback_food_position(&self) -> Option<PlantTarget> {
        Some(PlantTarget {
            position: self.root,
            quality: grass_leaf_food_quality(self),
        })
    }

    fn should_respawn(&self, _bounds: WorldBounds) -> bool {
        false
    }

    fn should_rehome(&self, _bounds: WorldBounds) -> bool {
        false
    }

    fn should_rehome_in_cover_world(&self, world: &CoverWorld) -> bool {
        self.health <= 0.08 || !world.is_open_ground(self.root, 8.0)
    }

    fn receive_flora_damage(&mut self, kind: FloraDamageKind, damage: f64) {
        match kind {
            FloraDamageKind::SapDrain => {
                self.sap_health = (self.sap_health - damage * 0.78).max(0.0);
                self.leaf_health = (self.leaf_health - damage * 0.2).max(0.0);
            }
            FloraDamageKind::LeafChew => {
                self.leaf_health = (self.leaf_health - damage * 0.92).max(0.0);
                self.sap_health = (self.sap_health - damage * 0.06).max(0.0);
            }
        }
        self.health = combined_plant_health(self.leaf_health, self.sap_health);
    }
}

fn sample_point_in_rect(rect: Rect, inset: f64, rng: &mut Lcg) -> Point {
    let min_x = rect.x + inset.min(rect.width * 0.35);
    let max_x = rect.x + rect.width - inset.min(rect.width * 0.35);
    let min_y = rect.y + inset.min(rect.height * 0.35);
    let max_y = rect.y + rect.height - inset.min(rect.height * 0.35);

    if max_x <= min_x || max_y <= min_y {
        return Point {
            x: rect.x + rect.width * 0.5,
            y: rect.y + rect.height * 0.5,
        };
    }

    Point {
        x: rng.range(min_x, max_x),
        y: rng.range(min_y, max_y),
    }
}

fn random_plant_target(rng: &mut Lcg, targets: &[PlantTarget]) -> Option<Point> {
    let total_quality: f64 = targets
        .iter()
        .map(|target| plant_target_quality(target.quality))
        .sum();
    if total_quality <= 0.0 {
        return None;
    }

    let mut pick = rng.range(0.0, total_quality);
    for target in targets {
        pick -= plant_target_quality(target.quality);
        if pick <= 0.0 {
            return Some(target.position);
        }
    }

    targets.last().map(|target| target.position)
}

fn random_preferred_target(
    rng: &mut Lcg,
    preferred: &[PlantTarget],
    fallback: &[PlantTarget],
    preferred_chance: f64,
) -> Option<Point> {
    if preferred.is_empty() {
        random_plant_target(rng, fallback)
    } else if fallback.is_empty() || rng.bool(preferred_chance) {
        random_plant_target(rng, preferred)
    } else {
        random_plant_target(rng, fallback)
    }
}

fn aphid_host_offset(rng: &mut Lcg) -> Point {
    let angle = rng.range(0.0, TAU);
    let distance = rng.range(3.0, 12.0);

    Point {
        x: angle.cos() * distance,
        y: angle.sin() * distance * 0.64 - rng.range(2.0, 9.0),
    }
}

fn sample_open_point_in_rect(
    area: Rect,
    bounds: WorldBounds,
    cover_regions: &[CoverRegion],
    rng: &mut Lcg,
    clearance: f64,
    attempts: usize,
) -> Option<Point> {
    let min_x = area.x.max(clearance);
    let max_x = (area.x + area.width).min(bounds.width - clearance);
    let min_y = area.y.max(clearance);
    let max_y = (area.y + area.height).min(bounds.height - clearance);

    if max_x <= min_x || max_y <= min_y {
        return None;
    }

    for _ in 0..attempts {
        let point = Point {
            x: rng.range(min_x, max_x),
            y: rng.range(min_y, max_y),
        };

        if is_open_ground_in_regions(point, bounds, cover_regions, clearance) {
            return Some(point);
        }
    }

    None
}

fn is_open_ground_in_regions(
    point: Point,
    bounds: WorldBounds,
    cover_regions: &[CoverRegion],
    clearance: f64,
) -> bool {
    if point.x < clearance
        || point.x > bounds.width - clearance
        || point.y < clearance
        || point.y > bounds.height - clearance
    {
        return false;
    }

    !cover_regions.iter().any(|region| {
        region.kind.blocks_ground() && region.rect.expanded(clearance).contains(point)
    })
}

fn nearest_shelter_rect(origin: Point, cover_regions: &[CoverRegion], inset: f64) -> Option<Rect> {
    let mut nearest = None;
    let mut nearest_distance = f64::INFINITY;

    for region in cover_regions {
        if region.kind != CoverRegionKind::Shelter
            || region.rect.width <= inset * 1.5
            || region.rect.height <= inset * 1.5
        {
            continue;
        }

        let distance = distance_to_rect(origin, region.rect);
        if distance < nearest_distance {
            nearest = Some(region.rect);
            nearest_distance = distance;
        }
    }

    nearest
}

fn shelter_rest_home(
    origin: Point,
    cover_regions: &[CoverRegion],
    inset: f64,
) -> (Option<Rect>, Point) {
    if let Some(rect) = nearest_shelter_rect(origin, cover_regions, inset) {
        let (anchor, _) = shelter_anchor_from_hint(rect, origin, inset, 10.0);
        (Some(rect), anchor)
    } else {
        (None, origin)
    }
}

fn shelter_anchor_from_hint(
    rect: Rect,
    hint: Point,
    inset: f64,
    peek_clearance: f64,
) -> (Point, Point) {
    let inset_x = inset.min(rect.width * 0.35);
    let inset_y = inset.min(rect.height * 0.35);
    let min_x = rect.x + inset_x;
    let max_x = rect.x + rect.width - inset_x;
    let min_y = rect.y + inset_y;
    let max_y = rect.y + rect.height - inset_y;

    let hide_anchor = Point {
        x: if max_x <= min_x {
            rect.x + rect.width * 0.5
        } else {
            hint.x.clamp(min_x, max_x)
        },
        y: if max_y <= min_y {
            rect.y + rect.height * 0.5
        } else {
            hint.y.clamp(min_y, max_y)
        },
    };

    let left_distance = (hide_anchor.x - rect.x).abs();
    let right_distance = (rect.x + rect.width - hide_anchor.x).abs();
    let top_distance = (hide_anchor.y - rect.y).abs();
    let bottom_distance = (rect.y + rect.height - hide_anchor.y).abs();

    let peek_point = if left_distance <= right_distance
        && left_distance <= top_distance
        && left_distance <= bottom_distance
    {
        Point {
            x: rect.x - peek_clearance,
            y: hide_anchor.y,
        }
    } else if right_distance <= top_distance && right_distance <= bottom_distance {
        Point {
            x: rect.x + rect.width + peek_clearance,
            y: hide_anchor.y,
        }
    } else if top_distance <= bottom_distance {
        Point {
            x: hide_anchor.x,
            y: rect.y - peek_clearance,
        }
    } else {
        Point {
            x: hide_anchor.x,
            y: rect.y + rect.height + peek_clearance,
        }
    };

    (hide_anchor, peek_point)
}

fn distance_to_rect(point: Point, rect: Rect) -> f64 {
    let nearest_x = point.x.clamp(rect.x, rect.x + rect.width);
    let nearest_y = point.y.clamp(rect.y, rect.y + rect.height);
    point.distance_to(Point {
        x: nearest_x,
        y: nearest_y,
    })
}

fn flee_point_from(
    origin: Point,
    threat: Point,
    update: &UpdateContext,
    distance: f64,
    clearance: f64,
) -> Point {
    let base_angle = threat.angle_to(origin);

    for step in 0..12 {
        let turn = ((step + 1) / 2) as f64 * 0.34;
        let direction = if step % 2 == 0 { 1.0 } else { -1.0 };
        let angle = base_angle + turn * direction;
        let travel = distance * (1.0 - step as f64 * 0.035).max(0.58);
        let candidate = Point {
            x: origin.x + angle.cos() * travel,
            y: origin.y + angle.sin() * travel,
        };

        if is_open_ground_in_regions(candidate, update.bounds, update.cover_regions, clearance) {
            return candidate;
        }
    }

    origin
}

fn fauna_emergence_clearance(kind: FaunaKind) -> f64 {
    match kind {
        FaunaKind::Aphid => 6.0,
        FaunaKind::LeafBeetle => 9.0,
        FaunaKind::GroundBeetle => 12.0,
        FaunaKind::Centipede => 16.0,
        FaunaKind::Ladybug => 10.0,
    }
}

fn ground_beetle_food_available(update: &UpdateContext) -> bool {
    !update.leaf_beetle_targets.is_empty() || !update.aphid_targets.is_empty()
}

fn centipede_food_available(update: &UpdateContext) -> bool {
    !update.ground_beetle_targets.is_empty() || !update.leaf_beetle_targets.is_empty()
}

fn ground_beetle_prey_target(
    origin: Point,
    update: &UpdateContext,
    leaf_max_distance: f64,
    aphid_max_distance: f64,
) -> Option<(Point, SmallPredatorPrey)> {
    nearest_visible_target(
        origin,
        update.leaf_beetle_targets,
        leaf_max_distance,
        update.cover_regions,
        8.0,
    )
    .map(|target| (target, SmallPredatorPrey::LeafBeetle))
    .or_else(|| {
        nearest_visible_target(
            origin,
            update.aphid_targets,
            aphid_max_distance,
            update.cover_regions,
            6.0,
        )
        .map(|target| (target, SmallPredatorPrey::Aphid))
    })
}

fn centipede_prey_target(
    origin: Point,
    update: &UpdateContext,
    ground_max_distance: f64,
    leaf_max_distance: f64,
) -> Option<(Point, CentipedePrey)> {
    nearest_visible_target(
        origin,
        update.ground_beetle_targets,
        ground_max_distance,
        update.cover_regions,
        10.0,
    )
    .map(|target| (target, CentipedePrey::GroundBeetle))
    .or_else(|| {
        nearest_visible_target(
            origin,
            update.leaf_beetle_targets,
            leaf_max_distance,
            update.cover_regions,
            8.0,
        )
        .map(|target| (target, CentipedePrey::LeafBeetle))
    })
}

fn dispersal_target_from(origin: Point, bounds: WorldBounds, phase: f64, margin: f64) -> Point {
    let distances = [
        origin.x.max(0.0),
        (bounds.width - origin.x).max(0.0),
        origin.y.max(0.0),
        (bounds.height - origin.y).max(0.0),
    ];
    let mut nearest_edge = 0;
    let mut nearest_distance = distances[0];
    for (index, distance) in distances.iter().enumerate().skip(1) {
        if *distance < nearest_distance {
            nearest_edge = index;
            nearest_distance = *distance;
        }
    }

    let drift = phase.sin() * 90.0;
    match nearest_edge {
        0 => Point {
            x: -margin,
            y: (origin.y + drift).clamp(-margin, bounds.height + margin),
        },
        1 => Point {
            x: bounds.width + margin,
            y: (origin.y + drift).clamp(-margin, bounds.height + margin),
        },
        2 => Point {
            x: (origin.x + drift).clamp(-margin, bounds.width + margin),
            y: -margin,
        },
        _ => Point {
            x: (origin.x + drift).clamp(-margin, bounds.width + margin),
            y: bounds.height + margin,
        },
    }
}

fn ladybug_is_airborne(state: LadybugState) -> bool {
    matches!(state, LadybugState::Flying | LadybugState::Dispersing)
}

fn fallback_leaf_beetle_home(rng: &mut Lcg, bounds: WorldBounds) -> (Rect, Point, Point) {
    let safe_width = bounds.width.max(1.0);
    let safe_height = bounds.height.max(1.0);
    let horizontal_width = (safe_width * 0.22)
        .clamp(72.0_f64, 180.0_f64)
        .min(safe_width);
    let horizontal_height = 34.0_f64.min(safe_height);
    let vertical_width = 30.0_f64.min(safe_width);
    let vertical_height = (safe_height * 0.24)
        .clamp(72.0_f64, 156.0_f64)
        .min(safe_height);
    let edge = (rng.next_f64() * 4.0).floor() as i32;

    let rect = match edge {
        0 => Rect {
            x: rng.range(0.0, (safe_width - horizontal_width).max(0.0)),
            y: 0.0,
            width: horizontal_width,
            height: horizontal_height,
        },
        1 => Rect {
            x: rng.range(0.0, (safe_width - horizontal_width).max(0.0)),
            y: (safe_height - horizontal_height).max(0.0),
            width: horizontal_width,
            height: horizontal_height,
        },
        2 => Rect {
            x: 0.0,
            y: rng.range(0.0, (safe_height - vertical_height).max(0.0)),
            width: vertical_width,
            height: vertical_height,
        },
        _ => Rect {
            x: (safe_width - vertical_width).max(0.0),
            y: rng.range(0.0, (safe_height - vertical_height).max(0.0)),
            width: vertical_width,
            height: vertical_height,
        },
    };

    let hide_anchor = sample_point_in_rect(rect, 8.0, rng);
    let peek_point = match edge {
        0 => Point {
            x: hide_anchor.x,
            y: (rect.y + rect.height + 14.0).clamp(0.0, safe_height),
        },
        1 => Point {
            x: hide_anchor.x,
            y: (rect.y - 14.0).clamp(0.0, safe_height),
        },
        2 => Point {
            x: (rect.x + rect.width + 14.0).clamp(0.0, safe_width),
            y: hide_anchor.y,
        },
        _ => Point {
            x: (rect.x - 14.0).clamp(0.0, safe_width),
            y: hide_anchor.y,
        },
    };

    (rect, hide_anchor, peek_point)
}

fn fallback_ground_beetle_position(rng: &mut Lcg, bounds: WorldBounds, edge_only: bool) -> Point {
    let safe_width = bounds.width.max(54.0);
    let safe_height = bounds.height.max(54.0);
    if edge_only {
        let edge = (rng.next_f64() * 4.0).floor() as i32;
        let margin = 26.0;
        match edge {
            0 => Point {
                x: rng.range(margin, (safe_width - margin).max(margin)),
                y: margin,
            },
            1 => Point {
                x: rng.range(margin, (safe_width - margin).max(margin)),
                y: safe_height - margin,
            },
            2 => Point {
                x: margin,
                y: rng.range(margin, (safe_height - margin).max(margin)),
            },
            _ => Point {
                x: safe_width - margin,
                y: rng.range(margin, (safe_height - margin).max(margin)),
            },
        }
    } else {
        Point {
            x: rng.range(14.0, (safe_width - 14.0).max(14.0)),
            y: rng.range(14.0, (safe_height - 14.0).max(14.0)),
        }
    }
}

fn ground_beetle_patrol_target(beetle: &GroundBeetle, update: &UpdateContext) -> Point {
    let origin = beetle.motion.position;
    let base_angle = beetle.shell_phase * 0.7
        + beetle.antenna_phase * 0.4
        + update.time * 0.08
        + beetle.patrol_cursor as f64 * 0.76;

    for step in 0..16 {
        let angle = base_angle + step as f64 * 0.61;
        let distance = 54.0 + ((beetle.patrol_cursor + step as u32) % 6) as f64 * 24.0;
        let candidate = Point {
            x: origin.x + angle.cos() * distance,
            y: origin.y + angle.sin() * (distance * 0.72),
        };

        if is_open_ground_in_regions(candidate, update.bounds, update.cover_regions, 12.0) {
            return candidate;
        }
    }

    origin
}

fn centipede_patrol_target(centipede: &Centipede, update: &UpdateContext) -> Point {
    let origin = centipede.motion.position;
    let base_angle = centipede.body_phase * 0.62
        + centipede.antenna_phase * 0.34
        + update.time * 0.11
        + centipede.patrol_cursor as f64 * 0.54;

    for step in 0..18 {
        let angle = base_angle + step as f64 * 0.49;
        let distance = 72.0 + ((centipede.patrol_cursor + step as u32) % 7) as f64 * 28.0;
        let candidate = Point {
            x: origin.x + angle.cos() * distance,
            y: origin.y + angle.sin() * (distance * 0.7),
        };

        if is_open_ground_in_regions(candidate, update.bounds, update.cover_regions, 16.0) {
            return candidate;
        }
    }

    origin
}

fn ladybug_ground_target(ladybug: &Ladybug, update: &UpdateContext) -> Point {
    let origin = ladybug.motion.position;
    let anchor = nearest_visible_target(
        origin,
        update.aphid_targets,
        180.0,
        update.cover_regions,
        5.0,
    )
    .or_else(|| {
        nearest_visible_target(
            origin,
            update.flora_targets,
            220.0,
            update.cover_regions,
            5.0,
        )
    })
    .unwrap_or(ladybug.walk_target);
    let base_angle = ladybug.shell_phase * 0.68
        + ladybug.flight_phase * 0.24
        + update.time * 0.12
        + ladybug.patrol_cursor as f64 * 0.74;

    for step in 0..16 {
        let angle = base_angle + step as f64 * 0.57;
        let distance = 10.0 + ((ladybug.patrol_cursor + step as u32) % 5) as f64 * 8.0;
        let candidate = Point {
            x: anchor.x + angle.cos() * distance,
            y: anchor.y + angle.sin() * (distance * 0.72),
        };

        if is_open_ground_in_regions(candidate, update.bounds, update.cover_regions, 8.0) {
            return candidate;
        }
    }

    if is_open_ground_in_regions(anchor, update.bounds, update.cover_regions, 8.0) {
        anchor
    } else {
        origin
    }
}

fn ladybug_flight_target(ladybug: &Ladybug, update: &UpdateContext) -> Point {
    let origin = ladybug.motion.position;
    let mut preferred_anchor = None;
    let mut preferred_score = f64::INFINITY;

    for aphid in update.aphid_targets {
        let distance = origin.distance_to(*aphid);
        if !(64.0..320.0).contains(&distance) {
            continue;
        }
        if !visible_target_between(origin, *aphid, update.cover_regions, 5.0) {
            continue;
        }

        let score = (distance - 144.0).abs() * 0.72;
        if score < preferred_score {
            preferred_score = score;
            preferred_anchor = Some(*aphid);
        }
    }

    for flora in update.flora_targets {
        let distance = origin.distance_to(*flora);
        if !(72.0..360.0).contains(&distance) {
            continue;
        }
        if !visible_target_between(origin, *flora, update.cover_regions, 5.0) {
            continue;
        }

        let score = (distance - 180.0).abs();
        if score < preferred_score {
            preferred_score = score;
            preferred_anchor = Some(*flora);
        }
    }

    if let Some(anchor) = preferred_anchor {
        let base_angle = ladybug.wing_phase * 0.54
            + ladybug.flight_phase * 0.7
            + update.time * 0.08
            + ladybug.patrol_cursor as f64 * 0.63;

        for step in 0..14 {
            let angle = base_angle + step as f64 * 0.48;
            let distance = 12.0 + ((ladybug.patrol_cursor + step as u32) % 4) as f64 * 10.0;
            let candidate = Point {
                x: anchor.x + angle.cos() * distance,
                y: anchor.y + angle.sin() * (distance * 0.68),
            };

            if origin.distance_to(candidate) > 56.0
                && is_open_ground_in_regions(candidate, update.bounds, update.cover_regions, 8.0)
            {
                return candidate;
            }
        }
    }

    let base_angle = ladybug.shell_phase * 0.52
        + ladybug.flight_phase * 0.76
        + update.time * 0.1
        + ladybug.patrol_cursor as f64 * 0.69;
    for step in 0..18 {
        let angle = base_angle + step as f64 * 0.52;
        let distance = 86.0 + ((ladybug.patrol_cursor + step as u32) % 7) as f64 * 22.0;
        let candidate = Point {
            x: origin.x + angle.cos() * distance,
            y: origin.y + angle.sin() * (distance * 0.62),
        };

        if is_open_ground_in_regions(candidate, update.bounds, update.cover_regions, 8.0) {
            return candidate;
        }
    }

    ladybug_ground_target(ladybug, update)
}

fn centipede_segment_count() -> usize {
    13
}

fn centipede_segment_spacing(size: f64) -> f64 {
    size * 0.72
}

fn centipede_trail_sample_step(size: f64) -> f64 {
    (size * 0.26).clamp(1.4, 2.6)
}

fn centipede_trail_length(size: f64) -> f64 {
    centipede_segment_spacing(size) * (centipede_segment_count().saturating_sub(1) as f64 + 1.9)
}

fn lerp_point(start: Point, end: Point, t: f64) -> Point {
    Point {
        x: start.x + (end.x - start.x) * t,
        y: start.y + (end.y - start.y) * t,
    }
}

fn seed_centipede_trail(head: Point, heading: f64, size: f64) -> Vec<Point> {
    let length = centipede_trail_length(size);
    let step = centipede_trail_sample_step(size);
    let sample_count = (length / step).ceil() as usize;
    let mut trail = Vec::with_capacity(sample_count + 1);
    let dx = -heading.cos();
    let dy = -heading.sin();

    for index in 0..=sample_count {
        let distance = (length - index as f64 * step).max(0.0);
        trail.push(Point {
            x: head.x + dx * distance,
            y: head.y + dy * distance,
        });
    }

    trail
}

fn extend_centipede_trail(
    trail: &mut Vec<Point>,
    previous_position: Point,
    current_position: Point,
    heading: f64,
    size: f64,
) {
    if trail.is_empty() {
        *trail = seed_centipede_trail(current_position, heading, size);
        return;
    }

    let step = centipede_trail_sample_step(size);
    let mut anchor = *trail.last().unwrap_or(&previous_position);
    if anchor.distance_to(previous_position) > step * 1.5 {
        trail.push(previous_position);
        anchor = previous_position;
    }

    let distance = anchor.distance_to(current_position);
    if distance <= step {
        if let Some(last) = trail.last_mut() {
            *last = current_position;
        }
    } else {
        let step_count = (distance / step).floor() as usize;
        for index in 1..=step_count {
            let t = (index as f64 * step / distance).min(1.0);
            trail.push(lerp_point(anchor, current_position, t));
        }

        if trail
            .last()
            .is_none_or(|point| point.distance_to(current_position) > 0.05)
        {
            trail.push(current_position);
        }
    }

    trim_centipede_trail(trail, centipede_trail_length(size));
}

fn trim_centipede_trail(trail: &mut Vec<Point>, max_length: f64) {
    if trail.len() < 2 {
        return;
    }

    let mut length = 0.0;
    let mut keep_from = 0;
    for index in (1..trail.len()).rev() {
        length += trail[index].distance_to(trail[index - 1]);
        if length > max_length {
            keep_from = index - 1;
            break;
        }
    }

    if keep_from > 0 {
        trail.drain(0..keep_from);
    }
}

fn sample_centipede_trail_point(trail: &[Point], distance_from_head: f64) -> Point {
    if trail.is_empty() {
        return Point { x: 0.0, y: 0.0 };
    }

    let mut remaining = distance_from_head.max(0.0);
    for index in (1..trail.len()).rev() {
        let start = trail[index - 1];
        let end = trail[index];
        let segment_length = start.distance_to(end);
        if segment_length <= 0.001 {
            continue;
        }

        if remaining <= segment_length {
            let t = 1.0 - remaining / segment_length;
            return lerp_point(start, end, t);
        }

        remaining -= segment_length;
    }

    trail[0]
}

fn sample_centipede_trail_heading(trail: &[Point], distance_from_head: f64, fallback: f64) -> f64 {
    let sample_span = 3.2;
    let ahead = sample_centipede_trail_point(trail, (distance_from_head - sample_span).max(0.0));
    let behind = sample_centipede_trail_point(trail, distance_from_head + sample_span);

    if ahead.distance_to(behind) <= 0.001 {
        fallback
    } else {
        behind.angle_to(ahead)
    }
}

fn centipede_splat_candidate(
    centipede: &Centipede,
    point: Point,
    viewport: Viewport,
) -> Option<(f64, Box<dyn SceneObject>)> {
    if !point_visible_in_viewport(
        centipede.motion.position,
        viewport,
        DRAW_MARGIN + centipede_trail_length(centipede.motion.size),
    ) {
        return None;
    }

    let segment_count = centipede_segment_count();
    let segment_spacing = centipede_segment_spacing(centipede.motion.size);
    let mut nearest_hit: Option<(f64, Point, f64)> = None;

    for segment_index in 0..segment_count {
        let distance_from_head = segment_index as f64 * segment_spacing;
        let segment_point = sample_centipede_trail_point(&centipede.trail, distance_from_head);
        let hit_distance = segment_point.distance_to(point);
        let segment_radius = (centipede.motion.size * if segment_index == 0 { 0.92 } else { 0.74 })
            .max(MIN_SPLAT_HIT_RADIUS);

        if hit_distance > segment_radius {
            continue;
        }

        if nearest_hit
            .as_ref()
            .is_none_or(|(best_distance, _, _)| hit_distance < *best_distance)
        {
            let heading = sample_centipede_trail_heading(
                &centipede.trail,
                distance_from_head,
                centipede.motion.heading,
            );
            nearest_hit = Some((hit_distance, segment_point, heading));
        }
    }

    nearest_hit.map(|(distance, segment_point, heading)| {
        (
            distance,
            Box::new(InsectSplat::new(
                segment_point,
                heading,
                centipede.motion.size * 1.2,
                CENTIPEDE_SPLAT_PALETTE,
                centipede.body_phase + centipede.antenna_phase,
            )) as Box<dyn SceneObject>,
        )
    })
}

fn move_motion_toward(
    motion: &mut MotionState,
    target: Point,
    update: &UpdateContext,
    profile: MotionProfile,
    ignored_shelter: Option<Rect>,
    drive: f64,
    extra_wander: f64,
) -> f64 {
    let distance = motion.position.distance_to(target).max(0.001);
    let target_pull = (distance / 18.0).min(1.0) * drive;
    let target_vector_x = (target.x - motion.position.x) / distance;
    let target_vector_y = (target.y - motion.position.y) / distance;
    let (avoid_x, avoid_y) = avoidance_vector_with_ignored_shelter(
        motion.position,
        motion.size,
        update.bounds,
        update.cover_regions,
        profile,
        ignored_shelter,
    );
    let target_focus = target_pull.clamp(0.0, 1.0);
    let free_roam = 1.0 - target_focus * 0.78;
    let walk_clock = update.time * (0.32 + motion.base_speed * 0.22);
    let walk_seed = motion.wander_phase + motion.stride_phase * 0.37 + motion.size * 5.3;
    let walk_bias = smooth_random_walk(walk_seed, walk_clock);
    let turn_bias = smooth_random_walk(
        walk_seed + 31.7,
        walk_clock * 0.63 + motion.position.x * 0.002,
    );
    let drift_bias = smooth_random_walk(
        walk_seed + 73.1,
        walk_clock * 0.41 + motion.position.y * 0.002,
    );
    let wander_heading =
        motion.heading + walk_bias * 1.22 + turn_bias * 0.58 + drift_bias * 0.34 + extra_wander;
    let wander_strength = profile.wander_weight * (1.35 + free_roam * 2.25);
    let forward_strength = 0.1 + free_roam * 0.08;

    let steer_x = target_vector_x * target_pull
        + avoid_x
        + motion.heading.cos() * forward_strength
        + wander_heading.cos() * wander_strength;
    let steer_y = target_vector_y * target_pull
        + avoid_y
        + motion.heading.sin() * forward_strength
        + wander_heading.sin() * wander_strength;
    let desired_heading = steer_y.atan2(steer_x);
    let delta = shortest_angle(motion.heading, desired_heading);
    motion.heading += delta * (profile.turn_rate + drive * 0.04) * update.dt;

    let gait = (update.time * profile.gait_frequency + motion.stride_phase)
        .sin()
        .abs();
    let speed = motion.base_speed
        * (0.74
            + gait * profile.gait_influence
            + target_pull * 0.42
            + avoid_x.abs() * profile.avoidance_speed_gain
            + avoid_y.abs() * profile.avoidance_speed_gain);
    let step = speed * update.dt * profile.step_scale;
    if distance <= step {
        motion.position = target;
    } else {
        motion.position.x += motion.heading.cos() * step;
        motion.position.y += motion.heading.sin() * step;
    }

    distance
}

fn avoidance_vector_with_ignored_shelter(
    position: Point,
    size: f64,
    bounds: WorldBounds,
    cover_regions: &[CoverRegion],
    profile: MotionProfile,
    ignored_shelter: Option<Rect>,
) -> (f64, f64) {
    let mut avoid_x = 0.0;
    let mut avoid_y = 0.0;

    for region in cover_regions {
        if ignored_shelter
            .is_some_and(|home| region.kind == CoverRegionKind::Shelter && region.rect == home)
        {
            continue;
        }

        let Some((padding, push_strength)) = region.kind.movement_profile(profile) else {
            continue;
        };

        let obstacle = region.rect;
        let expanded = Rect {
            x: obstacle.x - padding,
            y: obstacle.y - padding,
            width: obstacle.width + padding * 2.0,
            height: obstacle.height + padding * 2.0,
        };

        if position.x < expanded.x
            || position.x > expanded.x + expanded.width
            || position.y < expanded.y
            || position.y > expanded.y + expanded.height
        {
            continue;
        }

        let nearest_x = position.x.clamp(expanded.x, expanded.x + expanded.width);
        let nearest_y = position.y.clamp(expanded.y, expanded.y + expanded.height);
        let mut dx = position.x - nearest_x;
        let mut dy = position.y - nearest_y;

        if dx.abs() < 0.001 && dy.abs() < 0.001 {
            dx = position.x - (expanded.x + expanded.width * 0.5);
            dy = position.y - (expanded.y + expanded.height * 0.5);
        }

        let distance = (dx * dx + dy * dy).sqrt().max(0.001);
        let influence = (34.0 + size * 2.0).max(distance);
        let strength = 1.0 - (distance / influence).min(1.0);

        avoid_x += dx / distance * strength * push_strength;
        avoid_y += dy / distance * strength * push_strength;
    }

    if position.x < profile.edge_margin {
        avoid_x += (profile.edge_margin - position.x) / profile.edge_margin;
    }
    if position.x > bounds.width - profile.edge_margin {
        avoid_x -= (position.x - (bounds.width - profile.edge_margin)) / profile.edge_margin;
    }
    if position.y < profile.edge_margin {
        avoid_y += (profile.edge_margin - position.y) / profile.edge_margin;
    }
    if position.y > bounds.height - profile.edge_margin {
        avoid_y -= (position.y - (bounds.height - profile.edge_margin)) / profile.edge_margin;
    }

    (avoid_x, avoid_y)
}

#[allow(dead_code)]
fn draw_maturing_fauna(fauna: &MaturingFauna, render: &RenderContext) -> Result<(), JsValue> {
    let screen_x = fauna.position.x - render.viewport.x;
    let screen_y = fauna.position.y - render.viewport.y;

    if screen_x < -DRAW_MARGIN
        || screen_x > render.viewport.width + DRAW_MARGIN
        || screen_y < -DRAW_MARGIN
        || screen_y > render.viewport.height + DRAW_MARGIN
    {
        return Ok(());
    }

    let progress = (fauna.age / fauna.emerge_after).clamp(0.0, 1.0);
    let pulse = (render.time * 2.2 + fauna.phase).sin() * 0.08 + 1.0;
    let size = fauna.size * (0.62 + progress * 0.54) * pulse;
    let alpha = 0.22 + progress * 0.52;
    let (fill, edge, segment_count) = match fauna.kind {
        FaunaKind::Aphid => ("rgba(126, 176, 75, 0.62)", "rgba(78, 118, 42, 0.42)", 2),
        FaunaKind::LeafBeetle => ("rgba(178, 116, 48, 0.56)", "rgba(80, 54, 34, 0.38)", 3),
        FaunaKind::GroundBeetle => ("rgba(70, 82, 64, 0.54)", "rgba(20, 28, 24, 0.42)", 4),
        FaunaKind::Centipede => ("rgba(156, 92, 42, 0.52)", "rgba(74, 38, 22, 0.42)", 7),
        FaunaKind::Ladybug => ("rgba(106, 54, 38, 0.56)", "rgba(38, 26, 24, 0.42)", 4),
    };

    let context = render.context;
    context.save();
    context.translate(screen_x, screen_y)?;
    context.rotate((fauna.phase + progress * 0.45).sin() * 0.22)?;
    context.set_global_alpha(alpha);
    context.set_fill_style(&JsValue::from_str(fill));
    context.set_stroke_style(&JsValue::from_str(edge));
    context.set_line_width(0.7);

    if segment_count <= 3 {
        context.begin_path();
        context.ellipse(0.0, 0.0, size * 1.28, size * 0.72, 0.0, 0.0, TAU)?;
        context.fill();
        context.stroke();
    } else {
        for index in 0..segment_count {
            let t = index as f64 / (segment_count - 1) as f64;
            let x = (t - 0.5) * size * 3.6;
            let y = (render.time * 1.3 + fauna.phase + t * TAU).sin() * size * 0.16;
            context.begin_path();
            context.ellipse(x, y, size * 0.54, size * 0.42, 0.0, 0.0, TAU)?;
            context.fill();
            context.stroke();
        }
    }

    context.restore();
    Ok(())
}

fn draw_insect_splat(splat: &InsectSplat, render: &RenderContext) -> Result<(), JsValue> {
    let screen_x = splat.position.x - render.viewport.x;
    let screen_y = splat.position.y - render.viewport.y;
    let size = splat.size;
    let margin = DRAW_MARGIN + size * 4.2;

    if screen_x < -margin
        || screen_x > render.viewport.width + margin
        || screen_y < -margin
        || screen_y > render.viewport.height + margin
    {
        return Ok(());
    }

    let context = render.context;
    let progress = (splat.age / splat.life_span).clamp(0.0, 1.0);
    let alpha = (1.0 - progress).powf(1.45);
    let spread = splat.spread * (0.82 + smoothstep(0.0, 0.68, progress) * 0.52);
    let fill = splat.palette.fill(render.dark_mode);
    let edge = splat.palette.edge(render.dark_mode);
    let speck = splat.palette.speck(render.dark_mode);

    context.save();
    context.translate(screen_x, screen_y)?;
    context.rotate(splat.heading * 0.24 + splat.seed.sin() * 0.16)?;
    context.set_global_alpha(alpha);
    context.set_line_cap("round");
    context.set_line_join("round");
    context.set_shadow_blur(5.0 * (1.0 - progress));
    context.set_shadow_color(fill);
    context.set_fill_style(&JsValue::from_str(fill));

    context.begin_path();
    context.ellipse(0.0, 0.0, spread * 0.72, spread * 0.44, 0.12, 0.0, TAU)?;
    context.fill();

    for index in 0..5 {
        let phase = splat.seed * 0.6 + index as f64 * 1.14;
        let distance = spread * (0.32 + (index as f64 * 0.09));
        let blob_x = phase.cos() * distance;
        let blob_y = phase.sin() * distance * 0.72;
        let blob_rx = size * (0.26 + (phase.sin().abs() * 0.22));
        let blob_ry = size * (0.18 + (phase.cos().abs() * 0.16));

        context.begin_path();
        context.ellipse(blob_x, blob_y, blob_rx, blob_ry, phase * 0.36, 0.0, TAU)?;
        context.fill();
    }

    context.set_shadow_blur(0.0);
    context.set_fill_style(&JsValue::from_str(speck));
    for index in 0..3 {
        let phase = splat.seed * 0.9 + index as f64 * 1.66;
        let distance = spread * (0.18 + index as f64 * 0.12);
        context.begin_path();
        context.ellipse(
            phase.cos() * distance,
            phase.sin() * distance * 0.8,
            size * (0.08 + index as f64 * 0.03),
            size * (0.05 + index as f64 * 0.02),
            0.0,
            0.0,
            TAU,
        )?;
        context.fill();
    }

    context.set_stroke_style(&JsValue::from_str(edge));
    context.set_line_width((0.46 + size * 0.04).clamp(0.46, 0.92));
    context.begin_path();
    context.ellipse(0.0, 0.0, spread * 0.74, spread * 0.46, 0.12, 0.0, TAU)?;
    context.stroke();
    context.restore();
    Ok(())
}

fn draw_aphid(aphid: &Aphid, render: &RenderContext) -> Result<(), JsValue> {
    let screen_x = aphid.motion.position.x - render.viewport.x;
    let screen_y = aphid.motion.position.y - render.viewport.y;

    if screen_x < -DRAW_MARGIN
        || screen_x > render.viewport.width + DRAW_MARGIN
        || screen_y < -DRAW_MARGIN
        || screen_y > render.viewport.height + DRAW_MARGIN
    {
        return Ok(());
    }

    let context = render.context;
    let size = aphid.motion.size;
    let alpha = match aphid.state {
        AphidState::Feeding => 0.82,
        AphidState::Wandering => 0.88,
        AphidState::Dropping => 0.72,
    };
    let leg_wave = (render.time * 13.2 + aphid.motion.stride_phase).sin();
    let body_fill = if render.dark_mode {
        "rgba(132, 196, 78, 0.9)"
    } else {
        "rgba(126, 184, 72, 0.88)"
    };
    let body_shadow = if render.dark_mode {
        "rgba(68, 122, 42, 0.52)"
    } else {
        "rgba(72, 116, 38, 0.46)"
    };
    let limb_color = if render.dark_mode {
        "rgba(54, 82, 36, 0.82)"
    } else {
        "rgba(48, 70, 32, 0.78)"
    };
    let highlight = if render.dark_mode {
        "rgba(226, 255, 188, 0.22)"
    } else {
        "rgba(246, 255, 212, 0.2)"
    };

    context.save();
    context.translate(screen_x, screen_y)?;
    context.rotate(aphid.motion.heading)?;
    context.set_global_alpha(alpha);
    context.set_line_cap("round");
    context.set_line_join("round");
    context.set_stroke_style(&JsValue::from_str(limb_color));
    context.set_line_width(0.44);

    context.begin_path();
    for index in 0..3 {
        let y = (-0.36 + index as f64 * 0.36) * size;
        let sweep = leg_wave * (0.08 + index as f64 * 0.035) * size;
        let reach = size * (0.72 - index as f64 * 0.08);

        context.move_to(-size * 0.08, y);
        context.line_to(-reach, y - sweep);
        context.move_to(size * 0.12, y);
        context.line_to(reach * 0.58, y + sweep);
    }

    let antenna_sway = 0.8 + (render.time * 7.6 + aphid.antenna_phase).sin() * 0.18;
    context.move_to(size * 0.56, -size * 0.08);
    context.line_to(size * 1.08, -size * 0.34 * antenna_sway);
    context.move_to(size * 0.56, size * 0.08);
    context.line_to(size * 1.08, size * 0.34 * antenna_sway);
    context.stroke();

    context.set_fill_style(&JsValue::from_str(body_fill));
    context.begin_path();
    context.ellipse(-size * 0.14, 0.0, size * 0.72, size * 0.5, 0.0, 0.0, TAU)?;
    context.fill();

    context.set_fill_style(&JsValue::from_str(body_shadow));
    context.begin_path();
    context.ellipse(
        -size * 0.24,
        size * 0.14,
        size * 0.52,
        size * 0.26,
        -0.08,
        0.0,
        TAU,
    )?;
    context.fill();

    context.set_fill_style(&JsValue::from_str(body_fill));
    context.begin_path();
    context.ellipse(size * 0.5, 0.0, size * 0.28, size * 0.24, 0.0, 0.0, TAU)?;
    context.fill();

    context.set_stroke_style(&JsValue::from_str(limb_color));
    context.set_line_width(0.38);
    context.begin_path();
    for side in [-1.0, 1.0] {
        context.move_to(-size * 0.62, side * size * 0.2);
        context.line_to(-size * 0.96, side * size * 0.34);
    }
    context.stroke();

    context.set_fill_style(&JsValue::from_str(highlight));
    context.begin_path();
    context.ellipse(
        -size * 0.28,
        -size * 0.18,
        size * 0.22,
        size * 0.1,
        -0.24,
        0.0,
        TAU,
    )?;
    context.fill();

    context.restore();
    Ok(())
}

fn draw_leaf_beetle(beetle: &LeafBeetle, render: &RenderContext) -> Result<(), JsValue> {
    let screen_x = beetle.motion.position.x - render.viewport.x;
    let screen_y = beetle.motion.position.y - render.viewport.y;

    if screen_x < -DRAW_MARGIN
        || screen_x > render.viewport.width + DRAW_MARGIN
        || screen_y < -DRAW_MARGIN
        || screen_y > render.viewport.height + DRAW_MARGIN
    {
        return Ok(());
    }

    let alpha = match beetle.state {
        LeafBeetleState::Hidden => 0.22,
        LeafBeetleState::Peeking => 0.92,
        LeafBeetleState::Foraging => 1.0,
        LeafBeetleState::Eating => 0.98,
        LeafBeetleState::Returning => 0.86,
    };
    let profile = LEAF_BEETLE_RENDER;
    let context = render.context;
    let size = beetle.motion.size;
    let leg_wave = (render.time * 10.8 + beetle.motion.stride_phase).sin();
    let body_shift = (render.time * 1.8 + beetle.shell_phase).sin() * 0.03 * size;
    let antenna_sway = 0.82
        + (render.time * 8.4 + beetle.antenna_phase).sin()
            * match beetle.state {
                LeafBeetleState::Hidden => 0.04,
                LeafBeetleState::Peeking => 0.1,
                LeafBeetleState::Foraging => 0.16,
                LeafBeetleState::Eating => 0.08,
                LeafBeetleState::Returning => 0.12,
            };
    let limb_color = if render.dark_mode {
        "rgba(44, 52, 36, 0.88)"
    } else {
        "rgba(52, 64, 40, 0.82)"
    };
    let abdomen_fill = if render.dark_mode {
        "rgba(116, 170, 72, 0.96)"
    } else {
        "rgba(124, 186, 82, 0.94)"
    };
    let abdomen_shadow = if render.dark_mode {
        "rgba(58, 108, 34, 0.36)"
    } else {
        "rgba(72, 126, 42, 0.3)"
    };
    let outline = if render.dark_mode {
        "rgba(28, 40, 22, 0.76)"
    } else {
        "rgba(38, 52, 26, 0.7)"
    };
    let thorax_fill = if render.dark_mode {
        "rgba(84, 122, 48, 0.94)"
    } else {
        "rgba(98, 142, 54, 0.92)"
    };
    let head_fill = if render.dark_mode {
        "rgba(46, 60, 30, 0.96)"
    } else {
        "rgba(58, 74, 36, 0.92)"
    };
    let highlight = if render.dark_mode {
        "rgba(220, 244, 176, 0.18)"
    } else {
        "rgba(248, 255, 214, 0.16)"
    };
    let glow_color = if render.dark_mode {
        "rgba(136, 196, 76, 0.14)"
    } else {
        "rgba(144, 208, 88, 0.1)"
    };

    context.save();
    context.translate(screen_x, screen_y)?;
    context.rotate(beetle.motion.heading)?;
    context.set_global_alpha(alpha);
    context.set_line_cap("round");
    context.set_line_join("round");
    context.set_stroke_style(&JsValue::from_str(limb_color));
    context.set_line_width(profile.limb_width);

    context.begin_path();
    for index in 0..profile.leg_pairs {
        let offset = profile.leg_offset_start + index as f64 * profile.leg_offset_step;
        let sweep =
            leg_wave * (profile.leg_sweep_base + index as f64 * profile.leg_sweep_step) * size;
        let reach =
            size * (profile.leg_reach_base - index as f64 * profile.leg_reach_step).max(0.3);
        let y = offset * size;
        let anchor_x = profile.leg_anchor_x * size - 0.02 * size;

        context.move_to(anchor_x, y);
        context.line_to(anchor_x - reach * 0.34, y - sweep * 0.5);
        context.line_to(-reach * 0.82, y - sweep - size * 0.02);
        context.move_to(anchor_x + size * 0.04, y);
        context.line_to(anchor_x + reach * 0.18, y + sweep * 0.36);
        context.line_to(
            reach * profile.front_leg_factor * 0.78,
            y + sweep + size * 0.02,
        );
    }
    context.stroke();

    let antenna_start_x = profile.antenna_start_x * size + size * 0.12;
    let antenna_end_x = profile.antenna_end_x * size + size * 0.1;
    let antenna_end_y = profile.antenna_end_y * antenna_sway * size;
    context.set_line_width(0.56);
    context.begin_path();
    context.move_to(antenna_start_x, -profile.antenna_start_y * size * 0.8);
    context.line_to(antenna_end_x, -antenna_end_y * profile.antenna_spread);
    context.move_to(antenna_start_x, profile.antenna_start_y * size * 0.8);
    context.line_to(antenna_end_x, antenna_end_y * profile.antenna_spread);
    context.stroke();

    let abdomen_center_x = -0.2 * size + body_shift;
    let thorax_center_x = 0.36 * size + body_shift * 0.5;
    let head_center_x = 0.84 * size;

    context.set_shadow_blur(5.0);
    context.set_shadow_color(glow_color);
    context.begin_path();
    context.ellipse(
        abdomen_center_x,
        0.0,
        size * 0.78,
        size * 0.56,
        0.06,
        0.0,
        TAU,
    )?;
    context.set_fill_style(&JsValue::from_str(abdomen_fill));
    context.fill();

    context.set_shadow_blur(0.0);
    context.set_fill_style(&JsValue::from_str(abdomen_shadow));
    context.begin_path();
    context.ellipse(
        abdomen_center_x - size * 0.12,
        size * 0.18,
        size * 0.62,
        size * 0.32,
        -0.1,
        0.0,
        TAU,
    )?;
    context.fill();

    context.set_fill_style(&JsValue::from_str(thorax_fill));
    context.begin_path();
    context.ellipse(
        thorax_center_x,
        0.0,
        size * 0.42,
        size * 0.3,
        -0.06,
        0.0,
        TAU,
    )?;
    context.fill();

    context.set_fill_style(&JsValue::from_str(head_fill));
    context.begin_path();
    context.ellipse(head_center_x, 0.0, size * 0.2, size * 0.18, 0.0, 0.0, TAU)?;
    context.fill();

    context.set_fill_style(&JsValue::from_str(highlight));
    context.begin_path();
    context.ellipse(
        abdomen_center_x - size * 0.18,
        -size * 0.2,
        size * 0.24,
        size * 0.12,
        -0.32,
        0.0,
        TAU,
    )?;
    context.ellipse(
        thorax_center_x - size * 0.04,
        -size * 0.08,
        size * 0.12,
        size * 0.06,
        -0.24,
        0.0,
        TAU,
    )?;
    context.fill();

    context.set_stroke_style(&JsValue::from_str(outline));
    context.set_line_width(0.5);
    context.begin_path();
    context.ellipse(
        abdomen_center_x,
        0.0,
        size * 0.78,
        size * 0.56,
        0.06,
        0.0,
        TAU,
    )?;
    context.ellipse(
        thorax_center_x,
        0.0,
        size * 0.42,
        size * 0.3,
        -0.06,
        0.0,
        TAU,
    )?;
    context.ellipse(head_center_x, 0.0, size * 0.2, size * 0.18, 0.0, 0.0, TAU)?;
    context.stroke();

    context.restore();
    Ok(())
}

fn draw_ladybug(ladybug: &Ladybug, render: &RenderContext) -> Result<(), JsValue> {
    let screen_x = ladybug.motion.position.x - render.viewport.x;
    let screen_y = ladybug.motion.position.y - render.viewport.y;
    let size = ladybug.motion.size;

    if screen_x < -DRAW_MARGIN
        || screen_x > render.viewport.width + DRAW_MARGIN
        || screen_y < -DRAW_MARGIN - size * 3.0
        || screen_y > render.viewport.height + DRAW_MARGIN
    {
        return Ok(());
    }

    let wing_open = if ladybug.state == LadybugState::Dispersing {
        0.55 + (render.time * 18.6 + ladybug.wing_phase).sin().abs() * 0.45
    } else if ladybug.state == LadybugState::Flying {
        let flight_progress = smoothstep(0.0, 8.0, ladybug.state_time)
            * (1.0 - smoothstep(66.0, 82.0, ladybug.state_time));
        let wing_beat = 0.48 + (render.time * 18.6 + ladybug.wing_phase).sin().abs() * 0.52;
        flight_progress * wing_beat
    } else {
        0.0
    };
    let flight_lift = if ladybug_is_airborne(ladybug.state) {
        7.4 + wing_open * 10.8 + (render.time * 16.8 + ladybug.wing_phase).sin().abs() * 3.2
    } else {
        0.0
    };
    let shell_shift = (render.time * 1.24 + ladybug.shell_phase).sin() * 0.03 * size;
    let leg_wave = (render.time * 10.4 + ladybug.motion.stride_phase).sin();
    let antenna_sway = 0.76
        + (render.time * 7.2 + ladybug.flight_phase).sin()
            * if ladybug_is_airborne(ladybug.state) {
                0.24
            } else {
                0.14
            };
    let alpha = if ladybug_is_airborne(ladybug.state) {
        0.98
    } else {
        0.94
    };
    let limb_color = if render.dark_mode {
        "rgba(24, 24, 26, 0.94)"
    } else {
        "rgba(18, 18, 20, 0.92)"
    };
    let shell_red = if render.dark_mode {
        "rgba(220, 62, 40, 0.96)"
    } else {
        "rgba(206, 48, 30, 0.95)"
    };
    let shell_red_shadow = if render.dark_mode {
        "rgba(128, 22, 12, 0.32)"
    } else {
        "rgba(120, 18, 12, 0.28)"
    };
    let shell_outline = if render.dark_mode {
        "rgba(42, 18, 14, 0.76)"
    } else {
        "rgba(34, 16, 12, 0.72)"
    };
    let spot_color = if render.dark_mode {
        "rgba(10, 10, 12, 0.95)"
    } else {
        "rgba(8, 8, 10, 0.94)"
    };
    let shell_highlight = if render.dark_mode {
        "rgba(255, 214, 194, 0.2)"
    } else {
        "rgba(255, 234, 220, 0.18)"
    };
    let head_fill = if render.dark_mode {
        "rgba(18, 18, 20, 0.96)"
    } else {
        "rgba(14, 14, 16, 0.94)"
    };
    let shoulder_patch = if render.dark_mode {
        "rgba(246, 230, 188, 0.82)"
    } else {
        "rgba(248, 236, 194, 0.84)"
    };
    let wing_fill = if render.dark_mode {
        "rgba(248, 242, 234, 0.22)"
    } else {
        "rgba(255, 249, 241, 0.28)"
    };
    let wing_outline = if render.dark_mode {
        "rgba(244, 236, 228, 0.24)"
    } else {
        "rgba(244, 234, 224, 0.3)"
    };

    let shell_center_x = -0.02 * size + shell_shift;
    let shell_center_y = 0.0;
    let shell_rx = 1.04 * size;
    let shell_ry = 0.84 * size;

    let context = render.context;
    context.save();
    context.translate(screen_x, screen_y - flight_lift)?;
    context.rotate(ladybug.motion.heading + wing_open * 0.04)?;
    context.set_global_alpha(alpha);
    context.set_line_cap("round");
    context.set_line_join("round");

    if ladybug_is_airborne(ladybug.state) {
        context.save();
        context.set_global_alpha((0.12 + wing_open * 0.18) * alpha);
        context.set_fill_style(&JsValue::from_str(if render.dark_mode {
            "rgba(0, 0, 0, 0.28)"
        } else {
            "rgba(0, 0, 0, 0.16)"
        }));
        context.begin_path();
        context.ellipse(
            -size * 0.06,
            flight_lift + size * 0.2,
            size * (0.92 - wing_open * 0.16),
            size * (0.34 - wing_open * 0.04).max(0.18),
            0.0,
            0.0,
            TAU,
        )?;
        context.fill();
        context.restore();
    }

    context.set_stroke_style(&JsValue::from_str(limb_color));
    context.set_line_width(0.78);
    context.begin_path();
    for index in 0..3 {
        let offset = (-0.52 + index as f64 * 0.5) * size;
        let sweep = leg_wave * (0.15 + index as f64 * 0.05) * size * (1.0 - wing_open * 0.55);
        let reach = size * (0.86 - index as f64 * 0.08).max(0.56) * (1.0 - wing_open * 0.36);
        let anchor_x = -0.02 * size;

        context.move_to(anchor_x, offset);
        context.line_to(anchor_x - reach * 0.38, offset - sweep);
        context.line_to(-reach, offset - sweep - size * 0.08);
        context.move_to(anchor_x + size * 0.12, offset);
        context.line_to(anchor_x + reach * 0.16, offset + sweep * 0.82);
        context.line_to(reach * 0.44, offset + sweep + size * 0.06);
    }
    context.stroke();

    let antenna_start_x = size * 0.76;
    let antenna_end_x = size * 1.3;
    let antenna_end_y = size * 0.28 * antenna_sway;
    context.set_line_width(0.68);
    context.begin_path();
    context.move_to(antenna_start_x, -size * 0.08);
    context.line_to(antenna_end_x, -antenna_end_y);
    context.move_to(antenna_start_x, size * 0.08);
    context.line_to(antenna_end_x, antenna_end_y);
    context.stroke();

    if wing_open > 0.04 {
        context.save();
        context.set_global_alpha((0.22 + wing_open * 0.36) * alpha);
        context.set_fill_style(&JsValue::from_str(wing_fill));
        context.set_stroke_style(&JsValue::from_str(wing_outline));
        context.set_line_width(0.46);

        context.begin_path();
        for side in [-1.0, 1.0] {
            context.ellipse(
                shell_center_x - size * 0.24,
                side * size * (0.26 + wing_open * 0.58),
                size * (1.06 + wing_open * 0.24),
                size * (0.3 + wing_open * 0.04),
                side * (0.88 + wing_open * 0.24),
                0.0,
                TAU,
            )?;
        }
        context.fill();
        context.stroke();
        context.restore();
    }

    context.set_fill_style(&JsValue::from_str(head_fill));
    context.begin_path();
    context.ellipse(size * 0.54, 0.0, size * 0.48, size * 0.34, -0.04, 0.0, TAU)?;
    context.ellipse(size * 0.98, 0.0, size * 0.24, size * 0.21, -0.08, 0.0, TAU)?;
    context.fill();

    context.set_fill_style(&JsValue::from_str(shoulder_patch));
    context.begin_path();
    context.ellipse(
        size * 0.4,
        -size * 0.22,
        size * 0.16,
        size * 0.08,
        -0.3,
        0.0,
        TAU,
    )?;
    context.ellipse(
        size * 0.4,
        size * 0.22,
        size * 0.16,
        size * 0.08,
        0.3,
        0.0,
        TAU,
    )?;
    context.fill();

    context.set_shadow_blur(6.0 + wing_open * 3.0);
    context.set_shadow_color(if render.dark_mode {
        "rgba(218, 82, 44, 0.16)"
    } else {
        "rgba(218, 72, 36, 0.14)"
    });

    for side in [-1.0, 1.0] {
        let offset_y = side * size * (0.18 + wing_open * 0.16);
        let rotation = side * wing_open * 0.2;

        context.begin_path();
        context.ellipse(
            shell_center_x,
            shell_center_y + offset_y,
            shell_rx,
            shell_ry * 0.56,
            rotation,
            0.0,
            TAU,
        )?;
        context.set_fill_style(&JsValue::from_str(shell_red));
        context.fill();

        context.set_shadow_blur(0.0);
        context.set_fill_style(&JsValue::from_str(shell_red_shadow));
        context.begin_path();
        context.ellipse(
            shell_center_x - size * 0.16,
            shell_center_y + offset_y + side * size * 0.12,
            shell_rx * 0.88,
            shell_ry * 0.28,
            rotation - side * 0.08,
            0.0,
            TAU,
        )?;
        context.fill();

        context.set_fill_style(&JsValue::from_str(spot_color));
        let spots = [
            (-0.46, -0.18, 0.22, 0.16),
            (-0.02, 0.02, 0.25, 0.18),
            (0.48, 0.1, 0.2, 0.15),
        ];
        context.begin_path();
        for (sx, sy, rx, ry) in spots {
            context.ellipse(
                shell_center_x + sx * size,
                shell_center_y + offset_y + side * sy * size,
                size * rx,
                size * ry,
                side * (0.12 + rotation),
                0.0,
                TAU,
            )?;
        }
        context.fill();

        context.set_fill_style(&JsValue::from_str(shell_highlight));
        context.begin_path();
        context.ellipse(
            shell_center_x - size * 0.18,
            shell_center_y + offset_y - side * size * 0.16,
            size * 0.34,
            size * 0.14,
            -0.24,
            0.0,
            TAU,
        )?;
        context.fill();
    }

    context.set_fill_style(&JsValue::from_str(spot_color));
    context.begin_path();
    context.ellipse(
        shell_center_x + size * 0.06,
        0.0,
        size * 0.18,
        size * 0.14,
        0.0,
        0.0,
        TAU,
    )?;
    context.fill();

    context.set_stroke_style(&JsValue::from_str(shell_outline));
    context.set_line_width(0.74);
    context.begin_path();
    for side in [-1.0, 1.0] {
        context.ellipse(
            shell_center_x,
            shell_center_y + side * size * (0.18 + wing_open * 0.16),
            shell_rx,
            shell_ry * 0.56,
            side * wing_open * 0.2,
            0.0,
            TAU,
        )?;
    }

    context.move_to(
        shell_center_x - shell_rx * 0.94,
        -size * (0.02 + wing_open * 0.04),
    );
    context.line_to(
        shell_center_x + shell_rx * 0.9,
        size * (0.02 + wing_open * 0.04),
    );
    context.stroke();

    context.restore();
    Ok(())
}

fn draw_ground_beetle(beetle: &GroundBeetle, render: &RenderContext) -> Result<(), JsValue> {
    let screen_x = beetle.motion.position.x - render.viewport.x;
    let screen_y = beetle.motion.position.y - render.viewport.y;

    if screen_x < -DRAW_MARGIN
        || screen_x > render.viewport.width + DRAW_MARGIN
        || screen_y < -DRAW_MARGIN
        || screen_y > render.viewport.height + DRAW_MARGIN
    {
        return Ok(());
    }

    let alpha = match beetle.state {
        GroundBeetleState::Sheltering => 0.28,
        GroundBeetleState::Patrolling => 0.9,
        GroundBeetleState::Hunting => 1.0,
        GroundBeetleState::Feeding => 0.94,
        GroundBeetleState::Dispersing => 0.72,
    };
    let profile = GROUND_BEETLE_RENDER;
    let context = render.context;
    let size = beetle.motion.size;
    let leg_wave = (render.time * 8.8 + beetle.motion.stride_phase).sin();
    let antenna_sway = 0.82 + (render.time * 5.9 + beetle.antenna_phase).sin() * 0.18;
    let shell_shift = (render.time * 0.9 + beetle.shell_phase).sin() * 0.03 * size;
    let limb_color = if render.dark_mode {
        "rgba(26, 28, 34, 0.94)"
    } else {
        "rgba(18, 19, 24, 0.92)"
    };
    let shell_base = if render.dark_mode {
        "rgba(30, 66, 56, 0.94)"
    } else {
        "rgba(20, 58, 48, 0.92)"
    };
    let shell_inner = if render.dark_mode {
        "rgba(66, 132, 103, 0.44)"
    } else {
        "rgba(58, 126, 96, 0.38)"
    };
    let shell_edge = if render.dark_mode {
        "rgba(10, 20, 18, 0.92)"
    } else {
        "rgba(9, 18, 16, 0.88)"
    };
    let shell_sheen = if render.dark_mode {
        "rgba(118, 196, 152, 0.16)"
    } else {
        "rgba(132, 212, 168, 0.12)"
    };
    let ridge_color = if render.dark_mode {
        "rgba(104, 174, 132, 0.22)"
    } else {
        "rgba(88, 160, 120, 0.18)"
    };
    let pronotum_fill = if render.dark_mode {
        "rgba(24, 26, 32, 0.96)"
    } else {
        "rgba(18, 19, 25, 0.94)"
    };
    let pronotum_highlight = if render.dark_mode {
        "rgba(116, 126, 150, 0.14)"
    } else {
        "rgba(128, 138, 164, 0.12)"
    };
    let glow_color = if render.dark_mode {
        "rgba(62, 138, 100, 0.14)"
    } else {
        "rgba(54, 124, 92, 0.1)"
    };

    context.save();
    context.translate(screen_x, screen_y)?;
    context.rotate(beetle.motion.heading)?;
    context.set_global_alpha(alpha);
    context.set_line_cap("round");
    context.set_line_join("round");
    context.set_stroke_style(&JsValue::from_str(limb_color));
    context.set_line_width(profile.limb_width);

    context.begin_path();
    for index in 0..profile.leg_pairs {
        let offset = profile.leg_offset_start + index as f64 * profile.leg_offset_step;
        let sweep =
            leg_wave * (profile.leg_sweep_base + index as f64 * profile.leg_sweep_step) * size;
        let reach =
            size * (profile.leg_reach_base - index as f64 * profile.leg_reach_step).max(0.58);
        let y = offset * size;
        let anchor_x = profile.leg_anchor_x * size + size * 0.04;

        context.move_to(anchor_x, y);
        context.line_to(anchor_x - reach * 0.42, y - sweep * 0.55);
        context.line_to(-reach, y - sweep - size * 0.12);
        context.move_to(anchor_x + size * 0.08, y);
        context.line_to(anchor_x + reach * 0.22, y + sweep * 0.45);
        context.line_to(reach * profile.front_leg_factor, y + sweep + size * 0.08);
    }
    context.stroke();

    let antenna_start_x = profile.antenna_start_x * size + size * 0.22;
    let antenna_end_x = profile.antenna_end_x * size + size * 0.12;
    let antenna_end_y = profile.antenna_end_y * antenna_sway * size;
    context.set_line_width(0.74);
    context.begin_path();
    context.move_to(antenna_start_x, -profile.antenna_start_y * size);
    context.line_to(antenna_end_x, -antenna_end_y * profile.antenna_spread);
    context.move_to(antenna_start_x, profile.antenna_start_y * size);
    context.line_to(antenna_end_x, antenna_end_y * profile.antenna_spread);
    context.stroke();

    let shell_center_x = -0.14 * size + shell_shift;
    let shell_center_y = 0.0;
    let shell_rx = 1.14 * size;
    let shell_ry = 0.7 * size;

    context.set_shadow_blur(7.0);
    context.set_shadow_color(glow_color);
    context.begin_path();
    context.ellipse(
        shell_center_x,
        shell_center_y,
        shell_rx,
        shell_ry,
        0.03,
        0.0,
        TAU,
    )?;
    context.set_fill_style(&JsValue::from_str(shell_base));
    context.fill();

    context.save();
    context.begin_path();
    context.ellipse(
        shell_center_x,
        shell_center_y,
        shell_rx,
        shell_ry,
        0.03,
        0.0,
        TAU,
    )?;
    context.clip();

    context.set_shadow_blur(0.0);
    context.set_fill_style(&JsValue::from_str(shell_edge));
    context.begin_path();
    context.ellipse(
        shell_center_x - size * 0.22,
        shell_center_y + size * 0.26,
        shell_rx * 0.96,
        shell_ry * 0.7,
        -0.06,
        0.0,
        TAU,
    )?;
    context.fill();

    context.set_fill_style(&JsValue::from_str(shell_inner));
    context.begin_path();
    context.ellipse(
        shell_center_x - size * 0.08,
        shell_center_y - size * 0.04,
        shell_rx * 0.76,
        shell_ry * 0.66,
        0.0,
        0.0,
        TAU,
    )?;
    context.fill();

    context.set_fill_style(&JsValue::from_str(shell_sheen));
    context.begin_path();
    context.ellipse(
        shell_center_x - size * 0.26,
        shell_center_y - size * 0.22,
        shell_rx * 0.38,
        shell_ry * 0.2,
        -0.18,
        0.0,
        TAU,
    )?;
    context.fill();

    context.set_stroke_style(&JsValue::from_str(ridge_color));
    context.set_line_width(0.46);
    context.begin_path();
    for index in -3..=3 {
        let y = index as f64 * size * 0.18;
        context.move_to(shell_center_x - shell_rx * 0.72, shell_center_y + y);
        context.line_to(shell_center_x + shell_rx * 0.88, shell_center_y + y * 0.74);
    }
    context.stroke();
    context.restore();

    context.set_shadow_blur(0.0);
    context.set_stroke_style(&JsValue::from_str(shell_edge));
    context.set_line_width(0.78);
    context.begin_path();
    context.ellipse(
        shell_center_x,
        shell_center_y,
        shell_rx,
        shell_ry,
        0.03,
        0.0,
        TAU,
    )?;
    context.move_to(shell_center_x - shell_rx * 0.96, shell_center_y);
    context.line_to(shell_center_x + shell_rx * 0.92, shell_center_y);
    context.stroke();

    context.set_fill_style(&JsValue::from_str(pronotum_fill));
    context.begin_path();
    context.ellipse(size * 0.6, 0.0, size * 0.54, size * 0.36, -0.02, 0.0, TAU)?;
    context.fill();

    context.begin_path();
    context.ellipse(size * 1.2, 0.0, size * 0.28, size * 0.22, -0.04, 0.0, TAU)?;
    context.fill();

    context.set_fill_style(&JsValue::from_str(pronotum_highlight));
    context.begin_path();
    context.ellipse(
        size * 0.52,
        -size * 0.14,
        size * 0.18,
        size * 0.08,
        -0.2,
        0.0,
        TAU,
    )?;
    context.ellipse(
        shell_center_x - size * 0.1,
        -size * 0.08,
        shell_rx * 0.44,
        shell_ry * 0.14,
        -0.14,
        0.0,
        TAU,
    )?;
    context.fill();

    context.restore();
    Ok(())
}

fn draw_centipede(centipede: &Centipede, render: &RenderContext) -> Result<(), JsValue> {
    let screen_x = centipede.motion.position.x - render.viewport.x;
    let screen_y = centipede.motion.position.y - render.viewport.y;
    let size = centipede.motion.size;
    let body_margin = DRAW_MARGIN + centipede_trail_length(size) + size * 2.4;

    if screen_x < -body_margin
        || screen_x > render.viewport.width + body_margin
        || screen_y < -body_margin
        || screen_y > render.viewport.height + body_margin
    {
        return Ok(());
    }

    let alpha = match centipede.state {
        CentipedeState::Sheltering => 0.26,
        CentipedeState::Patrolling => 0.88,
        CentipedeState::Hunting => 1.0,
        CentipedeState::Feeding => 0.94,
        CentipedeState::Dispersing => 0.7,
    };
    let context = render.context;
    let segment_count = centipede_segment_count();
    let segment_spacing = centipede_segment_spacing(size);
    let leg_wave = render.time * 11.2 + centipede.motion.stride_phase;
    let antenna_wave = 0.8 + (render.time * 5.4 + centipede.antenna_phase).sin() * 0.24;
    let body_fill = if render.dark_mode {
        "rgba(54, 44, 40, 0.95)"
    } else {
        "rgba(44, 35, 31, 0.92)"
    };
    let body_shadow = if render.dark_mode {
        "rgba(22, 17, 16, 0.96)"
    } else {
        "rgba(18, 14, 13, 0.92)"
    };
    let segment_edge = if render.dark_mode {
        "rgba(10, 7, 7, 0.96)"
    } else {
        "rgba(14, 10, 10, 0.9)"
    };
    let body_highlight = if render.dark_mode {
        "rgba(144, 106, 90, 0.14)"
    } else {
        "rgba(124, 92, 78, 0.12)"
    };
    let leg_color = if render.dark_mode {
        "rgba(230, 156, 72, 0.78)"
    } else {
        "rgba(214, 136, 46, 0.74)"
    };
    let fang_color = if render.dark_mode {
        "rgba(214, 88, 34, 0.82)"
    } else {
        "rgba(188, 72, 24, 0.8)"
    };
    let head_fill = if render.dark_mode {
        "rgba(116, 52, 30, 0.9)"
    } else {
        "rgba(102, 44, 26, 0.88)"
    };
    let glow_color = if render.dark_mode {
        "rgba(164, 90, 38, 0.12)"
    } else {
        "rgba(138, 74, 30, 0.08)"
    };

    let mut segments = Vec::with_capacity(segment_count);
    for index in 0..segment_count {
        let distance = index as f64 * segment_spacing;
        let t = index as f64 / (segment_count.saturating_sub(1) as f64);
        let point = sample_centipede_trail_point(&centipede.trail, distance);
        let heading =
            sample_centipede_trail_heading(&centipede.trail, distance, centipede.motion.heading);
        let bulk = (1.0 - ((t - 0.42).abs() / 0.58)).clamp(0.0, 1.0);
        let taper = 1.0 - t * 0.18;
        let rx = if index == 0 {
            size * 0.28
        } else {
            size * (0.16 + bulk * 0.17) * taper
        };
        let ry = if index == 0 {
            size * 0.22
        } else {
            size * (0.12 + bulk * 0.14) * taper
        };
        segments.push((point, heading, rx, ry, t));
    }

    context.save();
    context.set_global_alpha(alpha);
    context.set_line_cap("round");
    context.set_line_join("round");
    context.set_stroke_style(&JsValue::from_str(leg_color));
    context.set_line_width(0.46 + size * 0.055);

    context.begin_path();
    for (index, (point, heading, rx, ry, t)) in
        segments.iter().enumerate().skip(1).take(segment_count - 2)
    {
        let sweep = (leg_wave + index as f64 * 0.74).sin();
        let reach = size * (1.0 - *t * 0.22).max(0.62);
        let forward_x = heading.cos();
        let forward_y = heading.sin();
        let side_x = -forward_y;
        let side_y = forward_x;

        for side in [-1.0, 1.0] {
            let anchor = Point {
                x: point.x - forward_x * rx * 0.08 + side_x * side * ry * 0.84,
                y: point.y - forward_y * rx * 0.08 + side_y * side * ry * 0.84,
            };
            let knee = Point {
                x: anchor.x - forward_x * reach * 0.28
                    + side_x * side * (reach * 0.34 + sweep * size * 0.08),
                y: anchor.y - forward_y * reach * 0.28
                    + side_y * side * (reach * 0.34 + sweep * size * 0.08),
            };
            let tip = Point {
                x: knee.x - forward_x * reach * 0.48
                    + side_x * side * (reach * 0.42 + sweep * size * 0.14),
                y: knee.y - forward_y * reach * 0.48
                    + side_y * side * (reach * 0.42 + sweep * size * 0.14),
            };

            context.move_to(anchor.x - render.viewport.x, anchor.y - render.viewport.y);
            context.line_to(knee.x - render.viewport.x, knee.y - render.viewport.y);
            context.line_to(tip.x - render.viewport.x, tip.y - render.viewport.y);
        }
    }
    context.stroke();

    context.set_shadow_blur(6.0);
    context.set_shadow_color(glow_color);
    context.set_stroke_style(&JsValue::from_str(body_fill));
    for window in segments.windows(2).rev() {
        let front = &window[0];
        let back = &window[1];
        let line_width = (front.3.min(back.3) * 1.8).max(1.2);
        context.set_line_width(line_width);
        context.begin_path();
        context.move_to(front.0.x - render.viewport.x, front.0.y - render.viewport.y);
        context.line_to(back.0.x - render.viewport.x, back.0.y - render.viewport.y);
        context.stroke();
    }

    for (index, (point, heading, rx, ry, _)) in segments.iter().enumerate().rev() {
        let fill = if index == 0 {
            head_fill
        } else if index >= segment_count.saturating_sub(2) {
            "rgba(78, 42, 28, 0.9)"
        } else {
            body_fill
        };

        context.begin_path();
        context.ellipse(
            point.x - render.viewport.x,
            point.y - render.viewport.y,
            *rx,
            *ry,
            *heading,
            0.0,
            TAU,
        )?;
        context.set_fill_style(&JsValue::from_str(fill));
        context.fill();
    }

    context.set_shadow_blur(0.0);
    for (index, (point, heading, rx, ry, _)) in segments.iter().enumerate().rev() {
        let forward_x = heading.cos();
        let forward_y = heading.sin();
        let side_x = -forward_y;
        let side_y = forward_x;

        context.set_fill_style(&JsValue::from_str(body_shadow));
        context.begin_path();
        context.ellipse(
            point.x - render.viewport.x + side_x * rx * 0.12 - forward_x * rx * 0.08,
            point.y - render.viewport.y + side_y * ry * 0.18 - forward_y * rx * 0.08,
            rx * 0.92,
            ry * 0.68,
            *heading,
            0.0,
            TAU,
        )?;
        context.fill();

        context.set_fill_style(&JsValue::from_str(body_highlight));
        context.begin_path();
        context.ellipse(
            point.x - render.viewport.x - side_x * rx * 0.14 + forward_x * rx * 0.04,
            point.y - render.viewport.y - side_y * ry * 0.24 + forward_y * rx * 0.04,
            rx * 0.42,
            ry * 0.22,
            *heading - 0.18,
            0.0,
            TAU,
        )?;
        context.fill();

        context.set_stroke_style(&JsValue::from_str(segment_edge));
        context.set_line_width(0.5);
        context.begin_path();
        context.ellipse(
            point.x - render.viewport.x,
            point.y - render.viewport.y,
            *rx,
            *ry,
            *heading,
            0.0,
            TAU,
        )?;
        context.stroke();

        if index > 0 {
            let next = &segments[index - 1];
            context.begin_path();
            context.move_to(point.x - render.viewport.x, point.y - render.viewport.y);
            context.line_to(next.0.x - render.viewport.x, next.0.y - render.viewport.y);
            context.stroke();
        }
    }

    let (head_point, head_heading, head_rx, head_ry, _) = segments[0];
    let head_forward_x = head_heading.cos();
    let head_forward_y = head_heading.sin();
    let head_side_x = -head_forward_y;
    let head_side_y = head_forward_x;
    let head_center = Point {
        x: head_point.x + head_forward_x * head_rx * 0.92,
        y: head_point.y + head_forward_y * head_rx * 0.92,
    };
    context.set_fill_style(&JsValue::from_str(head_fill));
    context.begin_path();
    context.ellipse(
        head_center.x - render.viewport.x,
        head_center.y - render.viewport.y,
        size * 0.32,
        size * 0.24,
        head_heading,
        0.0,
        TAU,
    )?;
    context.fill();

    context.set_fill_style(&JsValue::from_str(body_highlight));
    context.begin_path();
    context.ellipse(
        head_center.x - render.viewport.x - head_side_x * size * 0.12
            + head_forward_x * size * 0.02,
        head_center.y - render.viewport.y - head_side_y * size * 0.12
            + head_forward_y * size * 0.02,
        size * 0.12,
        size * 0.06,
        head_heading - 0.2,
        0.0,
        TAU,
    )?;
    context.fill();

    context.set_stroke_style(&JsValue::from_str(fang_color));
    context.set_line_width(0.64);
    context.begin_path();
    context.move_to(
        head_center.x - render.viewport.x + head_side_x * head_ry * 0.14,
        head_center.y - render.viewport.y + head_side_y * head_ry * 0.14,
    );
    context.line_to(
        head_center.x - render.viewport.x
            + head_forward_x * size * 0.92
            + head_side_x * size * 0.42 * antenna_wave,
        head_center.y - render.viewport.y
            + head_forward_y * size * 0.92
            + head_side_y * size * 0.42 * antenna_wave,
    );
    context.move_to(
        head_center.x - render.viewport.x - head_side_x * head_ry * 0.14,
        head_center.y - render.viewport.y - head_side_y * head_ry * 0.14,
    );
    context.line_to(
        head_center.x - render.viewport.x + head_forward_x * size * 0.92
            - head_side_x * size * 0.42 * antenna_wave,
        head_center.y - render.viewport.y + head_forward_y * size * 0.92
            - head_side_y * size * 0.42 * antenna_wave,
    );
    context.move_to(
        head_center.x - render.viewport.x + head_side_x * head_ry * 0.1,
        head_center.y - render.viewport.y + head_side_y * head_ry * 0.1,
    );
    context.line_to(
        head_center.x - render.viewport.x
            + head_forward_x * size * 1.22
            + head_side_x * size * 0.72 * antenna_wave,
        head_center.y - render.viewport.y
            + head_forward_y * size * 1.22
            + head_side_y * size * 0.72 * antenna_wave,
    );
    context.move_to(
        head_center.x - render.viewport.x - head_side_x * head_ry * 0.1,
        head_center.y - render.viewport.y - head_side_y * head_ry * 0.1,
    );
    context.line_to(
        head_center.x - render.viewport.x + head_forward_x * size * 1.22
            - head_side_x * size * 0.72 * antenna_wave,
        head_center.y - render.viewport.y + head_forward_y * size * 1.22
            - head_side_y * size * 0.72 * antenna_wave,
    );
    context.stroke();

    context.set_stroke_style(&JsValue::from_str(segment_edge));
    context.set_line_width(0.48);
    context.begin_path();
    context.ellipse(
        head_center.x - render.viewport.x,
        head_center.y - render.viewport.y,
        size * 0.32,
        size * 0.24,
        head_heading,
        0.0,
        TAU,
    )?;
    context.stroke();

    context.restore();
    Ok(())
}

fn draw_strawberry_plant(plant: &StrawberryPlant, render: &RenderContext) -> Result<(), JsValue> {
    let screen_x = plant.root.x - render.viewport.x;
    let screen_y = plant.root.y - render.viewport.y;

    if !point_visible_in_viewport(
        plant.root,
        render.viewport,
        DRAW_MARGIN + plant.leaf_span + 58.0,
    ) {
        return Ok(());
    }

    let leaf_growth = ease_out_cubic(smoothstep(0.0, 0.28, plant.life));
    let bloom_growth = ease_out_cubic(smoothstep(0.26, 0.6, plant.life));
    let fruit_growth = ease_out_cubic(smoothstep(0.48, 0.88, plant.life));
    let ripe_growth = smoothstep(0.72, 1.02, plant.life);
    let runner_growth = ease_out_cubic(smoothstep(0.6, 1.0, plant.life));
    let health_vigor = plant_vigor(plant.health);
    let leaf_vigor = plant_vigor(plant.leaf_health);
    let sap_vigor = plant_vigor(plant.sap_health);
    let plant_fade = health_vigor;
    let leaf_sway =
        (render.time * 0.7 + plant.leaf_phase + plant.sway_phase).sin() * (0.9 + leaf_growth * 0.8);
    let crown_lift = -1.8 - leaf_growth * 1.6;
    let leaf_span = plant.leaf_span * (0.3 + leaf_growth * 0.7) * (0.62 + leaf_vigor * 0.38);

    let leaf_fill = if render.dark_mode {
        "rgba(110, 178, 96, 0.3)"
    } else {
        "rgba(92, 148, 68, 0.34)"
    };
    let leaf_stroke = if render.dark_mode {
        "rgba(170, 226, 152, 0.42)"
    } else {
        "rgba(74, 118, 48, 0.46)"
    };
    let runner_color = if render.dark_mode {
        "rgba(164, 198, 122, 0.34)"
    } else {
        "rgba(116, 148, 70, 0.34)"
    };
    let blossom_petal = if render.dark_mode {
        "rgba(255, 248, 240, 0.84)"
    } else {
        "rgba(255, 252, 247, 0.88)"
    };
    let blossom_center = if render.dark_mode {
        "rgba(255, 214, 108, 0.82)"
    } else {
        "rgba(234, 188, 62, 0.84)"
    };
    let berry_red = if render.dark_mode {
        "rgba(222, 76, 64, 0.88)"
    } else {
        "rgba(212, 52, 42, 0.9)"
    };
    let berry_unripe = if render.dark_mode {
        "rgba(184, 214, 114, 0.72)"
    } else {
        "rgba(152, 184, 88, 0.76)"
    };
    let berry_seed = if render.dark_mode {
        "rgba(255, 228, 176, 0.78)"
    } else {
        "rgba(255, 228, 156, 0.84)"
    };
    let calyx_color = if render.dark_mode {
        "rgba(178, 216, 118, 0.52)"
    } else {
        "rgba(112, 154, 54, 0.54)"
    };

    let context = render.context;
    context.save();
    context.translate(screen_x, screen_y)?;
    context.set_line_cap("round");
    context.set_line_join("round");
    context.set_global_alpha(plant_fade.max(0.0));

    if runner_growth > 0.02 {
        context.save();
        context.set_stroke_style(&JsValue::from_str(runner_color));
        context.set_line_width(0.7 + runner_growth * 0.55);
        context.set_global_alpha((0.16 + runner_growth * 0.42) * plant_fade.max(0.0) * sap_vigor);

        for index in 0..plant.runner_count {
            let runner_offset = strawberry_runner_offset(plant, index);
            let end_x = runner_offset.x * runner_growth;
            let end_y = runner_offset.y * runner_growth;
            let mid_x = end_x * 0.48 + (plant.runner_phase + index as f64 * 1.2).sin() * 2.2;
            let mid_y = end_y * 0.46 - 1.8;

            context.begin_path();
            context.move_to(0.0, crown_lift * 0.1);
            context.line_to(mid_x, mid_y);
            context.line_to(end_x, end_y);
            context.stroke();

            if runner_growth > 0.4 {
                let leaflet_rotation = runner_offset.y.atan2(runner_offset.x) * 0.24;
                context.set_fill_style(&JsValue::from_str(leaf_fill));
                context.begin_path();
                context.ellipse(
                    end_x - 1.8,
                    end_y - 0.8,
                    2.4,
                    1.3,
                    leaflet_rotation - 0.36,
                    0.0,
                    TAU,
                )?;
                context.ellipse(
                    end_x + 1.9,
                    end_y - 0.9,
                    2.4,
                    1.3,
                    leaflet_rotation + 0.36,
                    0.0,
                    TAU,
                )?;
                context.ellipse(end_x, end_y - 2.4, 2.2, 1.2, leaflet_rotation, 0.0, TAU)?;
                context.fill();
            }
        }
        context.restore();
    }

    context.save();
    context.set_fill_style(&JsValue::from_str(leaf_fill));
    context.set_stroke_style(&JsValue::from_str(leaf_stroke));
    context.set_line_width(0.76 + leaf_growth * 0.52);
    context.set_global_alpha((0.2 + leaf_growth * 0.58) * plant_fade.max(0.0));

    let leaves = [
        (
            -leaf_span * 0.42 - leaf_sway * 0.24,
            crown_lift - leaf_span * 0.12,
            leaf_span * 0.36,
            leaf_span * 0.22,
            -0.64,
        ),
        (
            leaf_span * 0.4 + leaf_sway * 0.22,
            crown_lift - leaf_span * 0.08,
            leaf_span * 0.34,
            leaf_span * 0.22,
            0.58,
        ),
        (
            -leaf_span * 0.12,
            crown_lift - leaf_span * 0.38 - leaf_sway * 0.14,
            leaf_span * 0.28,
            leaf_span * 0.18,
            -0.12,
        ),
        (
            leaf_span * 0.16,
            crown_lift - leaf_span * 0.24 + leaf_sway * 0.08,
            leaf_span * 0.28,
            leaf_span * 0.18,
            0.16,
        ),
        (
            0.0,
            crown_lift - leaf_span * 0.1,
            leaf_span * 0.22,
            leaf_span * 0.16,
            0.0,
        ),
    ];

    for (center_x, center_y, radius_x, radius_y, rotation) in leaves {
        context.begin_path();
        context.ellipse(center_x, center_y, radius_x, radius_y, rotation, 0.0, TAU)?;
        context.fill();
        context.stroke();

        context.begin_path();
        context.move_to(center_x, center_y + radius_y * 0.84);
        context.line_to(center_x, center_y - radius_y * 0.84);
        context.stroke();
    }
    context.restore();

    if bloom_growth > 0.02 {
        context.save();
        context.set_global_alpha((0.2 + bloom_growth * 0.7) * plant_fade.max(0.0));

        for index in 0..plant.flower_count {
            let orbit = index as f64 / plant.flower_count.max(1) as f64;
            let angle = orbit * TAU + plant.bloom_phase * 0.84;
            let stem_x = angle.cos() * leaf_span * (0.14 + orbit * 0.08);
            let stem_y = crown_lift - 1.6 - orbit * 1.2;
            let blossom_x = stem_x + angle.cos() * (3.0 + bloom_growth * 3.2);
            let blossom_y = stem_y - 4.0 - angle.sin() * 2.1;

            context.set_stroke_style(&JsValue::from_str(runner_color));
            context.set_line_width(0.72 + bloom_growth * 0.38);
            context.begin_path();
            context.move_to(stem_x * 0.46, crown_lift * 0.16);
            context.line_to(stem_x, stem_y);
            context.line_to(blossom_x, blossom_y);
            context.stroke();

            context.set_fill_style(&JsValue::from_str(blossom_petal));
            context.begin_path();
            for petal_index in 0..5 {
                let petal_angle = petal_index as f64 / 5.0 * TAU + angle * 0.24;
                context.ellipse(
                    blossom_x + petal_angle.cos() * 1.8,
                    blossom_y + petal_angle.sin() * 1.6,
                    2.0 + bloom_growth * 0.5,
                    1.18 + bloom_growth * 0.24,
                    petal_angle,
                    0.0,
                    TAU,
                )?;
            }
            context.fill();

            context.set_fill_style(&JsValue::from_str(blossom_center));
            context.begin_path();
            context.ellipse(blossom_x, blossom_y, 1.18, 1.02, 0.0, 0.0, TAU)?;
            context.fill();
        }
        context.restore();
    }

    if fruit_growth > 0.02 {
        context.save();
        context.set_global_alpha((0.18 + fruit_growth * 0.78) * plant_fade.max(0.0));

        for index in 0..plant.berry_count {
            let orbit = index as f64 / plant.berry_count.max(1) as f64;
            let angle = orbit * TAU + plant.fruit_phase * 0.72 + 0.34;
            let stem_x = angle.cos() * leaf_span * (0.12 + orbit * 0.1);
            let stem_y = crown_lift - 1.0 - orbit * 0.8;
            let berry_x = stem_x + angle.cos() * (4.0 + fruit_growth * 2.8);
            let berry_y = stem_y + 4.6 + angle.sin() * 1.8;
            let berry_fill = if ripe_growth > 0.38 + orbit * 0.22 {
                berry_red
            } else {
                berry_unripe
            };

            context.set_stroke_style(&JsValue::from_str(runner_color));
            context.set_line_width(0.72);
            context.begin_path();
            context.move_to(stem_x * 0.42, crown_lift * 0.12);
            context.line_to(stem_x, stem_y);
            context.line_to(berry_x, berry_y - 2.2);
            context.stroke();

            context.set_fill_style(&JsValue::from_str(berry_fill));
            context.begin_path();
            context.ellipse(
                berry_x,
                berry_y,
                2.6 + fruit_growth * 0.9,
                3.4 + fruit_growth * 1.1,
                angle * 0.08,
                0.0,
                TAU,
            )?;
            context.fill();

            context.begin_path();
            context.move_to(berry_x, berry_y + 4.0);
            context.line_to(berry_x - 1.9, berry_y + 1.8);
            context.line_to(berry_x + 1.9, berry_y + 1.8);
            context.close_path();
            context.fill();

            context.set_stroke_style(&JsValue::from_str(calyx_color));
            context.set_line_width(0.68);
            context.begin_path();
            for leaf_index in 0..5 {
                let calyx_angle = leaf_index as f64 / 5.0 * TAU + angle * 0.12;
                context.move_to(berry_x, berry_y - 2.8);
                context.line_to(
                    berry_x + calyx_angle.cos() * 2.6,
                    berry_y - 4.4 + calyx_angle.sin() * 1.6,
                );
            }
            context.stroke();

            context.set_fill_style(&JsValue::from_str(berry_seed));
            context.begin_path();
            for seed_index in 0..6 {
                let seed_angle = seed_index as f64 / 6.0 * TAU + orbit * 1.1;
                let seed_radius = 1.2 + (seed_index % 2) as f64 * 0.7;
                context.ellipse(
                    berry_x + seed_angle.cos() * seed_radius,
                    berry_y + seed_angle.sin() * (seed_radius * 1.18),
                    0.26,
                    0.22,
                    seed_angle,
                    0.0,
                    TAU,
                )?;
            }
            context.fill();
        }
        context.restore();
    }

    context.restore();
    Ok(())
}

fn draw_grass_clump(grass: &GrassClump, render: &RenderContext) -> Result<(), JsValue> {
    let screen_x = grass.root.x - render.viewport.x;
    let screen_y = grass.root.y - render.viewport.y;

    if !point_visible_in_viewport(
        grass.root,
        render.viewport,
        DRAW_MARGIN + grass.spread + grass.max_height,
    ) {
        return Ok(());
    }

    let growth = ease_out_cubic(smoothstep(0.0, 0.24, grass.life));
    let health_vigor = plant_vigor(grass.health);
    let leaf_vigor = plant_vigor(grass.leaf_health);
    let plant_fade = health_vigor;

    let base_fill = if render.dark_mode {
        "rgba(66, 126, 46, 0.22)"
    } else {
        "rgba(70, 130, 38, 0.18)"
    };
    let blade_deep = if render.dark_mode {
        "rgba(84, 168, 58, 0.54)"
    } else {
        "rgba(54, 126, 34, 0.52)"
    };
    let blade_mid = if render.dark_mode {
        "rgba(132, 214, 78, 0.62)"
    } else {
        "rgba(90, 164, 42, 0.58)"
    };
    let blade_light = if render.dark_mode {
        "rgba(190, 238, 118, 0.42)"
    } else {
        "rgba(128, 194, 58, 0.42)"
    };

    let context = render.context;
    context.save();
    context.translate(screen_x, screen_y)?;
    context.set_line_cap("round");
    context.set_line_join("round");
    context.set_global_alpha(plant_fade.max(0.0));

    context.save();
    context.set_global_alpha((0.08 + growth * 0.2) * plant_fade.max(0.0));
    context.set_fill_style(&JsValue::from_str(base_fill));
    context.begin_path();
    context.ellipse(
        0.0,
        1.8,
        grass.spread * 0.58,
        3.8 + growth * 2.4,
        0.0,
        0.0,
        TAU,
    )?;
    context.fill();
    context.restore();

    let count = grass.blade_count.max(1);
    context.set_global_alpha((0.18 + growth * 0.68) * plant_fade.max(0.0));
    context.set_line_width(0.66 + growth * 0.22);
    for blade_group in 0..3 {
        context.set_stroke_style(&JsValue::from_str(match blade_group {
            0 => blade_deep,
            1 => blade_mid,
            _ => blade_light,
        }));
        context.begin_path();
        for index in (blade_group..count).step_by(3) {
            let blade_seed = grass.seed + grass.blade_phase + index as f64 * 1.618_033_988_75;
            let orbit = (index as f64 + 0.5) / count as f64;
            let scatter = (blade_seed * 12.989_8).sin();
            let height_noise = (blade_seed * 78.233).cos().abs();
            let root_x = (orbit - 0.5) * grass.spread + scatter * grass.spread * 0.08;
            let root_y = (blade_seed * 0.37).cos().abs() * 2.0;
            let height = grass.max_height
                * (0.38 + height_noise * 0.72)
                * growth
                * (0.64 + leaf_vigor * 0.36);
            let lean = (root_x / grass.spread.max(1.0)) * 0.9 + (blade_seed * 2.11).sin() * 0.58;
            let sway = (render.time * (0.75 + height_noise * 0.42) + grass.sway_phase + blade_seed)
                .sin()
                * (0.55 + height * 0.032);
            let tip_x = root_x + lean * height * 0.34 + sway;
            let tip_y = -height;
            let mid_x = root_x + (tip_x - root_x) * 0.48 + (blade_seed * 1.73).cos() * 2.2;
            let mid_y = -height * (0.4 + (blade_seed * 0.47).sin().abs() * 0.2);

            context.move_to(root_x, root_y);
            context.line_to(mid_x, mid_y);
            context.line_to(tip_x, tip_y);
        }
        context.stroke();
    }

    context.restore();
    Ok(())
}

fn draw_dandelion(dandelion: &Dandelion, render: &RenderContext) -> Result<(), JsValue> {
    let screen_x = dandelion.root.x - render.viewport.x;
    let screen_y = dandelion.root.y - render.viewport.y;

    if !point_visible_in_viewport(
        dandelion.root,
        render.viewport,
        DRAW_MARGIN
            + dandelion.max_height
            + dandelion.bloom_radius * 7.0
            + dandelion.seed_count as f64 * 4.0,
    ) {
        return Ok(());
    }

    let leaf_growth = ease_out_cubic(smoothstep(0.0, 0.34, dandelion.life));
    let stem_growth = ease_out_cubic(smoothstep(0.14, 0.6, dandelion.life));
    let bloom_growth = ease_out_cubic(smoothstep(0.48, 0.74, dandelion.life));
    let bloom_visibility = bloom_growth * (1.0 - smoothstep(0.82, 0.94, dandelion.life));
    let seed_head_growth = ease_out_cubic(smoothstep(0.72, 0.92, dandelion.life));
    let shed_progress = smoothstep(0.88, 1.04, dandelion.life);
    let health_vigor = plant_vigor(dandelion.health);
    let leaf_vigor = plant_vigor(dandelion.leaf_health);
    let sap_vigor = plant_vigor(dandelion.sap_health);
    let plant_fade = health_vigor;

    let stem_height = dandelion.max_height * stem_growth * (0.7 + sap_vigor * 0.3);
    let sway = (render.time * 0.78 + dandelion.sway_phase).sin()
        * (0.7 + stem_height * 0.015 + seed_head_growth * 0.4);
    let stem_tip_x = dandelion.stem_curve * stem_growth * 3.2 + sway;
    let stem_tip_y = -stem_height;
    let stem_mid_x = stem_tip_x * 0.42;
    let stem_mid_y = stem_tip_y * 0.52;
    let leaf_sway = (render.time * 0.62 + dandelion.leaf_phase).sin() * (0.7 + leaf_growth * 0.9);
    let leaf_span = dandelion.leaf_span * (0.24 + leaf_growth * 0.76) * (0.58 + leaf_vigor * 0.42);
    let bloom_radius =
        dandelion.bloom_radius * (0.32 + bloom_growth * 0.68) * (0.7 + sap_vigor * 0.3);
    let seed_radius =
        dandelion.bloom_radius * (0.62 + seed_head_growth * 0.54) * (0.78 + health_vigor * 0.22);

    let stem_color = if render.dark_mode {
        "rgba(180, 210, 116, 0.44)"
    } else {
        "rgba(84, 111, 38, 0.56)"
    };
    let leaf_fill = if render.dark_mode {
        "rgba(194, 227, 124, 0.22)"
    } else {
        "rgba(107, 138, 56, 0.26)"
    };
    let leaf_stroke = if render.dark_mode {
        "rgba(214, 238, 164, 0.34)"
    } else {
        "rgba(88, 124, 37, 0.42)"
    };
    let bud_color = if render.dark_mode {
        "rgba(189, 217, 122, 0.46)"
    } else {
        "rgba(102, 136, 52, 0.5)"
    };
    let petal_color = if render.dark_mode {
        "rgba(255, 214, 98, 0.84)"
    } else {
        "rgba(236, 182, 42, 0.82)"
    };
    let center_color = if render.dark_mode {
        "rgba(255, 192, 75, 0.88)"
    } else {
        "rgba(224, 148, 22, 0.82)"
    };
    let glow_color = if render.dark_mode {
        "rgba(255, 205, 78, 0.14)"
    } else {
        "rgba(230, 171, 42, 0.12)"
    };
    let seed_filament_color = if render.dark_mode {
        "rgba(245, 238, 218, 0.7)"
    } else {
        "rgba(248, 241, 225, 0.76)"
    };
    let seed_core_color = if render.dark_mode {
        "rgba(205, 172, 112, 0.5)"
    } else {
        "rgba(171, 118, 56, 0.52)"
    };
    let seed_kernel_color = if render.dark_mode {
        "rgba(217, 182, 116, 0.82)"
    } else {
        "rgba(146, 102, 46, 0.8)"
    };

    let context = render.context;
    context.save();
    context.translate(screen_x, screen_y)?;
    context.set_line_cap("round");
    context.set_line_join("round");
    context.set_global_alpha(plant_fade.max(0.0));

    context.save();
    context.set_global_alpha((0.18 + leaf_growth * 0.62) * plant_fade.max(0.0));
    context.set_fill_style(&JsValue::from_str(leaf_fill));
    context.set_stroke_style(&JsValue::from_str(leaf_stroke));
    context.set_line_width(0.75 + leaf_growth * 0.55);

    let leaves = [
        (
            -leaf_span * 0.34 - leaf_sway * 0.35,
            -1.6 - leaf_growth * 0.8,
            leaf_span * 0.42,
            0.75 + leaf_growth * 0.75,
            -0.46 - leaf_sway * 0.03,
        ),
        (
            leaf_span * 0.38 + leaf_sway * 0.28,
            -3.0 - leaf_growth * 0.95,
            leaf_span * 0.48,
            0.72 + leaf_growth * 0.72,
            0.34 + leaf_sway * 0.03,
        ),
        (
            -0.4 + leaf_sway * 0.12,
            -4.8 - leaf_growth * 1.35,
            leaf_span * 0.26,
            0.58 + leaf_growth * 0.52,
            -0.08,
        ),
    ];

    for (center_x, center_y, radius_x, radius_y, rotation) in leaves {
        context.begin_path();
        context.ellipse(center_x, center_y, radius_x, radius_y, rotation, 0.0, TAU)?;
        context.fill();
        context.stroke();

        let vein_dir = if center_x < 0.0 { -1.0 } else { 1.0 };
        context.begin_path();
        context.move_to(
            center_x - vein_dir * radius_x * 0.68,
            center_y + radius_y * 0.12,
        );
        context.line_to(
            center_x + vein_dir * radius_x * 0.68,
            center_y - radius_y * 0.08,
        );
        context.stroke();
    }
    context.restore();

    if stem_growth > 0.01 {
        context.save();
        context.set_global_alpha((0.22 + stem_growth * 0.66) * plant_fade.max(0.0));
        context.set_stroke_style(&JsValue::from_str(stem_color));
        context.set_line_width(0.8 + stem_growth * 0.75);
        context.begin_path();
        context.move_to(0.0, 0.0);
        context.line_to(stem_mid_x, stem_mid_y);
        context.line_to(stem_tip_x, stem_tip_y);
        context.stroke();
        context.restore();
    }

    let bloom_x = stem_tip_x;
    let bloom_y = stem_tip_y;
    let bud_visibility = smoothstep(0.34, 0.5, dandelion.life) * (1.0 - bloom_growth * 0.86);

    if bud_visibility > 0.02 {
        context.save();
        context.set_global_alpha(bud_visibility * plant_fade.max(0.0));
        context.set_fill_style(&JsValue::from_str(bud_color));
        context.begin_path();
        context.ellipse(
            bloom_x,
            bloom_y,
            dandelion.bloom_radius * 0.36,
            dandelion.bloom_radius * 0.22,
            stem_tip_x * 0.05,
            0.0,
            TAU,
        )?;
        context.fill();
        context.restore();
    }

    if bloom_visibility > 0.02 {
        context.save();
        context.set_global_alpha(bloom_visibility * plant_fade.max(0.0));
        context.set_shadow_blur(6.0 + bloom_growth * 4.0);
        context.set_shadow_color(glow_color);
        context.set_stroke_style(&JsValue::from_str(petal_color));
        context.set_line_width(0.8 + bloom_growth * 0.3);

        let petal_count = 12 + dandelion.seed_count % 4;
        context.begin_path();
        for index in 0..petal_count {
            let orbit = index as f64 / petal_count as f64;
            let angle = orbit * TAU + render.time * 0.05 + dandelion.sway_phase;
            let petal_length =
                bloom_radius * (0.8 + (orbit * TAU + render.time * 0.2).sin() * 0.08);
            context.move_to(
                bloom_x + angle.cos() * bloom_radius * 0.22,
                bloom_y + angle.sin() * bloom_radius * 0.18,
            );
            context.line_to(
                bloom_x + angle.cos() * petal_length,
                bloom_y + angle.sin() * petal_length * 0.9,
            );
        }
        context.stroke();

        context.set_shadow_blur(0.0);
        context.set_fill_style(&JsValue::from_str(center_color));
        context.begin_path();
        context.ellipse(
            bloom_x,
            bloom_y,
            bloom_radius * 0.42,
            bloom_radius * 0.38,
            0.0,
            0.0,
            TAU,
        )?;
        context.fill();
        context.restore();
    }

    if seed_head_growth > 0.02 {
        context.save();
        context.set_global_alpha(seed_head_growth * plant_fade.max(0.0));
        context.set_stroke_style(&JsValue::from_str(seed_filament_color));
        context.set_fill_style(&JsValue::from_str(seed_kernel_color));
        context.set_line_width(0.65 + seed_head_growth * 0.18);

        let attached_count = ((1.0 - shed_progress) * dandelion.seed_count as f64)
            .ceil()
            .clamp(0.0, dandelion.seed_count as f64) as usize;
        for index in 0..attached_count {
            let orbit = index as f64 / dandelion.seed_count as f64;
            let angle = orbit * TAU + dandelion.seed_phase + render.time * 0.04;
            let filament_length =
                seed_radius * (0.74 + (render.time * 0.28 + orbit * TAU).sin() * 0.05);
            draw_dandelion_seed(
                context,
                bloom_x,
                bloom_y,
                angle,
                filament_length,
                1.35 + seed_head_growth * 0.7,
            )?;
        }

        let released_count = (shed_progress * dandelion.seed_count as f64)
            .floor()
            .clamp(0.0, dandelion.seed_count as f64) as usize;
        context.set_global_alpha((0.24 + shed_progress * 0.58) * plant_fade.max(0.0));
        for index in 0..released_count {
            let orbit = index as f64 / dandelion.seed_count as f64;
            let drift_angle = -0.34 + (orbit * TAU + dandelion.seed_phase * 0.7).sin() * 0.22;
            let drift = 7.0 + shed_progress * 24.0 + orbit * 10.0;
            let rise = 3.0 + shed_progress * 13.0 + index as f64 * 0.55;
            let flutter = (render.time * 0.82 + orbit * 8.0 + dandelion.seed_phase).sin() * 2.1;
            let anchor_x = bloom_x + drift + flutter;
            let anchor_y = bloom_y - rise + (render.time * 0.46 + orbit * 6.0).cos() * 1.8;
            draw_dandelion_seed(
                context,
                anchor_x,
                anchor_y,
                drift_angle,
                2.1 + seed_head_growth * 1.2,
                1.3 + shed_progress * 0.8,
            )?;
        }

        context.set_global_alpha((0.18 + seed_head_growth * 0.3) * plant_fade.max(0.0));
        context.set_fill_style(&JsValue::from_str(seed_core_color));
        context.begin_path();
        context.ellipse(
            bloom_x,
            bloom_y,
            seed_radius * 0.2,
            seed_radius * 0.18,
            0.0,
            0.0,
            TAU,
        )?;
        context.fill();
        context.restore();
    }

    context.restore();
    Ok(())
}

fn draw_dandelion_seed(
    context: &CanvasRenderingContext2d,
    anchor_x: f64,
    anchor_y: f64,
    angle: f64,
    filament_length: f64,
    tuft_size: f64,
) -> Result<(), JsValue> {
    let kernel_x = anchor_x + angle.cos() * filament_length;
    let kernel_y = anchor_y + angle.sin() * filament_length;
    let tuft_x = kernel_x + angle.cos() * (tuft_size * 0.68);
    let tuft_y = kernel_y + angle.sin() * (tuft_size * 0.68);

    context.begin_path();
    context.move_to(anchor_x, anchor_y);
    context.line_to(kernel_x, kernel_y);
    for index in 0..4 {
        let spread = -0.78 + index as f64 * 0.52;
        let spoke_angle = angle + std::f64::consts::PI + spread;
        context.move_to(tuft_x, tuft_y);
        context.line_to(
            tuft_x + spoke_angle.cos() * tuft_size,
            tuft_y + spoke_angle.sin() * tuft_size,
        );
    }
    context.stroke();

    context.begin_path();
    context.ellipse(kernel_x, kernel_y, 0.62, 0.46, angle, 0.0, TAU)?;
    context.fill();

    Ok(())
}

fn dandelion_released_seed_count(life: f64, seed_count: usize) -> usize {
    (smoothstep(0.88, 1.04, life) * seed_count as f64)
        .floor()
        .clamp(0.0, seed_count as f64) as usize
}

fn strawberry_released_runner_count(life: f64, runner_count: usize) -> usize {
    (smoothstep(0.76, 1.02, life) * runner_count as f64)
        .floor()
        .clamp(0.0, runner_count as f64) as usize
}

fn dandelion_seed_landing_point(dandelion: &Dandelion, index: usize) -> Point {
    let orbit = index as f64 / dandelion.seed_count.max(1) as f64;
    let drift_angle = -0.34 + (orbit * TAU + dandelion.seed_phase * 0.7).sin() * 0.22;
    let travel = 16.0
        + orbit * 42.0
        + index as f64 * 3.8
        + (orbit * TAU + dandelion.seed_phase * 1.2).cos() * 4.5;
    let crosswind = (orbit * TAU + dandelion.leaf_phase).sin() * 18.0
        + (orbit * TAU * 2.0 + dandelion.sway_phase).cos() * 6.0;

    Point {
        x: dandelion.root.x + travel + drift_angle.cos() * 8.0,
        y: dandelion.root.y + crosswind + drift_angle.sin() * 10.0,
    }
}

fn strawberry_runner_offset(plant: &StrawberryPlant, index: usize) -> Point {
    let orbit = index as f64 / plant.runner_count.max(1) as f64;
    let angle = orbit * TAU + plant.runner_phase * 0.82 - 0.9;
    let travel = 18.0
        + orbit * 22.0
        + (orbit * TAU + plant.runner_phase).sin() * 4.4
        + plant.leaf_span * 0.6;
    let lateral = (orbit * TAU * 1.6 + plant.leaf_phase).sin() * 5.4;

    Point {
        x: angle.cos() * travel + lateral * 0.3,
        y: angle.sin() * travel * 0.54 + lateral + 8.0,
    }
}

fn strawberry_runner_landing_point(plant: &StrawberryPlant, index: usize) -> Point {
    let offset = strawberry_runner_offset(plant, index);
    Point {
        x: plant.root.x + offset.x,
        y: plant.root.y + offset.y,
    }
}

fn nearest_flora_target(
    origin: Point,
    flora_targets: &[PlantTarget],
    max_distance: f64,
) -> Option<Point> {
    let mut nearest = None;
    let mut nearest_score = f64::INFINITY;

    for target in flora_targets {
        let distance = origin.distance_to(target.position);
        if distance > max_distance {
            continue;
        }

        let score = distance / plant_target_quality(target.quality);
        if score < nearest_score {
            nearest = Some(target.position);
            nearest_score = score;
        }
    }

    nearest
}

fn nearest_preferred_target(
    origin: Point,
    preferred_targets: &[PlantTarget],
    fallback_targets: &[PlantTarget],
    preferred_max_distance: f64,
    fallback_max_distance: f64,
) -> Option<Point> {
    nearest_flora_target(origin, preferred_targets, preferred_max_distance)
        .or_else(|| nearest_flora_target(origin, fallback_targets, fallback_max_distance))
}

fn nearest_preferred_visible_target(
    origin: Point,
    preferred_targets: &[PlantTarget],
    fallback_targets: &[PlantTarget],
    preferred_max_distance: f64,
    fallback_max_distance: f64,
    cover_regions: &[CoverRegion],
    clearance: f64,
) -> Option<Point> {
    nearest_visible_plant_target(
        origin,
        preferred_targets,
        preferred_max_distance,
        cover_regions,
        clearance,
    )
    .or_else(|| {
        nearest_visible_plant_target(
            origin,
            fallback_targets,
            fallback_max_distance,
            cover_regions,
            clearance,
        )
    })
}

fn nearest_visible_plant_target(
    origin: Point,
    targets: &[PlantTarget],
    max_distance: f64,
    cover_regions: &[CoverRegion],
    clearance: f64,
) -> Option<Point> {
    let mut nearest = None;
    let mut nearest_score = f64::INFINITY;

    for target in targets {
        let distance = origin.distance_to(target.position);
        if distance > max_distance {
            continue;
        }
        if !visible_target_between(origin, target.position, cover_regions, clearance) {
            continue;
        }

        let score = distance / plant_target_quality(target.quality);
        if score < nearest_score {
            nearest = Some(target.position);
            nearest_score = score;
        }
    }

    nearest
}

fn nearest_visible_target(
    origin: Point,
    targets: &[Point],
    max_distance: f64,
    cover_regions: &[CoverRegion],
    clearance: f64,
) -> Option<Point> {
    let mut nearest = None;
    let mut nearest_distance = max_distance;

    for target in targets {
        let distance = origin.distance_to(*target);
        if distance >= nearest_distance {
            continue;
        }
        if !visible_target_between(origin, *target, cover_regions, clearance) {
            continue;
        }

        nearest = Some(*target);
        nearest_distance = distance;
    }

    nearest
}

fn tracked_visible_target(
    origin: Point,
    remembered: Option<Point>,
    targets: &[Point],
    max_target_drift: f64,
    max_origin_distance: f64,
    cover_regions: &[CoverRegion],
    clearance: f64,
) -> Option<Point> {
    let remembered = remembered?;
    let mut tracked = None;
    let mut best_score = f64::INFINITY;

    for target in targets {
        let target_drift = remembered.distance_to(*target);
        if target_drift > max_target_drift {
            continue;
        }

        let origin_distance = origin.distance_to(*target);
        if origin_distance > max_origin_distance {
            continue;
        }

        if !visible_target_between(origin, *target, cover_regions, clearance) {
            continue;
        }

        let score = target_drift * 1.4 + origin_distance * 0.08;
        if score < best_score {
            tracked = Some(*target);
            best_score = score;
        }
    }

    tracked
}

fn target_switched(previous: Option<Point>, current: Option<Point>, tolerance: f64) -> bool {
    match (previous, current) {
        (None, Some(_)) => true,
        (Some(previous), Some(current)) => previous.distance_to(current) > tolerance,
        _ => false,
    }
}

fn visible_target_between(
    origin: Point,
    target: Point,
    cover_regions: &[CoverRegion],
    clearance: f64,
) -> bool {
    !point_covered_by_ui(target, cover_regions, clearance)
        && !cover_blocks_between(origin, target, cover_regions, clearance)
}

fn point_covered_by_ui(point: Point, cover_regions: &[CoverRegion], clearance: f64) -> bool {
    cover_regions.iter().any(|region| {
        region.kind.blocks_visibility() && region.rect.expanded(clearance).contains(point)
    })
}

fn cover_blocks_between(
    origin: Point,
    target: Point,
    cover_regions: &[CoverRegion],
    clearance: f64,
) -> bool {
    let distance = origin.distance_to(target);
    if distance <= 0.001 {
        return false;
    }

    let steps = ((distance / 18.0).ceil() as usize).clamp(2, 36);
    for step in 1..steps {
        let t = step as f64 / steps as f64;
        let sample = lerp_point(origin, target, t);
        if point_covered_by_ui(sample, cover_regions, clearance) {
            return true;
        }
    }

    false
}

fn combined_plant_health(leaf_health: f64, sap_health: f64) -> f64 {
    (leaf_health.clamp(0.0, 1.0) * 0.58 + sap_health.clamp(0.0, 1.0) * 0.42).clamp(0.0, 1.0)
}

fn plant_target_quality(quality: f64) -> f64 {
    quality.clamp(0.08, 1.8)
}

fn plant_vigor(health: f64) -> f64 {
    0.38 + health.clamp(0.0, 1.0) * 0.62
}

fn dandelion_leaf_food_quality(plant: &Dandelion) -> f64 {
    let leaf_stage =
        smoothstep(0.04, 0.42, plant.life) * (1.0 - smoothstep(0.98, 1.14, plant.life));
    let bloom_bonus = smoothstep(0.46, 0.78, plant.life) * 0.16;
    0.12 + plant.leaf_health * (0.88 * leaf_stage + bloom_bonus) + plant.health * 0.16
}

fn dandelion_aphid_host_quality(plant: &Dandelion) -> f64 {
    let soft_growth =
        smoothstep(0.02, 0.48, plant.life) * (1.0 - smoothstep(0.92, 1.14, plant.life));
    0.08 + plant.sap_health * 0.52 * soft_growth + plant.health * 0.1
}

fn strawberry_aphid_host_quality(plant: &StrawberryPlant) -> f64 {
    let soft_growth =
        smoothstep(0.03, 0.44, plant.life) * (1.0 - smoothstep(1.02, 1.2, plant.life));
    let flower_fruit_bonus = smoothstep(0.28, 0.72, plant.life) * 0.22;
    0.16 + plant.sap_health * (0.82 * soft_growth + flower_fruit_bonus) + plant.health * 0.12
}

fn strawberry_leaf_food_quality(plant: &StrawberryPlant) -> f64 {
    let leaf_stage = smoothstep(0.05, 0.5, plant.life) * (1.0 - smoothstep(1.0, 1.18, plant.life));
    0.1 + plant.leaf_health * 0.56 * leaf_stage + plant.health * 0.1
}

fn grass_aphid_host_quality(grass: &GrassClump) -> f64 {
    let soft_growth = smoothstep(0.02, 0.34, grass.life);
    0.06 + grass.sap_health * 0.34 * soft_growth + grass.health * 0.08
}

fn grass_leaf_food_quality(grass: &GrassClump) -> f64 {
    let leaf_stage = smoothstep(0.02, 0.42, grass.life);
    0.08 + grass.leaf_health * 0.44 * leaf_stage + grass.health * 0.1
}

fn ease_out_cubic(value: f64) -> f64 {
    let clamped = value.clamp(0.0, 1.0);
    1.0 - (1.0 - clamped).powi(3)
}

fn smoothstep(start: f64, end: f64, value: f64) -> f64 {
    if (end - start).abs() < f64::EPSILON {
        return (value >= end) as i32 as f64;
    }

    let t = ((value - start) / (end - start)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn smooth_random_walk(seed: f64, value: f64) -> f64 {
    let start = value.floor();
    let blend = smoothstep(start, start + 1.0, value);
    let from = signed_unit_noise(seed + start * 37.371);
    let to = signed_unit_noise(seed + (start + 1.0) * 37.371);
    from + (to - from) * blend
}

fn signed_unit_noise(value: f64) -> f64 {
    let noise = (value * 12.989_8).sin() * 43_758.545_3;
    (noise - noise.floor()) * 2.0 - 1.0
}

fn shortest_angle(current: f64, target: f64) -> f64 {
    let mut delta = target - current;

    while delta > std::f64::consts::PI {
        delta -= TAU;
    }

    while delta < -std::f64::consts::PI {
        delta += TAU;
    }

    delta
}
