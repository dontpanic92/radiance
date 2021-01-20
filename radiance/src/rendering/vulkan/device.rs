use std::rc::Rc;

use ash::{Instance, prelude::VkResult, vk::{CommandBufferBeginInfo, CommandPool, CommandPoolCreateInfo, CopyDescriptorSet, DescriptorPool, DescriptorPoolCreateInfo, DescriptorSet, DescriptorSetAllocateInfo, DescriptorSetLayout, DescriptorSetLayoutCreateInfo, Fence, ImageView, ImageViewCreateInfo, PhysicalDevice, Queue, SubmitInfo, WriteDescriptorSet}};
use ash::{
    version::DeviceV1_0,
    vk::{CommandBuffer, CommandBufferAllocateInfo, DescriptorPoolResetFlags},
};

use super::creation_helpers;

pub struct Device {
    instance: Rc<Instance>,
    device: ash::Device,
}

impl Device {
    pub fn new(
        instance: Rc<Instance>,
        physical_device: PhysicalDevice,
        graphics_queue_family_index: u32,
    ) -> Self {
        let device = creation_helpers::create_device(
            &instance,
            physical_device,
            graphics_queue_family_index,
        )
        .unwrap();

        Self { instance, device }
    }

    pub fn vk_device(&self) -> &ash::Device {
        &self.device
    }

    pub fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> Queue {
        unsafe {
            self.device
                .get_device_queue(queue_family_index, queue_index)
        }
    }

    pub fn create_command_pool(
        &self,
        create_info: &CommandPoolCreateInfo,
    ) -> VkResult<CommandPool> {
        unsafe { self.device.create_command_pool(&create_info, None) }
    }

    pub fn create_descriptor_pool(
        &self,
        create_info: &DescriptorPoolCreateInfo,
    ) -> VkResult<DescriptorPool> {
        unsafe { self.device.create_descriptor_pool(&create_info, None) }
    }

    pub fn create_descriptor_set_layout(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo,
    ) -> VkResult<DescriptorSetLayout> {
        unsafe { self.device.create_descriptor_set_layout(&create_info, None) }
    }

    pub fn allocate_descriptor_sets(
        &self,
        create_info: &DescriptorSetAllocateInfo,
    ) -> VkResult<Vec<DescriptorSet>> {
        unsafe { self.device.allocate_descriptor_sets(&create_info) }
    }

    pub fn update_descriptor_sets(
        &self,
        write_descriptor_sets: &[WriteDescriptorSet],
        copy_descriptor_sets: &[CopyDescriptorSet],
    ) {
        unsafe {
            self.device
                .update_descriptor_sets(write_descriptor_sets, copy_descriptor_sets)
        }
    }

    pub fn reset_descriptor_pool(&self, pool: DescriptorPool) -> VkResult<()> {
        unsafe {
            self.device
                .reset_descriptor_pool(pool, DescriptorPoolResetFlags::empty())
        }
    }

    pub fn destroy_descriptor_set_layout(&self, descriptor_set_layout: DescriptorSetLayout) {
        unsafe {
            self.device
                .destroy_descriptor_set_layout(descriptor_set_layout, None);
        }
    }

    pub fn destroy_descriptor_pool(&self, descriptor_pool: DescriptorPool) {
        unsafe {
            self.device.destroy_descriptor_pool(descriptor_pool, None);
        }
    }

    pub fn allocate_command_buffers(
        &self,
        allocation_info: &CommandBufferAllocateInfo,
    ) -> VkResult<Vec<CommandBuffer>> {
        unsafe { self.device.allocate_command_buffers(&allocation_info) }
    }

    pub fn begin_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        begin_info: &CommandBufferBeginInfo,
    ) -> VkResult<()> {
        unsafe {
            self.device
                .begin_command_buffer(command_buffer, &begin_info)
        }
    }

    pub fn end_command_buffer(&self, command_buffer: CommandBuffer) -> VkResult<()> {
        unsafe { self.device.end_command_buffer(command_buffer) }
    }

    pub fn queue_submit(&self, queue: Queue, submits: &[SubmitInfo], fence: Fence) -> VkResult<()> {
        unsafe { self.device.queue_submit(queue, submits, fence) }
    }

    pub fn queue_wait_idle(&self, queue: Queue) -> VkResult<()> {
        unsafe { self.device.queue_wait_idle(queue) }
    }

    pub fn free_command_buffers(
        &self,
        command_pool: CommandPool,
        command_buffers: &[CommandBuffer],
    ) {
        unsafe {
            self.device
                .free_command_buffers(command_pool, &command_buffers)
        }
    }

    pub fn create_image_view(&self, create_info: &ImageViewCreateInfo) -> VkResult<ImageView> {
        unsafe { self.device.create_image_view(&create_info, None) }
    }

    pub fn destroy_image_view(&self, image_view: ImageView) {
        unsafe {
            self.device.destroy_image_view(image_view, None);
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
