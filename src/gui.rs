use eframe::egui;
use log::{error, info, warn};

use crate::work::*;

const WINDOW_WIDTH: f32 = 380.0;
const WINDOW_HEIGHT: f32 = 560.0;
const CENTER: (f32, f32) = (
    (WINDOW_WIDTH - WINDOW_WIDTH * 0.60),
    (WINDOW_HEIGHT - WINDOW_HEIGHT * 0.5),
);
const PADDING: f32 = 10.0;

#[derive(Default, PartialEq)]
pub struct GuiMenu {
    workday: Workday,
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
}

impl eframe::App for GuiMenu {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // for the reset button
        let Self {
            workday: _,
            allowed_to_close: _,
            show_confirmation_dialog: _,
        } = self;

        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .default_pos(CENTER)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.collapsing("Themes", |ui| {
                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if ui.add(egui::Button::new("🇽")).clicked() {
                        self.on_close_event();
                    }
                    egui::reset_button(ui, self);
                });
            });
            ui.add_space(2.0);
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(3.0);
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new("HOmeOFfice")
                        .size(50.0)
                        .strong()
                        .color(egui::Color32::from_rgb(73, 166, 153)),
                );
            });
            ui.add_space(3.0);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            ui.vertical_centered_justified(|ui| {
                ui.hyperlink("leann.phydon@gmail.com");
                ui.label(
                    egui::RichText::new("PoweredByRust")
                        .color(egui::Color32::from_rgb(156, 16, 39)),
                );
            });
            ui.add_space(2.0);
        });

        // CentralPanel must be added after all other panels!
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(2.0);
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new("✏ Take a Note: ")
                        .heading()
                        .strong()
                        .color(egui::Color32::from_rgb(6, 165, 149)),
                );
            });
            ui.add_space(PADDING);

            ui.vertical_centered(|ui| {
                if ui
                    .add_sized([120., 25.], egui::Button::new("💾 Save"))
                    .clicked()
                {
                    println!("Save clicked");
                    if let Ok(workday) = Workday::new() {
                        info!("{:?}", workday.day);
                        info!("{:?}", workday.date);
                    } else {
                        error!("Error creating Workday");
                    }
                }

                ui.add_space(PADDING);
            });
            ui.separator();

            ui.add_space(2.0);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if ui
                        .add_sized([120., 25.], egui::Button::new("⛃ Load"))
                        .clicked()
                    {
                        println!("Loaded clicked");
                    }
                });
            });

            ui.vertical_centered_justified(|ui| {
                egui::containers::ScrollArea::both().show(ui, |ui| {
                    egui::Grid::new("my_grid")
                        .num_columns(4)
                        .spacing([10.0, 4.0])
                        .striped(true)
                        .max_col_width(WINDOW_WIDTH)
                        .show(ui, |ui| {
                            let mut idx: u64 = 1;
                            for _ in 0..=10 {
                                ui.label(format!("{}", idx));

                                ui.label(
                                    egui::RichText::new(format!("{}", "Something",))
                                        .size(20.0)
                                        .color(egui::Color32::from_rgb(76, 116, 166)),
                                );

                                ui.label(
                                    egui::RichText::new(format!("{}", "important",))
                                        .size(20.0)
                                        .color(egui::Color32::from_rgb(76, 116, 166)),
                                );

                                ui.label(
                                    egui::RichText::new(format!("{}", "in here",))
                                        .size(25.0)
                                        .color(egui::Color32::from_rgb(22, 146, 196)),
                                );

                                ui.end_row();
                                idx += 1;
                            }
                        });
                });
            });
            ui.add_space(2.0);
        });
    }
}
