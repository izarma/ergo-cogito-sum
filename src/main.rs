use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_players);
        app.add_systems(Update, (hello_world,greet_people).chain());
    }
}


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
}

fn hello_world(time: Res<Time>, mut timer: ResMut<GreetTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("hello world!");
    }
    
}

fn add_players(mut commands: Commands) {
    commands.spawn((Person, Name("Gorrister".to_string())));
    commands.spawn((Person, Name("Benny".to_string())));
    commands.spawn((Person, Name("Ellen".to_string())));
    commands.spawn((Person, Name("Nimdok".to_string())));
    commands.spawn((Person, Name("Ted".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}