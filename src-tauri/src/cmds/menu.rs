use gdk::keys::constants;
use gdk::{keys::Key, Display, ModifierType};
use gtk::{prelude::*, traits::WidgetExt, AccelFlags, AccelGroup, Menu};
use serde::Deserialize;
use std::collections::HashMap;
use std::{env, mem, thread::sleep, time};
use tauri::{Emitter, Runtime, Window};

#[derive(Clone, Deserialize)]
pub struct MenuItem {
    pub label: Option<String>,
    pub disabled: Option<bool>,
    pub shortcut: Option<String>,
    pub event: Option<String>,
    pub payload: Option<String>,
    pub subitems: Option<Vec<MenuItem>>,
    pub icon: Option<MenuItemIcon>,
    pub checked: Option<bool>,
    pub is_separator: Option<bool>,
}

#[derive(Clone, Deserialize)]
pub struct MenuItemIcon {
    pub path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Clone, Deserialize)]
pub struct Position {
    x: f64,
    y: f64,
    is_absolute: Option<bool>,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self {
            label: None,
            disabled: Some(false),
            shortcut: None,
            event: None,
            payload: None,
            subitems: None,
            icon: None,
            checked: Some(false),
            is_separator: Some(false),
        }
    }
}
pub fn get_key_map() -> HashMap<&'static str, gdk::keys::Key> {
    let mut key_map = HashMap::new();
    key_map.insert("cmd", constants::Control_L); // Alias for "ctrl"
    key_map.insert("cmd_or_ctrl", constants::Control_L); // Alias for "ctrl"
    key_map.insert("shift", constants::Shift_L);
    key_map.insert("alt", constants::Alt_L);
    key_map.insert("ctrl", constants::Control_L);
    key_map.insert("opt", constants::Alt_L); // Alias for "alt"
    key_map.insert("altgr", constants::Alt_R);
    key_map.insert("super", constants::Super_L);
    key_map.insert("win", constants::Super_L);
    key_map.insert("meta", constants::Super_L);
    key_map.insert("plus", constants::plus);
    key_map.insert("space", constants::space);
    key_map.insert("tab", constants::Tab);
    key_map.insert("capslock", constants::Caps_Lock);
    key_map.insert("numlock", constants::Num_Lock);
    key_map.insert("scrolllock", constants::Scroll_Lock);
    key_map.insert("backspace", constants::BackSpace);
    key_map.insert("delete", constants::Delete);
    key_map.insert("insert", constants::Insert);
    key_map.insert("return", constants::Return);
    key_map.insert("enter", constants::Return);
    key_map.insert("up", constants::Up);
    key_map.insert("down", constants::Down);
    key_map.insert("left", constants::Left);
    key_map.insert("right", constants::Right);
    key_map.insert("home", constants::Home);
    key_map.insert("end", constants::End);
    key_map.insert("pageup", constants::Page_Up);
    key_map.insert("pagedown", constants::Page_Down);
    key_map.insert("escape", constants::Escape);
    key_map.insert("esc", constants::Escape);
    key_map.insert("num0", constants::KP_0);
    key_map.insert("num1", constants::KP_1);
    key_map.insert("num2", constants::KP_2);
    key_map.insert("num3", constants::KP_3);
    key_map.insert("num4", constants::KP_4);
    key_map.insert("num5", constants::KP_5);
    key_map.insert("num6", constants::KP_6);
    key_map.insert("num7", constants::KP_7);
    key_map.insert("num8", constants::KP_8);
    key_map.insert("num9", constants::KP_9);
    key_map.insert("numdec", constants::KP_Decimal);
    key_map.insert("numadd", constants::KP_Add);
    key_map.insert("numsub", constants::KP_Subtract);
    key_map.insert("nummult", constants::KP_Multiply);
    key_map.insert("numdiv", constants::KP_Divide);
    key_map.insert("f1", constants::F1);
    key_map.insert("f2", constants::F2);
    key_map.insert("f3", constants::F3);
    key_map.insert("f4", constants::F4);
    key_map.insert("f5", constants::F5);
    key_map.insert("f6", constants::F6);
    key_map.insert("f7", constants::F7);
    key_map.insert("f8", constants::F8);
    key_map.insert("f9", constants::F9);
    key_map.insert("f10", constants::F10);
    key_map.insert("f11", constants::F11);
    key_map.insert("f12", constants::F12);
    key_map.insert("f13", constants::F13);
    key_map.insert("f14", constants::F14);
    key_map.insert("f15", constants::F15);
    key_map.insert("f16", constants::F16);
    key_map.insert("f17", constants::F17);
    key_map.insert("f18", constants::F18);
    key_map.insert("f19", constants::F19);
    key_map.insert("f20", constants::F20);
    key_map.insert("f21", constants::F21);
    key_map.insert("f22", constants::F22);
    key_map.insert("f23", constants::F23);
    key_map.insert("f24", constants::F24);

    key_map
}
pub fn get_mod_map() -> HashMap<&'static str, gdk::ModifierType> {
    use gdk::ModifierType;

    let mut mod_map = HashMap::new();
    mod_map.insert("cmd", ModifierType::CONTROL_MASK); // Alias for "ctrl"
    mod_map.insert("cmd_or_ctrl", ModifierType::CONTROL_MASK); // Alias for "ctrl"
    mod_map.insert("shift", ModifierType::SHIFT_MASK);
    mod_map.insert("alt", ModifierType::MOD1_MASK);
    mod_map.insert("ctrl", ModifierType::CONTROL_MASK);
    mod_map.insert("opt", ModifierType::MOD1_MASK); // Alias for "alt"
    mod_map.insert("altgr", ModifierType::MOD5_MASK);
    mod_map.insert("super", ModifierType::SUPER_MASK);
    mod_map.insert("win", ModifierType::SUPER_MASK);
    mod_map.insert("meta", ModifierType::META_MASK);

    mod_map
}

