use gtk4::prelude::*;

use gtk4::{EventControllerKey, TreeView};

use crate::notebook_enums::NUMBER_OF_NOTEBOOK_MAIN_TABS;

#[derive(Clone)]
pub struct GuiMainNotebook {
    pub notebook_main: gtk4::Notebook,

    pub scrolled_window_duplicate_finder: gtk4::ScrolledWindow,
    pub scrolled_window_empty_folder_finder: gtk4::ScrolledWindow,
    pub scrolled_window_empty_files_finder: gtk4::ScrolledWindow,
    pub scrolled_window_temporary_files_finder: gtk4::ScrolledWindow,
    pub scrolled_window_big_files_finder: gtk4::ScrolledWindow,
    pub scrolled_window_similar_images_finder: gtk4::ScrolledWindow,
    pub scrolled_window_similar_videos_finder: gtk4::ScrolledWindow,
    pub scrolled_window_same_music_finder: gtk4::ScrolledWindow,
    pub scrolled_window_invalid_symlinks: gtk4::ScrolledWindow,
    pub scrolled_window_broken_files: gtk4::ScrolledWindow,

    pub tree_view_duplicate_finder: gtk4::TreeView,
    pub tree_view_empty_folder_finder: gtk4::TreeView,
    pub tree_view_empty_files_finder: gtk4::TreeView,
    pub tree_view_temporary_files_finder: gtk4::TreeView,
    pub tree_view_big_files_finder: gtk4::TreeView,
    pub tree_view_similar_images_finder: gtk4::TreeView,
    pub tree_view_similar_videos_finder: gtk4::TreeView,
    pub tree_view_same_music_finder: gtk4::TreeView,
    pub tree_view_invalid_symlinks: gtk4::TreeView,
    pub tree_view_broken_files: gtk4::TreeView,

    pub evk_tree_view_duplicate_finder: gtk4::EventControllerKey, // TODO, in GTK4 this can be changed to e.g. add_widget - https://discourse.gnome.org/t/how-to-convert-code-to-use-eventcontrollerkey/8198/2
    pub evk_tree_view_empty_folder_finder: gtk4::EventControllerKey,
    pub evk_tree_view_empty_files_finder: gtk4::EventControllerKey,
    pub evk_tree_view_temporary_files_finder: gtk4::EventControllerKey,
    pub evk_tree_view_big_files_finder: gtk4::EventControllerKey,
    pub evk_tree_view_similar_images_finder: gtk4::EventControllerKey,
    pub evk_tree_view_similar_videos_finder: gtk4::EventControllerKey,
    pub evk_tree_view_same_music_finder: gtk4::EventControllerKey,
    pub evk_tree_view_invalid_symlinks: gtk4::EventControllerKey,
    pub evk_tree_view_broken_files: gtk4::EventControllerKey,

    pub entry_similar_images_minimal_size: gtk4::Entry,
    pub entry_similar_images_maximal_size: gtk4::Entry,
    pub entry_similar_videos_minimal_size: gtk4::Entry,
    pub entry_similar_videos_maximal_size: gtk4::Entry,
    pub entry_duplicate_minimal_size: gtk4::Entry,
    pub entry_duplicate_maximal_size: gtk4::Entry,
    pub entry_same_music_minimal_size: gtk4::Entry,
    pub entry_same_music_maximal_size: gtk4::Entry,

    pub entry_big_files_number: gtk4::Entry,

    //// Check Buttons
    pub check_button_music_title: gtk4::CheckButton,
    pub check_button_music_artist: gtk4::CheckButton,
    pub check_button_music_album_title: gtk4::CheckButton,
    pub check_button_music_album_artist: gtk4::CheckButton,
    pub check_button_music_year: gtk4::CheckButton,

    //// Radio Buttons
    // Duplicates
    pub radio_button_duplicates_name: gtk4::CheckButton,
    pub radio_button_duplicates_size: gtk4::CheckButton,
    pub radio_button_duplicates_hashmb: gtk4::CheckButton,
    pub radio_button_duplicates_hash: gtk4::CheckButton,

    pub scale_similarity_similar_images: gtk4::Scale,
    pub scale_similarity_similar_videos: gtk4::Scale,

    pub radio_button_hash_type_blake3: gtk4::CheckButton,
    pub radio_button_hash_type_crc32: gtk4::CheckButton,
    pub radio_button_hash_type_xxh3: gtk4::CheckButton,

