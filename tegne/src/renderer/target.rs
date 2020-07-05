// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Target - struct that collects draw calls to be used in a renderer

use crate::color::Color;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::pipeline::Light;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::resource::Builtins;
use crate::resource::Id;
use crate::resource::IdRef;
use crate::resource::ResourceManager;

pub struct Target<'a> {
    orders_by_shader: Vec<OrdersByShader>,
    clear: Color,
    lights: Vec<Light>,
    current_shader: IdRef,
    current_material: IdRef,
    current_albedo: IdRef,
    current_framebuffer: Option<IdRef>,
    current_font: IdRef,
    has_shadows: bool,
    wireframes: bool,
    sampler_nearest: bool,
    sampler_clamp: bool,
    sampler_no_mipmaps: bool,
    bias: f32,
    resources: &'a ResourceManager,
    builtins: Builtins,
}

pub(crate) struct OrdersByShader {
    shader: IdRef,
    orders_by_material: Vec<OrdersByMaterial>,
}

pub(crate) struct OrdersByMaterial {
    material: IdRef,
    orders: Vec<Order>,
}

#[derive(Copy, Clone)]
pub(crate) struct Order {
    pub(crate) mesh: IdRef,
    pub(crate) albedo: IdRef,
    pub(crate) framebuffer: Option<IdRef>,
    pub(crate) model: Matrix4,
    pub(crate) has_shadows: bool,
    pub(crate) sampler_index: i32,
}

impl<'a> Target<'a> {
    pub(crate) fn new(resources: &'a ResourceManager, builtins: &Builtins) -> Result<Self> {
        Ok(Self {
            orders_by_shader: vec![],
            clear: Color::rgba_norm(0.7, 0.7, 0.7, 1.0),
            lights: vec![],
            current_shader: builtins.phong_shader.id_ref(),
            current_material: builtins.white_material.id_ref(),
            current_albedo: builtins.white_texture.id_ref(),
            current_framebuffer: None,
            current_font: builtins.roboto_font.id_ref(),
            has_shadows: false,
            wireframes: false,
            sampler_nearest: false,
            sampler_clamp: false,
            sampler_no_mipmaps: false,
            bias: 0.004,
            builtins: builtins.clone(),
            resources,
        })
    }

    pub fn draw(&mut self, mesh: &Id<Mesh>, transform: impl Into<Transform>) {
        self.add_order(Order {
            mesh: mesh.id_ref(),
            albedo: self.current_albedo,
            framebuffer: self.current_framebuffer,
            model: transform.into().as_matrix(),
            has_shadows: true,
            sampler_index: self.sampler_combination(),
        });
    }

    pub fn draw_cube(&mut self, transform: impl Into<Transform>) {
        self.add_order(Order {
            mesh: self.builtins.cube_mesh.id_ref(),
            albedo: self.current_albedo,
            framebuffer: self.current_framebuffer,
            model: transform.into().as_matrix(),
            has_shadows: true,
            sampler_index: self.sampler_combination(),
        });
    }

    pub fn draw_sphere(&mut self, transform: impl Into<Transform>) {
        self.add_order(Order {
            mesh: self.builtins.sphere_mesh.id_ref(),
            albedo: self.current_albedo,
            framebuffer: self.current_framebuffer,
            model: transform.into().as_matrix(),
            has_shadows: true,
            sampler_index: self.sampler_combination(),
        });
    }

    pub fn draw_surface(&mut self) {
        self.add_order(Order {
            mesh: self.builtins.surface_mesh.id_ref(),
            albedo: self.current_albedo,
            framebuffer: self.current_framebuffer,
            model: Transform::from([0.0, 0.0, 0.0]).as_matrix(),
            has_shadows: false,
            sampler_index: self.sampler_combination(),
        });
    }

    pub fn blit_framebuffer(&mut self, framebuffer: &Id<Framebuffer>) {
        let temp_shader = self.current_shader;
        self.current_shader = self.builtins.blit_shader.id_ref();
        self.current_framebuffer = Some(framebuffer.id_ref());

        self.draw_surface();

        self.current_shader = temp_shader;
        self.current_framebuffer = None;
    }

    pub fn draw_text(&mut self, text: impl AsRef<str>, transform: impl Into<Transform>) {
        let temp_shader = self.current_shader;
        self.current_shader = self.builtins.font_shader.id_ref();
        let text_str = text.as_ref();

        self.resources.with_font(self.current_font, |font| {
            let mut current_transform = transform.into();
            let x_scale = current_transform.scale.x;
            current_transform.position.x -=
                font.char_bearing(text_str.chars().next().unwrap()) * x_scale;
            let albedo = font.texture();

            for c in text_str.chars() {
                if c == ' ' {
                    let space_advance = font.char_advance('_');
                    current_transform.position.x += space_advance * x_scale;
                    continue;
                }

                let mesh = font.char_mesh(c);
                self.add_order(Order {
                    mesh,
                    albedo,
                    framebuffer: self.current_framebuffer,
                    model: current_transform.as_matrix(),
                    has_shadows: false,
                    sampler_index: self.sampler_combination(),
                });

                current_transform.position.x += font.char_advance(c) * x_scale;
            }
        });

        self.current_shader = temp_shader;
    }

