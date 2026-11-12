use eframe::egui;
use std::time::Instant;

#[derive(Clone)]
struct Parameter {
    code: &'static str,
    name: &'static str,
    value: String,
    unit: &'static str,
}

struct Acs150App {
    running: bool,
    frequency_ref: f32,
    output_frequency: f32,
    direction_forward: bool,
    fault: Option<&'static str>,
    menu_open: bool,
    selected_parameter: usize,
    parameters: Vec<Parameter>,
    last_tick: Instant,
    status_message: String,
}

impl Default for Acs150App {
    fn default() -> Self {
        Self {
            running: false,
            frequency_ref: 25.0,
            output_frequency: 0.0,
            direction_forward: true,
            fault: None,
            menu_open: false,
            selected_parameter: 0,
            parameters: vec![
                Parameter { code: "9902", name: "Application macro", value: "ABB STANDARD".into(), unit: "" },
                Parameter { code: "9905", name: "Motor nominal voltage", value: "230".into(), unit: "V" },
                Parameter { code: "9906", name: "Motor nominal current", value: "1.5".into(), unit: "A" },
                Parameter { code: "9907", name: "Motor nominal frequency", value: "50.0".into(), unit: "Hz" },
                Parameter { code: "9908", name: "Motor nominal speed", value: "1420".into(), unit: "rpm" },
                Parameter { code: "1001", name: "EXT1 commands", value: "KEYPAD".into(), unit: "" },
                Parameter { code: "1103", name: "Reference select", value: "KEYPAD".into(), unit: "" },
                Parameter { code: "2007", name: "Minimum frequency", value: "0.0".into(), unit: "Hz" },
                Parameter { code: "2008", name: "Maximum frequency", value: "50.0".into(), unit: "Hz" },
            ],
            last_tick: Instant::now(),
            status_message: "Simulation ready".into(),
        }
    }
}

impl Acs150App {
    fn start(&mut self) {
        if self.fault.is_some() {
            self.status_message = "Clear the active fault with RESET first.".into();
        } else {
            self.running = true;
            self.status_message = "Drive started (simulation only).".into();
        }
    }

    fn stop(&mut self) {
        self.running = false;
        self.status_message = "Drive stopped.".into();
    }

    fn reset(&mut self) {
        self.fault = None;
        self.running = false;
        self.output_frequency = 0.0;
        self.status_message = "Fault reset; drive stopped.".into();
    }

    fn raise_fault(&mut self, code: &'static str) {
        self.fault = Some(code);
        self.running = false;
        self.output_frequency = 0.0;
        self.status_message = format!("Fault {code} simulated.");
    }

    fn tick(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f32().min(0.25);
        self.last_tick = now;
        let target = if self.running && self.fault.is_none() { self.frequency_ref } else { 0.0 };
        let ramp_hz_per_sec = 8.0;
        let step = ramp_hz_per_sec * dt;
        if self.output_frequency < target {
            self.output_frequency = (self.output_frequency + step).min(target);
        } else if self.output_frequency > target {
            self.output_frequency = (self.output_frequency - step).max(target);
        }
    }
}

