// This contains every structure (part) for the User Interface

// use std::{async_iter::from_iter, str::SplitWhitespace};

use eframe::egui::{Separator, Color32};

pub const PADDING :f32 = 5.0;

// Colors
const TEXT_COLOR: Color32 = Color32::from_rgb(205, 211, 221);


// Todo List :
pub struct TodoList {
    tasks: Vec<Task>,
}

struct Task {
    title : String,
    finished : bool,
}

impl TodoList {
    // Instaciate function
    pub fn new() -> TodoList {
        // Loops 20 times to create a placeholder task as 'a'
        let iter = (0..5).map(|a| Task {
            title: format!("Task {}", a),
            finished: true,
        });
        TodoList {
            tasks: Vec::from_iter(iter), // might be without < >
        }
    }
    // Render function
    pub fn render_task_list(&self, ui: &mut eframe::egui::Ui) {
        for t in &self.tasks {
            ui.add_space(PADDING);
            // render name
            let title_text = &t.title;
            ui.colored_label(TEXT_COLOR, title_text);
            
            // render checkbox
            ui.add_space(PADDING);
            
            // ui.add(doc_link_label("Checkbox", "checkbox"));  // Don't know what this is and it breaks app
            let mut checked = t.finished;
            ui.checkbox(&mut checked, "Finished?");
            
            // Seperator
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}

