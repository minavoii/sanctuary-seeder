use std::thread;

use slint::{ComponentHandle, PhysicalPosition, PhysicalSize, SharedString, Weak, WindowPosition};

use crate::ui::types::{AppWindow, MessageDialog, QuestionDialog};

/// Displays an error message.
pub fn show_message(message: String, parent_position: PhysicalPosition, parent_size: PhysicalSize) {
    let dialog = MessageDialog::new().unwrap();
    let dialog_weak = dialog.as_weak();

    dialog.set_message(SharedString::from(message));
    dialog.on_ok_clicked(move || dialog_weak.upgrade().unwrap().hide().unwrap());

    // Center the dialog within the main window
    let position = PhysicalPosition::new(
        parent_position.x + (parent_size.width / 2) as i32 - (dialog.get__width() as i32 / 2),
        parent_position.y + (parent_size.height / 2) as i32 - (dialog.get__height() as i32 / 2),
    );

    dialog
        .window()
        .set_position(WindowPosition::Physical(position));

    dialog.show().unwrap();
}

/// Displays a question that can be answered by `Yes` or `No`.
pub fn show_question(
    ui_weak: Weak<AppWindow>,
    message: String,
    parent_position: PhysicalPosition,
    parent_size: PhysicalSize,
    callback: fn(Weak<AppWindow>, PhysicalPosition, PhysicalSize),
) {
    let dialog = QuestionDialog::new().unwrap();
    let dialog_weak = dialog.as_weak();
    let dialog_weak2 = dialog.as_weak();

    // Center the dialog within the main window
    let position = PhysicalPosition::new(
        parent_position.x + (parent_size.width / 2) as i32 - (dialog.get__width() as i32 / 2),
        parent_position.y + (parent_size.height / 2) as i32 - (dialog.get__height() as i32 / 2),
    );

    dialog
        .window()
        .set_position(WindowPosition::Physical(position));

    dialog.set_message(SharedString::from(message));
    dialog.on_no_clicked(move || dialog_weak2.upgrade().unwrap().hide().unwrap());
    dialog.on_yes_clicked(move || {
        dialog_weak.upgrade().unwrap().hide().unwrap();

        let ui_weak = ui_weak.clone();

        thread::spawn(move || callback(ui_weak, parent_position, parent_size));
    });

    dialog.show().unwrap();
}
