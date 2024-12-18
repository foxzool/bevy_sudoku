use crate::color::{DARK_BLACK, DARK_GRAY, EXTRA_LIGHT_GRAY, GRAY, LIGHT_GRAY, WHITE_COLOR};
use crate::game::cell_state::CellValue;
use crate::game::{AutoCandidateMode, CleanCell, NewCandidate, NewDigit, SelectedCell};
use bevy::prelude::*;
use sudoku::board::{CellState, Digit};

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<SelectedTab>()
        .add_systems(
            Update,
            (update_control_tab, show_number).run_if(resource_changed::<SelectedTab>),
        )
        .add_systems(
            Update,
            (update_auto_candidate_icon,).run_if(resource_changed::<AutoCandidateMode>),
        );
}

#[derive(Component)]
pub struct ControlDigit;

#[derive(Component)]
pub struct ControlCandidate;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
enum ControlTab {
    #[default]
    Normal,
    Candidate,
}

#[derive(Component)]
struct ChangeTab(ControlTab);

#[derive(Resource, Debug, Deref, DerefMut, Default)]
struct SelectedTab(ControlTab);

pub(crate) fn control_board(
    asset_server: &Res<AssetServer>,
    font: &Handle<Font>,
    builder: &mut ChildBuilder,
) {
    builder
        .spawn((
            Node {
                margin: UiRect {
                    left: Val::Px(40.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                max_width: Val::Px(240.0),
                display: Display::Block,
                ..default()
            },
            // BackgroundColor(GRAY.into()),
        ))
        .with_children(|builder| {
            // keyboard
            builder
                .spawn((
                    Name::new("keyboard_split"),
                    Node {
                        width: Val::Px(240.0),
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    // 切换按钮
                    builder
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(140.0),
                                height: Val::Px(38.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(0.0)),
                                padding: UiRect::axes(Val::Px(6.0), Val::Px(1.0)),
                                ..Default::default()
                            },
                            BackgroundColor(*DARK_BLACK),
                            ChangeTab(ControlTab::Normal),
                            BorderRadius::left(Val::Px(3.0)),
                            // BorderColor(WHITE_COLOR),
                        ))
                        .with_child((
                            Text::new("Normal"),
                            TextFont {
                                font: font.clone(),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(WHITE_COLOR),
                        ))
                        .observe(
                            |trigger: Trigger<Pointer<Click>>,
                             mut selected_tab: ResMut<SelectedTab>| {
                                selected_tab.0 = ControlTab::Normal;
                            },
                        );

                    builder
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(140.0),
                                height: Val::Px(38.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                padding: UiRect::axes(Val::Px(6.0), Val::Px(1.0)),
                                ..Default::default()
                            },
                            BackgroundColor(WHITE_COLOR),
                            ChangeTab(ControlTab::Candidate),
                            BorderRadius::right(Val::Px(3.0)),
                            BorderColor(*LIGHT_GRAY),
                        ))
                        .with_child((
                            Text::new("Candidate"),
                            TextFont {
                                font: font.clone(),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(*DARK_GRAY),
                        ))
                        .observe(
                            |trigger: Trigger<Pointer<Click>>,
                             mut selected_tab: ResMut<SelectedTab>| {
                                selected_tab.0 = ControlTab::Candidate;
                            },
                        );
                });

            // 数字键盘
            builder
                .spawn((
                    Name::new("keyboard_container"),
                    Node {
                        width: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        justify_content: JustifyContent::SpaceBetween,
                        align_content: AlignContent::SpaceBetween,
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    for i in 1..=9 {
                        builder
                            .spawn((
                                Node {
                                    width: Val::Px(70.0),
                                    height: Val::Px(70.0),
                                    border: UiRect::all(Val::Px(1.0)),
                                    margin: UiRect {
                                        top: Val::Px(14.0),
                                        ..default()
                                    },
                                    align_items: AlignItems::Center,
                                    justify_items: JustifyItems::Center,
                                    align_content: AlignContent::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BorderRadius::all(Val::Px(3.0)),
                                BackgroundColor(*EXTRA_LIGHT_GRAY),
                                BorderColor(*GRAY),
                                ControlNumber(i),
                            ))
                            .observe(mouse_click_control_digit)
                            .with_children(|builder| {
                                // 数字格子
                                builder.spawn((
                                    Text::new(i.to_string()),
                                    TextFont {
                                        font: asset_server.load("fonts/franklin-normal-700.ttf"),
                                        font_size: 32.0,
                                        ..default()
                                    },
                                    TextColor(*DARK_BLACK),
                                    Visibility::Visible,
                                    ControlDigit,
                                ));

                                // 候选格子容器
                                builder
                                    .spawn((
                                        Visibility::Hidden,
                                        ControlCandidate,
                                        Node {
                                            height: Val::Percent(100.0),
                                            display: Display::Grid,
                                            aspect_ratio: Some(1.0),
                                            position_type: PositionType::Absolute,
                                            grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                                            grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|builder| {
                                        // 9个候选数字格子
                                        for k in 1..=9u8 {
                                            let visibility = if k == i {
                                                Visibility::Inherited
                                            } else {
                                                Visibility::Hidden
                                            };
                                            builder.spawn((
                                                visibility,
                                                Text::new(k.to_string()),
                                                TextFont {
                                                    font: asset_server
                                                        .load("fonts/franklin-normal-700.ttf"),
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(*DARK_BLACK),
                                                TextLayout::new_with_justify(JustifyText::Center),
                                                Node {
                                                    align_items: AlignItems::Center,
                                                    justify_items: JustifyItems::Center,
                                                    align_content: AlignContent::Center,
                                                    justify_content: JustifyContent::Center,
                                                    margin: UiRect {
                                                        top: Val::Px(4.),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                            ));
                                        }
                                    });
                            });
                    }

                    // 删除按钮
                    builder
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(48.0),
                                border: UiRect::all(Val::Px(1.0)),
                                margin: UiRect {
                                    top: Val::Px(14.0),
                                    ..default()
                                },
                                align_items: AlignItems::Center,
                                justify_items: JustifyItems::Center,
                                align_content: AlignContent::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BorderRadius::all(Val::Px(3.0)),
                            BackgroundColor(*EXTRA_LIGHT_GRAY),
                            BorderColor(*GRAY),
                        ))
                        .observe(
                            |_trigger: Trigger<Pointer<Click>>,
                             selected_cell: Single<Entity, With<SelectedCell>>,
                             mut commands: Commands| {
                                commands.trigger_targets(CleanCell, vec![*selected_cell]);
                            },
                        )
                        .with_children(|builder| {
                            builder.spawn((
                                ImageNode {
                                    image: asset_server.load("textures/close.png"),
                                    ..default()
                                },
                                Node {
                                    height: Val::Px(18.0),
                                    width: Val::Px(18.0),
                                    ..default()
                                },
                            ));
                        });

                    // 自动候选模式
                    builder
                        .spawn((
                            Name::new("auto"),
                            Node {
                                margin: UiRect {
                                    top: Val::Px(10.0),
                                    ..default()
                                },
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                        ))
                        .observe(on_click_auto_candidate)
                        .with_children(|builder| {
                            builder.spawn((
                                ImageNode::new(asset_server.load("textures/blank-check-box.png")),
                                Node {
                                    width: Val::Px(13.0),
                                    height: Val::Px(13.0),
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                AutoCandidateNotCheck,
                            ));

                            builder.spawn((
                                Visibility::Hidden,
                                ImageNode::new(asset_server.load("textures/check.png")),
                                Node {
                                    position_type: PositionType::Absolute,
                                    width: Val::Px(13.0),
                                    height: Val::Px(13.0),
                                    ..default()
                                },
                                AutoCandidateCheck,
                            ));

                            builder.spawn((
                                Text::new("Auto Candidate Mode"),
                                TextFont {
                                    font: asset_server.load("fonts/franklin-normal-600.ttf"),
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(*DARK_BLACK),
                                Node {
                                    margin: UiRect {
                                        left: Val::Px(18.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

#[derive(Component)]
struct AutoCandidateNotCheck;

#[derive(Component)]
struct AutoCandidateCheck;

fn on_click_auto_candidate(_trigger: Trigger<Pointer<Click>>, mut auto: ResMut<AutoCandidateMode>) {
    auto.0 = !auto.0;
}

fn update_auto_candidate_icon(
    auto: Res<AutoCandidateMode>,
    mut check: Query<&mut Visibility, (With<AutoCandidateCheck>, Without<AutoCandidateNotCheck>)>,
    mut not_check: Query<
        &mut Visibility,
        (Without<AutoCandidateCheck>, With<AutoCandidateNotCheck>),
    >,
) {
    if auto.0 {
        for mut visibility in check.iter_mut() {
            *visibility = Visibility::Visible;
        }
        for mut visibility in not_check.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    } else {
        for mut visibility in check.iter_mut() {
            *visibility = Visibility::Hidden;
        }
        for mut visibility in not_check.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}

fn show_number(
    selected_tab: Res<SelectedTab>,
    mut normal_cell: Query<&mut Visibility, (With<ControlDigit>, Without<ControlCandidate>)>,
    mut candidate: Query<&mut Visibility, (With<ControlCandidate>, Without<ControlDigit>)>,
) {
    match selected_tab.0 {
        ControlTab::Normal => {
            for mut visibility in normal_cell.iter_mut() {
                *visibility = Visibility::Visible;
            }
            for mut visibility in candidate.iter_mut() {
                *visibility = Visibility::Hidden;
            }
        }
        ControlTab::Candidate => {
            for mut visibility in normal_cell.iter_mut() {
                *visibility = Visibility::Hidden;
            }
            for mut visibility in candidate.iter_mut() {
                *visibility = Visibility::Visible;
            }
        }
    }
}

fn update_control_tab(
    selected_tab: Res<SelectedTab>,
    mut tab_query: Query<(
        &ChangeTab,
        &mut Node,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
    )>,
    mut text_color: Query<&mut TextColor>,
) {
    for (change_tab, mut node, mut bg, mut border_color, children) in tab_query.iter_mut() {
        if change_tab.0 == selected_tab.0 {
            bg.0 = *DARK_BLACK;
            border_color.0 = WHITE_COLOR;
            for child in children {
                if let Ok(mut text_color) = text_color.get_mut(*child) {
                    text_color.0 = WHITE_COLOR;
                }
            }
        } else {
            bg.0 = WHITE_COLOR;
            border_color.0 = *LIGHT_GRAY;
            for child in children {
                if let Ok(mut text_color) = text_color.get_mut(*child) {
                    text_color.0 = *DARK_GRAY;
                }
            }
        }

        // normal tab selected
        if selected_tab.0 == ControlTab::Normal {
            if change_tab.0 == ControlTab::Normal {
                node.border = UiRect::all(Val::Px(0.0));
            } else {
                node.border = UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(1.0),
                    top: Val::Px(1.0),
                    bottom: Val::Px(1.0),
                }
            }
        } else {
            if change_tab.0 == ControlTab::Candidate {
                node.border = UiRect::all(Val::Px(0.0));
            } else {
                node.border = UiRect {
                    left: Val::Px(1.0),
                    right: Val::Px(0.0),
                    top: Val::Px(1.0),
                    bottom: Val::Px(1.0),
                }
            }
        }
    }
}

#[derive(Component)]
struct ControlNumber(u8);

fn mouse_click_control_digit(
    trigger: Trigger<Pointer<Click>>,
    q_cell: Query<&ControlNumber>,
    selected_cell: Single<Entity, With<SelectedCell>>,
    mut commands: Commands,
    auto_mode: Res<AutoCandidateMode>,
    selected_tab: Res<SelectedTab>,
) {
    println!("mouse_click_control_digit");
    if let Ok(cell_value) = q_cell.get(trigger.entity()) {

        match selected_tab.0 {
            ControlTab::Normal => {
                info!("New digit: {} ", cell_value.0);
                commands.trigger_targets(NewDigit::new(cell_value.0), vec![*selected_cell]);
            }
            ControlTab::Candidate => {
                info!("New candidate: {} ", cell_value.0);
                commands.trigger_targets(NewCandidate::new(cell_value.0), vec![*selected_cell]);
            }
        }
    }
}
