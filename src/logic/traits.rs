use super::config::ConfigLogic;

pub trait AbstractLogicEntity {
    fn run(&mut self);
    fn update_config(&mut self, config: &ConfigLogic);
}
