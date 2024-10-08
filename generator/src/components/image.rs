use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error,
    style::{ComponentAlign, ComponentStyle, RawComponentStyle},
};
use tiny_skia::{Pixmap, Transform, IntSize, PixmapPaint};
use image::io::Reader as ImageReader; // Используем crate `image` для загрузки изображений
use crate::edges::padding::Padding;
use std::env;
use std::path::{Path, PathBuf};

pub const IMAGE_PADDING: f32 = 0.;

pub struct Image {
    path: String,
    width: f32,
    children: Vec<Box<dyn Component>>,
}


fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home_dir) = env::home_dir() {
            // Заменяем `~` на домашнюю директорию
            let without_tilde = path.strip_prefix("~/").unwrap();
            return PathBuf::from(home_dir).join(without_tilde);
        }
    }
    PathBuf::from(path)
}

impl Component for Image {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        RawComponentStyle::default()
    }

    fn draw_self(
        &self,
        pixmap: &mut Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        style: &ComponentStyle,
        parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let x = render_params.x;
        let y = render_params.y;

        // Загружаем изображение с помощью библиотеки image
        let img = ImageReader::open(expand_tilde(&self.path))
            .map_err(|e| render_error::RenderError::Other(format!("Failed to open image: {}", e)))?
            .decode()
            .map_err(|e| render_error::RenderError::Other(format!("Failed to decode image: {}", e)))?;
        let img = img.to_rgba8(); // Преобразуем изображение в формат RGBA

        let img_width = img.width() as f32;
        let img_height = img.height() as f32;

        let img_pixmap = Pixmap::from_vec(
            img.into_vec(),
            IntSize::from_wh(img_width as u32, img_height as u32).ok_or_else(|| render_error::RenderError::Other("Invalid image size".to_string()))?
        ).ok_or_else(|| render_error::RenderError::Other("Invalid image data".to_string()))?;

        let sc = 0.25;

        pixmap.draw_pixmap(
            (parent_style.width / sc / 2.0 - img_width * sc * 2.0) as i32,
            (y / sc - img_height * sc - 20.0) as i32,
            img_pixmap.as_ref(),
            &PixmapPaint::default(),
            Transform::from_scale(context.scale_factor * sc, context.scale_factor * sc),
            None,
        );

        Ok(())
    }
}

impl Image {
    pub fn new(path: String, width: f32) -> Image {
        Image {
            path,
            width,
            children: vec![],
        }
    }
}
