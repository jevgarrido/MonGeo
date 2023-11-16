//! MonGeo main file.

use bevy::prelude::*;
use regex::Regex;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .add_systems(Startup, setup)
        // Object managment
        .add_systems(Update, operations_manager)
        // Command interaction
        .add_systems(Update, update_command_sequence)
        .add_systems(Update, update_display_command_text)
        .run();
}

fn operations_manager(
    mut input_query_cmd: Query<&mut CommandSequence>,
    mut elms: Query<&mut ElmCollection>,
) {
    let mut cmd = input_query_cmd.single_mut();

    // Detect 'spawn'
    if Regex::new(format!(r"\b{}\b", "spawn").as_str())
        .unwrap()
        .is_match(&cmd.sequence)
    {
        elms.single_mut().spawn();
        cmd.sequence.clear();
    }

    // Detect 'print'
    if Regex::new(format!(r"\b{}\b", "print").as_str())
        .unwrap()
        .is_match(&cmd.sequence)
    {
        for elm in elms.single().0.iter() {
            println!("{}", elm.name)
        }
        cmd.sequence.clear();
    }
}

fn update_command_sequence(
    mut input_query_cmd: Query<&mut CommandSequence>,
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
) {
    let mut cmd = input_query_cmd.single_mut();

    if kbd.just_pressed(KeyCode::Escape) {
        cmd.sequence.clear();
    }

    if kbd.just_pressed(KeyCode::Back) {
        cmd.sequence.pop();
    }

    for ev in evr_char.read() {
        if !ev.char.is_control() {
            cmd.sequence.push(ev.char);
        }
    }
}

fn update_display_command_text(cmd: Query<&CommandSequence>, mut text: Query<&mut Text>) {
    text.single_mut().sections[0].value = cmd.single().sequence.clone();
}

fn setup(mut commands: Commands) {
    commands.spawn(ElmCollection::default());
    commands.spawn(CommandSequence::default());

    commands.spawn(Camera2dBundle::default());

    // UI
    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::DARK_GRAY,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        }),
    );
}

#[derive(Component, Default)]
struct CommandSequence {
    sequence: String,
}

/// Geometric element that exists in one of the projection planes.
#[derive(Component, Default)]
struct Element {
    name: String,
}

impl Element {
    fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Component, Default)]
struct ElmCollection(Vec<Element>);

impl ElmCollection {
    fn spawn(&mut self) {
        let mut duplicated = true;
        let mut count: usize = 0;
        let mut name = names::generate();

        while duplicated {
            if count >= names::TOTAL_PERM {
                duplicated = true;
                break;
            } else {
                duplicated = false;
            }

            count += 1;

            for elm in self.0.iter() {
                if name == elm.name {
                    duplicated = true;
                    name = names::generate();
                    break;
                }
            }
        }

        if duplicated {
            println!("Cannot spawn more elements");
        } else {
            self.0.push(Element::new(name));
        }
    }
}

mod names {
    //! Module provides a random name generator.
    //!
    use rand::prelude::*;

    pub const LENGTH: u32 = 2;
    pub const TOTAL_PERM: usize = CHARSET.len().pow(LENGTH);

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz";

    pub fn generate() -> String {
        (1..=LENGTH)
            .map(|_| {
                let idx = rand::thread_rng().gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}
