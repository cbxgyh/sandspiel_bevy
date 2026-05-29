use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::collections::VecDeque;

const EMPTY_CELL: Cell = Cell {
    species: Species::Empty,
    ra: 0,
    rb: 0,
    clock: 0,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Species {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Gas = 4,
    Cloner = 5,
    Fire = 6,
    Wood = 7,
    Lava = 8,
    Ice = 9,
    Plant = 11,
    Acid = 12,
    Stone = 13,
    Dust = 14,
    Mite = 15,
    Oil = 16,
    Rocket = 17,
    Fungus = 18,
    Seed = 19,
}

pub const ALL_SPECIES: [Species; 19] = [
    Species::Empty,
    Species::Wall,
    Species::Sand,
    Species::Water,
    Species::Gas,
    Species::Cloner,
    Species::Fire,
    Species::Wood,
    Species::Lava,
    Species::Ice,
    Species::Plant,
    Species::Acid,
    Species::Stone,
    Species::Dust,
    Species::Mite,
    Species::Oil,
    Species::Rocket,
    Species::Fungus,
    Species::Seed,
];

impl Species {
    pub fn label(self) -> &'static str {
        match self {
            Species::Empty => "Empty",
            Species::Wall => "Wall",
            Species::Sand => "Sand",
            Species::Water => "Water",
            Species::Gas => "Gas",
            Species::Cloner => "Cloner",
            Species::Fire => "Fire",
            Species::Wood => "Wood",
            Species::Lava => "Lava",
            Species::Ice => "Ice",
            Species::Plant => "Plant",
            Species::Acid => "Acid",
            Species::Stone => "Stone",
            Species::Dust => "Dust",
            Species::Mite => "Mite",
            Species::Oil => "Oil",
            Species::Rocket => "Rocket",
            Species::Fungus => "Fungus",
            Species::Seed => "Seed",
        }
    }

    pub fn from_raw(raw: u8) -> Option<Self> {
        Some(match raw {
            0 => Species::Empty,
            1 => Species::Wall,
            2 => Species::Sand,
            3 => Species::Water,
            4 => Species::Gas,
            5 => Species::Cloner,
            6 => Species::Fire,
            7 => Species::Wood,
            8 => Species::Lava,
            9 => Species::Ice,
            11 => Species::Plant,
            12 => Species::Acid,
            13 => Species::Stone,
            14 => Species::Dust,
            15 => Species::Mite,
            16 => Species::Oil,
            17 => Species::Rocket,
            18 => Species::Fungus,
            19 => Species::Seed,
            _ => return None,
        })
    }

    fn update(self, cell: Cell, api: SandApi<'_>) {
        match self {
            Species::Empty | Species::Wall => {}
            Species::Sand => update_sand(cell, api),
            Species::Dust => update_dust(cell, api),
            Species::Water => update_water(cell, api),
            Species::Stone => update_stone(cell, api),
            Species::Gas => update_gas(cell, api),
            Species::Cloner => update_cloner(cell, api),
            Species::Rocket => update_rocket(cell, api),
            Species::Fire => update_fire(cell, api),
            Species::Wood => update_wood(cell, api),
            Species::Lava => update_lava(cell, api),
            Species::Ice => update_ice(cell, api),
            Species::Plant => update_plant(cell, api),
            Species::Acid => update_acid(cell, api),
            Species::Mite => update_mite(cell, api),
            Species::Oil => update_oil(cell, api),
            Species::Fungus => update_fungus(cell, api),
            Species::Seed => update_seed(cell, api),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wind {
    pub dx: u8,
    pub dy: u8,
    pub pressure: u8,
    pub density: u8,
}

impl Default for Wind {
    fn default() -> Self {
        Self {
            dx: 126,
            dy: 126,
            pressure: 0,
            density: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct WindState {
    vx: f32,
    vy: f32,
    pressure: f32,
    density: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub species: Species,
    pub ra: u8,
    pub rb: u8,
    pub clock: u8,
}

impl Cell {
    fn with_random(species: Species, generation: u8, rng: &mut SmallRng) -> Self {
        Self {
            species,
            ra: 100 + (rng.r#gen::<f32>() * 50.0) as u8,
            rb: 0,
            clock: generation,
        }
    }
}

pub struct Universe {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    undo_stack: VecDeque<Vec<Cell>>,
    winds: Vec<Wind>,
    burns: Vec<Wind>,
    wind_state: Vec<WindState>,
    wind_scratch: Vec<WindState>,
    generation: u8,
    rng: SmallRng,
}

pub struct SandApi<'a> {
    x: i32,
    y: i32,
    universe: &'a mut Universe,
}

impl<'a> SandApi<'a> {
    pub fn get(&mut self, dx: i32, dy: i32) -> Cell {
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx >= self.universe.width || ny < 0 || ny >= self.universe.height {
            return Cell {
                species: Species::Wall,
                ra: 0,
                rb: 0,
                clock: self.universe.generation,
            };
        }
        self.universe.get_cell(nx, ny)
    }

    pub fn set(&mut self, dx: i32, dy: i32, value: Cell) {
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx >= self.universe.width || ny < 0 || ny >= self.universe.height {
            return;
        }
        let index = self.universe.get_index(nx, ny);
        self.universe.cells[index] = value;
        self.universe.cells[index].clock = self.universe.generation.wrapping_add(1);
    }

    pub fn get_fluid(&mut self) -> Wind {
        let index = self.universe.get_index(self.x, self.y);
        self.universe.winds[index]
    }

    pub fn set_fluid(&mut self, value: Wind) {
        let index = self.universe.get_index(self.x, self.y);
        self.universe.burns[index] = value;
    }

    pub fn rand_int(&mut self, n: i32) -> i32 {
        self.universe.rng.gen_range(0..n)
    }

    pub fn once_in(&mut self, n: i32) -> bool {
        self.rand_int(n) == 0
    }

    pub fn rand_float(&mut self) -> f32 {
        self.universe.rng.r#gen::<f32>()
    }

    pub fn rand_dir(&mut self) -> i32 {
        (self.rand_int(1000) % 3) - 1
    }

    pub fn rand_dir_2(&mut self) -> i32 {
        if self.rand_int(1000) % 2 == 0 { -1 } else { 1 }
    }

    pub fn rand_vec(&mut self) -> (i32, i32) {
        match self.rand_int(2000) % 9 {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            7 => (0, 1),
            _ => (0, 0),
        }
    }

    pub fn rand_vec_8(&mut self) -> (i32, i32) {
        match self.rand_int(8) {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            _ => (0, 1),
        }
    }

    pub fn new_cell(&mut self, species: Species) -> Cell {
        Cell::with_random(species, self.universe.generation, &mut self.universe.rng)
    }
}

impl Universe {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            cells: vec![EMPTY_CELL; size],
            undo_stack: VecDeque::with_capacity(50),
            winds: vec![Wind::default(); size],
            burns: vec![Wind::default(); size],
            wind_state: vec![WindState::default(); size],
            wind_scratch: vec![WindState::default(); size],
            generation: 0,
            rng: SmallRng::seed_from_u64(0x734f_6b89_de5f_83cc),
        }
    }

    pub fn reset(&mut self) {
        self.cells.fill(EMPTY_CELL);
        self.undo_stack.clear();
        self.reset_fluids();
        self.generation = 0;
    }

    pub fn push_undo(&mut self) {
        self.undo_stack.push_front(self.cells.clone());
        self.undo_stack.truncate(50);
    }

    pub fn pop_undo(&mut self) {
        if let Some(state) = self.undo_stack.pop_front() {
            self.cells = state;
        }
    }

    pub fn tick(&mut self) {
        self.update_wind_field();

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x, y);
                let wind = self.get_wind(x, y);
                Self::blow_wind(
                    cell,
                    wind,
                    SandApi {
                        universe: self,
                        x,
                        y,
                    },
                );
            }
        }

        self.generation = self.generation.wrapping_add(1);

        for x in 0..self.width {
            let scan_x = if self.generation % 2 == 0 {
                self.width - (1 + x)
            } else {
                x
            };

            for y in 0..self.height {
                let idx = self.get_index(scan_x, y);
                let cell = self.get_cell(scan_x, y);
                self.burns[idx] = Wind::default();

                Self::update_cell(
                    cell,
                    SandApi {
                        universe: self,
                        x: scan_x,
                        y,
                    },
                );
            }
        }

        self.generation = self.generation.wrapping_add(1);
        self.apply_burn_sources();
    }

    pub fn paint(&mut self, x: i32, y: i32, size: i32, species: Species) {
        let radius = size as f32 / 2.0;
        let floor = (radius + 1.0) as i32;
        let ceil = (radius + 1.5) as i32;

        for dx in -floor..ceil {
            for dy in -floor..ceil {
                if (dx * dx + dy * dy) as f32 > radius * radius {
                    continue;
                }

                let px = x + dx;
                let py = y + dy;
                if px < 0 || px >= self.width || py < 0 || py >= self.height {
                    continue;
                }

                if self.get_cell(px, py).species == Species::Empty || species == Species::Empty {
                    let index = self.get_index(px, py);
                    self.cells[index] = Cell {
                        species,
                        ra: 60
                            + size as u8
                            + (self.rng.r#gen::<f32>() * 30.0) as u8
                            + ((self.generation % 127) as i8 - 60).unsigned_abs(),
                        rb: 0,
                        clock: self.generation,
                    };
                }
            }
        }
    }

    pub fn apply_wind_brush(&mut self, x: i32, y: i32, size: i32, dx: f32, dy: f32) {
        let radius = size.max(2) as f32;
        let pressure = (dx.abs() + dy.abs()) * 0.65;

        for ox in -(size + 1)..=(size + 1) {
            for oy in -(size + 1)..=(size + 1) {
                let px = x + ox;
                let py = y + oy;
                if px < 0 || px >= self.width || py < 0 || py >= self.height {
                    continue;
                }

                let distance = ((ox * ox + oy * oy) as f32).sqrt();
                if distance > radius {
                    continue;
                }

                let falloff = 1.0 - (distance / radius).min(1.0);
                let index = self.get_index(px, py);
                let state = &mut self.wind_state[index];
                state.vx += dx * falloff * 0.24;
                state.vy += dy * falloff * 0.24;
                state.pressure = (state.pressure + pressure * falloff * 0.35).min(255.0);
                state.density = (state.density + 12.0 * falloff).min(255.0);
            }
        }
    }

    pub fn write_cell_texture(&self, target: &mut [u8]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let source = self.get_cell(x, y);
                let index = ((y * self.width + x) * 4) as usize;
                target[index] = source.species as u8;
                target[index + 1] = source.ra;
                target[index + 2] = source.rb;
                target[index + 3] = source.clock;
            }
        }
    }

    pub fn write_fluid_texture(&self, target: &mut [u8]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let state = self.wind_state[self.get_index(x, y)];
                let index = ((y * self.width + x) * 4) as usize;
                target[index] = encode_channel(state.vx * 16.0);
                target[index + 1] = encode_channel(state.vy * 16.0);
                target[index + 2] = state.density.clamp(0.0, 255.0) as u8;
                target[index + 3] = state.pressure.clamp(0.0, 255.0) as u8;
            }
        }
    }

    fn reset_fluids(&mut self) {
        self.winds.fill(Wind::default());
        self.burns.fill(Wind::default());
        self.wind_state.fill(WindState::default());
        self.wind_scratch.fill(WindState::default());
    }

    fn update_wind_field(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);
                let current = self.wind_state[index];

                let mut sum_vx = current.vx;
                let mut sum_vy = current.vy;
                let mut sum_pressure = current.pressure;
                let mut sum_density = current.density;
                let mut neighbors = 1.0;

                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || nx >= self.width || ny < 0 || ny >= self.height {
                        continue;
                    }

                    let neighbor = self.wind_state[self.get_index(nx, ny)];
                    sum_vx += neighbor.vx;
                    sum_vy += neighbor.vy;
                    sum_pressure += neighbor.pressure;
                    sum_density += neighbor.density;
                    neighbors += 1.0;
                }

                let occupied = self.get_cell(x, y).species != Species::Empty;
                let solid_drag = if occupied { 0.8 } else { 1.0 };
                let avg_vx = sum_vx / neighbors;
                let avg_vy = sum_vy / neighbors;
                let avg_pressure = sum_pressure / neighbors;
                let avg_density = sum_density / neighbors;

                let buoyancy = avg_density * 0.0012 + avg_pressure * 0.0009;
                self.wind_scratch[index] = WindState {
                    vx: ((current.vx * 0.42) + (avg_vx * 0.58)) * 0.986 * solid_drag,
                    vy: (((current.vy * 0.40) + (avg_vy * 0.60)) + buoyancy) * 0.986 * solid_drag,
                    pressure: ((current.pressure * 0.62) + (avg_pressure * 0.38)) * 0.94,
                    density: ((current.density * 0.72) + (avg_density * 0.28)) * 0.965,
                };
            }
        }

        std::mem::swap(&mut self.wind_state, &mut self.wind_scratch);

        for index in 0..self.wind_state.len() {
            let state = self.wind_state[index];
            self.winds[index] = Wind {
                dx: encode_channel(state.vy * 16.0),
                dy: encode_channel(state.vx * 16.0),
                pressure: state.pressure.clamp(0.0, 255.0) as u8,
                density: state.density.clamp(0.0, 255.0) as u8,
            };
        }
    }

    fn apply_burn_sources(&mut self) {
        for index in 0..self.burns.len() {
            let burn = self.burns[index];
            if burn == Wind::default() {
                continue;
            }

            let state = &mut self.wind_state[index];
            state.vx += burn.dx as f32 / 120.0;
            state.vy += burn.dy as f32 / 90.0;
            state.pressure = state.pressure.max((burn.pressure as f32) * 2.0).min(255.0);
            state.density = state.density.max((burn.density as f32) * 1.15).min(255.0);
        }
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (x * self.height + y) as usize
    }

    fn get_cell(&self, x: i32, y: i32) -> Cell {
        self.cells[self.get_index(x, y)]
    }

    fn get_wind(&self, x: i32, y: i32) -> Wind {
        self.winds[self.get_index(x, y)]
    }

    fn blow_wind(cell: Cell, wind: Wind, mut api: SandApi<'_>) {
        if cell.clock.wrapping_sub(api.universe.generation) == 1 {
            return;
        }
        if cell.species == Species::Empty {
            return;
        }

        let threshold = match cell.species {
            Species::Empty | Species::Wall | Species::Cloner => 500,
            Species::Stone | Species::Wood => 70,
            Species::Plant | Species::Lava | Species::Ice => 60,
            Species::Fungus => 54,
            Species::Oil => 50,
            Species::Seed => 35,
            Species::Sand | Species::Mite | Species::Rocket => 30,
            Species::Dust => 10,
            Species::Fire | Species::Gas => 5,
            Species::Water | Species::Acid => 40,
        };

        let wx = wind.dy as i32 - 126;
        let wy = wind.dx as i32 - 126;
        let mut dx = 0;
        let mut dy = 0;

        if wx > threshold {
            dx = 1;
        }
        if wy > threshold {
            dy = 1;
        }
        if wx < -threshold {
            dx = -1;
        }
        if wy < -threshold {
            dy = -1;
        }

        if (dx != 0 || dy != 0) && api.get(dx, dy).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);

            if dy == -1
                && api.get(dx, -2).species == Species::Empty
                && matches!(
                    cell.species,
                    Species::Sand
                        | Species::Water
                        | Species::Lava
                        | Species::Acid
                        | Species::Mite
                        | Species::Dust
                        | Species::Oil
                        | Species::Rocket
                )
            {
                dy = -2;
            }

            api.set(dx, dy, cell);
        }
    }

    fn update_cell(cell: Cell, api: SandApi<'_>) {
        if cell.clock.wrapping_sub(api.universe.generation) == 1 {
            return;
        }
        cell.species.update(cell, api);
    }
}

