use imgui::*;

use std::sync::Mutex;

mod support;

fn main() {
    let mut system = support::init(file!());
    system.imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;

    let main_dockspace: Mutex<Option<imgui::docking::DockNode>> = Mutex::new(None);

    system.main_loop(move |_, ui| {
        // Create top level floating window
        Window::new(im_str!("Hello Dock Space"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                // Mark window as a dock space
                let dock_id = ui.dockspace(im_str!("space"));
                *(main_dockspace.lock().unwrap()) = Some(dock_id);

                ui.text("Hi");
            });

        Window::new(im_str!("Left")).build(ui, || {
            ui.text("Left window!");

            if ui.button(im_str!("Dock me")) {
                match &*main_dockspace.lock().unwrap() {
                    Some(d) => d.split(
                        Direction::Left,
                        0.5,
                        |id| id.dock_window(im_str!("Left")),
                        |_id| {},
                    ),
                    None => {}
                }
            }
        });
    });
}
