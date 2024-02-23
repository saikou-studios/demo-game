use bevy::diagnostic::{
    DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::hierarchy::BuildChildren;
use bevy::input::ButtonInput;
use bevy::prelude::{
    Color, Commands, Component, KeyCode, NodeBundle, PositionType, Query, Res, TextBundle,
    Visibility, With, Without, ZIndex,
};
use bevy::text::{Text, TextSection, TextStyle};
use bevy::ui::{BackgroundColor, Style, UiRect, Val};

#[derive(Component)]
pub(crate) struct DiagnosticsRoot;
#[derive(Component)]
pub(crate) struct FpsText;
#[derive(Component)]
pub(crate) struct EntitiesText;

pub(crate) fn setup_diagnostics(mut commands: Commands) {
    let root = commands
        .spawn((
            DiagnosticsRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_a(0.0)),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.0f32),
                    top: Val::Px(1.0f32),
                    bottom: Val::Auto,
                    left: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    let text_fps = commands
        .spawn((
            FpsText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    let text_entities = commands
        .spawn((
            EntitiesText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: ", Entities: ".to_string(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands
        .entity(root)
        .push_children(&[text_fps, text_entities]);
}

pub(crate) fn diagnostics_text_update(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_q: Query<&mut Text, (With<FpsText>, Without<EntitiesText>)>,
    mut entities_q: Query<&mut Text, (With<EntitiesText>, Without<FpsText>)>,
) {
    for mut text in &mut fps_q {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.sections[1].value = format!("{value:>4.0}");
            text.sections[1].style.color = if value >= 120.0 {
                Color::GREEN
            } else if value >= 60.0 {
                Color::rgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                Color::RED
            };
        } else {
            text.sections[1].value = "N/A".to_string();
            text.sections[1].style.color = Color::WHITE;
        }
    }

    for mut text in &mut entities_q {
        if let Some(value) = diagnostics
            .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
            .and_then(|entities| entities.value())
        {
            text.sections[1].value = format!("{value:>3.0}");
            text.sections[1].style.color = Color::PURPLE;
        } else {
            text.sections[1].value = "N/A".to_string();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

pub(crate) fn diagnostics_show_hide(
    mut query: Query<&mut Visibility, With<DiagnosticsRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = query.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
