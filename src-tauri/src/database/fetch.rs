use crate::data_types::{Note, Person};
use crate::database::DatabaseManager;

impl DatabaseManager {
    pub fn get_persons(&self) -> Result<Vec<Person>, Box<dyn std::error::Error>> {
        let mut stmt = self
            .connection
            .prepare("SELECT person_id, person_name, person_summary FROM person;")?;

        let users_iter = stmt.query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                summary: row.get(2)?,
            })
        })?;

        let mut users = vec![];
        for user in users_iter {
            users.push(user?);
        }

        Ok(users)
    }

    pub fn get_notes(&self, person_id: i32) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let mut stmt = self.connection.prepare(
            "SELECT note_id, note_created_on, note_text FROM note where person_id = ?1;",
        )?;

        let notes_iter = stmt.query_map([&person_id], |row| {
            Ok(Note {
                id: row.get(0)?,
                created_on: row.get(1)?,
                text: row.get(2)?,
                person_id,
            })
        })?;

        let mut notes = vec![];
        for note in notes_iter {
            notes.push(note?);
        }

        Ok(notes)
    }
}
