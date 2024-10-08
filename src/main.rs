use calc::{calculator::Caculator, keypad};
// it's an example
use eframe::egui;

use keypad::Keypad;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 1200.0])
            .with_resizable(true)
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "Caculator App",
        options,
        Box::new(|cc| {
            // Use the dark theme
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
    keypad: Keypad,
    exp: String,
    result: String,
}

impl MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Alex".to_owned(),
            age: 42,
            keypad: Keypad::new(),
            exp: String::new(),
            result: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Caculator")
            .default_pos([50.0, 50.0])
            .title_bar(true)
            .max_width(1200.0)
            .max_height(1200.0)
            .default_size([600.0, 500.0])
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.exp);
                });

                // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

                ui.horizontal(|ui| {
                    if ui.button("+").clicked() {
                        self.exp.push_str("+");
                    }
                    if ui.button("-").clicked() {
                        self.exp.push_str("-");
                    }
                    if ui.button("*").clicked() {
                        self.exp.push_str("*");
                    }
                    if ui.button("/").clicked() {
                        self.exp.push_str("/");
                    }
                    if ui.button("^").clicked() {
                        self.exp.push_str("^");
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("C").clicked() {
                        self.exp.clear();
                    }
                    if ui.button("(").clicked() {
                        self.exp.push_str("(");
                    }
                    if ui.button(")").clicked() {
                        self.exp.push_str(")");
                    }
                });

                ui.horizontal(|ui| {
                    for i in 7..=10 {
                        if ui.button(format!("{}", i)).clicked() {
                            self.exp.push_str(&i.to_string());
                        }
                    }
                });
                ui.horizontal(|ui| {
                    for i in 4..=7 {
                        if ui.button(format!("{}", i)).clicked() {
                            self.exp.push_str(&i.to_string());
                        }
                    }
                });
                ui.horizontal(|ui| {
                    for i in 1..=4 {
                        if ui.button(format!("{}", i)).clicked() {
                            self.exp.push_str(&i.to_string());
                        }
                    }
                });
                ui.horizontal(|ui| {
                    let btn = ui.button("0");
                    if btn.clicked() {
                        self.exp.push_str("0");
                    }

                    if ui.button(format!(".")).clicked() {
                        self.exp.push_str(".");
                    }
                    if ui.button("=").clicked() {
                        match Caculator::new(self.exp.clone()).calculate() {
                            Ok(v) => {
                                self.result = v.to_string();
                            }
                            Err(e) => {
                                self.result = e.to_string();
                            }
                        }
                    }
                });

                ui.label(format!("Hello '{}', age {}", self.name, self.age));
                ui.label(format!("calculate result: {}", self.result));
            });

        // self.keypad.show(ctx);
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        self.keypad.bump_events(ctx, raw_input);
    }
}
