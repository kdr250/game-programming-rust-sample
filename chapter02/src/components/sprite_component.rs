use std::rc::Rc;

use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

use crate::{
    components::component::Component,
    math::{self},
};

pub trait SpriteComponent: Component {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        if let Some(texture) = self.get_texture() {
            let owner = self.get_owner().borrow();
            let width = self.get_texture_width() as f32 * owner.get_scale();
            let height = self.get_texture_height() as f32 * owner.get_scale();
            let rect = Rect::new(
                (owner.get_position().x - width / 2.0) as i32,
                (owner.get_position().y - height / 2.0) as i32,
                width as u32,
                height as u32,
            );

            canvas
                .copy_ex(
                    &texture,
                    None,
                    Some(rect),
                    -math::to_degrees(owner.get_rotation()) as f64,
                    None,
                    false,
                    false,
                )
                .unwrap();
        }
    }

    fn get_texture(&self) -> Option<&Rc<Texture>>;

    fn set_texture(&mut self, texture: Rc<Texture>);

    fn get_draw_order(&self) -> i32;

    fn get_texture_height(&self) -> u32;

    fn get_texture_width(&self) -> u32;
}

macro_rules! impl_getters_setters {
    () => {
        fn get_texture(&self) -> Option<&Rc<Texture>> {
            self.texture.as_ref()
        }

        fn set_texture(&mut self, texture: Rc<Texture>) {
            let query = texture.query();
            self.texture_height = query.height;
            self.texture_width = query.width;
            self.texture = Some(texture);
        }

        fn get_draw_order(&self) -> i32 {
            self.draw_order
        }

        fn get_texture_height(&self) -> u32 {
            self.texture_height
        }

        fn get_texture_width(&self) -> u32 {
            self.texture_width
        }
    };
}

pub(crate) use impl_getters_setters;
