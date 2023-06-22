use uuid::Uuid;

#[inline]
pub fn project_dir() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from("com", "unknownrori", "unts")
}

#[inline]
pub fn generate_uuid() -> Uuid {
    uuid::Uuid::new_v4()
}

#[cfg(target_family = "windows")]
#[inline]
pub fn seperator() -> char {
    '\\'
}

#[cfg(target_family = "unix")]
#[inline]
pub fn seperator() -> char {
    '/'
}
