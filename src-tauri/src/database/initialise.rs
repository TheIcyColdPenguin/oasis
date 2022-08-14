use crate::database::DatabaseManager;

use rusqlite::Connection;

impl DatabaseManager {
    pub fn initialise(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
        connection.execute("PRAGMA foreign_keys = ON;", ())?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS person (
                person_id INTEGER PRIMARY KEY,
                person_name TEXT NOT NULL,
                person_summary TEXT
            );",
            (),
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS note (
                note_id INTEGER PRIMARY KEY,
                note_created_on DATETIME DEFAULT CURRENT_TIMESTAMP,
                note_text TEXT DEFAULT '',
                person_id INTEGER NOT NULL,

                FOREIGN KEY (person_id) REFERENCES person(person_id)
            );",
            (),
        )?;

        #[cfg(debug_assertions)]
        {
            connection.execute(
                "
            INSERT OR IGNORE INTO person (person_id, person_name, person_summary) VALUES (?1, ?2, ?3);",
                (&1, &"John Doe", &"A person"),
            )?;

            connection.execute(
                "
            INSERT OR IGNORE INTO note (note_id, note_text, person_id) VALUES (?1, ?2, ?3);",
                (&1, &"Note content here", &1),
            )?;
        }

        Ok(())
    }
}