fn encode_channel(value: f32) -> u8 {
    (126.0 + value.clamp(-126.0, 129.0)).clamp(0.0, 255.0) as u8
}

fn adjacency_right(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, 1) => (1, 1),
        (1, 1) => (1, 0),
        (1, 0) => (1, -1),
        (1, -1) => (0, -1),
        (0, -1) => (-1, -1),
        (-1, -1) => (-1, 0),
        (-1, 0) => (-1, 1),
        (-1, 1) => (0, 1),
        _ => (0, 0),
    }
}

fn adjacency_left(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, 1) => (-1, 1),
        (1, 1) => (0, 1),
        (1, 0) => (1, 1),
        (1, -1) => (1, 0),
        (0, -1) => (1, -1),
        (-1, -1) => (0, -1),
        (-1, 0) => (-1, -1),
        (-1, 1) => (-1, 0),
        _ => (0, 0),
    }
}

fn join_dy_dx(dx: i32, dy: i32) -> u8 {
    (((dx + 1) * 3) + (dy + 1)) as u8
}

fn split_dy_dx(value: u8) -> (i32, i32) {
    let raw = value as i32;
    ((raw / 3) - 1, (raw % 3) - 1)
}

fn update_sand(cell: Cell, mut api: SandApi<'_>) {
    let dx = api.rand_dir_2();
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if matches!(
        below.species,
        Species::Water | Species::Gas | Species::Oil | Species::Acid
    ) {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

fn update_dust(cell: Cell, mut api: SandApi<'_>) {
    let dx = api.rand_dir();
    let fluid = api.get_fluid();

    if fluid.pressure > 120 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Fire,
                ra: 150u8.saturating_add(cell.ra / 10),
                rb: 0,
                clock: 0,
            },
        );
        api.set_fluid(Wind {
            dx: 0,
            dy: 0,
            pressure: 80,
            density: 5,
        });
        return;
    }

    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

