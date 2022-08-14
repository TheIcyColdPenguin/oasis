use crate::data_types::Person;
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
}