fn parse_shortcut(shortcut: &str) -> (u32, ModifierType) {
    let key_map = get_key_map();
    let mod_map = get_mod_map(); // This should map strings like "ctrl" to ModifierType
    let parts: Vec<&str> = shortcut.split('+').collect();

    // Assuming last part is always the key
    let key_str = parts.last().unwrap_or(&"");

    // Get the key from the key map
    let key = if let Some(key) = key_map.get(key_str) {
        // Clone the key value to get ownership of it
        key.clone()
    } else {
        // If the key is not in the map, assume it's a character
        Key::from_name(key_str)
    };

    let key_u32 = key_to_u32(key);

    let mut mods = ModifierType::empty();

    // Process all parts except the last one as modifiers
    for &mod_str in &parts[..parts.len() - 1] {
        if let Some(&mod_type) = mod_map.get(mod_str) {
            mods.insert(mod_type);
        }
    }

    (key_u32, mods)
}
fn key_to_u32(key: gdk::keys::Key) -> u32 {
    unsafe { mem::transmute(key) }
}

pub fn on_context_menu<R: Runtime>(
    pos: Option<Position>,
    items: Option<Vec<MenuItem>>,
    window: Window<R>,
) {
    // Create and show the context menu
    let gtk_window = window.gtk_window().unwrap();

    // Check if the window is realized
    if !gtk_window.is_realized() {
        gtk_window.realize();
    }

    // Create a new menu.
    let menu = Menu::new();
    if let Some(menu_items) = items {
        for item in menu_items.iter() {
            append_menu_item(&window, &gtk_window, &menu, item);
        }
    }

    let (mut x, mut y) = match pos {
        Some(ref position) => (position.x as i32, position.y as i32),
        None => {
            if let Some(display) = Display::default() {
                if let Some(seat) = display.default_seat() {
                    let pointer = seat.pointer();
                    let (_screen, x, y) = match pointer {
                        Some(p) => p.position(),
                        None => {
                            eprintln!("Failed to get pointer position");
                            (display.default_screen(), 0, 0)
                        }
                    };
                    (x, y)
                } else {
                    eprintln!("Failed to get default seat");
                    (0, 0)
                }
            } else {
                eprintln!("Failed to get default display");
                (0, 0)
            }
        }
    };

    let is_absolute = if let Some(position) = pos.clone() {
        position.is_absolute
    } else {
        Some(false)
    };
    if is_absolute.unwrap_or(false) || pos.is_none() {
        // Adjust x and y if the coordinates are not relative to the window
        let window_position = window.outer_position().unwrap();
        x -= window_position.x;
        y -= window_position.y;
    }

    // Required otherwise the menu doesn't show properly
    sleep(time::Duration::from_millis(100));

    // Delay the display of the context menu to ensure the window is ready
    glib::idle_add_local(move || {
        // Show the context menu at the specified position.
        let gdk_window = gtk_window.window().unwrap();
        let rect = &gdk::Rectangle::new(x, y, 0, 0);
        let mut event = gdk::Event::new(gdk::EventType::ButtonPress);
        event.set_device(
            gdk_window
                .display()
                .default_seat()
                .and_then(|d| d.pointer())
                .as_ref(),
        );
        menu.show_all();
        menu.popup_at_rect(
            &gdk_window,
            rect,
            gdk::Gravity::NorthWest,
            gdk::Gravity::NorthWest,
            Some(&event),
        );
        // Continue(false)
        glib::ControlFlow::Continue
    });
}

