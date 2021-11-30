use gtk4::prelude::*;
use gtk4::prelude::*;
use gtk4::Builder;
use gtk4::Inhibit;

#[derive(Clone)]
pub struct GuiAbout {
    pub about_dialog: gtk4::AboutDialog,

    pub button_repository: gtk4::Button,
    pub button_donation: gtk4::Button,
    pub button_instruction: gtk4::Button,
}

impl GuiAbout {
    pub fn create_from_builder() -> Self {
        let glade_src = include_str!("../ui/about_dialog.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let about_dialog: gtk4::AboutDialog = builder.object("about_dialog").unwrap();

        let button_repository: gtk4::Button = builder.object("button_repository").unwrap();
        button_repository.set_tooltip_text(Some("Link to repository page with source code."));
        let button_donation: gtk4::Button = builder.object("button_donation").unwrap();
        button_donation.set_tooltip_text(Some("Link to donation page."));
        let button_instruction: gtk4::Button = builder.object("button_instruction").unwrap();
        button_instruction.set_tooltip_text(Some("Link to instruction page."));

        Self {
            about_dialog,
            button_repository,
            button_donation,
            button_instruction,
        }
    }
}