    pub radio_button_resize_algorithm_lanczos3: gtk4::CheckButton,
    pub radio_button_resize_algorithm_nearest: gtk4::CheckButton,
    pub radio_button_resize_algorithm_triangle: gtk4::CheckButton,
    pub radio_button_resize_algorithm_gaussian: gtk4::CheckButton,
    pub radio_button_resize_algorithm_catmullrom: gtk4::CheckButton,

    pub radio_button_similar_hash_algorithm_gradient: gtk4::CheckButton,
    pub radio_button_similar_hash_algorithm_blockhash: gtk4::CheckButton,
    pub radio_button_similar_hash_algorithm_mean: gtk4::CheckButton,
    pub radio_button_similar_hash_algorithm_vertgradient: gtk4::CheckButton,
    pub radio_button_similar_hash_algorithm_doublegradient: gtk4::CheckButton,

    pub radio_button_similar_hash_size_4: gtk4::CheckButton,
    pub radio_button_similar_hash_size_8: gtk4::CheckButton,
    pub radio_button_similar_hash_size_16: gtk4::CheckButton,

    pub image_preview_similar_images: gtk4::Image,
    pub image_preview_duplicates: gtk4::Image,
}

impl GuiMainNotebook {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let notebook_main: gtk4::Notebook = builder.object("notebook_main").unwrap();

        let scrolled_window_duplicate_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_duplicate_finder").unwrap();
        let scrolled_window_empty_folder_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_empty_folder_finder").unwrap();
        let scrolled_window_empty_files_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_empty_files_finder").unwrap();
        let scrolled_window_temporary_files_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_temporary_files_finder").unwrap();
        let scrolled_window_big_files_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_big_files_finder").unwrap();
        let scrolled_window_similar_images_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_similar_images_finder").unwrap();
        let scrolled_window_similar_videos_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_similar_videos_finder").unwrap();
        let scrolled_window_same_music_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_same_music_finder").unwrap();
        let scrolled_window_invalid_symlinks: gtk4::ScrolledWindow = builder.object("scrolled_window_invalid_symlinks").unwrap();
        let scrolled_window_broken_files: gtk4::ScrolledWindow = builder.object("scrolled_window_broken_files").unwrap();

        let tree_view_duplicate_finder: gtk4::TreeView = TreeView::new();
        tree_view_duplicate_finder.set_widget_name("PIERD");
        let tree_view_empty_folder_finder: gtk4::TreeView = TreeView::new();
        let tree_view_empty_files_finder: gtk4::TreeView = TreeView::new();
        let tree_view_temporary_files_finder: gtk4::TreeView = TreeView::new();
        let tree_view_big_files_finder: gtk4::TreeView = TreeView::new();
        let tree_view_similar_images_finder: gtk4::TreeView = TreeView::new();
        let tree_view_similar_videos_finder: gtk4::TreeView = TreeView::new();
        let tree_view_same_music_finder: gtk4::TreeView = TreeView::new();
        let tree_view_invalid_symlinks: gtk4::TreeView = TreeView::new();
        let tree_view_broken_files: gtk4::TreeView = TreeView::new();

        let evk_tree_view_duplicate_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_duplicate_finder.add_controller(&evk_tree_view_duplicate_finder);
        let evk_tree_view_empty_folder_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_empty_folder_finder.add_controller(&evk_tree_view_empty_folder_finder);
        let evk_tree_view_empty_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_empty_files_finder.add_controller(&evk_tree_view_empty_files_finder);
        let evk_tree_view_temporary_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_temporary_files_finder.add_controller(&evk_tree_view_temporary_files_finder);
        let evk_tree_view_big_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_big_files_finder.add_controller(&evk_tree_view_big_files_finder);
        let evk_tree_view_similar_images_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_similar_images_finder.add_controller(&evk_tree_view_similar_images_finder);
        let evk_tree_view_similar_videos_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_similar_videos_finder.add_controller(&evk_tree_view_similar_videos_finder);
        let evk_tree_view_same_music_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_same_music_finder.add_controller(&evk_tree_view_same_music_finder);
        let evk_tree_view_invalid_symlinks: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_invalid_symlinks.add_controller(&evk_tree_view_invalid_symlinks);
        let evk_tree_view_broken_files: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_broken_files.add_controller(&evk_tree_view_broken_files);

