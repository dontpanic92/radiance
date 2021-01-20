use ash::prelude::VkResult;
use ash::version::DeviceV1_0;
use ash::{vk, Device};
use std::rc::Rc;

pub struct Sampler {
    device: Rc<Device>,
    sampler: vk::Sampler,
}

impl Sampler {
    pub fn new(device: &Rc<Device>) -> VkResult<Self> {
        let sampler_info = vk::SamplerCreateInfo::builder()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::REPEAT)
            .address_mode_v(vk::SamplerAddressMode::REPEAT)
            .address_mode_w(vk::SamplerAddressMode::REPEAT)
            .anisotropy_enable(true)
            .max_anisotropy(16.)
            .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.)
            .min_lod(0.)
            .max_lod(0.)
            .build();
        let sampler = unsafe { device.create_sampler(&sampler_info, None)? };
        Ok(Self {
            device: device.clone(),
            sampler,
        })
    }

    pub fn vk_sampler(&self) -> vk::Sampler {
        self.sampler
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_sampler(self.sampler, None);
        }
    }
}
