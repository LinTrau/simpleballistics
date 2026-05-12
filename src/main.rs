mod domain;
mod engine;
mod physics;

use domain::{State, Environment, ProjectileConfig};
use nalgebra::Vector3;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let env = Environment {
        gravity: Vector3::new(0.0, 0.0, -9.806),
        air_density: 1.225,
    };

    let config = ProjectileConfig {
        mass: 0.5,
        drag_coefficient: 0.47,
        area: 0.01,
    };

    let angle: f64 = 45.0;
    let v0 = 100.0;
    let rad = angle.to_radians();
    let initial_velocity = Vector3::new(v0 * rad.cos(), 0.0, v0 * rad.sin());

    let mut current_state = State::new(Vector3::zeros(), initial_velocity, 0.0);
    let dt = 0.01;

    // 创建 CSV Writer
    let file = File::create("trajectory.csv")?;
    let mut wtr = csv::Writer::from_writer(file);

    // 写入表头
    wtr.write_record(&["Time", "X", "Y", "Z", "Velocity"])?;

    while current_state.position.z >= 0.0 {
        let f = physics::total_force(&current_state, &env, &config);
        current_state = engine::step(&current_state, &config, f, dt);

        // 写入数据行
        wtr.write_record(&[
            current_state.time.to_string(),
            current_state.position.x.to_string(),
            current_state.position.y.to_string(),
            current_state.position.z.to_string(),
            current_state.velocity.norm().to_string(),
        ])?;
    }

    wtr.flush()?; // 确保所有数据写入磁盘
    println!("模拟完成，数据已保存至 trajectory.csv");
    Ok(())
}
