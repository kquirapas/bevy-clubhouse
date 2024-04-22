use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input, keyboard_zoom_in, sprite_animation))
        .run();
}

#[derive(Component, Eq, PartialEq)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

#[derive(Component)]
enum Motion {
    Idle,
    Walking,
    Running,
}

const IDLE_ANIMATION_INDICES: AnimationIndices = AnimationIndices { first: 0, last: 7 };
const WALK_ANIMATION_INDICES: AnimationIndices = AnimationIndices { first: 8, last: 15 };
const RUN_ANIMATION_INDICES: AnimationIndices = AnimationIndices {
    first: 16,
    last: 23,
};

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64., 64.), 24, 1, None, None);
    let atlas_layout = texture_atlas_layouts.add(layout);

    let main_camera = Camera2dBundle::default();
    let main_background = SpriteBundle {
        texture: asset_server.load("bg.png"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    };

    let main_player = (
        Person,
        Name(String::from("Player")),
        Direction::Right,
        Motion::Idle,
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: atlas_layout,
                index: IDLE_ANIMATION_INDICES.first,
            },
            ..default()
        },
        IDLE_ANIMATION_INDICES,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    );

    // Spawning
    commands.spawn(main_camera);
    commands.spawn(main_background);
    commands.spawn(main_player);
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_state: Query<
        (&mut Direction, &mut Motion, &mut Transform, &mut Sprite),
        With<Person>,
    >,
) {
    const SPEED: f32 = 50.;

    // Movement
    for (mut direction, mut motion, mut transform, mut sprite) in &mut player_state {
        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += SPEED * time.delta_seconds();
            *direction = Direction::Right;
            sprite.flip_x = false;
            *motion = Motion::Walking;
        } else if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= SPEED * time.delta_seconds();
            *direction = Direction::Left;
            sprite.flip_x = true;
            *motion = Motion::Walking;
        } else if keys.just_released(KeyCode::KeyD) || keys.just_released(KeyCode::KeyA) {
            *motion = Motion::Idle;
        }
    }
}

fn sprite_animation(
    time: Res<Time>,
    mut query: Query<(
        &Motion,
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
    )>,
) {
    for (motion, mut indices, mut timer, mut atlas) in &mut query {
        match motion {
            Motion::Idle => {
                if *indices == WALK_ANIMATION_INDICES || *indices == RUN_ANIMATION_INDICES {
                    *indices = IDLE_ANIMATION_INDICES;
                    atlas.index = indices.first;
                }

                if timer.tick(time.delta()).just_finished() {
                    atlas.index = if atlas.index == indices.last {
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                }
            }
            Motion::Walking => {
                if *indices == IDLE_ANIMATION_INDICES {
                    *indices = WALK_ANIMATION_INDICES;
                    atlas.index = indices.first;
                }

                if timer.tick(time.delta()).just_finished() {
                    atlas.index = if atlas.index == indices.last {
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                }
            }
            Motion::Running => {
                if *indices == IDLE_ANIMATION_INDICES {
                    *indices = RUN_ANIMATION_INDICES;
                    atlas.index = indices.first;
                }

                if timer.tick(time.delta()).just_finished() {
                    atlas.index = if atlas.index == indices.last {
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                }
            }
        }
    }
}

fn keyboard_zoom_in(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    // Zoom
    if keys.just_pressed(KeyCode::KeyW) {
        for mut projection in query.iter_mut() {
            projection.scale -= 0.2;

            if projection.scale < 0.2 {
                projection.scale = 0.2;
            }

            println!("Current zoom scale: {}", projection.scale);
        }
    } else if keys.just_pressed(KeyCode::KeyS) {
        for mut projection in query.iter_mut() {
            projection.scale += 0.2;

            if projection.scale > 1. {
                projection.scale = 1.;
            }

            println!("Current zoom scale: {}", projection.scale);
        }
    }
}
