pub struct Structure {
    id: u32,
    module_tpye: ModuleType,
    children: Vec<u32>,
    connections: Vec<(u32, u32)>,
}

pub enum ModuleType {
    Module,
    Group,
    MaximumFusion,
}