use crate::{handle_clet_event, paint_clet, start_clet};

pub fn simulation_start() {
    wipic_simulation::simulation_start(start_clet, paint_clet, handle_clet_event);
}
