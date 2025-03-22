pub trait Component: Sized + 'static {
    fn update(&mut self, delta_time: f32);
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8; 6]) -> Self;
}

pub trait Entity: Sized {
    fn add_component<C: Component>(&mut self, component: C);
    fn remove_component<C: Component>(&mut self);
    fn get_component<C: Component>(&self) -> Option<&C>;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8; 6]) -> Self;
}