fn update_stone(cell: Cell, mut api: SandApi<'_>) {
    if api.get(-1, -1).species == Species::Stone && api.get(1, -1).species == Species::Stone {
        return;
    }

    let fluid = api.get_fluid();
    if fluid.pressure > 120 && api.rand_int(1) == 0 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Sand,
                ra: cell.ra,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if matches!(
        below.species,
        Species::Water | Species::Gas | Species::Oil | Species::Acid
    ) {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

fn update_water(cell: Cell, mut api: SandApi<'_>) {
    let mut dx = api.rand_dir();
    let below = api.get(0, 1);
    let diagonal = api.get(dx, 1);

    if below.species == Species::Empty || below.species == Species::Oil {
        let mut ra = cell.ra;
        api.set(0, 0, below);
        if api.once_in(20) {
            ra = 100 + api.rand_int(50) as u8;
        }
        api.set(0, 1, Cell { ra, ..cell });
        return;
    } else if diagonal.species == Species::Empty || diagonal.species == Species::Oil {
        api.set(0, 0, diagonal);
        api.set(dx, 1, cell);
        return;
    } else if api.get(-dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, cell);
        return;
    }

    let left = cell.ra % 2 == 0;
    dx = if left { 1 } else { -1 };
    let side = api.get(dx, 0);
    let far_side = api.get(dx * 2, 0);

    if side.species == Species::Empty && far_side.species == Species::Empty {
        api.set(0, 0, far_side);
        api.set(2 * dx, 0, Cell { rb: 6, ..cell });
        let (sx, sy) = api.rand_vec_8();
        let neighbor = api.get(sx, sy);
        if neighbor.species == Species::Water && neighbor.ra % 2 != cell.ra % 2 {
            api.set(
                sx,
                sy,
                Cell {
                    ra: cell.ra,
                    ..cell
                },
            );
        }
    } else if side.species == Species::Empty || side.species == Species::Oil {
        api.set(0, 0, side);
        api.set(dx, 0, Cell { rb: 3, ..cell });
        let (sx, sy) = api.rand_vec_8();
        let neighbor = api.get(sx, sy);
        if neighbor.species == Species::Water && neighbor.ra % 2 != cell.ra % 2 {
            api.set(
                sx,
                sy,
                Cell {
                    ra: cell.ra,
                    ..cell
                },
            );
        }
    } else if cell.rb == 0 {
        if api.get(-dx, 0).species == Species::Empty {
            api.set(
                0,
                0,
                Cell {
                    ra: ((cell.ra as i32) + dx) as u8,
                    ..cell
                },
            );
        }
    } else {
        api.set(
            0,
            0,
            Cell {
                rb: cell.rb - 1,
                ..cell
            },
        );
    }
}

fn update_oil(cell: Cell, mut api: SandApi<'_>) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let neighbor = api.get(dx, dy);
    let mut new_cell = cell;

    if (rb == 0 && neighbor.species == Species::Fire)
        || neighbor.species == Species::Lava
        || (neighbor.species == Species::Oil && neighbor.rb > 1 && neighbor.rb < 20)
    {
        new_cell = Cell {
            species: Species::Oil,
            ra: cell.ra,
            rb: 50,
            clock: 0,
        };
    }

    if rb > 1 {
        new_cell = Cell {
            species: Species::Oil,
            ra: cell.ra,
            rb: rb - 1,
            clock: 0,
        };
        api.set_fluid(Wind {
            dx: 0,
            dy: 10,
            pressure: 10,
            density: 180,
        });

        if rb % 4 != 0 && neighbor.species == Species::Empty && neighbor.species != Species::Water {
            let ra = 20 + api.rand_int(30) as u8;
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra,
                    rb: 0,
                    clock: 0,
                },
            );
        }

        if neighbor.species == Species::Water {
            new_cell = Cell {
                species: Species::Oil,
                ra: 50,
                rb: 0,
                clock: 0,
            };
        }
    } else if rb == 1 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Empty,
                ra: cell.ra,
                rb: 90,
                clock: 0,
            },
        );
        return;
    }

    if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, new_cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, new_cell);
    } else if api.get(-dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, new_cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, new_cell);
    } else if api.get(-dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, new_cell);
    } else {
        api.set(0, 0, new_cell);
    }
}

