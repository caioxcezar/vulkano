use vulkano::{
    device::{physical::PhysicalDevice, Device, DeviceCreateInfo, QueueCreateInfo},
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

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("Nenhum dispositivo disponível");
    // Queues são como threads de uma CPU.
    for family in physical.queue_families() {
        println!(
            "Foi encontrada um 'queue family' com {:?} 'queue(s)'",
            family.queues_count()
        );
    }

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("Não foi possível encontrar uma 'queue family' grafica");

    // Ao criar um dispositivo é retornado o proprio dispositivo e a lista de
    // de filas para submissão de operações
    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo {
            // Passando a familia que será utilizada
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            ..Default::default()
        },
    )
    .expect("Falha ao criar dispositivo");

    let queue = queues.next().unwrap();
}
