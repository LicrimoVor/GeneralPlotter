use super::traits::AbstractLogicEntity;
use crate::ui::ConfigLogic;

pub struct Extractor {}

impl Default for Extractor {
    fn default() -> Self {
        Self {}
    }
}

impl AbstractLogicEntity for Extractor {
    fn update_config(&mut self, config: &ConfigLogic) {}
    fn run(&mut self) {}
}
