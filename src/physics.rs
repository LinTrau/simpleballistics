use crate::domain::{State, Environment, ProjectileConfig};
use nalgebra::Vector3;

pub fn total_force(state: &State, env: &Environment, config: &ProjectileConfig) -> Vector3<f64> {
    let epsi_m = 1e-8;
    let epsi_v = 1e-10;

    let g = if config.mass < epsi_m {
        Vector3::zeros()
    } else {
        config.mass * env.gravity
    };

    let v_norm = state.velocity.norm();
    if v_norm < epsi_v {
        return g;
    }

    let fr = -0.5 
        * env.air_density 
        * v_norm 
        * v_norm 
        * config.drag_coefficient 
        * config.area 
        * state.velocity.normalize();

    g + fr
}
