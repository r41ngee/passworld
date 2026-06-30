use std::path::PathBuf;

fn get_data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    dirs::data_dir()
        .map(|dir| dir.join("passworld"))
        .ok_or_else(|| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "data directory not found",
            )) as Box<dyn std::error::Error>
        })
}

pub fn ensure_dir_created() -> Result<bool, Box<dyn std::error::Error>> {
    let dir = get_data_dir()?;
    let exists = std::fs::exists(dir)?;
    Ok(exists)
}

pub fn create_if_not_exists() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_data_dir()?;
    let created = ensure_dir_created()?;
    if !created {
        std::fs::create_dir_all(path)?;
    }

    Ok(())
}