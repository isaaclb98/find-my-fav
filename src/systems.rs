use crate::database::{
    get_image_path_from_database, get_remaining_participants, initialize_database,
};
use crate::resources::ImageFolderPath;
use crate::styles::{NODE_BUNDLE_EMPTY_COLUMN_STYLE, NODE_BUNDLE_EMPTY_ROW_STYLE};
use bevy::gltf::GltfAssetLabel::Node;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use image::GenericImageView;
use rand::prelude::SliceRandom;
use rand::{random, thread_rng};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

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

fn get_image(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    image_path: PathBuf,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let window_width = window.width();
    let window_height = window.height();

    let handle = asset_server.load(image_path);

    commands.spawn(SpriteBundle {
        texture: handle.clone(),
        transform: Transform {
            translation: Vec3::new(window_width / 2.0, window_height / 2.0, 0.0),
            scale: Vec3::new(0.3, 0.3, 1.0),
            ..Default::default()
        },
        ..default()
    });
}

#[derive(Resource, Default)]
pub struct ParticipantsDeque {
    deque: VecDeque<u64>,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TournamentState {
    #[default]
    Generating,
    Displaying,
    Deciding,
}

pub fn get_participants_for_round(
    mut commands: Commands,
    mut participants_deque_resource: ResMut<ParticipantsDeque>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    let start_time = Instant::now();

    let mut participants = get_remaining_participants().unwrap();

    let mut rng = thread_rng();
    participants.shuffle(&mut rng);

    for participant in participants {
        participants_deque_resource.deque.push_back(participant);
    }

    let duration = start_time.elapsed();
    println!("get_participants_for_round took {:?}", duration);
    next_tournament_state.set(TournamentState::Displaying);
}

#[derive(Component)]
pub struct BothImageComponents;

#[derive(Component)]
pub struct LeftImageComponent;

#[derive(Component)]
pub struct RightImageComponent;

pub fn generate_images_to_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    participants_deque_resource: Res<ParticipantsDeque>,
    mut next_tournament_state: ResMut<NextState<TournamentState>>,
) {
    let start_time = Instant::now();

    let window: &Window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    let image_id_1 = participants_deque_resource
        .deque
        .get(0)
        .expect("Nothing left to pop off of the deque.");
    let image_id_2 = participants_deque_resource
        .deque
        .get(1)
        .expect("Nothing left to pop off of the deque.");

    let image_path_1 =
        get_image_path_from_database(image_id_1).expect("Failed to get image path from database");
    let image_path_2 =
        get_image_path_from_database(image_id_2).expect("Failed to get image path from database");

    // Load the image using the `image` crate
    let image_1 = image::open(&image_path_1).expect("Failed to load image");
    let image_2 = image::open(&image_path_2).expect("Failed to load image");

    let duration = start_time.elapsed();
    println!("getting the images took {:?}", duration);

    // Get the image dimensions
    let (width_1, height_1) = image_1.dimensions();
    println!("Image 1 dimensions: {}x{}", width_1, height_1);
    let (width_2, height_2) = image_2.dimensions();
    println!("Image 2 dimensions: {}x{}", width_2, height_2);

    let image_aspect_ratio_1 = width_1 as f32 / height_1 as f32;
    let image_aspect_ratio_2 = width_2 as f32 / height_2 as f32;

    // Calculate the target width to be half of the window's width
    let target_width = window_width / 2.0;

    // Calculate the target height to maintain the aspect ratio
    let target_height_1 = target_width / image_aspect_ratio_1;
    let target_height_2 = target_width / image_aspect_ratio_2;

    // If the target height is greater than the window height, adjust the target dimensions
    let (final_width_1, final_height_1) = if target_height_1 > window_height {
        let adjusted_height = window_height;
        let adjusted_width = adjusted_height * image_aspect_ratio_1;
        (adjusted_width, adjusted_height)
    } else {
        (target_width, target_height_1)
    };
    let (final_width_2, final_height_2) = if target_height_2 > window_height {
        let adjusted_height = window_height;
        let adjusted_width = adjusted_height * image_aspect_ratio_2;
        (adjusted_width, adjusted_height)
    } else {
        (target_width, target_height_2)
    };

    // Load the image as a Bevy asset
    let texture_handle_1: Handle<Image> = asset_server.load(image_path_1);
    let texture_handle_2: Handle<Image> = asset_server.load(image_path_2);

    let start_time = Instant::now();

    commands
        .spawn((
            NodeBundle {
                style: NODE_BUNDLE_EMPTY_ROW_STYLE,
                ..default()
            },
            BothImageComponents,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: NODE_BUNDLE_EMPTY_ROW_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // image 1
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(final_width_1),
                                height: Val::Px(final_height_1),
                                ..Default::default()
                            },
                            image: UiImage::new(texture_handle_1),
                            ..default()
                        },
                        LeftImageComponent {},
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: NODE_BUNDLE_EMPTY_ROW_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // image 2
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(final_width_2),
                                height: Val::Px(final_height_2),
                                ..Default::default()
                            },
                            image: UiImage::new(texture_handle_2),
                            ..default()
                        },
                        RightImageComponent {},
                    ));
                });
        });

    let duration = start_time.elapsed();
    println!("displaying the images took {:?}", duration);

    next_tournament_state.set(TournamentState::Deciding);
}
