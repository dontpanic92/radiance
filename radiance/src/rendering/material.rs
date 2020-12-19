use image::RgbaImage;

use super::{texture::TextureDef, ShaderDef, SIMPLE_SHADER_DEF};
use std::{path::PathBuf, io::Read};

pub trait Material: downcast_rs::Downcast + std::fmt::Debug {}

downcast_rs::impl_downcast!(Material);

pub struct MaterialDef {
    name: String,
    shader: ShaderDef,
    textures: Vec<TextureDef>,
}

impl MaterialDef {
    pub fn new(name: &str, shader: ShaderDef, textures: Vec<TextureDef>) -> Self {
        Self {
            name: name.to_string(),
            textures,
            shader,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn shader(&self) -> &ShaderDef {
        &self.shader
    }

    pub fn textures(&self) -> &[TextureDef] {
        &self.textures
    }
}

pub struct SimpleMaterialDef;
impl SimpleMaterialDef {
    pub fn create<R: Read>(reader: &mut R) -> MaterialDef {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).unwrap();
        println!("SimpleMaterialDef buf len: {}", buf.len());
        let data = image::open(&buf).and_then(|img| Ok(img.to_rgba())).ok();
        println!("data: {:?}", data.is_some());

        MaterialDef::new(
            "simple_material",
            SIMPLE_SHADER_DEF.clone(),
            vec![TextureDef::ImageTextureDef(data)],
        )
    }
}
