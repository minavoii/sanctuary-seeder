use std::{
    iter,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use rusqlite::{params, params_from_iter, Connection, Error, Result};
use slint::{ComponentHandle, PhysicalPosition, PhysicalSize, Weak};

use crate::{
    structs::{game::Game, game_manager},
    ui::{dialog, enums::condition::Condition, types::ProgressDialog},
};

pub fn generate(
    dialog: Weak<ProgressDialog>,
    parent_position: PhysicalPosition,
    parent_size: PhysicalSize,
    stop: Arc<AtomicBool>,
) -> Result<()> {
    create_tables()?;

    let mut conn = Connection::open("seeds.db")?;

    let sql_mapping = format!(
        "INSERT INTO Randomizer VALUES ({})",
        std::iter::repeat("?")
            .take(107)
            .collect::<Vec<&str>>()
            .join(",")
    );

    let sql_bravery = format!(
        "INSERT INTO Bravery VALUES ({})",
        std::iter::repeat("?")
            .take(31)
            .collect::<Vec<&str>>()
            .join(",")
    );

    let sql_relic = format!(
        "INSERT INTO Relic VALUES ({})",
        std::iter::repeat("?")
            .take(14)
            .collect::<Vec<&str>>()
            .join(",")
    );

    // For transactions, let's handle 1000 seeds at a time
    for i in 0..1000 {
        let tx = conn.transaction()?;
        let mut randomizer = tx.prepare(&sql_mapping)?;
        let mut bravery = tx.prepare(&sql_bravery)?;
        let mut relic = tx.prepare(&sql_relic)?;

        let mut insert_bravery = |id: u32, game: &Game| -> Result<()> {
            if let Some(data) = &game.bravery_data {
                bravery.execute(params!(
                    id,
                    data.familiar,
                    data.starters[1],
                    data.starters[2],
                    data.swimming,
                    data.bex,
                    data.cryomancer.unwrap(),
                    data.cryomancer_required,
                    data.end_of_time[0],
                    data.end_of_time[1],
                    data.end_of_time[2],
                    data.army[0],
                    data.army[1],
                    data.army[2],
                    data.army[3],
                    data.army[4],
                    data.army[5],
                    data.army.get(6),
                    data.eggs[0],
                    data.eggs[1],
                    data.eggs[2],
                    data.eggs[3],
                    data.eggs[4],
                    data.eggs[5],
                    data.eggs[6],
                    data.eggs[7],
                    data.eggs[8],
                    data.eggs[9],
                    data.eggs[10],
                    data.eggs[11],
                    data.eggs[12],
                ))?;
            }

            Ok(())
        };

        for j in 0..1000 {
            let seed = i * 1000 + j;
            let game = game_manager::generate_game(seed, true, false, true);

            let mut params = vec![Some(seed)];
            params.extend(game.mapping.unwrap().iter().skip(4).take(106));

            // Randomizer
            randomizer.execute(params_from_iter(params))?;

            // Randomizer + Relics
            relic.execute(params_from_iter(
                iter::once(seed * 3).chain(game.relics.unwrap().list),
            ))?;

            // Bravery
            let game = game_manager::generate_game(seed, false, true, true);
            insert_bravery(seed * 2, &game)?;

            // Bravery + Relics
            relic.execute(params_from_iter(
                iter::once(seed * 3 + 1).chain(game.relics.unwrap().list),
            ))?;

            // Randomizer + Bravery
            let game = game_manager::generate_game(seed, true, true, true);
            insert_bravery(seed * 2 + 1, &game)?;

            // Randomizer + Bravery + Relics
            if let Some(relics) = game.relics {
                relic.execute(params_from_iter(
                    iter::once(seed * 3 + 2).chain(relics.list),
                ))?;
            }
        }

        randomizer.finalize()?;
        bravery.finalize()?;
        relic.finalize()?;
        tx.commit()?;

        if stop.load(Ordering::Relaxed) {
            // return Ok(());
            return Err(Error::ExecuteReturnedResults);
        }

        // Update UI
        let progress = i as f32 / 1000.;
        let progress_text = i as i32 / 10;

        dialog
            .upgrade_in_event_loop(move |dialog| {
                dialog.set_progress(progress);
                dialog.set_progress_text(progress_text);
            })
            .unwrap();
    }

    dialog
        .upgrade_in_event_loop(move |dialog| {
            dialog.hide().unwrap();
            dialog::show_message(
                String::from("Database generated with success!"),
                parent_position,
                parent_size,
            );
        })
        .unwrap();

    Ok(())
}

pub fn find_seeds(
    is_randomizer: bool,
    is_bravery: bool,
    is_relic: bool,
    conditions: Arc<Mutex<Vec<Condition>>>,
) -> Result<Vec<u32>> {
    let conn = Connection::open("seeds.db")?;
    let sql = build_query(is_randomizer, is_bravery, is_relic, conditions);

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |row| Ok(row.get::<usize, u32>(0)))?;
    let seeds = rows.map(|x| x.unwrap().unwrap()).collect::<Vec<u32>>();

    Ok(seeds)
}

/// Creates all tables for the database.
fn create_tables() -> Result<()> {
    let conn = Connection::open("seeds.db")?;

    // Randomizer
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS Randomizer (\"Id\" INTEGER NOT NULL,{},PRIMARY KEY(\"Id\"))",
        (0..)
            .take(106)
            .map(|x| format!("\"M{x}\" INTEGER NOT NULL"))
            .collect::<Vec<String>>()
            .join(",")
    );
    conn.execute(&sql, ())?;

    // Bravery
    let sql = std::include_str!("../../res/out/tables/Bravery.sql");
    conn.execute(sql, ())?;

    // Relic
    let sql = std::include_str!("../../res/out/tables/Relic.sql");
    conn.execute(sql, ())?;

    Ok(())
}

/// Builds the full query to find seeds.
fn build_query(
    is_randomizer: bool,
    is_bravery: bool,
    is_relic: bool,
    conditions: Arc<Mutex<Vec<Condition>>>,
) -> String {
    // As the Randomizer mapping is determined first,
    // there's only 1 mapping for all game mode combination per seed,
    // making Randomizer.Id always equals to the seed
    let mut query = String::from("SELECT Randomizer.Id as 'Seed' FROM Randomizer ");

    // Bravery is determined 2nd, will be affected by Randomizer
    if is_bravery {
        query += "JOIN Bravery ON Bravery.Id = Randomizer.Id * 2 ";

        if is_randomizer {
            query += "+ 1 ";
        }
    }

    // Relic is determined 3rd, will be affected by Randomizer and/or Bravery
    if is_relic {
        query += "JOIN Relic ON Relic.Id = Randomizer.Id * 3 "; // Randomizer only

        if is_bravery {
            query += if is_randomizer { "+ 2 " } else { "+ 1 " };
        }
    }

    query += &build_query_conditions(conditions);
    query
}

/// Builds the `WHERE` clause of the sql query.
fn build_query_conditions(conditions: Arc<Mutex<Vec<Condition>>>) -> String {
    format!(
        "WHERE {}",
        conditions
            .lock()
            .unwrap()
            .iter()
            .map(|x| x.to_sql())
            .collect::<Vec<String>>()
            .join(" AND ")
    )
}
