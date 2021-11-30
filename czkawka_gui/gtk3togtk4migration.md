# Errors

`resize` is missing, xlib is required now(not sure about api) - TODO I used this for fitting window size to content, maybe similar method exists?  
`children` dialog no longer returns children of Notebook  
`set_relative_to` is missing in Popup    
`connect_clicked` missing on CheckButton    
`connect_key_release_event` and `connect_button_release_event` missing  
`join_group` -> `set_group`  

`connect_clicked` to e.g. `connect_move_focus`  
`opening_enter_function_ported` - have different name  
`connect_button_press_event` missing  
`add` change to `set_child(Some())`  
`gdk::EventType::DoubleButtonPress` missing  

`\n[^\n]+shadow-type[^\n]+\n`  
`\n[^\n]+caps-lock-warning[^\n]+\n`  
`\n[^\n]+resize-mode[^\n]+\n`  
`\n[^\n]+window-position[^\n]+\n`  
`\n[^\n]+type-hint[^\n]+\n`  
`\n[^\n]+layout-style[^\n]+\n`  
`\n[^\n]+action_area[^\n]+\n`  
`\n[^\n]+layout-style[^\n]+\n`  

` internal-child="vbox"` - remove this  
` internal-child="action_area"`

change `.parent` to `transient for`
## Remove
`use gtk4::WindowPosition;`
`[a-zA-Z_]+.set_position\(WindowPosition::Center\);`
`window_progress.resize(1, 1);`

# To find deprecated
`gtk4::WindowPosition`

# Refactor
`show_all` method is missing - all widgets are visible by default so remove all occurrences of this - TODO Probably some usages of show_all should be changed to only show  
Treepath no longer can be displayed with `{}`  
`set_select_function` don't need to be boxed  
`buffer.text()` returns valid text, so `match` can be removed around such statements


# Replace
`.add(` -> `.append(`
`connect_delete_event` -> `connect_close_request` , also remove second argument
`show_all` -> `show`
`gtk::` with `gtk4::`  
`RadioButton` with `CheckButton`, groups needs to be implemented manually  
`ButtonBox` with `Box`
`use gtk4::prelude::*;` with `use gtk4::prelude::*;use gtk4::prelude::*;use gtk4::Inhibit;` - looks that now this isn't in prelude, probably can be added also in GTK3

`.value(` with `.get(` - TreeModel - this will broke examples from below  
`scale_similarity_similar_images.get()` with `scale_similarity_similar_images.value()`   
`scale_similarity_similar_videos.get()` with `scale_similarity_similar_videos.value()`

`.buffer().unwrap()` with `.buffer()` - TextView always return valid thing

`.path(&iter).unwrap()` with `.path(&iter)` - Treemodel, looks that always return valid thing  
`model.path(&current_iter).unwrap()` with `model.path(&current_iter)` as above  
`model.path(&next_iter).unwrap()` with `model.path(&next_iter)`

`window_main.set_title("Czkawka")` with `window_main.set_title(Some("Czkawka"))` - title can be empty 