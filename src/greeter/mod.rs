use bevy::prelude::*;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);

#[derive(Resource)]
pub struct GreetTimer(Timer);

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ozzy Osbourne".to_string())));
    commands.spawn((Person, Name("Dio".to_string())));
    commands.spawn((Person, Name("Rob Halford".to_string())));
}

pub fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0)
        }
    }
}
