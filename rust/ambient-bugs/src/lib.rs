#![allow(deprecated)]

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const TAU: f64 = std::f64::consts::PI * 2.0;

#[wasm_bindgen]
pub struct BugField {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    viewport_width: f64,
    viewport_height: f64,
    page_width: f64,
    page_height: f64,
    viewport_x: f64,
    viewport_y: f64,
    dpr: f64,
    bugs: Vec<Bug>,
    obstacles: Vec<Rect>,
    rng: Lcg,
    last_time: Option<f64>,
    dark_mode: bool,
}

#[derive(Clone, Copy)]
struct Bug {
    x: f64,
    y: f64,
    heading: f64,
    base_speed: f64,
    size: f64,
    stride_phase: f64,
    wander_phase: f64,
}

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

struct Lcg {
    state: u64,
}

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
            viewport_width: 1.0,
            viewport_height: 1.0,
            page_width: 1.0,
            page_height: 1.0,
            viewport_x: 0.0,
            viewport_y: 0.0,
            dpr: 1.0,
            bugs: Vec::new(),
            obstacles: Vec::new(),
            rng: Lcg::new(seed),
            last_time: None,
            dark_mode: false,
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
        self.viewport_width = viewport_width.max(1.0);
        self.viewport_height = viewport_height.max(1.0);
        self.page_width = page_width.max(self.viewport_width);
        self.page_height = page_height.max(self.viewport_height);
        self.dpr = dpr.clamp(1.0, 2.0);
        self.clamp_viewport();

        self.canvas
            .set_width((self.viewport_width * self.dpr).round().max(1.0) as u32);
        self.canvas
            .set_height((self.viewport_height * self.dpr).round().max(1.0) as u32);

        self.context
            .set_transform(self.dpr, 0.0, 0.0, self.dpr, 0.0, 0.0)?;
        self.sync_bug_count();
        Ok(())
    }

    pub fn set_viewport(&mut self, viewport_x: f64, viewport_y: f64) {
        self.viewport_x = viewport_x.max(0.0);
        self.viewport_y = viewport_y.max(0.0);
        self.clamp_viewport();
    }

    pub fn set_dark_mode(&mut self, dark_mode: bool) {
        self.dark_mode = dark_mode;
    }

    pub fn set_obstacles(&mut self, rects: Vec<f32>) {
        self.obstacles.clear();

        for rect in rects.chunks_exact(4) {
            let width = rect[2] as f64;
            let height = rect[3] as f64;

            if width <= 0.0 || height <= 0.0 {
                continue;
            }

            self.obstacles.push(Rect {
                x: rect[0] as f64,
                y: rect[1] as f64,
                width,
                height,
            });
        }
    }

    pub fn frame(&mut self, timestamp: f64) -> Result<(), JsValue> {
        let dt = if let Some(last_time) = self.last_time {
            ((timestamp - last_time) / 16.666_7).clamp(0.45, 1.8)
        } else {
            1.0
        };
        self.last_time = Some(timestamp);

        self.context
            .clear_rect(0.0, 0.0, self.viewport_width, self.viewport_height);

        let time_seconds = timestamp * 0.001;
        for index in 0..self.bugs.len() {
            self.update_bug(index, dt, time_seconds);
        }

        for bug in &self.bugs {
            self.draw_bug(bug, time_seconds)?;
        }

        Ok(())
    }
}

impl BugField {
    fn sync_bug_count(&mut self) {
        let desired = ((self.page_width * self.page_height) / 130_000.0)
            .round()
            .clamp(12.0, 56.0) as usize;

        match self.bugs.len().cmp(&desired) {
            std::cmp::Ordering::Less => {
                let missing = desired - self.bugs.len();
                for _ in 0..missing {
                    let bug = self.spawn_bug(true);
                    self.bugs.push(bug);
                }
            }
            std::cmp::Ordering::Greater => self.bugs.truncate(desired),
            std::cmp::Ordering::Equal => {}
        }

        for index in 0..self.bugs.len() {
            if self.bugs[index].x > self.page_width + 80.0
                || self.bugs[index].y > self.page_height + 80.0
            {
                self.bugs[index] = self.spawn_bug(false);
            }
        }
    }

