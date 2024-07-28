use crate::tournament::components::{ImageClickedEvent, ImageComponent};
use bevy::prelude::*;

pub fn interact_with_image_button(
    mut button_query: Query<
        (&Interaction, &ImageComponent, Entity),
        (Changed<Interaction>, With<ImageComponent>),
    >,
    mut image_clicked_event: EventWriter<ImageClickedEvent>,
) {
    if let Ok((interaction, image_component, entity)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                println!(
                    "{:?} clicked. ImageComponent: {:?}",
                    entity, image_component
                );
                image_clicked_event.send(ImageClickedEvent {
                    id: image_component.id,
                });
            }
            _ => {}
        }
    }
}