fn update_gas(cell: Cell, mut api: SandApi<'_>) {
    let (dx, dy) = api.rand_vec();
    let neighbor = api.get(dx, dy);

    if cell.rb == 0 {
        api.set(0, 0, Cell { rb: 5, ..cell });
    }

    if neighbor.species == Species::Empty {
        if cell.rb < 3 {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, dy, cell);
        } else {
            api.set(0, 0, Cell { rb: 1, ..cell });
            api.set(
                dx,
                dy,
                Cell {
                    rb: cell.rb - 1,
                    ..cell
                },
            );
        }
    } else if (dx != 0 || dy != 0) && neighbor.species == Species::Gas && neighbor.rb < 4 {
        api.set(0, 0, EMPTY_CELL);
        api.set(
            dx,
            dy,
            Cell {
                rb: neighbor.rb + cell.rb,
                ..cell
            },
        );
    }
}

fn update_cloner(cell: Cell, mut api: SandApi<'_>) {
    let mut clone_species = Species::from_raw(cell.rb).unwrap_or(Species::Sand);
    let generation = api.universe.generation;

    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if cell.rb == 0 {
                let neighbor_species = api.get(dx, dy).species;
                if neighbor_species != Species::Empty
                    && neighbor_species != Species::Cloner
                    && neighbor_species != Species::Wall
                {
                    clone_species = neighbor_species;
                    api.set(
                        0,
                        0,
                        Cell {
                            species: cell.species,
                            ra: 200,
                            rb: clone_species as u8,
                            clock: 0,
                        },
                    );
                    break;
                }
            } else if api.rand_int(100) > 90 && api.get(dx, dy).species == Species::Empty {
                let ra =
                    80 + api.rand_int(30) as u8 + ((generation % 127) as i8 - 60).unsigned_abs();
                api.set(
                    dx,
                    dy,
                    Cell {
                        species: clone_species,
                        ra,
                        rb: 0,
                        clock: 0,
                    },
                );
                break;
            }
        }
    }
}