        let entry_similar_images_minimal_size: gtk4::Entry = builder.object("entry_similar_images_minimal_size").unwrap();
        let entry_similar_images_maximal_size: gtk4::Entry = builder.object("entry_similar_images_maximal_size").unwrap();
        let entry_similar_videos_minimal_size: gtk4::Entry = builder.object("entry_similar_videos_minimal_size").unwrap();
        let entry_similar_videos_maximal_size: gtk4::Entry = builder.object("entry_similar_videos_maximal_size").unwrap();
        let entry_duplicate_minimal_size: gtk4::Entry = builder.object("entry_duplicate_minimal_size").unwrap();
        let entry_duplicate_maximal_size: gtk4::Entry = builder.object("entry_duplicate_maximal_size").unwrap();
        let entry_same_music_minimal_size: gtk4::Entry = builder.object("entry_same_music_minimal_size").unwrap();
        let entry_same_music_maximal_size: gtk4::Entry = builder.object("entry_same_music_maximal_size").unwrap();

        let entry_big_files_number: gtk4::Entry = builder.object("entry_big_files_number").unwrap();

        //// Check Buttons
        let check_button_music_title: gtk4::CheckButton = builder.object("check_button_music_title").unwrap();
        let check_button_music_artist: gtk4::CheckButton = builder.object("check_button_music_artist").unwrap();
        let check_button_music_album_title: gtk4::CheckButton = builder.object("check_button_music_album_title").unwrap();
        let check_button_music_album_artist: gtk4::CheckButton = builder.object("check_button_music_album_artist").unwrap();
        let check_button_music_year: gtk4::CheckButton = builder.object("check_button_music_year").unwrap();

        //// Radio Buttons
        let radio_button_duplicates_name: gtk4::CheckButton = builder.object("radio_button_duplicates_name").unwrap();
        let radio_button_duplicates_size: gtk4::CheckButton = builder.object("radio_button_duplicates_size").unwrap();
        let radio_button_duplicates_hashmb: gtk4::CheckButton = builder.object("radio_button_duplicates_hashmb").unwrap();
        let radio_button_duplicates_hash: gtk4::CheckButton = builder.object("radio_button_duplicates_hash").unwrap();

        let scale_similarity_similar_images: gtk4::Scale = builder.object("scale_similarity_similar_images").unwrap();
        let scale_similarity_similar_videos: gtk4::Scale = builder.object("scale_similarity_similar_videos").unwrap();

        let radio_button_hash_type_blake3: gtk4::CheckButton = builder.object("radio_button_hash_type_blake3").unwrap();
        let radio_button_hash_type_crc32: gtk4::CheckButton = builder.object("radio_button_hash_type_crc32").unwrap();
        let radio_button_hash_type_xxh3: gtk4::CheckButton = builder.object("radio_button_hash_type_xxh3").unwrap();

        let radio_button_resize_algorithm_lanczos3: gtk4::CheckButton = builder.object("radio_button_resize_algorithm_lanczos3").unwrap();
        let radio_button_resize_algorithm_nearest: gtk4::CheckButton = builder.object("radio_button_resize_algorithm_nearest").unwrap();
        let radio_button_resize_algorithm_triangle: gtk4::CheckButton = builder.object("radio_button_resize_algorithm_triangle").unwrap();
        let radio_button_resize_algorithm_gaussian: gtk4::CheckButton = builder.object("radio_button_resize_algorithm_gaussian").unwrap();
        let radio_button_resize_algorithm_catmullrom: gtk4::CheckButton = builder.object("radio_button_resize_algorithm_catmullrom").unwrap();

        let radio_button_similar_hash_algorithm_gradient: gtk4::CheckButton = builder.object("radio_button_similar_hash_algorithm_gradient").unwrap();
        let radio_button_similar_hash_algorithm_blockhash: gtk4::CheckButton = builder.object("radio_button_similar_hash_algorithm_blockhash").unwrap();
        let radio_button_similar_hash_algorithm_mean: gtk4::CheckButton = builder.object("radio_button_similar_hash_algorithm_mean").unwrap();
        let radio_button_similar_hash_algorithm_vertgradient: gtk4::CheckButton = builder.object("radio_button_similar_hash_algorithm_vertgradient").unwrap();
        let radio_button_similar_hash_algorithm_doublegradient: gtk4::CheckButton = builder.object("radio_button_similar_hash_algorithm_doublegradient").unwrap();

