mod sim;

use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{AsBindGroup, Extent3d, ShaderType, TextureDimension, TextureFormat},
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
    window::PrimaryWindow,
};
use sim::{ALL_SPECIES, Species, Universe};

const GRID_WIDTH: i32 = 300;
const GRID_HEIGHT: i32 = 300;
const BOARD_SCALE: f32 = 2.1;
const BOARD_TRANSLATION: Vec3 = Vec3::new(165.0, -15.0, 0.0);
const FIXED_STEP: f32 = 1.0 / 60.0;
const BRUSH_SIZES: [i32; 5] = [1, 3, 7, 19, 39];

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.95, 0.92, 0.88)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "sandspiel_bevy".into(),
                        resolution: (1440, 920).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(Material2dPlugin::<SandMaterial>::default())
        .insert_resource(SandState::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                button_actions,
                keyboard_actions,
                paint_from_mouse,
                step_simulation,
                sync_textures,
                update_material_time,
                refresh_button_styles,
                update_status_text,
            ),
        )
        .run();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tool {
    Wind,
    Element(Species),
}

impl Default for Tool {
    fn default() -> Self {
        Self::Element(Species::Water)
    }
}

#[derive(Resource)]
struct SandState {
    universe: Universe,
    selected_tool: Tool,
    brush_size: usize,
    paused: bool,
    boot: BootSequence,
    drag_last_cell: Option<IVec2>,
    drag_last_world: Option<Vec2>,
    time_accumulator: f32,
}

impl Default for SandState {
    fn default() -> Self {
        Self {
            universe: Universe::new(GRID_WIDTH, GRID_HEIGHT),
            selected_tool: Tool::default(),
            brush_size: 2,
            paused: false,
            boot: BootSequence::default(),
            drag_last_cell: None,
            drag_last_world: None,
            time_accumulator: 0.0,
        }
    }
}

#[derive(Default)]
struct BootSequence {
    phase: BootPhase,
    x: f32,
    timer: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum BootPhase {
    #[default]
    Sand,
    Seeds,
    Done,
}

impl BootSequence {
    fn stop(&mut self) {
        self.phase = BootPhase::Done;
    }

    fn advance(&mut self, universe: &mut Universe, dt: f32) {
        self.timer += dt;

        match self.phase {
            BootPhase::Sand => {
                while self.timer >= 0.016 {
                    self.timer -= 0.016;
                    if self.x == 0.0 {
                        self.x = 5.0;
                    }
                    if self.x > (GRID_WIDTH - 5) as f32 {
                        self.phase = BootPhase::Seeds;
                        self.x = 40.0;
                        self.timer = 0.0;
                        break;
                    }
                    let y = GRID_HEIGHT as f32 - 40.0 + 5.0 * (self.x / 20.0).sin();
                    let size = 10 + (self.x as i32 % 6);
                    universe.paint(self.x as i32, y as i32, size, Species::Sand);
                    self.x += 10.0;
                }
            }
            BootPhase::Seeds => {
                while self.timer >= 0.18 {
                    self.timer -= 0.18;
                    if self.x > (GRID_WIDTH - 40) as f32 {
                        self.phase = BootPhase::Done;
                        break;
                    }
                    let y = GRID_HEIGHT as f32 / 2.0 + 20.0 * (self.x / 20.0).sin();
                    universe.paint(self.x as i32, y as i32, 6, Species::Seed);
                    self.x += 52.0 + (self.x as i32 % 10) as f32;
                }
            }
            BootPhase::Done => {}
        }
    }
}

#[derive(Resource)]
struct RenderHandles {
    cell_image: Handle<Image>,
    fluid_image: Handle<Image>,
    material: Handle<SandMaterial>,
}

#[derive(Component)]
struct Board;

#[derive(Component)]
struct StatusText;

#[derive(Component, Clone, Copy)]
struct UiButton(UiAction);

#[derive(Clone, Copy)]
enum UiAction {
    TogglePause,
    Reset,
    Undo,
    Wind,
    Brush(usize),
    Species(Species),
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct SandMaterial {
    #[texture(0)]
    #[sampler(1)]
    cells: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    fluid: Handle<Image>,
    #[uniform(4)]
    params: SandMaterialParams,
}

#[derive(Clone, Copy, Debug, ShaderType)]
struct SandMaterialParams {
    time: f32,
    width: f32,
    height: f32,
    _padding: f32,
}

impl Material2d for SandMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sand_material.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<SandMaterial>>,
) {
    commands.spawn(Camera2d);

    let cell_image = images.add(make_texture());
    let fluid_image = images.add(make_texture());
    let material = materials.add(SandMaterial {
        cells: cell_image.clone(),
        fluid: fluid_image.clone(),
        params: SandMaterialParams {
            time: 0.0,
            width: GRID_WIDTH as f32,
            height: GRID_HEIGHT as f32,
            _padding: 0.0,
        },
    });

    commands.insert_resource(RenderHandles {
        cell_image: cell_image.clone(),
        fluid_image: fluid_image.clone(),
        material: material.clone(),
    });

    commands.spawn((
        Board,
        Mesh2d(meshes.add(Rectangle::new(
            GRID_WIDTH as f32 * BOARD_SCALE,
            GRID_HEIGHT as f32 * BOARD_SCALE,
        ))),
        MeshMaterial2d(material),
        Transform::from_translation(BOARD_TRANSLATION),
    ));

    spawn_ui(&mut commands);
}

fn make_texture() -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width: GRID_WIDTH as u32,
            height: GRID_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();
    image
}

