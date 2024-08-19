use dotenvy::Result;

#[derive(Clone, Debug)]
pub struct EnvFile(Vec<(String, String)>);

impl EnvFile {
    pub fn from_path(path: &str) -> Result<Self> {
        dotenvy::from_filename_iter(path)
            .and_then(|file| file.collect())
            .map(Self)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.0.iter().map(|(a, b)| (a, b))
    }
}