        let radio_button_similar_hash_size_4: gtk4::CheckButton = builder.object("radio_button_similar_hash_size_4").unwrap();
        let radio_button_similar_hash_size_8: gtk4::CheckButton = builder.object("radio_button_similar_hash_size_8").unwrap();
        let radio_button_similar_hash_size_16: gtk4::CheckButton = builder.object("radio_button_similar_hash_size_16").unwrap();

        let image_preview_similar_images: gtk4::Image = builder.object("image_preview_similar_images").unwrap();
        let image_preview_duplicates: gtk4::Image = builder.object("image_preview_duplicates").unwrap();

        Self {
            notebook_main,
            scrolled_window_duplicate_finder,
            scrolled_window_empty_folder_finder,
            scrolled_window_empty_files_finder,
            scrolled_window_temporary_files_finder,
            scrolled_window_big_files_finder,
            scrolled_window_similar_images_finder,
            scrolled_window_similar_videos_finder,
            scrolled_window_same_music_finder,
            scrolled_window_invalid_symlinks,
            scrolled_window_broken_files,
            tree_view_duplicate_finder,
            tree_view_empty_folder_finder,
            tree_view_empty_files_finder,
            tree_view_temporary_files_finder,
            tree_view_big_files_finder,
            tree_view_similar_images_finder,
            tree_view_similar_videos_finder,
            tree_view_same_music_finder,
            tree_view_invalid_symlinks,
            tree_view_broken_files,
            evk_tree_view_duplicate_finder,
            evk_tree_view_empty_folder_finder,
            evk_tree_view_empty_files_finder,
            evk_tree_view_temporary_files_finder,
            evk_tree_view_big_files_finder,
            evk_tree_view_similar_images_finder,
            evk_tree_view_similar_videos_finder,
            evk_tree_view_same_music_finder,
            evk_tree_view_invalid_symlinks,
            evk_tree_view_broken_files,
            entry_similar_images_minimal_size,
            entry_similar_images_maximal_size,
            entry_similar_videos_minimal_size,
            entry_similar_videos_maximal_size,
            entry_duplicate_minimal_size,
            entry_big_files_number,
            entry_same_music_minimal_size,
            check_button_music_title,
            check_button_music_artist,
            check_button_music_album_title,
            check_button_music_album_artist,
            check_button_music_year,
            radio_button_duplicates_name,
            radio_button_duplicates_size,
            radio_button_duplicates_hashmb,
            radio_button_duplicates_hash,
            scale_similarity_similar_images,
            scale_similarity_similar_videos,
            radio_button_hash_type_blake3,
            radio_button_hash_type_crc32,
            radio_button_hash_type_xxh3,
            radio_button_resize_algorithm_lanczos3,
            radio_button_resize_algorithm_nearest,
            radio_button_resize_algorithm_triangle,
            radio_button_resize_algorithm_gaussian,
            radio_button_resize_algorithm_catmullrom,
            radio_button_similar_hash_algorithm_gradient,
            radio_button_similar_hash_algorithm_blockhash,
            radio_button_similar_hash_algorithm_mean,
            radio_button_similar_hash_algorithm_vertgradient,
            radio_button_similar_hash_algorithm_doublegradient,
            radio_button_similar_hash_size_4,
            radio_button_similar_hash_size_8,
            radio_button_similar_hash_size_16,
            image_preview_similar_images,
            entry_duplicate_maximal_size,
            entry_same_music_maximal_size,
            image_preview_duplicates,
        }
    }

    pub fn get_main_tree_views(&self) -> [TreeView; NUMBER_OF_NOTEBOOK_MAIN_TABS] {
        [
            self.tree_view_duplicate_finder.clone(),
            self.tree_view_empty_folder_finder.clone(),
            self.tree_view_big_files_finder.clone(),
            self.tree_view_empty_files_finder.clone(),
            self.tree_view_temporary_files_finder.clone(),
            self.tree_view_similar_images_finder.clone(),
            self.tree_view_similar_videos_finder.clone(),
            self.tree_view_same_music_finder.clone(),
            self.tree_view_invalid_symlinks.clone(),
            self.tree_view_broken_files.clone(),
        ]
    }
}
