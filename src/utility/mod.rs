use uuid::Uuid;

#[inline]
pub fn project_dir() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from("com", "unknownrori", "unts")
}

#[inline]
pub fn generate_uuid() -> Uuid {
    uuid::Uuid::new_v4()
}
