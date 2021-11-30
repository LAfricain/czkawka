use gtk4::prelude::*;



#[derive(Clone)]
pub struct GuiHeader {
    pub button_settings: gtk4::Button,
    pub button_app_info: gtk4::Button,
}

impl GuiHeader {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let button_settings: gtk4::Button = builder.object("button_settings").unwrap();
        let button_app_info: gtk4::Button = builder.object("button_app_info").unwrap();

        button_settings.set_tooltip_text(Some("Opens settings dialog"));
        button_app_info.set_tooltip_text(Some("Opens dialog with info about app"));

        Self { button_settings, button_app_info }
    }
}
