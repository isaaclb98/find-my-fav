use crate::systems::{LeftImageComponent, RightImageComponent};
use bevy::prelude::*;

#[derive(Event)]
pub struct ImageClickedEvent {
    pub(crate) left_image: bool,
}

pub fn interact_with_left_image_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<LeftImageComponent>)>,
    mut image_clicked_event: EventWriter<ImageClickedEvent>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image_clicked_event.send(ImageClickedEvent { left_image: true });
                println!("Left image clicked.");
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn interact_with_right_image_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<RightImageComponent>)>,
    mut image_clicked_event: EventWriter<ImageClickedEvent>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image_clicked_event.send(ImageClickedEvent { left_image: false });
                println!("Right image clicked.");
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
