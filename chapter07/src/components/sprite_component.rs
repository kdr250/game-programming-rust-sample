use std::{cell::RefCell, ptr::null, rc::Rc};

use gl::{TRIANGLES, UNSIGNED_INT};

use crate::{
    actors::actor::Actor,
    components::component::Component,
    graphics::{shader::Shader, texture::Texture},
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
};

pub trait SpriteComponent: Component {
    fn draw(&self, shader: &Shader) {
        if let Some(texture) = self.get_texture() {
            // Scale the quad by the width/height of texture
            let scale_mat = Matrix4::create_scale_xyz(
                self.get_texture_width() as f32,
                self.get_texture_height() as f32,
                1.0,
            );

            let world = scale_mat * self.get_owner().borrow().get_world_transform().clone();

            // Set world transform
            shader.set_matrix_uniform("uWorldTransform", world);
            // Set current texture
            texture.set_active();

            unsafe {
                // Draw
                gl::DrawElements(TRIANGLES, 6, UNSIGNED_INT, null());
            }
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
            self.texture_height = texture.get_height() as u32;
            self.texture_width = texture.get_width() as u32;
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

use super::component::{self, State};

pub struct DefaultSpriteComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    texture: Option<Rc<Texture>>,
    draw_order: i32,
    texture_width: u32,
    texture_height: u32,
}

impl DefaultSpriteComponent {
    pub fn new(owner: Rc<RefCell<dyn Actor>>, draw_order: i32) -> Rc<RefCell<Self>> {
        let this = Self {
            id: component::generate_id(),
            owner: owner.clone(),
            update_order: 100,
            state: State::Active,
            texture: None,
            draw_order,
            texture_height: 0,
            texture_width: 0,
        };

        let result = Rc::new(RefCell::new(this));

        owner.borrow_mut().add_component(result.clone());

        owner
            .borrow()
            .get_asset_manager()
            .borrow_mut()
            .add_sprite(result.clone());

        result
    }
}

impl SpriteComponent for DefaultSpriteComponent {
    impl_getters_setters! {}
}

impl Component for DefaultSpriteComponent {
    fn update(
        &mut self,
        _delta_time: f32,
        _owner_info: &(Vector3, Quaternion, Vector3, Matrix4),
    ) -> (Option<Vector3>, Option<Quaternion>) {
        (None, None)
    }

    component::impl_getters_setters! {}
}
