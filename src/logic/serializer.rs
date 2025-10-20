use super::config::ConfigLogic;
use super::traits::AbstractLogicEntity;

pub struct Serializer {}

impl Default for Serializer {
    fn default() -> Self {
        Self {}
    }
}

impl AbstractLogicEntity for Serializer {
    fn update_config(&mut self, config: &ConfigLogic) {}
    fn run(&mut self) {}
}
