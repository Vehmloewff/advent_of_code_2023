use std::{env, path::PathBuf};

use tokio::fs::{create_dir, metadata, read_to_string, write};

pub struct InputsCache {
    path: PathBuf,
}

impl InputsCache {
    pub async fn new() -> InputsCache {
        let home = env::var("HOME").unwrap();
        let path = PathBuf::from(home).join(".cache/advent_of_code_2023");

        if !metadata(path.clone()).await.is_ok_and(|meta| meta.is_dir()) {
            create_dir(path.clone()).await.unwrap()
        }

        InputsCache { path }
    }

    pub async fn get(&self, day: u64) -> Option<String> {
        read_to_string(self.path.join(format!("day_{day}.txt")))
            .await
            .ok()
    }

    pub async fn set(&self, day: u64, text: String) {
        write(self.path.join(format!("day_{day}.txt")), text)
            .await
            .unwrap()
    }
}