fn spawn_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(320.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(18.0)),
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.96, 0.92, 0.88, 0.9)),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new("sandspiel_bevy"),
                    TextFont {
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.18, 0.16, 0.15)),
                ));
                panel.spawn((
                    Text::new("Rust + Bevy 0.18 + WGSL"),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.42, 0.38, 0.34)),
                ));

                spawn_button_row(
                    panel,
                    &[
                        ("Pause", UiAction::TogglePause),
                        ("Reset", UiAction::Reset),
                        ("Undo", UiAction::Undo),
                    ],
                );

                panel.spawn((
                    Text::new("Brush"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.22, 0.20, 0.18)),
                ));
                panel
                    .spawn((
                        Node {
                            flex_wrap: FlexWrap::Wrap,
                            column_gap: Val::Px(8.0),
                            row_gap: Val::Px(8.0),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|sizes| {
                        for (index, size) in BRUSH_SIZES.iter().enumerate() {
                            spawn_button(sizes, &size.to_string(), UiAction::Brush(index), None);
                        }
                    });

                panel.spawn((
                    Text::new("Tools"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.22, 0.20, 0.18)),
                ));
                panel
                    .spawn((
                        Node {
                            flex_wrap: FlexWrap::Wrap,
                            column_gap: Val::Px(8.0),
                            row_gap: Val::Px(8.0),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|tools| {
                        spawn_button(tools, "Wind", UiAction::Wind, Some(button_color_for_wind()));
                        for species in ALL_SPECIES {
                            spawn_button(
                                tools,
                                species.label(),
                                UiAction::Species(species),
                                Some(button_color_for_species(species)),
                            );
                        }
                    });

                panel.spawn((
                    Text::new(""),
                    StatusText,
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.24, 0.22, 0.20)),
                ));
            });
        });
}

fn spawn_button_row(parent: &mut ChildSpawnerCommands<'_>, buttons: &[(&str, UiAction)]) {
    parent
        .spawn((
            Node {
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(8.0),
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|row| {
            for (label, action) in buttons {
                spawn_button(row, label, *action, None);
            }
        });
}

fn spawn_button(
    parent: &mut ChildSpawnerCommands<'_>,
    label: &str,
    action: UiAction,
    tint: Option<Color>,
) {
    let mut background = tint.unwrap_or(Color::srgb(0.86, 0.82, 0.78));
    background.set_alpha(0.9);

    parent
        .spawn((
            Button,
            UiButton(action),
            Node {
                padding: UiRect::axes(Val::Px(10.0), Val::Px(8.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(background),
            BorderColor::all(Color::srgba(0.16, 0.14, 0.12, 0.25)),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.12, 0.11, 0.10)),
            ));
        });
}

fn button_actions(
    mut interactions: Query<(&Interaction, &UiButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<SandState>,
) {
    for (interaction, button) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        state.boot.stop();
        match button.0 {
            UiAction::TogglePause => state.paused = !state.paused,
            UiAction::Reset => {
                state.universe.reset();
                state.drag_last_cell = None;
                state.drag_last_world = None;
            }
            UiAction::Undo => state.universe.pop_undo(),
            UiAction::Wind => state.selected_tool = Tool::Wind,
            UiAction::Brush(index) => state.brush_size = index,
            UiAction::Species(species) => state.selected_tool = Tool::Element(species),
        }
    }
}

fn keyboard_actions(keys: Res<ButtonInput<KeyCode>>, mut state: ResMut<SandState>) {
    if keys.just_pressed(KeyCode::Space) {
        state.paused = !state.paused;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        state.boot.stop();
        state.universe.reset();
    }
    if keys.just_pressed(KeyCode::KeyZ)
        && (keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight))
    {
        state.universe.pop_undo();
    }
    if keys.just_pressed(KeyCode::Digit1) {
        state.brush_size = 0;
    }
    if keys.just_pressed(KeyCode::Digit2) {
        state.brush_size = 1;
    }
    if keys.just_pressed(KeyCode::Digit3) {
        state.brush_size = 2;
    }
    if keys.just_pressed(KeyCode::Digit4) {
        state.brush_size = 3;
    }
    if keys.just_pressed(KeyCode::Digit5) {
        state.brush_size = 4;
    }
}