fn update_rocket(cell: Cell, mut api: SandApi<'_>) {
    if cell.rb == 0 {
        api.set(
            0,
            0,
            Cell {
                ra: 0,
                rb: 100,
                ..cell
            },
        );
        return;
    }

    let clone_species = if cell.rb != 100 {
        Species::from_raw(cell.rb).unwrap_or(Species::Sand)
    } else {
        Species::Sand
    };

    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    if cell.rb == 100
        && sample.species != Species::Empty
        && sample.species != Species::Rocket
        && sample.species != Species::Wall
        && sample.species != Species::Cloner
    {
        api.set(
            0,
            0,
            Cell {
                ra: 1,
                rb: sample.species as u8,
                ..cell
            },
        );
        return;
    }

    if cell.ra == 0 {
        let dx = api.rand_dir();
        let below = api.get(0, 1);
        if below.species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else if api.get(dx, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 1, cell);
        } else if matches!(
            below.species,
            Species::Water | Species::Gas | Species::Oil | Species::Acid
        ) {
            api.set(0, 0, below);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else if cell.ra == 1 {
        api.set(0, 0, Cell { ra: 2, ..cell });
    } else if cell.ra == 2 {
        let (mut dx, mut dy) = api.rand_vec_8();
        if api.get(dx, dy).species != Species::Empty {
            dx *= -1;
            dy *= -1;
        }
        api.set(
            0,
            0,
            Cell {
                ra: 100 + join_dy_dx(dx, dy),
                ..cell
            },
        );
    } else if cell.ra > 50 {
        let (dx, dy) = split_dy_dx(cell.ra - 100);
        let neighbor = api.get(dx, dy * 2);

        if matches!(
            neighbor.species,
            Species::Empty | Species::Fire | Species::Rocket
        ) {
            let trail_a = api.new_cell(clone_species);
            let trail_b = api.new_cell(clone_species);
            api.set(0, 0, trail_a);
            api.set(0, dy, trail_b);

            let (ndx, ndy) = match api.rand_int(100) % 5 {
                0 => adjacency_left((dx, dy)),
                1 => adjacency_right((dx, dy)),
                _ => (dx, dy),
            };
            api.set(
                dx,
                dy * 2,
                Cell {
                    ra: 100 + join_dy_dx(ndx, ndy),
                    ..cell
                },
            );
        } else {
            api.set(0, 0, EMPTY_CELL);
        }
    }
}

fn update_fire(cell: Cell, mut api: SandApi<'_>) {
    let mut degraded = cell;
    degraded.ra = cell.ra.wrapping_sub((2 + api.rand_dir()) as u8);

    let (dx, dy) = api.rand_vec();
    api.set_fluid(Wind {
        dx: 0,
        dy: 150,
        pressure: 1,
        density: 120,
    });

    if matches!(api.get(dx, dy).species, Species::Gas | Species::Dust) {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::Fire,
                ra: (150 + (dx + dy) * 10) as u8,
                rb: 0,
                clock: 0,
            },
        );
        api.set_fluid(Wind {
            dx: 0,
            dy: 0,
            pressure: 80,
            density: 40,
        });
    }

    if cell.ra < 5 || api.get(dx, dy).species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
    } else if api.get(dx, dy).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, degraded);
    } else {
        api.set(0, 0, degraded);
    }
}