impl eframe::App for Acs150App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tick();
        ctx.request_repaint_after(std::time::Duration::from_millis(80));

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("ABB ACS150 Simulator");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("RUST • WINDOWS • EDUCATIONAL EDITION")
                        .color(egui::Color32::from_rgb(226, 184, 75)).strong());
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            ui.columns(2, |cols| {
                let left = &mut cols[0];
                left.heading("Local Control Panel (LCP)");
                left.add_space(8.0);

                egui::Frame::group(left.style())
                    .fill(egui::Color32::from_rgb(190, 207, 164))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::DARK_GRAY))
                    .inner_margin(egui::Margin::same(16.0))
                    .show(left, |ui| {
                        ui.set_min_height(145.0);
                        let dark = egui::Color32::from_rgb(27, 43, 27);
                        if let Some(code) = self.fault {
                            ui.label(egui::RichText::new(format!("FAULT  {code}")).color(dark).monospace().size(21.0).strong());
                            ui.label(egui::RichText::new("RESET REQUIRED").color(dark).monospace().size(17.0));
                        } else if self.menu_open {
                            let p = &self.parameters[self.selected_parameter];
                            ui.label(egui::RichText::new(format!("PAR {}  {}", p.code, p.name)).color(dark).monospace().size(17.0).strong());
                            ui.label(egui::RichText::new(format!("VALUE: {} {}", p.value, p.unit)).color(dark).monospace().size(18.0));
                        } else {
                            ui.label(egui::RichText::new(format!("{}   {}   LOC",
                                if self.running { "RUN" } else { "STOP" },
                                if self.direction_forward { "FWD" } else { "REV" }))
                                .color(dark).monospace().size(21.0).strong());
                            ui.label(egui::RichText::new(format!("REF  {:05.1} Hz", self.frequency_ref)).color(dark).monospace().size(19.0));
                        }
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new(format!("OUT  {:05.1} Hz", self.output_frequency)).color(dark).monospace().size(19.0));
                        ui.label(egui::RichText::new(if self.menu_open { "UP/DOWN SELECT • ENTER" } else { "MENU: PARAMETERS" }).color(dark).monospace());
                    });

                left.add_space(12.0);
                left.horizontal(|ui| {
                    if ui.button("MENU").clicked() { self.menu_open = !self.menu_open; }
                    if ui.button("▲").clicked() {
                        if self.menu_open { self.selected_parameter = (self.selected_parameter + self.parameters.len() - 1) % self.parameters.len(); }
                        else { self.frequency_ref = (self.frequency_ref + 1.0).min(50.0); }
                    }
                    if ui.button("▼").clicked() {
                        if self.menu_open { self.selected_parameter = (self.selected_parameter + 1) % self.parameters.len(); }
                        else { self.frequency_ref = (self.frequency_ref - 1.0).max(0.0); }
                    }
                    if ui.button("ENTER").clicked() {
                        self.status_message = "Parameter editing is planned for the next version.".into();
                    }
                });
                left.add_space(14.0);
                left.horizontal(|ui| {
                    if ui.add_sized([105.0, 38.0], egui::Button::new(egui::RichText::new("HAND / START").strong().color(egui::Color32::BLACK)).fill(egui::Color32::from_rgb(105, 198, 139))).clicked() { self.start(); }
                    if ui.add_sized([75.0, 38.0], egui::Button::new(egui::RichText::new("STOP").strong()).fill(egui::Color32::from_rgb(220, 105, 100))).clicked() { self.stop(); }
                    if ui.add_sized([75.0, 38.0], egui::Button::new(egui::RichText::new("RESET").strong().color(egui::Color32::BLACK)).fill(egui::Color32::from_rgb(226, 184, 75))).clicked() { self.reset(); }
                });
                left.add_space(16.0);
                left.checkbox(&mut self.direction_forward, "Forward direction (unchecked = reverse)");

                let right = &mut cols[1];
                right.heading("Drive monitor");
                right.add_space(8.0);
                let state = if let Some(code) = self.fault { format!("FAULT {code}") } else if self.running { "RUNNING".into() } else { "STOPPED".into() };
                right.label(egui::RichText::new(state).size(19.0).strong().color(
                    if self.fault.is_some() { egui::Color32::RED } else if self.running { egui::Color32::GREEN } else { egui::Color32::GRAY }
                ));
                right.add_space(10.0);
                right.label(format!("Frequency reference: {:.1} Hz", self.frequency_ref));
                right.add(egui::Slider::new(&mut self.frequency_ref, 0.0..=50.0).suffix(" Hz"));
                right.add_space(6.0);
                right.label(format!("Output frequency: {:.1} Hz", self.output_frequency));
                right.label(format!("Direction: {}", if self.direction_forward { "FWD" } else { "REV" }));
                right.label("Control source: LOCAL");
                right.add_space(18.0);
                right.heading("Fault simulation");
                if right.button("Overcurrent — F0001").clicked() { self.raise_fault("F0001"); }
                if right.button("DC overvoltage — F0002").clicked() { self.raise_fault("F0002"); }
                if right.button("Motor overtemperature — F0004").clicked() { self.raise_fault("F0004"); }
                right.add_space(14.0);
                right.heading("Parameter list");
                egui::ScrollArea::vertical().max_height(170.0).show(right, |ui| {
                    for (i, p) in self.parameters.iter().enumerate() {
                        let label = format!("{}  {} = {} {}", p.code, p.name, p.value, p.unit);
                        if ui.selectable_label(self.selected_parameter == i, label).clicked() {
                            self.selected_parameter = i;
                            self.menu_open = true;
                        }
                    }
                });
            });
            ui.add_space(12.0);
            ui.separator();
            ui.label(&self.status_message);
            ui.colored_label(egui::Color32::from_rgb(220, 175, 75),
                "Educational simulation only — no connection to a physical ABB ACS150 drive.");
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([980.0, 680.0])
            .with_min_inner_size([820.0, 580.0])
            .with_title("ACS150 Simulator"),
        ..Default::default()
    };
    eframe::run_native(
        "ACS150 Simulator",
        options,
        Box::new(|_cc| Box::new(Acs150App::default())),
    )
}
