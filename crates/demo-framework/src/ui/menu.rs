use bevy::{
    app::{App, Plugin},
    asset::Handle,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        schedule::OnEnter,
        system::{Commands, Res},
    },
    hierarchy::{BuildChildren, ChildBuilder},
    prelude::NodeBundle,
    render::{color::Color, texture::Image},
    text::TextStyle,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, TextBundle},
        AlignItems, FlexDirection, JustifyContent, PositionType, Style, UiImage, UiRect, Val,
        ZIndex,
    },
};

use crate::{loading::TextureAssets, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), (setup_background, setup_menu));
    }
}

#[derive(Component)]
pub struct MainMenuBg;

const PALETTE: [Color; 2] = [
    Color::rgb(0.902, 0.855, 0.773), // off-white
    Color::rgb(0.275, 0.204, 0.220), // brown
];

const TEXT_COLOR: Color = PALETTE[0];
const TITLE_TEXT_COLOR: Color = PALETTE[1];
const BTN_BG_COLOR: Color = PALETTE[1];

fn setup_background(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            z_index: ZIndex::Global(0),
            ..Default::default()
        })
        .insert(MainMenuBg)
        .with_children(|parent| {
            let spawn_bg_img =
                |parent: &mut ChildBuilder<'_>, image: Handle<Image>, z_index: bevy::ui::ZIndex| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            left: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            ..Default::default()
                        },
                        z_index,
                        image: UiImage::new(image),
                        ..Default::default()
                    });
                };

            spawn_bg_img(
                parent,
                texture_assets.background_one.clone(),
                bevy::ui::ZIndex::Local(0),
            );

            spawn_bg_img(
                parent,
                texture_assets.background_two.clone(),
                bevy::ui::ZIndex::Local(1),
            );

            spawn_bg_img(
                parent,
                texture_assets.background_three.clone(),
                bevy::ui::ZIndex::Local(2),
            );
        });
}

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    Quit,
}

fn setup_menu(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let btn_style = Style {
        width: Val::Px(150.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let btn_text_style = TextStyle {
        font_size: 30.0,
        font: texture_assets.dungeon_font.clone(),
        color: TEXT_COLOR,
        ..Default::default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                z_index: ZIndex::Global(1),
                ..Default::default()
            },
            MainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    // title
                    parent.spawn(
                        TextBundle::from_section(
                            "Saikou",
                            TextStyle {
                                font_size: 75.0,
                                font: texture_assets.dungeon_font.clone(),
                                color: TITLE_TEXT_COLOR,
                                ..Default::default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..Default::default()
                        }),
                    );

                    // btns
                    parent
                        .spawn((
                            ButtonBundle {
                                style: btn_style.clone(),
                                background_color: BTN_BG_COLOR.into(),
                                ..Default::default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Play", btn_text_style.clone()));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: btn_style.clone(),
                                background_color: BTN_BG_COLOR.into(),
                                ..Default::default()
                            },
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                btn_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: btn_style.clone(),
                                background_color: BTN_BG_COLOR.into(),
                                ..Default::default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Quit", btn_text_style.clone()));
                        });
                });
        });
}