    fn clamp_viewport(&mut self) {
        self.viewport_x = self
            .viewport_x
            .clamp(0.0, (self.page_width - self.viewport_width).max(0.0));
        self.viewport_y = self
            .viewport_y
            .clamp(0.0, (self.page_height - self.viewport_height).max(0.0));
    }

    fn spawn_bug(&mut self, edge_only: bool) -> Bug {
        let size = self.rng.range(5.0, 9.2);
        let heading = self.rng.range(0.0, TAU);
        let base_speed = self.rng.range(0.42, 0.92);
        let stride_phase = self.rng.range(0.0, TAU);
        let wander_phase = self.rng.range(0.0, TAU);

        let (x, y) = if edge_only {
            match (self.rng.next_f64() * 4.0).floor() as i32 {
                0 => (-20.0, self.rng.range(0.0, self.page_height)),
                1 => (
                    self.page_width + 20.0,
                    self.rng.range(0.0, self.page_height),
                ),
                2 => (self.rng.range(0.0, self.page_width), -20.0),
                _ => (
                    self.rng.range(0.0, self.page_width),
                    self.page_height + 20.0,
                ),
            }
        } else {
            (
                self.rng.range(-16.0, self.page_width + 16.0),
                self.rng.range(-16.0, self.page_height + 16.0),
            )
        };

        Bug {
            x,
            y,
            heading,
            base_speed,
            size,
            stride_phase,
            wander_phase,
        }
    }

    fn update_bug(&mut self, index: usize, dt: f64, time: f64) {
        let mut bug = self.bugs[index];
        let mut avoid_x = 0.0;
        let mut avoid_y = 0.0;

        for obstacle in &self.obstacles {
            let expanded = Rect {
                x: obstacle.x - 22.0,
                y: obstacle.y - 22.0,
                width: obstacle.width + 44.0,
                height: obstacle.height + 44.0,
            };

            if bug.x < expanded.x
                || bug.x > expanded.x + expanded.width
                || bug.y < expanded.y
                || bug.y > expanded.y + expanded.height
            {
                continue;
            }

            let nearest_x = bug.x.clamp(expanded.x, expanded.x + expanded.width);
            let nearest_y = bug.y.clamp(expanded.y, expanded.y + expanded.height);
            let mut dx = bug.x - nearest_x;
            let mut dy = bug.y - nearest_y;

            if dx.abs() < 0.001 && dy.abs() < 0.001 {
                dx = bug.x - (expanded.x + expanded.width * 0.5);
                dy = bug.y - (expanded.y + expanded.height * 0.5);
            }

            let distance = (dx * dx + dy * dy).sqrt().max(0.001);
            let influence = (34.0 + bug.size * 2.0).max(distance);
            let strength = 1.0 - (distance / influence).min(1.0);

            avoid_x += dx / distance * strength * 1.65;
            avoid_y += dy / distance * strength * 1.65;
        }

        let edge_margin = 34.0;
        if bug.x < edge_margin {
            avoid_x += (edge_margin - bug.x) / edge_margin;
        }
        if bug.x > self.page_width - edge_margin {
            avoid_x -= (bug.x - (self.page_width - edge_margin)) / edge_margin;
        }
        if bug.y < edge_margin {
            avoid_y += (edge_margin - bug.y) / edge_margin;
        }
        if bug.y > self.page_height - edge_margin {
            avoid_y -= (bug.y - (self.page_height - edge_margin)) / edge_margin;
        }

        let wander = (time * (0.8 + bug.base_speed * 0.6) + bug.wander_phase).sin() * 0.45
            + (time * 0.43 + bug.stride_phase).cos() * 0.18;

        let desired_heading =
            (bug.heading.sin() + avoid_y).atan2(bug.heading.cos() + avoid_x) + wander * 0.16;
        let delta = shortest_angle(bug.heading, desired_heading);
        bug.heading += delta * 0.12 * dt;

        let gait = (time * 7.5 + bug.stride_phase).sin().abs();
        let speed =
            bug.base_speed * (0.82 + gait * 0.28 + avoid_x.abs() * 0.09 + avoid_y.abs() * 0.09);
        bug.x += bug.heading.cos() * speed * dt * 3.1;
        bug.y += bug.heading.sin() * speed * dt * 3.1;

        if bug.x < -64.0
            || bug.x > self.page_width + 64.0
            || bug.y < -64.0
            || bug.y > self.page_height + 64.0
        {
            bug = self.spawn_bug(true);
        }

        self.bugs[index] = bug;
    }

