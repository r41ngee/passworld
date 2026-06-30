use std::path::PathBuf;

const DB_NAME: &str = "db.json";

fn dir_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    dirs::data_dir()
        .map(|dir| dir.join("passworld"))
        .ok_or_else(|| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "data directory not found",
            )) as Box<dyn std::error::Error>
        })
}

pub fn check_dir() -> Result<bool, Box<dyn std::error::Error>> {
    let dir = dir_path()?;
    let exists = std::fs::exists(dir)?;
    Ok(exists)
}

pub fn create_dir() -> Result<(), Box<dyn std::error::Error>> {
    let path = dir_path()?;
    let created = check_dir()?;
    if !created {
        std::fs::create_dir_all(path)?;
    }

    Ok(())
}

fn db_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(dir_path()?.join(DB_NAME))
}

pub fn check_db() -> Result<bool, Box<dyn std::error::Error>> {
    let path = dir_path()?.join(DB_NAME);

    Ok(path.exists())
}

pub fn create_db() -> Result<(), Box<dyn std::error::Error>> {
    let path = db_path()?;
    let created = check_db()?;
    if !created {
        std::fs::File::create(path)?;
    }

    Ok(())
}