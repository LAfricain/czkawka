use gtk4::prelude::*;

use gtk4::{EventControllerKey, TreeView};

#[derive(Clone)]
pub struct GuiUpperNotebook {
    pub notebook_upper: gtk4::Notebook,

    pub scrolled_window_included_directories: gtk4::ScrolledWindow,
    pub scrolled_window_excluded_directories: gtk4::ScrolledWindow,

    pub tree_view_included_directories: gtk4::TreeView,
    pub tree_view_excluded_directories: gtk4::TreeView,

    pub evk_tree_view_included_directories: gtk4::EventControllerKey,
    pub evk_tree_view_excluded_directories: gtk4::EventControllerKey,

    pub entry_excluded_items: gtk4::Entry,
    pub entry_allowed_extensions: gtk4::Entry,

    pub check_button_recursive: gtk4::CheckButton,

    pub buttons_manual_add_directory: gtk4::Button,
    pub buttons_add_included_directory: gtk4::Button,
    pub buttons_remove_included_directory: gtk4::Button,
    pub buttons_manual_add_excluded_directory: gtk4::Button,
    pub buttons_add_excluded_directory: gtk4::Button,
    pub buttons_remove_excluded_directory: gtk4::Button,
}

impl GuiUpperNotebook {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let notebook_upper: gtk4::Notebook = builder.object("notebook_upper").unwrap();

        let scrolled_window_included_directories: gtk4::ScrolledWindow = builder.object("scrolled_window_included_directories").unwrap();
        let scrolled_window_excluded_directories: gtk4::ScrolledWindow = builder.object("scrolled_window_excluded_directories").unwrap();

        let tree_view_included_directories: gtk4::TreeView = TreeView::new();
        let tree_view_excluded_directories: gtk4::TreeView = TreeView::new();

        let evk_tree_view_included_directories: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_included_directories.add_controller(&evk_tree_view_included_directories);
        let evk_tree_view_excluded_directories: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_excluded_directories.add_controller(&evk_tree_view_excluded_directories);

        let entry_allowed_extensions: gtk4::Entry = builder.object("entry_allowed_extensions").unwrap();
        let entry_excluded_items: gtk4::Entry = builder.object("entry_excluded_items").unwrap();

        let check_button_recursive: gtk4::CheckButton = builder.object("check_button_recursive").unwrap();
        check_button_recursive.set_tooltip_text(Some("If selected, search also for files which are not placed directly under chosen folders"));

        let buttons_manual_add_directory: gtk4::Button = builder.object("buttons_manual_add_directory").unwrap();
        let buttons_add_included_directory: gtk4::Button = builder.object("buttons_add_included_directory").unwrap();
        let buttons_remove_included_directory: gtk4::Button = builder.object("buttons_remove_included_directory").unwrap();
        let buttons_manual_add_excluded_directory: gtk4::Button = builder.object("buttons_manual_add_excluded_directory").unwrap();
        let buttons_add_excluded_directory: gtk4::Button = builder.object("buttons_add_excluded_directory").unwrap();
        let buttons_remove_excluded_directory: gtk4::Button = builder.object("buttons_remove_excluded_directory").unwrap();

        buttons_manual_add_directory.set_tooltip_text(Some("Allows to add directory name by hand"));
        buttons_add_included_directory.set_tooltip_text(Some("Add new directory to search"));
        buttons_remove_included_directory.set_tooltip_text(Some("Delete directory from search"));
        buttons_manual_add_excluded_directory.set_tooltip_text(Some("Allows to add directory name by hand"));
        buttons_add_excluded_directory.set_tooltip_text(Some("Add directory to be excluded in search"));
        buttons_remove_excluded_directory.set_tooltip_text(Some("Delete directory from excluded list"));

        Self {
            notebook_upper,
            scrolled_window_included_directories,
            scrolled_window_excluded_directories,
            tree_view_included_directories,
            tree_view_excluded_directories,
            evk_tree_view_included_directories,
            evk_tree_view_excluded_directories,
            entry_excluded_items,
            entry_allowed_extensions,
            check_button_recursive,
            buttons_manual_add_directory,
            buttons_add_included_directory,
            buttons_remove_included_directory,
            buttons_manual_add_excluded_directory,
            buttons_add_excluded_directory,
            buttons_remove_excluded_directory,
        }
    }
}
