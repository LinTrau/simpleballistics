use crate::domain::{State, ProjectileConfig};
use nalgebra::Vector3;

pub fn step(state: &State, projectile: &ProjectileConfig, force: Vector3<f64>, dt: f64) -> State {
    let acc = force / projectile.mass;

    let v_new = state.velocity + acc * dt;

    let p_new = state.position + v_new * dt;

    let t_new = state.time + dt;

    State {
        position: p_new,
        velocity: v_new,
        time: t_new,
    }
}
