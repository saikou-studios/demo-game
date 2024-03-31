use bevy::{
    app::{App, AppExit, Plugin, Update},
    asset::Handle,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
        query::{Changed, With},
        schedule::{
            common_conditions::in_state, IntoSystemConfigs, NextState, OnEnter, OnExit, States,
        },
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, ChildBuilder, DespawnRecursiveExt},
    prelude::NodeBundle,
    render::{camera::Camera, color::Color, texture::Image},
    text::TextStyle,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, TextBundle},
        widget::Button,
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, PositionType,
        Style, UiImage, UiRect, Val, ZIndex,
    },
};

use crate::{loading::TextureAssets, GameState};

const PALETTE: [Color; 4] = [
    Color::rgb(0.902, 0.855, 0.773),                 // off-white
    Color::rgb(0.275, 0.204, 0.220),                 // dark brown
    Color::rgb(0.337, 0.259, 0.220),                 // brown
    Color::rgb(119. / 255., 98. / 255., 96. / 255.), // light brown
];
const TEXT_COLOR: Color = PALETTE[0];
const TITLE_TEXT_COLOR: Color = PALETTE[1];
const BTN_COLOR: Color = PALETTE[1];
const HOVERED_BTN_COLOR: Color = PALETTE[2];
const PRESSED_BTN_COLOR: Color = PALETTE[3];

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    #[default]
    Disabled,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), (setup_background, setup_menu))
            .add_systems(
                OnExit(GameState::Menu),
                (despawn_screen::<MainMenuBg>, despawn_camera),
            )
            .add_systems(OnEnter(MenuState::Main), setup_main_menu)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<MainMenuScreen>)
            .add_systems(
                Update,
                (menu_action, update_btn_colors).run_if(in_state(GameState::Menu)),
            );
    }
}

fn setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

#[derive(Component)]
pub struct MainMenuScreen;

fn setup_main_menu(mut commands: Commands, texture_assets: Res<TextureAssets>) {
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
                            "Demo Game",
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
                                background_color: BTN_COLOR.into(),
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
                                background_color: BTN_COLOR.into(),
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
                                background_color: BTN_COLOR.into(),
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

#[derive(Component)]
pub struct MainMenuBg;

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
enum MenuButtonAction {
    Play,
    Settings,
    Quit,
}

fn menu_action(
    interaction_q: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_event: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_btn_action) in &interaction_q {
        if *interaction == Interaction::Pressed {
            match menu_btn_action {
                MenuButtonAction::Quit => {
                    app_exit_event.send(AppExit);
                }
                MenuButtonAction::Play => {
                    menu_state.set(MenuState::Disabled);
                    game_state.set(GameState::Playing);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
            }
        }
    }
}

fn update_btn_colors(
    mut interaction_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_q {
        *color = match *interaction {
            Interaction::Pressed => PRESSED_BTN_COLOR.into(),
            Interaction::Hovered => HOVERED_BTN_COLOR.into(),
            Interaction::None => BTN_COLOR.into(),
        }
    }
}

fn despawn_screen<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_camera(mut commands: Commands, camera_q: Query<Entity, With<Camera>>) {
    if let Ok(entity) = camera_q.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
