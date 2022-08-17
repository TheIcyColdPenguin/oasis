use crate::database::DatabaseManager;

use rusqlite::Connection;

impl DatabaseManager {
    pub fn initialise(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(debug_assertions)]
        {
            // cleanup tables
            // order matters due to foreign key constraints
            connection.execute("DROP TABLE IF EXISTS note;", ())?;
            connection.execute("DROP TABLE IF EXISTS person;", ())?;
        }

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
            // test person data

            connection.execute(
                "INSERT OR IGNORE INTO person (person_id, person_name, person_summary) VALUES (?1, ?2, ?3);",
                (&1, &"John Doe", &"A person")
            )?;
            connection.execute(
                "INSERT OR IGNORE INTO person (person_id, person_name, person_summary) VALUES (?1, ?2, ?3);",
                (&2, &"Jane Doe", &"Another person")
            )?;

            // test note data

            connection.execute(
                "INSERT OR IGNORE INTO note (note_id, note_text, person_id) VALUES (?1, ?2, ?3);",
                (&1, &"Note content here", &1),
            )?;
            connection.execute(
                "INSERT OR IGNORE INTO note (note_id, note_text, person_id) VALUES (?1, ?2, ?3);",
                (&2, &"more notes here", &1),
            )?;
        }

        Ok(())
    }
}
