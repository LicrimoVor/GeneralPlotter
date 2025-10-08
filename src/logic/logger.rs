use super::traits::AbstractLogicEntity;
use crate::ui::ConfigLogic;

pub struct Logger {}

impl Default for Logger {
    fn default() -> Self {
        Self {}
    }
}

impl AbstractLogicEntity for Logger {
    fn update_config(&mut self, config: &ConfigLogic) {}
    fn run(&mut self) {}
}
