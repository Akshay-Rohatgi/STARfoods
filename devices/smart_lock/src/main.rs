use rusqlite::{params, Connection, Result};
use std::{env, process};

fn setup_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS RoomAccess (
            room_id INTEGER PRIMARY KEY,
            room_name TEXT NOT NULL,
            access_codes TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute("DELETE FROM RoomAccess", [])?;

    conn.execute(
        "INSERT INTO RoomAccess (room_id, room_name, access_codes) VALUES (?, ?, ?)",
        params![1, "Pantry A [Team Cosmic]", ",3342,0000"],
    )?;
    conn.execute(
        "INSERT INTO RoomAccess (room_id, room_name, access_codes) VALUES (?, ?, ?)",
        params![2, "Pantry B [Team Alien]", ",5178,0000"],
    )?;
    conn.execute(
        "INSERT INTO RoomAccess (room_id, room_name, access_codes) VALUES (?, ?, ?)",
        params![3, "Pantry C [Team Moon]", ",9191,0000"],
    )?;
    conn.execute(
        "INSERT INTO RoomAccess (room_id, room_name, access_codes) VALUES (?, ?, ?)",
        params![4, "Pantry D [Team Comet (Tentative)]", ",0000"],
    )?;
    Ok(())
}

fn unlocked_rooms(conn: &Connection, code: &str) -> Result<Vec<String>> {
    let mut statement = conn.prepare(
        format!("SELECT room_name FROM RoomAccess WHERE access_codes LIKE '%,{}%'", code).as_str()
    )?;

    let names: Vec<String> = statement
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect();

    Ok(names)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("[!] Usage: smart_lock <code>");
        process::exit(1);
    }



    let path = env::var("DB_PATH").unwrap() + "/lock.db";
    let conn = Connection::open(path).expect("ERROR OPENING DB");
    // setup_db(&conn).expect("failed!");
    if !args[1].is_empty() {
        println!("UNLOCKED ROOMS: {:?}", unlocked_rooms(&conn, &args[1]).expect("UNLOCKING FAILED"));
    }
}
