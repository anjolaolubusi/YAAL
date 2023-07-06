mod applications;
use std::process::{Command, self};
use std::{io};
use applications::ApplicationEntry;
use eframe::egui;
use eframe::emath::Align2;
use eframe::epaint::Color32;

fn main() {
    let native_options = eframe::NativeOptions{
        always_on_top: true,
        centered: true,
        fullscreen: true,
        transparent: true,
        ..Default::default()
    };
    
    eframe::run_native("YAAL", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));

    // // Executes command (We set the output and error to /dev/null so we are not waiting on the program's output)
    // Command::new(&mut execute_command.pop().unwrap().command)
    // .stdout(process::Stdio::null())
    // .stderr(process::Stdio::null())
    // .spawn();
}


#[derive(Default)]
struct MyEguiApp {
    applications: Vec<ApplicationEntry>,
    user_input: String,
    isTyping: bool
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    fn default() -> Self {
        let list_of_dirs = applications::get_desktop_dirs();
        Self {
            applications: applications::get_applications(&list_of_dirs),
            user_input: "".to_owned(),
            isTyping: false
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let list_of_events = ctx.input( |i| {
            i.events.clone()
        });

        self.isTyping = ctx.input(|i| {
            i.events
                .iter()
                .any(|ev| match ev {
                   egui::Event::Text(_) => true,
                   _ => false
                })
        });
            

        egui::Area::new("userInput")
            .movable(false)
            .pivot(Align2::LEFT_TOP)
            .show(&ctx, |ui| {
                ui.style_mut().visuals.extreme_bg_color = Color32::from_black_alpha(10);
                let textInput = 
                    egui::TextEdit::singleline(&mut self.user_input)
                    .code_editor();
                


                let response = ui.add_sized([300.0, 20.0], textInput);
                if self.isTyping {
                    response.request_focus();
                    let new_text = list_of_events.iter().find(|ev| match ev {
                        egui::Event::Text(_) => true,
                        _ => false
                    });
                    if let egui::Event::Text(val) = new_text.unwrap(){
                        if !response.changed() {
                            self.user_input += val;
                            if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), response.id) {
                            let ccursor = egui::text::CCursor::new(self.user_input.chars().count());
                            state.set_ccursor_range(Some(egui::text::CCursorRange::one(ccursor)));
                            state.store(ui.ctx(), response.id);
                            response.request_focus();
                            }

                        }
                    }
                } 

            });

        egui::Area::new("ListOfPrograms")
            .movable(false)
            .anchor(Align2::CENTER_CENTER, [10.0, 10.0])
            .show(&ctx, |ui| {
                ui.style_mut().visuals.extreme_bg_color = Color32::from_black_alpha(32);
                for app in &self.applications{
                    let button = egui::Button::new(&app.application_name);
                    let response = ui.add(button);
                    if response.clicked() {
                        println!("{} was clicked", &app.application_name);
                        frame.close();
                    }
                }
            });
   }
}