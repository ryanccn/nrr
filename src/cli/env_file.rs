use dotenvy::Result;

#[derive(Clone, Debug)]
pub struct EnvFile {
    inner: Vec<(String, String)>,
}

impl From<Vec<(String, String)>> for EnvFile {
    fn from(inner: Vec<(String, String)>) -> Self {
        Self { inner }
    }
}

impl EnvFile {
    pub fn from_path(path: &str) -> Result<Self> {
        let file = dotenvy::from_filename_iter(path)?;
        let env: Vec<(String, String)> = file.collect::<Result<_>>()?;
        Ok(Self { inner: env })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.inner.iter().map(|(a, b)| (a, b))
    }
}