fn paint_from_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut state: ResMut<SandState>,
) {
    let pressed = buttons.pressed(MouseButton::Left);
    if !pressed {
        state.drag_last_cell = None;
        state.drag_last_world = None;
        return;
    }

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Some(world_position) = camera
        .0
        .viewport_to_world_2d(camera.1, cursor_position)
        .ok()
    else {
        return;
    };
    let Some(cell) = world_to_cell(world_position) else {
        return;
    };

    if buttons.just_pressed(MouseButton::Left) {
        state.universe.push_undo();
        state.boot.stop();
        state.drag_last_cell = Some(cell);
        state.drag_last_world = Some(world_position);
    }

    let last_cell = state.drag_last_cell.unwrap_or(cell);
    let points = raster_line(last_cell, cell);
    let brush = BRUSH_SIZES[state.brush_size];

    match state.selected_tool {
        Tool::Element(species) => {
            for point in points {
                state.universe.paint(point.x, point.y, brush, species);
            }
        }
        Tool::Wind => {
            let last_world = state.drag_last_world.unwrap_or(world_position);
            let delta = world_position - last_world;
            for point in points {
                state
                    .universe
                    .apply_wind_brush(point.x, point.y, brush, delta.x, delta.y);
            }
        }
    }

    state.drag_last_cell = Some(cell);
    state.drag_last_world = Some(world_position);
}

fn world_to_cell(world_position: Vec2) -> Option<IVec2> {
    let half_width = GRID_WIDTH as f32 * BOARD_SCALE * 0.5;
    let half_height = GRID_HEIGHT as f32 * BOARD_SCALE * 0.5;
    let local = world_position - BOARD_TRANSLATION.truncate();

    if local.x < -half_width
        || local.x > half_width
        || local.y < -half_height
        || local.y > half_height
    {
        return None;
    }

    let x = ((local.x + half_width) / BOARD_SCALE).floor() as i32;
    let y = ((half_height - local.y) / BOARD_SCALE).floor() as i32;

    if x < 0 || x >= GRID_WIDTH || y < 0 || y >= GRID_HEIGHT {
        return None;
    }

    Some(IVec2::new(x, y))
}

fn raster_line(start: IVec2, end: IVec2) -> Vec<IVec2> {
    let delta = end - start;
    let steps = delta.x.abs().max(delta.y.abs()).max(1);
    let mut points = Vec::with_capacity((steps + 1) as usize);
    for step in 0..=steps {
        let t = step as f32 / steps as f32;
        let x = (start.x as f32 + delta.x as f32 * t).round() as i32;
        let y = (start.y as f32 + delta.y as f32 * t).round() as i32;
        let point = IVec2::new(x, y);
        if points.last().copied() != Some(point) {
            points.push(point);
        }
    }
    points
}

fn step_simulation(time: Res<Time>, mut state: ResMut<SandState>) {
    if state.paused {
        return;
    }

    state.time_accumulator += time.delta_secs();
    while state.time_accumulator >= FIXED_STEP {
        state.time_accumulator -= FIXED_STEP;
        let mut boot = std::mem::take(&mut state.boot);
        boot.advance(&mut state.universe, FIXED_STEP);
        state.boot = boot;
        state.universe.tick();
    }
}

