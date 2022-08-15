use crate::data_types::Person;

#[tauri::command]
pub fn get_persons() -> Result<Vec<Person>, ()> {
    let lock = crate::DATABASE_MANAGER.lock().unwrap();
    let result = lock.get_persons().unwrap();
    Ok(result)
}
