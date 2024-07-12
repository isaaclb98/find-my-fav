use crate::database::{
    get_image_path_from_database, get_remaining_participants, initialize_database,
};
use crate::resources::ImageFolderPath;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::SliceRandom;
use rand::{random, thread_rng};
use std::path::PathBuf;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn initialize_database_if_image_folder_path(image_folder_path: Res<ImageFolderPath>) {
    if let Some(path) = &image_folder_path.image_folder_path {
        initialize_database(path.clone())
            .expect("Something went wrong when initializing the database.");
    }
}

pub fn check_if_tournament_in_progress() {}

fn get_image(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    image_path: PathBuf,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    image_loaded_event_writer: &mut EventWriter<ImageLoadedEvent>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let window_width = window.width();
    let window_height = window.height();

    let handle = asset_server.load(image_path);

    let entity = commands
        .spawn(SpriteBundle {
            texture: handle.clone(),
            transform: Transform {
                translation: Vec3::new(window_width / 2.0, window_height / 2.0, 0.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..Default::default()
            },
            ..default()
        })
        .id();

    commands.spawn(AwaitingImageDimensions {
        handle: handle.clone(),
    });

    image_loaded_event_writer.send(ImageLoadedEvent { entity, handle });
}

#[derive(Event)]
pub struct ImageLoadedEvent {
    entity: Entity,
    handle: Handle<Image>,
}

#[derive(Component)]
pub struct AwaitingImageDimensions {
    pub handle: Handle<Image>,
}
pub fn check_image_loaded(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    query: Query<(Entity, &AwaitingImageDimensions)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (entity, awaiting) in query.iter() {
        let window: &Window = window_query.get_single().unwrap();

        if let Some(image) = images.get(&awaiting.handle) {
            let dimensions = image.texture_descriptor.size;
            println!(
                "Image dimensions: width = {}, height = {}",
                dimensions.width, dimensions.height
            );

            let window_half_width = window.width() / 2.0;
            let window_width = window.width();
            let window_height = window.height();
            let image_aspect_ratio = dimensions.width as f32 / dimensions.height as f32;

            let scale_factor = if window_half_width / window_height > image_aspect_ratio {
                window_height / dimensions.height as f32
            } else {
                window_half_width / dimensions.width as f32
            };

            let scaled_width = dimensions.width as f32 * scale_factor;
            let scaled_height = dimensions.height as f32 * scale_factor;

            let x_position = (window_half_width - scaled_width) / 2.0;
            let y_position = (window_height - scaled_height) / 2.0;

            commands
                .entity(entity)
                .insert(Transform {
                    translation: Vec3::new(x_position, y_position, 0.0),
                    scale: Vec3::new(scale_factor, scale_factor, 1.0),
                    ..Default::default()
                })
                .remove::<AwaitingImageDimensions>();
        }
    }
}

pub fn get_two_participants(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut image_loaded_event_writer: EventWriter<ImageLoadedEvent>,
) {
    let mut participants = get_remaining_participants().unwrap();

    let mut rng = thread_rng();
    participants.shuffle(&mut rng);

    for pair in participants.chunks(2) {
        if pair.len() == 2 {
            let image_1 = get_image_path_from_database(pair[0]).expect("Er");

            get_image(
                &mut commands,
                &asset_server,
                image_1.clone(),
                &window_query,
                &mut image_loaded_event_writer,
            );
        }
    }
}
