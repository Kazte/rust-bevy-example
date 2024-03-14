use bevy::prelude::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people).chain());
    }
}

// fn hello_world() {
//     println!("hello world!");
// }

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("El Pepito".to_string())));
    commands.spawn((Person, Name("Naiblu Kum".to_string())));
    commands.spawn((Person, Name("Zick Zickzi".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // upgrade timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to the people

    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello, {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "El Pepito" {
            name.0 = "Pepe Argento".to_string();
            break;
        }
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