fn update_lava(cell: Cell, mut api: SandApi<'_>) {
    api.set_fluid(Wind {
        dx: 0,
        dy: 10,
        pressure: 0,
        density: 60,
    });

    let (dx, dy) = api.rand_vec();
    if matches!(api.get(dx, dy).species, Species::Gas | Species::Dust) {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::Fire,
                ra: (150 + (dx + dy) * 10) as u8,
                rb: 0,
                clock: 0,
            },
        );
    }

    let sample = api.get(dx, dy);
    if sample.species == Species::Water {
        api.set(
            0,
            0,
            Cell {
                species: Species::Stone,
                ra: (150 + (dx + dy) * 10) as u8,
                rb: 0,
                clock: 0,
            },
        );
        api.set(dx, dy, EMPTY_CELL);
    } else if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}

fn update_wood(cell: Cell, mut api: SandApi<'_>) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let neighbor_species = api.get(dx, dy).species;

    if (rb == 0 && neighbor_species == Species::Fire) || neighbor_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Wood,
                ra: cell.ra,
                rb: 90,
                clock: 0,
            },
        );
    }

    if rb > 1 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Wood,
                ra: cell.ra,
                rb: rb - 1,
                clock: 0,
            },
        );

        if rb % 4 == 0 && neighbor_species == Species::Empty {
            let ra = 30 + api.rand_int(60) as u8;
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra,
                    rb: 0,
                    clock: 0,
                },
            );
        }

        if neighbor_species == Species::Water {
            api.set(
                0,
                0,
                Cell {
                    species: Species::Wood,
                    ra: 50,
                    rb: 0,
                    clock: 0,
                },
            );
            api.set_fluid(Wind {
                dx: 0,
                dy: 0,
                pressure: 0,
                density: 220,
            });
        }
    } else if rb == 1 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Empty,
                ra: cell.ra,
                rb: 90,
                clock: 0,
            },
        );
    }
}

fn update_ice(cell: Cell, mut api: SandApi<'_>) {
    let (dx, dy) = api.rand_vec();
    let fluid = api.get_fluid();

    if fluid.pressure > 120 && api.rand_int(1) == 0 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Water,
                ra: cell.ra,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    let neighbor_species = api.get(dx, dy).species;
    if neighbor_species == Species::Fire || neighbor_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Water,
                ra: cell.ra,
                rb: cell.rb,
                clock: 0,
            },
        );
    } else if neighbor_species == Species::Water && api.rand_int(100) < 7 {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::Ice,
                ra: cell.ra,
                rb: cell.rb,
                clock: 0,
            },
        );
    }
}

