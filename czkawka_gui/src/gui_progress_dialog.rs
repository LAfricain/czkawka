use gtk4::prelude::*;

use gtk4::{Builder, Window};

#[derive(Clone)]
pub struct GuiProgressDialog {
    pub window_progress: gtk4::Window,

    pub progress_bar_current_stage: gtk4::ProgressBar,
    pub progress_bar_all_stages: gtk4::ProgressBar,

    pub label_stage: gtk4::Label,

    pub grid_progress_stages: gtk4::Grid,

    pub button_stop_in_dialog: gtk4::Button,
}

impl GuiProgressDialog {
    pub fn create_from_builder(window_main: &Window) -> Self {
        let glade_src = include_str!("../ui/progress.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let window_progress: gtk4::Window = builder.object("window_progress").unwrap();
        window_progress.set_transient_for(Some(window_main));
        window_progress.set_modal(true);

        let progress_bar_current_stage: gtk4::ProgressBar = builder.object("progress_bar_current_stage").unwrap();
        let progress_bar_all_stages: gtk4::ProgressBar = builder.object("progress_bar_all_stages").unwrap();

        let label_stage: gtk4::Label = builder.object("label_stage").unwrap();

        let grid_progress_stages: gtk4::Grid = builder.object("grid_progress_stages").unwrap();

        let button_stop_in_dialog: gtk4::Button = builder.object("button_stop_in_dialog").unwrap();

        Self {
            window_progress,
            progress_bar_current_stage,
            progress_bar_all_stages,
            label_stage,
            grid_progress_stages,
            button_stop_in_dialog,
        }
    }
}
