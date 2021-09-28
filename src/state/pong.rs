use amethyst::{GameData, SimpleState, SimpleTrans, StateData, Trans, StateEvent};
use amethyst::assets::{Handle, Loader};
use amethyst::core::{Time, Transform};
use amethyst::core::ecs::{Builder, World, WorldExt, Entity, Join, WriteStorage, ReadStorage};
use amethyst::renderer::{Camera, SpriteRender, SpriteSheet};
use amethyst::ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform};

use crate::ball::component::Ball;
use crate::paddle::component::{Paddle, Side};
use crate::timer::TimerText;
use rand::Rng;
use amethyst::input::{is_close_requested, is_key_down, VirtualKeyCode};
use crate::state::pause::PauseMenuState;
use crate::state::Pause;
use crate::persistence::window::WindowSettings;
use amethyst::core::math::Vector3;
use amethyst::window::ScreenDimensions;
use crate::persistence::Settings;
use crate::taunt::TauntComponent;
use amethyst::renderer::rendy::wsi::winit::{Event, WindowEvent};


pub struct Pong {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Handle<SpriteSheet>,
    created_entities: Vec<Entity>,
    window_settings: WindowSettings,
    camera: Option<Entity>,

}

impl Pong {
    pub fn new(sprite_sheet_handle: Handle<SpriteSheet>, window_settings: WindowSettings) -> Self {
        Pong { ball_spawn_timer: None, sprite_sheet_handle, created_entities: vec![], window_settings, camera: None }
    }
}

impl SimpleState for Pong {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.get_mut::<Pause>().unwrap().paused = false;

        self.ball_spawn_timer.replace(2.0);
        let (left, right) = self.initialise_paddles(world, self.sprite_sheet_handle.clone());
        self.re_init_camera(world);
        initialise_timer(world);
        self.created_entities.push(left);
        self.created_entities.push(right);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.get_mut::<Pause>().unwrap().paused = true;
        data.world.delete_entities(&self.created_entities).unwrap();
        if let Some(timer_text) = data.world.get_mut::<TimerText>() {
            if let Some(timer) = timer_text.timer.take() {
                if let Err(err) = data.world.delete_entity(timer) {
                    log::error!("Tried to remove wrong generation entity, err={}", err);
                }
            }
        }

    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.get_mut::<Pause>().unwrap().paused = true;

    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.rescale_if_res_updated(world);
        world.get_mut::<Pause>().unwrap().paused = false;
    }


    fn handle_event(&mut self, data: StateData<'_, GameData>, event: StateEvent) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Push] Pausing Game!");
                    pause()
                } else {
                    match *event {
                        Event::WindowEvent { ref event, .. } => {
                            match *event {
                                WindowEvent::Resized(_) => {
                                    self.rescale_if_res_updated(data.world);
                                    pause()
                                },
                                _ => Trans::None
                            }
                        },
                        Event::DeviceEvent { .. } => Trans::None,
                        Event::Awakened => Trans::None,
                        Event::Suspended(_) => pause(),
                    }
                }
            },
            _ => Trans::None
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.ball_spawn_timer.take() {
            // If the timer isn't expired yet, subtract the time that passed since the last update.
            {
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                // When timer expire, spawn the ball
                self.created_entities.push(self.initialise_ball(data.world, self.sprite_sheet_handle.clone()));
            } else {
                // If timer is not expired yet, put it back onto the state.
                self.ball_spawn_timer.replace(timer);
            }
        }
        Trans::None
    }

}

impl Pong {

    fn rescale_if_res_updated(&mut self, world: &mut World) {
        let dimensions = world.read_resource::<ScreenDimensions>();
        let mut settings = world.write_resource::<Settings>();

        let old = *settings;
        if settings.update_window(&dimensions) {
            self.window_settings = settings.window_settings;
            let other = *settings;
            drop(dimensions);
            drop(settings);
            self.rescale(world, old);
            other.persist_async();
        };
    }