fn update_plant(cell: Cell, mut api: SandApi<'_>) {
    let rb = cell.rb;
    let mut i = api.rand_int(100);
    let (dx, dy) = api.rand_vec();
    let neighbor_species = api.get(dx, dy).species;

    if (rb == 0 && neighbor_species == Species::Fire) || neighbor_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Plant,
                ra: cell.ra,
                rb: 20,
                clock: 0,
            },
        );
    }

    if neighbor_species == Species::Wood {
        let (spread_x, spread_y) = api.rand_vec();
        let drift = (i % 15) - 7;
        let new_ra = (cell.ra as i32 + drift) as u8;
        if api.get(spread_x, spread_y).species == Species::Empty {
            api.set(
                spread_x,
                spread_y,
                Cell {
                    species: Species::Plant,
                    ra: new_ra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
    }

    if api.rand_int(100) > 80
        && (neighbor_species == Species::Water
            || (neighbor_species == Species::Fungus
                && matches!(
                    api.get(-dx, dy).species,
                    Species::Empty | Species::Water | Species::Fungus
                )))
    {
        i = api.rand_int(100);
        let drift = (i % 15) - 7;
        let new_ra = (cell.ra as i32 + drift) as u8;
        api.set(
            dx,
            dy,
            Cell {
                ra: new_ra,
                rb: 0,
                ..cell
            },
        );
        api.set(-dx, dy, EMPTY_CELL);
    }

    if rb > 1 {
        api.set(
            0,
            0,
            Cell {
                ra: cell.ra,
                rb: rb - 1,
                ..cell
            },
        );
        if neighbor_species == Species::Empty {
            let ra = 20 + api.rand_int(30) as u8;
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
        if neighbor_species == Species::Water {
            api.set(
                0,
                0,
                Cell {
                    ra: 50,
                    rb: 0,
                    ..cell
                },
            );
        }
    } else if rb == 1 {
        api.set(0, 0, EMPTY_CELL);
    }

    if cell.ra > 50
        && api.get(1, 1).species != Species::Plant
        && api.get(-1, 1).species != Species::Plant
    {
        if api.get(0, 1).species == Species::Empty {
            let chance = (api.rand_float() * api.rand_float() * 100.0) as i32;
            let dec = api.rand_int(30) - 20;
            if chance + cell.ra as i32 > 165 {
                api.set(
                    0,
                    1,
                    Cell {
                        ra: (cell.ra as i32 + dec) as u8,
                        ..cell
                    },
                );
            }
        } else {
            api.set(
                0,
                0,
                Cell {
                    ra: cell.ra - 1,
                    ..cell
                },
            );
        }
    }
}

fn update_seed(cell: Cell, mut api: SandApi<'_>) {
    let rb = cell.rb;
    let ra = cell.ra;
    let (dx, dy) = api.rand_vec();
    let neighbor_species = api.get(dx, dy).species;

    if neighbor_species == Species::Fire || neighbor_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Fire,
                ra: 5,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    if rb == 0 {
        let falling_dx = api.rand_dir();
        let below_diag = api.get(falling_dx, 1).species;
        if matches!(below_diag, Species::Sand | Species::Plant | Species::Fungus) {
            let rb = (api.rand_int(253) + 1) as u8;
            api.set(0, 0, Cell { rb, ..cell });
            return;
        }

        let below = api.get(0, 1);
        if below.species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else if api.get(falling_dx, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(falling_dx, 1, cell);
        } else if matches!(
            below.species,
            Species::Water | Species::Gas | Species::Oil | Species::Acid
        ) {
            api.set(0, 0, below);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else if ra > 60 {
        let raise_dx = api.rand_dir();
        if api.rand_int(100) > 75 {
            if matches!(
                api.get(raise_dx, -1).species,
                Species::Empty | Species::Sand | Species::Seed
            ) && api.get(1, -1).species != Species::Plant
                && api.get(-1, -1).species != Species::Plant
            {
                let stem_ra = (ra as i32 - api.rand_int(10)) as u8;
                api.set(
                    raise_dx,
                    -1,
                    Cell {
                        ra: stem_ra,
                        ..cell
                    },
                );
                let ra2 = 80 + api.rand_int(30) as u8;
                api.set(
                    0,
                    0,
                    Cell {
                        species: Species::Plant,
                        ra: ra2,
                        rb: 0,
                        clock: 0,
                    },
                );
            } else {
                api.set(0, 0, EMPTY_CELL);
            }
        }
    } else if ra > 40 {
        let (mid_x, mid_y) = api.rand_vec();
        let (left_x, left_y) = adjacency_left((mid_x, mid_y));
        let (right_x, right_y) = adjacency_right((mid_x, mid_y));

        if matches!(
            api.get(mid_x, mid_y).species,
            Species::Empty | Species::Plant
        ) && (api.get(left_x, left_y).species == Species::Empty
            || api.get(right_x, right_y).species == Species::Empty)
        {
            let chance = (api.rand_float() * api.rand_float() * 100.0) as i32;
            let dec = 9 - api.rand_int(3);
            if chance + ra as i32 > 100 {
                api.set(
                    mid_x,
                    mid_y,
                    Cell {
                        ra: (ra as i32 - dec) as u8,
                        ..cell
                    },
                );
            }
        }
    } else if neighbor_species == Species::Water {
        let new_seed = api.new_cell(Species::Seed);
        api.set(dx, dy, new_seed);
    }
}

fn update_fungus(cell: Cell, mut api: SandApi<'_>) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let neighbor_species = api.get(dx, dy).species;

    if (rb == 0 && neighbor_species == Species::Fire) || neighbor_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Fungus,
                ra: cell.ra,
                rb: 10,
                clock: 0,
            },
        );
    }

    let mut i = api.rand_int(100);

    if neighbor_species != Species::Empty
        && neighbor_species != Species::Fungus
        && neighbor_species != Species::Fire
        && neighbor_species != Species::Ice
    {
        let (spread_x, spread_y) = api.rand_vec();
        let drift = (i % 15) - 7;
        let new_ra = (cell.ra as i32 + drift) as u8;
        if api.get(spread_x, spread_y).species == Species::Empty {
            api.set(
                spread_x,
                spread_y,
                Cell {
                    species: Species::Fungus,
                    ra: new_ra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
    }

    if i > 9
        && neighbor_species == Species::Wood
        && api.get(-dx, dy).species == Species::Wood
        && api.get(dx, -dy).species == Species::Wood
        && api.get(dx, dy).ra % 4 != 0
    {
        i = api.rand_int(100);
        let drift = (i % 15) - 7;
        let new_ra = (cell.ra as i32 + drift) as u8;
        api.set(
            dx,
            dy,
            Cell {
                ra: new_ra,
                rb: 0,
                ..cell
            },
        );
    }

    if rb > 1 {
        api.set(
            0,
            0,
            Cell {
                ra: cell.ra,
                rb: rb - 1,
                ..cell
            },
        );
        if neighbor_species == Species::Empty {
            let ra = 10 + api.rand_int(10) as u8;
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
        if neighbor_species == Species::Water {
            api.set(
                0,
                0,
                Cell {
                    ra: 50,
                    rb: 0,
                    ..cell
                },
            );
        }
    } else if rb == 1 {
        api.set(0, 0, EMPTY_CELL);
    }

    if cell.ra > 120 {
        let (mid_x, mid_y) = api.rand_vec();
        let (left_x, left_y) = adjacency_left((mid_x, mid_y));
        let (right_x, right_y) = adjacency_right((mid_x, mid_y));
        if api.get(mid_x, mid_y).species == Species::Empty
            && api.get(left_x, left_y).species != Species::Fungus
            && api.get(right_x, right_y).species != Species::Fungus
        {
            let chance = (api.rand_float() * api.rand_float() * 100.0) as i32;
            let dec = 15 - api.rand_int(20);
            if chance + cell.ra as i32 > 165 {
                api.set(
                    mid_x,
                    mid_y,
                    Cell {
                        ra: (cell.ra as i32 - dec) as u8,
                        ..cell
                    },
                );
            }
        }
    }
}

fn update_acid(cell: Cell, mut api: SandApi<'_>) {
    let dx = api.rand_dir();
    let mut degraded = cell;
    degraded.ra = cell.ra.wrapping_sub(60);
    if degraded.ra < 80 {
        degraded = EMPTY_CELL;
    }

    if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else if api.get(-dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, cell);
    } else if api.get(0, 1).species != Species::Wall && api.get(0, 1).species != Species::Acid {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, degraded);
    } else if api.get(dx, 0).species != Species::Wall && api.get(dx, 0).species != Species::Acid {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, degraded);
    } else if api.get(-dx, 0).species != Species::Wall && api.get(-dx, 0).species != Species::Acid {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, degraded);
    } else if api.get(0, -1).species != Species::Wall
        && api.get(0, -1).species != Species::Acid
        && api.get(0, -1).species != Species::Empty
    {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, degraded);
    } else {
        api.set(0, 0, cell);
    }
}

fn update_mite(cell: Cell, mut api: SandApi<'_>) {
    let mut i = api.rand_int(100);
    let mut dx = 0;
    if cell.ra < 20 {
        dx = cell.ra as i32 - 1;
    }

    let mut dy = 1;
    let mut mite = cell;

    if cell.rb > 10 {
        mite.rb = mite.rb.saturating_sub(1);
        dy = -1;
    } else if cell.rb > 1 {
        mite.rb = mite.rb.saturating_sub(1);
    } else {
        dx = 0;
    }

    let neighbor = api.get(dx, dy);
    let sx = (i % 3) - 1;
    i = api.rand_int(1000);
    let sy = (i % 3) - 1;
    let sample = api.get(sx, sy).species;

    if matches!(
        sample,
        Species::Fire | Species::Lava | Species::Water | Species::Oil
    ) {
        api.set(0, 0, EMPTY_CELL);
        return;
    }

    if matches!(sample, Species::Plant | Species::Wood | Species::Seed) && i > 800 {
        api.set(0, 0, EMPTY_CELL);
        api.set(sx, sy, cell);
        return;
    }

    if sample == Species::Dust {
        api.set(sx, sy, if i > 800 { cell } else { EMPTY_CELL });
    }

    if neighbor.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, mite);
    } else if dy == 1 && i > 800 {
        i = api.rand_int(100);
        let mut next_dx = (i % 3) - 1;
        if i < 6 {
            next_dx = dx;
        }

        mite.ra = (1 + next_dx) as u8;
        mite.rb = 10 + (i % 10) as u8;
        api.set(0, 0, mite);
    } else if api.get(-1, 0).species == Species::Mite
        && api.get(1, 0).species == Species::Mite
        && api.get(0, -1).species == Species::Mite
    {
        api.set(0, 0, EMPTY_CELL);
    } else if api.get(0, 1).species == Species::Ice {
        if api.get(dx, 0).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 0, mite);
        }
    } else {
        api.set(0, 0, mite);
    }
}
