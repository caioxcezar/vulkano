use vulkano::{
    device::physical::PhysicalDevice,
    instance::{Instance, InstanceCreateInfo},
};

fn main() {
    let instance = Instance::new(InstanceCreateInfo::default())
        .expect("Falha ao criar a instância de Vulkano");

    // Listando dispositivos(Placas de video) que suportam Vulkan
    let physical_devices = PhysicalDevice::enumerate(&instance);

    for device in physical_devices {
        println!(
            "Dispositivo compatível com vulkan: {}",
            device.properties().device_name
        );
    }
}
