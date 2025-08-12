mod blackhole;

use bevy::prelude::*;
use bevy::window::{WindowResolution, PresentMode};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use blackhole::{BlackHolePlugin};

#[derive(Component)]
struct InstructionsText;

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Text::new(
                "REALISTIC BLACK HOLE SIMULATION\n\
                Controls:\n\
                W/S - Move camera closer/farther\n\
                A/D - Rotate camera left/right\n\
                Q/E - Move camera up/down\n\
                SPACE - Toggle auto-rotation\n\
                ESC - Exit"
            ),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                max_width: Val::Px(300.0),
                ..default()
            },
        ))
        .insert(InstructionsText);
}

fn update_instructions(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut TextColor, With<InstructionsText>>,
    time: Res<Time>,
) {
    for mut text_color in &mut query {
        let pulse = (time.elapsed_secs() * 0.5).sin() * 0.1 + 0.9;
        text_color.0 = Color::srgb(0.8 * pulse, 0.8 * pulse, 0.8 * pulse);
    }

    if input.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Black Hole Simulation".into(),
                    resolution: WindowResolution::new(1280.0, 720.0),
                    present_mode: PresentMode::Fifo,
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
            BlackHolePlugin,
        ))
        .add_systems(Startup, setup_ui)
        .add_systems(Update, (
            update_instructions,
        ))
        .run();
}