    fn draw_bug(&self, bug: &Bug, time: f64) -> Result<(), JsValue> {
        let screen_x = bug.x - self.viewport_x;
        let screen_y = bug.y - self.viewport_y;
        let margin = 48.0;

        if screen_x < -margin
            || screen_x > self.viewport_width + margin
            || screen_y < -margin
            || screen_y > self.viewport_height + margin
        {
            return Ok(());
        }

        let ctx = &self.context;
        let leg_wave = (time * 9.0 + bug.stride_phase).sin();

        let shell_fill = if self.dark_mode {
            "rgba(255, 196, 122, 0.72)"
        } else {
            "rgba(10, 24, 57, 0.62)"
        };
        let shell_stroke = if self.dark_mode {
            "rgba(255, 241, 225, 0.34)"
        } else {
            "rgba(255, 215, 172, 0.28)"
        };
        let limb_stroke = if self.dark_mode {
            "rgba(255, 208, 146, 0.26)"
        } else {
            "rgba(10, 24, 57, 0.28)"
        };
        let highlight_fill = if self.dark_mode {
            "rgba(255, 247, 237, 0.20)"
        } else {
            "rgba(255, 207, 142, 0.34)"
        };
        let glow = if self.dark_mode {
            "rgba(255, 145, 32, 0.18)"
        } else {
            "rgba(255, 120, 0, 0.12)"
        };

        ctx.save();
        ctx.translate(screen_x, screen_y)?;
        ctx.rotate(bug.heading)?;
        ctx.set_line_cap("round");
        ctx.set_line_join("round");
        ctx.set_line_width(1.2);
        ctx.set_stroke_style(&JsValue::from_str(limb_stroke));

        for index in 0..3 {
            let offset = -bug.size * 0.46 + index as f64 * bug.size * 0.46;
            let sweep = leg_wave * (0.18 + index as f64 * 0.04) * bug.size;
            let reach = bug.size * (0.86 - index as f64 * 0.08);

            ctx.begin_path();
            ctx.move_to(-bug.size * 0.1, offset);
            ctx.line_to(-reach, offset - sweep);
            ctx.stroke();

            ctx.begin_path();
            ctx.move_to(-bug.size * 0.1, offset);
            ctx.line_to(reach * 0.4, offset + sweep);
            ctx.stroke();
        }

        ctx.begin_path();
        ctx.move_to(bug.size * 0.58, -bug.size * 0.12);
        ctx.line_to(bug.size * 0.92, -bug.size * 0.42);
        ctx.stroke();
        ctx.begin_path();
        ctx.move_to(bug.size * 0.58, bug.size * 0.12);
        ctx.line_to(bug.size * 0.92, bug.size * 0.42);
        ctx.stroke();

        ctx.set_shadow_blur(10.0);
        ctx.set_shadow_color(glow);
        ctx.set_fill_style(&JsValue::from_str(shell_fill));
        ctx.set_stroke_style(&JsValue::from_str(shell_stroke));
        ctx.set_line_width(0.8);

        ctx.begin_path();
        ctx.ellipse(
            -bug.size * 0.1,
            0.0,
            bug.size * 0.62,
            bug.size * 0.44,
            0.0,
            0.0,
            TAU,
        )?;
        ctx.fill();
        ctx.stroke();

        ctx.begin_path();
        ctx.ellipse(
            bug.size * 0.36,
            0.0,
            bug.size * 0.34,
            bug.size * 0.28,
            0.0,
            0.0,
            TAU,
        )?;
        ctx.fill();
        ctx.stroke();

        ctx.begin_path();
        ctx.ellipse(
            bug.size * 0.76,
            0.0,
            bug.size * 0.18,
            bug.size * 0.16,
            0.0,
            0.0,
            TAU,
        )?;
        ctx.fill();
        ctx.stroke();

        ctx.set_shadow_blur(0.0);
        ctx.set_fill_style(&JsValue::from_str(highlight_fill));
        ctx.begin_path();
        ctx.ellipse(
            -bug.size * 0.24,
            -bug.size * 0.12,
            bug.size * 0.22,
            bug.size * 0.12,
            -0.2,
            0.0,
            TAU,
        )?;
        ctx.fill();
        ctx.restore();
        Ok(())
    }
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