fn sync_textures(
    handles: Res<RenderHandles>,
    state: Res<SandState>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Some(image) = images.get_mut(&handles.cell_image) {
        if let Some(data) = image.data.as_mut() {
            state.universe.write_cell_texture(data);
        }
    }

    if let Some(image) = images.get_mut(&handles.fluid_image) {
        if let Some(data) = image.data.as_mut() {
            state.universe.write_fluid_texture(data);
        }
    }
}

fn update_material_time(
    time: Res<Time>,
    handles: Res<RenderHandles>,
    mut materials: ResMut<Assets<SandMaterial>>,
) {
    if let Some(material) = materials.get_mut(&handles.material) {
        material.params.time += time.delta_secs();
    }
}

fn refresh_button_styles(
    state: Res<SandState>,
    mut buttons: Query<(&UiButton, &mut BackgroundColor, &mut BorderColor)>,
) {
    for (button, mut background, mut border) in &mut buttons {
        let (selected, color) = match button.0 {
            UiAction::TogglePause => (state.paused, Color::srgb(0.80, 0.76, 0.72)),
            UiAction::Reset => (false, Color::srgb(0.85, 0.78, 0.72)),
            UiAction::Undo => (false, Color::srgb(0.81, 0.81, 0.76)),
            UiAction::Wind => (state.selected_tool == Tool::Wind, button_color_for_wind()),
            UiAction::Brush(index) => (state.brush_size == index, Color::srgb(0.85, 0.83, 0.78)),
            UiAction::Species(species) => (
                state.selected_tool == Tool::Element(species),
                button_color_for_species(species),
            ),
        };

        let mut fill = color;
        fill.set_alpha(if selected { 1.0 } else { 0.82 });
        *background = BackgroundColor(fill);
        *border = BorderColor::all(if selected {
            Color::srgb(0.10, 0.09, 0.08)
        } else {
            Color::srgba(0.16, 0.14, 0.12, 0.25)
        });
    }
}

fn update_status_text(state: Res<SandState>, mut text: Single<&mut Text, With<StatusText>>) {
    let tool = match state.selected_tool {
        Tool::Wind => "Wind".to_string(),
        Tool::Element(species) => species.label().to_string(),
    };

    text.0 = format!(
        "Selected: {tool}\nBrush: {}\n{}\n\nControls:\nLeft drag paint or push wind\nSpace pause/resume\nCtrl+Z undo\nR reset\n1-5 brush size",
        BRUSH_SIZES[state.brush_size],
        if state.paused {
            "Simulation paused"
        } else {
            "Simulation running"
        },
    );
}

fn button_color_for_wind() -> Color {
    Color::srgb(0.74, 0.87, 0.90)
}

fn button_color_for_species(species: Species) -> Color {
    match species {
        Species::Empty => Color::srgb(0.94, 0.92, 0.90),
        Species::Wall => Color::srgb(0.54, 0.52, 0.50),
        Species::Sand => Color::srgb(0.84, 0.72, 0.46),
        Species::Water => Color::srgb(0.42, 0.66, 0.90),
        Species::Gas => Color::srgb(0.92, 0.84, 0.82),
        Species::Cloner => Color::srgb(0.85, 0.65, 0.84),
        Species::Fire => Color::srgb(0.96, 0.56, 0.26),
        Species::Wood => Color::srgb(0.58, 0.39, 0.22),
        Species::Lava => Color::srgb(0.88, 0.32, 0.12),
        Species::Ice => Color::srgb(0.76, 0.90, 0.98),
        Species::Plant => Color::srgb(0.48, 0.72, 0.43),
        Species::Acid => Color::srgb(0.78, 0.90, 0.32),
        Species::Stone => Color::srgb(0.42, 0.42, 0.46),
        Species::Dust => Color::srgb(0.79, 0.64, 0.84),
        Species::Mite => Color::srgb(0.86, 0.42, 0.90),
        Species::Oil => Color::srgb(0.30, 0.28, 0.26),
        Species::Rocket => Color::srgb(0.96, 0.84, 0.70),
        Species::Fungus => Color::srgb(0.76, 0.66, 0.56),
        Species::Seed => Color::srgb(0.94, 0.80, 0.38),
    }
}
