use std::{
    fs,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

use slint::{
    CloseRequestResponse, ComponentHandle, ModelRc, PhysicalPosition, PhysicalSize, SharedString,
    StandardListViewItem, VecModel, Weak,
};

use crate::{
    seed_finder::db,
    ui::{
        dialog,
        enums::condition::Condition,
        types::{AppWindow, ProgressDialog},
    },
};

pub fn find(
    ui_weak: Weak<AppWindow>,
    is_randomizer: bool,
    is_bravery: bool,
    is_relic: bool,
    conditions: Arc<Mutex<Vec<Condition>>>,
    parent_position: PhysicalPosition,
    parent_size: PhysicalSize,
) {
    if fs::metadata("./seeds.db").is_err() {
        dialog::show_question(
            ui_weak,
            String::from(
                "Could not find the seeds database. Create it now?\nNote: this will take a while.",
            ),
            parent_position,
            parent_size,
            generate_db,
        );

        return;
    }

    if !is_randomizer && !is_bravery {
        dialog::show_message(
            String::from("Please select at least the Randomizer or Bravery game modes."),
            parent_position,
            parent_size,
        );

        return;
    }

    if conditions.lock().unwrap().len() == 0 {
        dialog::show_message(
            String::from("Please add at least 1 condition."),
            parent_position,
            parent_size,
        );

        return;
    }

    ui_weak
        .upgrade_in_event_loop(|ui| ui.set_loading_seeds(true))
        .unwrap();

    thread::spawn(move || {
        let seeds = db::find_seeds(is_randomizer, is_bravery, is_relic, conditions);

        if let Ok(seeds) = seeds {
            ui_weak
                .upgrade_in_event_loop(move |ui| {
                    let model = seeds
                        .iter()
                        .map(|x| {
                            ModelRc::new(VecModel::from(vec![StandardListViewItem::from(
                                SharedString::from(x.to_string()),
                            )]))
                        })
                        .collect::<Vec<ModelRc<StandardListViewItem>>>();

                    ui.set_found_seeds(ModelRc::new(VecModel::from(model)));
                    ui.set_loading_seeds(false);
                })
                .unwrap();
        }
    });
}

fn generate_db(
    ui_weak: Weak<AppWindow>,
    parent_position: PhysicalPosition,
    parent_size: PhysicalSize,
) {
    ui_weak
        .upgrade_in_event_loop(move |_| {
            let dialog = ProgressDialog::new().unwrap();
            let dialog_weak = dialog.as_weak();
            let stop_signal = Arc::new(AtomicBool::new(false));

            let position = PhysicalPosition::new(
                parent_position.x + (parent_size.width / 2) as i32
                    - (dialog.get__width() as i32 / 2),
                parent_position.y + (parent_size.height / 2) as i32
                    - (dialog.get__height() as i32 / 2),
            );

            dialog.set_message(SharedString::from("Generating database..."));
            dialog.window().set_position(position);
            dialog.on_cancel_clicked(close_dialog(dialog_weak.clone(), stop_signal.clone()));
            dialog
                .window()
                .on_close_requested(on_dialog_close(dialog_weak.clone(), stop_signal.clone()));
            dialog.show().unwrap();

            thread::spawn(move || {
                let res = db::generate(dialog_weak, parent_position, parent_size, stop_signal);

                if let Err(_) = res {
                    fs::remove_file("./seeds.db").ok();
                    fs::remove_file("./seeds.db-journal").ok();
                }
            });
        })
        .unwrap();
}

fn close_dialog(dialog_weak: Weak<ProgressDialog>, stop_signal: Arc<AtomicBool>) -> impl FnMut() {
    move || {
        dialog_weak.upgrade().unwrap().hide().unwrap();
        stop_signal.store(true, Ordering::Relaxed);
    }
}

fn on_dialog_close(
    dialog_weak: Weak<ProgressDialog>,
    stop_signal: Arc<AtomicBool>,
) -> impl FnMut() -> CloseRequestResponse {
    move || {
        dialog_weak.upgrade().unwrap().hide().unwrap();
        stop_signal.store(true, Ordering::Relaxed);
        CloseRequestResponse::HideWindow
    }
}