#[tauri::command]
pub fn show_context_menu<R: Runtime>(
    window: Window<R>,
    pos: Option<Position>,
    items: Option<Vec<MenuItem>>,
    // _theme: Option<tauri::Theme>,
) {
    on_context_menu(pos, items, window);
}

fn append_menu_item<R: Runtime>(
    window: &Window<R>,
    gtk_window: &gtk::ApplicationWindow,
    menu: &Menu,
    item: &MenuItem,
) {
    if item.is_separator.unwrap_or(false) {
        menu.append(&gtk::SeparatorMenuItem::builder().visible(true).build());
    } else {
        let menu_item = match item.checked {
            Some(state) => {
                // Create a CheckMenuItem for checkable items
                let check_menu_item = gtk::CheckMenuItem::new();
                check_menu_item.set_active(state);
                check_menu_item.upcast()
            }
            None => {
                // Create a regular MenuItem for non-checkable items
                gtk::MenuItem::new()
            }
        };

        // Create a Box to hold the image and label
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        hbox.set_homogeneous(false);

        // Handle icon
        if let Some(icon) = &item.icon {
            let image = gtk::Image::from_file(&icon.path);
            if let Some(width) = icon.width {
                if let Some(height) = icon.height {
                    image.set_pixel_size(width as i32);
                    image.set_pixel_size(height as i32);
                }
            }
            hbox.pack_start(&image, false, false, 0);
        }

        // Add label to the Box
        let label = item.label.as_deref().unwrap_or("");
        let accel_label = gtk::AccelLabel::new(label);
        accel_label.set_xalign(0.0); // Align the label to the left
        hbox.pack_start(&accel_label, true, true, 0);

        // Add the Box to the MenuItem
        menu_item.add(&hbox);

        // Handle enabled/disabled state
        if item.disabled.unwrap_or(false) {
            menu_item.set_sensitive(false);
        }

        // If an event is provided, you can connect to the "activate" signal (from item.event and item.payload)
        if let Some(event) = &item.event {
            let window_clone = window.clone();

            // payload may exist
            let payload_clone = item.payload.clone();

            // get event from String to str
            let event_clone = event.clone();
            menu_item.connect_activate(move |_| {
                window_clone
                    .emit(event_clone.as_str(), &payload_clone)
                    .unwrap(); // Emit the event to JavaScript
            });
        }

        // Handle shortcut
        if let Some(shortcut) = &item.shortcut {
            let accel_group = AccelGroup::new();
            gtk_window.add_accel_group(&accel_group);

            // Parse and assign the shortcut
            let (key, mods) = parse_shortcut(shortcut);
            accel_label.set_accel_widget(Some(&menu_item));
            menu_item.add_accelerator("activate", &accel_group, key, mods, AccelFlags::VISIBLE);
        }

        if let Some(subitems) = &item.subitems {
            let submenu = Menu::new();
            for subitem in subitems.iter() {
                append_menu_item(window, gtk_window, &submenu, subitem);
            }
            menu_item.set_submenu(Some(&submenu));
        }

        menu.append(&menu_item);
    }
}
