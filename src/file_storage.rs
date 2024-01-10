pub struct FileStorage {
    pub file_path: String,
    pub pwd: String,
}
impl FileStorage {
    pub async fn load_pwds(&self) -> std::io::Result<String> {
        std::fs::read_to_string(&self.file_path)
    }
    pub async fn save_pwds(&self, pwds: &str) -> std::io::Result<()> {
        std::fs::write(&self.file_path, pwds)
    }
}