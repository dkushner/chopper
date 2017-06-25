use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use vulkano::pipeline::shader::ShaderModule;

pub type Shader = u32;

pub struct ShaderManager {
    shaders: HashMap<String, Shader>,

    module: Vec<ShaderModule>,
}

impl ShaderManager {
    pub fn new() -> ShaderManager {
        ShaderManager {
            shaders: HashMap::new(),
            module: Vec::new(),
        }
    }

    pub fn load(&self, path: &Path) {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);

        let contents = &mut Vec::new();
        reader.read_to_end(contents);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/shaders/spir/frag.spv");

        let mut manager = ShaderManager::new();
        manager.load(path.as_path());

        println!("{:?}", path);
    }
}