    pub fn add_directional_light(
        &mut self,
        direction: impl Into<Vector3>,
        color: impl Into<Color>,
    ) {
        self.lights.push(Light {
            coords: direction.into().extend(0.0),
            color: color.into().to_rgba_norm_vec(),
        });
    }

    pub fn set_clear(&mut self, clear: impl Into<Color>) {
        self.clear = clear.into();
    }

    pub fn set_material(&mut self, material: &Id<Material>) {
        self.current_material = material.id_ref();
    }

    pub fn set_albedo_texture(&mut self, texture: &Id<Texture>) {
        self.current_albedo = texture.id_ref();
    }

    pub fn set_shader(&mut self, shader: &Id<Shader>) {
        self.current_shader = shader.id_ref();
    }

    pub fn set_framebuffer(&mut self, framebuffer: &Id<Framebuffer>) {
        self.current_framebuffer = Some(framebuffer.id_ref());
    }

    pub fn enable_wireframes(&mut self) {
        self.wireframes = true;
    }

    pub fn set_wireframes(&mut self, enable: bool) {
        self.wireframes = enable;
    }

    pub fn enable_sampler_nearest(&mut self) {
        self.sampler_nearest = true;
    }

    pub fn enable_sampler_clamp(&mut self) {
        self.sampler_clamp = true;
    }

    pub fn enable_sampler_no_mipmaps(&mut self) {
        self.sampler_no_mipmaps = true;
    }

    pub fn set_bias(&mut self, amount: f32) {
        self.bias = amount;
    }

    pub fn reset(&mut self) {
        self.current_material = self.builtins.white_material.id_ref();
        self.current_albedo = self.builtins.white_texture.id_ref();
        self.current_shader = self.builtins.phong_shader.id_ref();
        self.current_framebuffer = None;
        self.wireframes = false;
        self.sampler_nearest = false;
        self.sampler_clamp = false;
        self.sampler_no_mipmaps = false;
    }

    pub(crate) fn clear(&self) -> [f32; 4] {
        self.clear.to_rgba_norm()
    }

    pub(crate) fn orders_by_shader(&self) -> impl Iterator<Item = &OrdersByShader> {
        self.orders_by_shader.iter()
    }

    pub(crate) fn lights(&self) -> [Light; 3] {
        let mut lights: [Light; 3] = Default::default();
        lights[..self.lights.len()].clone_from_slice(&self.lights[..]);
        lights
    }

    pub(crate) fn has_shadows(&self) -> bool {
        self.has_shadows
    }

    pub(crate) fn bias(&self) -> f32 {
        self.bias
    }

    fn add_order(&mut self, order: Order) {
        let material = self.current_material;
        let shader = self.current_shader;

        if order.has_shadows {
            self.has_shadows = true;
        }

        match self
            .orders_by_shader
            .iter_mut()
            .find(|so| so.shader == shader)
        {
            Some(so) => match so
                .orders_by_material
                .iter_mut()
                .find(|mo| mo.material == material)
            {
                Some(mo) => mo.orders.push(order),
                None => so.orders_by_material.push(OrdersByMaterial {
                    material,
                    orders: vec![order],
                }),
            },
            None => self.orders_by_shader.push(OrdersByShader {
                shader,
                orders_by_material: vec![OrdersByMaterial {
                    material,
                    orders: vec![order],
                }],
            }),
        }

        if self.wireframes {
            let wireframe_shader = self.builtins.wireframe_shader.id_ref();
            match self
                .orders_by_shader
                .iter_mut()
                .find(|so| so.shader == wireframe_shader)
            {
                Some(so) => so.orders_by_material[0].orders.push(order),
                None => self.orders_by_shader.push(OrdersByShader {
                    shader: wireframe_shader,
                    orders_by_material: vec![OrdersByMaterial {
                        material: self.builtins.white_material.id_ref(),
                        orders: vec![order],
                    }],
                }),
            }
        }
    }

    fn sampler_combination(&self) -> i32 {
        let mut index = 0;
        if self.sampler_nearest {
            index += 4;
        }
        if self.sampler_clamp {
            index += 2;
        }
        if self.sampler_no_mipmaps {
            index += 1;
        }
        index
    }
}

impl OrdersByShader {
    pub(crate) fn shader(&self) -> IdRef {
        self.shader
    }

    pub(crate) fn orders_by_material(&self) -> impl Iterator<Item = &OrdersByMaterial> {
        self.orders_by_material.iter()
    }
}

impl OrdersByMaterial {
    pub(crate) fn material(&self) -> IdRef {
        self.material
    }

    pub(crate) fn orders(&self) -> impl Iterator<Item = Order> + '_ {
        self.orders.iter().cloned()
    }
}
