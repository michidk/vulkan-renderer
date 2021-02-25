use ash::{version::DeviceV1_0, vk};

use crate::{
    scene::{
        camera::{self, Camera},
        Scene,
    },
    vulkan::manager::VulkanManager,
};

pub struct GameLoop {}

impl GameLoop {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }

    pub(crate) fn init(&self) {}

    // todo: implement Update, Render traits and then create type def of combined type; then have a list of them in SceneManager, and call update for all of them
    pub(crate) fn update(&self, vulkan_manager: &mut VulkanManager, scene: &Scene) {
        let mut vk = vulkan_manager;

        scene.light_manager.update_buffer(
            &vk.device,
            &vk.allocator,
            &mut vk.light_buffer,
            &mut vk.descriptor_sets_light,
        );
        let image_index = vk.swapchain.aquire_next_image();
        vk.wait_for_fence();
        // camera.update_buffer(&vk.allocator, &mut &vk.uniform_buffer);
        for m in &mut vk.models {
            m.update_instance_buffer(&vk.allocator).unwrap();
        }
        &vk.update_commandbuffer(image_index as usize)
            .expect("updating the command buffer");

        let semaphores_finished = vk.render(image_index);
        vk.present(image_index, &semaphores_finished);
    }
}
