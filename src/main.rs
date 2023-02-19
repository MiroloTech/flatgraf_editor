// Import Customs and Dependancies
mod ui_structures;
use eframe::{NativeOptions, run_native, egui::{Vec2, CentralPanel, ScrollArea}, epi::App};
use ui_structures::{TodoList};

const WINDOW_SIZE:Vec2 = Vec2::new(540.0, 960.0);

impl App for TodoList {
    /*
    fn setup( &mut self,  ctx: &eframe::egui::CtxRef,  _frame: &mut eframe::epi::Frame<'_>,  _storage: Option<&dyn eframe::epi::Storage>, ) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_task_list(ui);
            });
        });
    }
    */
    
    // Get App Update
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        // UI in Container
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_task_list(ui);
            });
        });
    }
    
    
    // Get App name
    fn name(&self) -> &str {
        "TodoList"
    }
}

fn main() {
    // Instance Application
    let app = TodoList::new();
    let mut win_data = NativeOptions::default();
    win_data.initial_window_size = Some(WINDOW_SIZE);
    // Run App
    run_native(Box::new(app), win_data);
}


