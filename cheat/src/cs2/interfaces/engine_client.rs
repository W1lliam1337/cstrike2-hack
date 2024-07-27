use memory_macros::vfunc;

pub struct EngineClient {}

impl EngineClient {
    #[vfunc(35)]
    pub fn is_in_game(&self) -> bool {}
}
