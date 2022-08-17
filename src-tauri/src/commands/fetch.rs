use crate::data_types::{Note, Person};

#[tauri::command]
pub fn get_persons() -> Result<Vec<Person>, ()> {
    let lock = crate::DATABASE_MANAGER.lock().unwrap();
    let result = lock.get_persons().unwrap();
    Ok(result)
}

#[tauri::command]
pub fn get_notes(person_id: i32) -> Result<Vec<Note>, ()> {
    let lock = crate::DATABASE_MANAGER.lock().unwrap();
    let result = lock.get_notes(person_id).unwrap();
    Ok(result)
}
