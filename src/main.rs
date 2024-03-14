use bevy::prelude::*;

fn main() {
    // App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
    App::new().add_plugins(HelloPlugin).run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                add_people,
                greet_people,
                person_with_jobs,
                person_ready_for_hire,
                person_work_as,
            )
                .chain(),
        );
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((
        Person,
        Name("Pepito Argentinovich".to_string()),
        Employed {
            job: Job::Programmer,
        },
    ));
    commands.spawn((
        Person,
        Name("Naiblu Kum".to_string()),
        Employed { job: Job::Artist },
    ));
    commands.spawn((
        Person,
        Name("Zick Zickzi".to_string()),
        Employed { job: Job::Musician },
    ));
    commands.spawn((Person, Name("Pelado Boton".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello, {}!", name.0);
    }
}

fn person_with_jobs(query: Query<&Name, (With<Person>, With<Employed>)>) {
    for name in query.iter() {
        println!("{} has a job", name.0);
    }
}

fn person_ready_for_hire(query: Query<&Name, (With<Person>, Without<Employed>)>) {
    for name in query.iter() {
        println!("{} is ready for hire", name.0);
    }
}

fn person_work_as(query: Query<(&Name, &Employed), With<Person>>) {
    for (name, employed) in query.iter() {
        let job_name = match employed.job {
            Job::Programmer => "Programmer",
            Job::Artist => "Artist",
            Job::Musician => "Musician",
        };

        println!("{} works as {}", name.0, job_name);
    }
}

#[derive(Component)]
struct Person;

#[derive(Debug, Component)]
struct Name(String);

#[derive(Component)]
struct Employed {
    job: Job,
}

pub enum Job {
    Programmer,
    Artist,
    Musician,
}
