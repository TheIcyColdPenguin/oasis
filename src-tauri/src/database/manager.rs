use rusqlite::Connection;

pub struct DatabaseManager {
    pub connection: Connection,
}

impl DatabaseManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let connection = Connection::open("database.db")?;
        Self::initialise(&connection)?;

        Ok(Self { connection })
    }
}
