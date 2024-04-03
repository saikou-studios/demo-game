use bevy::{
    app::{App, Plugin},
    core::Name,
    ecs::{
        schedule::OnEnter,
        system::{Commands, Res},
    },
    hierarchy::{BuildChildren, Parent},
    render::view::RenderLayers,
    sprite::TextureAtlas,
    ui::{
        node_bundles::{ImageBundle, NodeBundle},
        AlignItems, JustifyContent, Style, UiImage, Val,
    },
};

use crate::{loading::TextureAssets, GameState};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_skill_selection);
    }
}

fn spawn_skill_selection(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("root_skill_container_node"))
        .insert(RenderLayers::all())
        .id();

    let skill_container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("skill_container"))
        .insert(RenderLayers::all())
        .id();

    commands.entity(root).add_child(skill_container);

    let mut skill_selectors = vec![];
    for i in 0..2 {
        skill_selectors.push(
            commands
                .spawn((
                    Name::new(format!("skill_selector_{}", i)),
                    ImageBundle {
                        style: Style {
                            width: Val::Px(80.0),
                            height: Val::Px(80.0),
                            ..Default::default()
                        },

                        image: UiImage::new(texture_assets.border.clone()),
                        ..Default::default()
                    },
                    TextureAtlas::from(texture_assets.border_layout.clone()),
                ))
                .insert(RenderLayers::all())
                .id(),
        );
    }

    commands
        .entity(skill_container)
        .push_children(&skill_selectors);
}