    fn rescale(&mut self, world: &mut World, old_settings: Settings) {
        world.exec(|(mut paddles, mut balls, taunts, mut transforms): (WriteStorage<Paddle>, WriteStorage<Ball>, ReadStorage<TauntComponent>, WriteStorage<Transform>)| {
            for (paddle, transform) in (&mut paddles, &mut transforms).join() {
                transform.set_scale(Vector3::new(self.window_settings.paddle_width_scale(), self.window_settings.paddle_height_scale(), 1.0));
                let new_y = transform.translation().y * self.window_settings.arena_height() / old_settings.window_settings.arena_height();
                if paddle.side == Side::Left {
                    transform.set_translation_xyz(self.window_settings.paddle_width() * 0.5, new_y, 0.0);
                } else if paddle.side == Side::Right {
                    transform.set_translation_xyz(self.window_settings.arena_width() - self.window_settings.paddle_width() * 0.5, new_y, 0.0);
                }

                paddle.width = self.window_settings.paddle_width();
                paddle.height = self.window_settings.paddle_height();
            }
            for (ball, transform) in (&mut balls, &mut transforms).join() {
                transform.set_scale(Vector3::new(self.window_settings.ball_scale(), self.window_settings.ball_scale(), 1.0));
                let x = transform.translation().x;
                let y = transform.translation().y;
                let new_x = x * self.window_settings.arena_width() / old_settings.window_settings.arena_width();
                let new_y = y * self.window_settings.arena_height() / old_settings.window_settings.arena_height();
                transform.set_translation_xyz(new_x, new_y, 0.0);
                ball.calculated_impact_y = None;
                ball.radius = self.window_settings.ball_radius();
                let rel_x = ball.velocity[0] / old_settings.window_settings.arena_width();
                let rel_y = ball.velocity[1] / old_settings.window_settings.arena_height();
                ball.velocity[0] = rel_x * self.window_settings.arena_width();
                ball.velocity[1] = rel_y * self.window_settings.arena_height();
            }
            for (_taunt, transform) in (&taunts, &mut transforms).join() {
                transform.set_translation_xyz(self.window_settings.arena_width() / 2.0, self.window_settings.arena_height() - self.window_settings.taunt_height() / 2.0, -1.0);
                transform.set_scale(Vector3::new(self.window_settings.taunt_scale(), self.window_settings.taunt_scale(), 1.0));
            }
        });
        self.re_init_camera(world);

    }

    fn re_init_camera(&mut self, world: &mut World) {
        // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
        let mut transform = Transform::default();
        transform.set_translation_xyz(self.window_settings.arena_width() * 0.5, self.window_settings.arena_height() * 0.5, 1.0);

        if let Some(old) = self.camera {
            world.delete_entity(old).unwrap();
        }
        self.camera = Some(world
            .create_entity()
            .with(Camera::standard_2d(self.window_settings.arena_width(), self.window_settings.arena_height()))
            .with(transform)
            .build())
    }

    /// Initialises one paddle on the left, and one paddle on the right.
    fn initialise_paddles(&self, world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> (Entity, Entity) {
        let mut left_transform = Transform::default();
        let mut right_transform = Transform::default();
        let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

        // Correctly position the paddles.
        let y = self.window_settings.arena_height() / 2.0;
        left_transform.set_translation_xyz(self.window_settings.paddle_width() * 0.5, y, 0.0);
        left_transform.set_scale(Vector3::new(self.window_settings.paddle_width_scale(), self.window_settings.paddle_height_scale(), 1.0));
        right_transform.set_translation_xyz(self.window_settings.arena_width() - self.window_settings.paddle_width() * 0.5, y, 0.0);
        right_transform.set_scale(Vector3::new(self.window_settings.paddle_width_scale(), self.window_settings.paddle_height_scale(), 1.0));

        // Create a left plank entity.
        (world
             .create_entity()
             .with(sprite_render.clone())
             .with(Paddle::new(Side::Left, self.window_settings.paddle_height(), self.window_settings.paddle_width()))
             .with(left_transform)
             .build(),
         world
             .create_entity()
             .with(sprite_render)
             .with(Paddle::new(Side::Right, self.window_settings.paddle_height(), self.window_settings.paddle_width()))
             .with(right_transform)
             .build())
    }



    /// Initialises one ball in the middle-ish of the arena.
    fn initialise_ball(&self, world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> Entity {
        // Create the translation.
        let mut local_transform = Transform::default();
        local_transform.set_translation_xyz(self.window_settings.arena_width() / 2.0, self.window_settings.arena_height() / 2.0, 0.0);
        local_transform.set_scale(Vector3::new(self.window_settings.ball_scale(), self.window_settings.ball_scale(), 1.0));

        // Assign the sprite for the ball. The ball is the second sprite in the sheet.
        let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

        let mut rn = rand::thread_rng();
        let neg_x = rn.gen_bool(0.5);
        let neg_y = rn.gen_bool(0.5);
        let mult_x = if neg_x { 1.0 } else { -1.0 };
        let mult_y = if neg_y { 1.0 } else { -1.0 };
        world
            .create_entity()
            .with(sprite_render)
            .with(Ball {
                radius: self.window_settings.ball_radius(),
                velocity: [self.window_settings.ball_velocity_x() * mult_x, self.window_settings.ball_velocity_y() * mult_y],
                calculated_impact_y: None,
            })
            .with(local_transform)
            .build()
    }
}

/// Initialises a ui scoreboard
fn initialise_timer(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let timer_transform = UiTransform::new(
        "TIMER".to_string(), Anchor::TopLeft, Anchor::TopLeft,
        0., -50., 1., 200., 50.,
    );


    let timer = world
        .create_entity()
        .with(timer_transform)
        .with(UiText::new(
            font.clone(),
            "0.0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(TimerText { game_time: -2.0, timer: Some(timer) });
}

fn pause() -> SimpleTrans {
    Trans::Push(Box::new(PauseMenuState::default()))
}
