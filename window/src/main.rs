slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move |use_slider: bool| {
            let ui = ui_handle.unwrap();
            if use_slider == false {ui.set_counter(ui.get_counter() + 1);}
        }
    });

    ui.on_request_decrise_value({
        let ui_handle = ui.as_weak();
        move |use_slider: bool| {
            let ui = ui_handle.unwrap();
            if use_slider == false {ui.set_counter(ui.get_counter() - 1);}
        }
    });

    ui.on_request_set_value_100({
        let ui_handle = ui.as_weak();
        move |use_slider: bool| {
            let ui = ui_handle.unwrap();
            if use_slider == false {ui.set_counter(100);}
        }
    });

    ui.on_update_counter({
        let ui_handle = ui.as_weak();
        move |slider_value: i32, use_slider: bool| { // Accept the slider value and use_slider state as inputs
            if let Some(ui) = ui_handle.upgrade() {
                if use_slider {
                    ui.set_counter(slider_value); // Update the counter only if use_slider is true
                }
            }
        }
    });

    ui.run()
}
