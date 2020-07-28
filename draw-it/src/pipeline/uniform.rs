// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// uniform structs to manage shader accessible uniform data

use ash::vk;
use std::sync::Arc;

use super::MaterialData;
use super::Sampler;
use super::SamplerAddress;
use super::SamplerFilter;
use super::SamplerMipmaps;
use super::SamplerOptions;
use super::ShaderLayout;
use super::WorldData;
use crate::buffer::BufferUsage;
use crate::buffer::DynamicBuffer;
use crate::device::Device;
use crate::error::Result;
use crate::image::ImageLayout;

pub(crate) struct WorldUniform {
    descriptor: Descriptor,
    buffer: DynamicBuffer,
}

pub(crate) struct MaterialUniform {
    descriptor: Descriptor,
    buffer: DynamicBuffer,
}

pub(crate) struct ImageUniform {
    descriptor: Descriptor,
    sampler_combinations: Vec<Sampler>,
    images: Vec<Option<vk::ImageView>>,
    skybox: Option<vk::ImageView>,
    should_update: bool,
    device: Arc<Device>,
}

pub(crate) struct ShadowMapUniform {
    descriptor: Descriptor,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Descriptor(pub u32, pub vk::DescriptorSet);

pub(crate) trait Uniform {
    fn descriptor(&self) -> Descriptor;
}

impl WorldUniform {
    pub(crate) fn new(device: &Arc<Device>, layout: &ShaderLayout) -> Result<Self> {
        let buffer = DynamicBuffer::new::<WorldData>(device, BufferUsage::Uniform, 1)?;

        let descriptor_set = layout.world_set(&buffer)?;
        let descriptor = Descriptor(0, descriptor_set);

        Ok(Self { buffer, descriptor })
    }

    pub(crate) fn update(&mut self, data: WorldData) -> Result<()> {
        self.buffer.update_data(&[data])
    }
}

impl Uniform for WorldUniform {
    fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}

impl MaterialUniform {
    pub(crate) fn new(device: &Arc<Device>, layout: &ShaderLayout) -> Result<Self> {
        let buffer = DynamicBuffer::new::<MaterialData>(device, BufferUsage::Uniform, 1)?;

        let descriptor_set = layout.material_set(&buffer)?;
        let descriptor = Descriptor(1, descriptor_set);

        Ok(Self { buffer, descriptor })
    }

    pub(crate) fn update(&mut self, data: MaterialData) -> Result<()> {
        self.buffer.update_data(&[data])
    }
}

impl Uniform for MaterialUniform {
    fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}

impl PartialEq for MaterialUniform {
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl ImageUniform {
    pub(crate) fn new(
        device: &Arc<Device>,
        layout: &ShaderLayout,
        anisotropy: f32,
    ) -> Result<Self> {
        profile_scope!("new");
        info!("using anisotropy level {}", anisotropy);

        let descriptor_set = layout.image_set()?;
        let descriptor = Descriptor(2, descriptor_set);

        // create sampler combinations
        let mut sampler_combinations = vec![];
        for filter in &[SamplerFilter::Linear, SamplerFilter::Nearest] {
            for address in &[SamplerAddress::Repeat, SamplerAddress::Clamp] {
                for mipmaps in &[SamplerMipmaps::Enabled, SamplerMipmaps::Disabled] {
                    sampler_combinations.push(Sampler::new(
                        device,
                        SamplerOptions {
                            anisotropy,
                            filter: *filter,
                            address: *address,
                            mipmaps: *mipmaps,
                        },
                    )?);
                }
            }
        }

        Ok(Self {
            descriptor,
            sampler_combinations,
            images: vec![],
            skybox: None,
            should_update: true,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn add(&mut self, image: vk::ImageView) -> i32 {
        let next_index = self.images.len();

        // find free index
        let index = self
            .images
            .iter()
            .position(|img| img.is_none())
            .unwrap_or(next_index);

        // add new or replace image
        if index == next_index {
            self.images.push(Some(image));
        } else {
            self.images[index] = Some(image);
        }

        self.should_update = true;
        index as i32
    }

    pub(crate) fn remove(&mut self, index: i32) {
        debug_assert!((index as usize) < self.images.len());

        // mark image as removed
        self.images[index as usize] = None;

        self.should_update = true;
    }

    pub(crate) fn set_skybox(&mut self, image: vk::ImageView) {
        self.skybox = Some(image);
        self.should_update = true;
    }

    pub(crate) fn update_if_needed(&mut self) {
        // update if image was added/removed
        if self.should_update {
            let mut writes = vec![];

            // configure image writes to descriptor
            let image_infos = (0..100)
                .map(|i| {
                    // get image or default image
                    let image = match self.images.get(i) {
                        Some(Some(img)) => *img,
                        _ => self.images[0].expect("bad code"),
                    };

                    vk::DescriptorImageInfo::builder()
                        .image_layout(ImageLayout::ShaderColor.flag())
                        .image_view(image)
                        .build()
                })
                .collect::<Vec<_>>();
            writes.push(
                vk::WriteDescriptorSet::builder()
                    .dst_set(self.descriptor.1)
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                    .image_info(&image_infos)
                    .build(),
            );

            // configure sampler writes to descriptor
            let sampler_info = self
                .sampler_combinations
                .iter()
                .map(|s| {
                    vk::DescriptorImageInfo::builder()
                        .image_layout(ImageLayout::ShaderColor.flag())
                        .sampler(s.handle())
                        .build()
                })
                .collect::<Vec<_>>();
            writes.push(
                vk::WriteDescriptorSet::builder()
                    .dst_set(self.descriptor.1)
                    .dst_binding(1)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::SAMPLER)
                    .image_info(&sampler_info)
                    .build(),
            );

            // configure skybox write to descriptor
            if let Some(skybox) = self.skybox {
                let skybox_info = [vk::DescriptorImageInfo::builder()
                    .image_layout(ImageLayout::ShaderColor.flag())
                    .image_view(skybox)
                    .build()];
                writes.push(
                    vk::WriteDescriptorSet::builder()
                        .dst_set(self.descriptor.1)
                        .dst_binding(2)
                        .dst_array_element(0)
                        .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                        .image_info(&skybox_info)
                        .build(),
                );
            };

            // write data to descriptor
            self.device.update_descriptor_sets(&writes);
            self.should_update = false;
        }
    }
}

impl Uniform for ImageUniform {
    fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}

impl ShadowMapUniform {
    pub(crate) fn new(layout: &ShaderLayout, views: [vk::ImageView; 4]) -> Result<Self> {
        let descriptor_set = layout.shadow_map_set(views)?;
        let descriptor = Descriptor(3, descriptor_set);

        Ok(Self { descriptor })
    }
}

impl Uniform for ShadowMapUniform {
    fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}
