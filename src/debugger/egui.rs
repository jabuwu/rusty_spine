use egui::*;
use egui_extras::*;

use crate::{AnimationState, Skeleton};

enum Command {
    SetAnimationByName { track_index: i32, name: String },
    SetEmptyAnimation { track_index: i32 },
    ClearTrack { track_index: i32 },
    SetToSetupPose,
}

pub fn egui_spine_debugger(
    ctx: &Context,
    title: impl Into<WidgetText>,
    skeleton: &mut Skeleton,
    animation_state: &mut AnimationState,
) {
    Window::new(title).show(ctx, |ui| {
        let mut commands = vec![];
        ui.heading("Tracks");
        let table = TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Size::initial(16.0).at_least(16.0))
            .column(Size::initial(110.0).at_least(110.0))
            .column(Size::initial(40.0).at_least(40.0))
            .column(Size::initial(70.0).at_least(70.0))
            .column(Size::remainder().at_least(60.0))
            .resizable(true);
        table
            .header(20., |mut header| {
                header.col(|ui| {
                    ui.label("#");
                });
                header.col(|ui| {
                    ui.label("Animation");
                });
                header.col(|ui| {
                    ui.label("Alpha");
                });
                header.col(|ui| {
                    ui.label("Timescale");
                });
            })
            .body(|mut body| {
                for (track_index, track) in animation_state.tracks_mut().enumerate() {
                    if let Some(mut track) = track {
                        body.row(20., |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}", track_index));
                            });
                            row.col(|ui| {
                                let mut selected = track.animation().name().to_owned();
                                egui::ComboBox::new(format!("track {}", track_index), "")
                                    .selected_text(track.animation().name())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut selected,
                                            "<none>".to_owned(),
                                            "<none>",
                                        );
                                        for animation in skeleton.data().animations() {
                                            ui.selectable_value(
                                                &mut selected,
                                                animation.name().to_owned(),
                                                animation.name(),
                                            );
                                        }
                                    });
                                if selected != track.animation().name() {
                                    if selected == "<none>" {
                                        commands.push(Command::ClearTrack {
                                            track_index: track_index as i32,
                                        });
                                    } else {
                                        commands.push(Command::SetAnimationByName {
                                            track_index: track_index as i32,
                                            name: selected,
                                        });
                                    }
                                }
                            });
                            row.col(|ui| {
                                let mut alpha = track.alpha();
                                ui.add(
                                    DragValue::new(&mut alpha)
                                        .speed(0.01)
                                        .clamp_range(0.0..=1.0),
                                );
                                track.set_alpha(alpha);
                            });
                            row.col(|ui| {
                                let mut timescale = track.timescale();
                                ui.add(DragValue::new(&mut timescale).speed(0.01));
                                track.set_timescale(timescale);
                            });
                        });
                    } else {
                        body.row(20., |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}", track_index));
                            });
                            row.col(|ui| {
                                let mut selected = "<none>".to_owned();
                                egui::ComboBox::new(format!("track {}", track_index), "")
                                    .selected_text("<none>")
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut selected,
                                            "<none>".to_owned(),
                                            "<none>",
                                        );
                                        for animation in skeleton.data().animations() {
                                            ui.selectable_value(
                                                &mut selected,
                                                animation.name().to_owned(),
                                                animation.name(),
                                            );
                                        }
                                    });
                                if selected != "<none>" {
                                    commands.push(Command::SetAnimationByName {
                                        track_index: track_index as i32,
                                        name: selected,
                                    });
                                }
                            });
                        });
                    }
                }
                body.row(20., |mut row| {
                    row.col(|ui| {
                        if ui.button("+").clicked() {
                            let track_count = animation_state.tracks_count();
                            animation_state.set_empty_animation(track_count, 0.);
                            commands.push(Command::SetEmptyAnimation {
                                track_index: track_count,
                            });
                        }
                    });
                });
            });
        ui.add_space(16.);
        if ui.button("Setup Pose").clicked() {
            commands.push(Command::SetToSetupPose);
        }
        for command in commands.into_iter() {
            match command {
                Command::SetAnimationByName { track_index, name } => {
                    animation_state
                        .set_animation_by_name(track_index, &name, true)
                        .unwrap();
                }
                Command::SetEmptyAnimation { track_index } => {
                    animation_state.set_empty_animation(track_index, 0.);
                }
                Command::ClearTrack { track_index } => {
                    animation_state.clear_track(track_index);
                }
                Command::SetToSetupPose => {
                    skeleton.set_to_setup_pose();
                }
            }
        }
    });
}
