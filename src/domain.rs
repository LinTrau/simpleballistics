extern crate nalgebra as na;
use na::{Vector3};

#[derive(Debug, Clone)]
pub struct State {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub time: f64,
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub gravity: Vector3<f64>,
    pub air_density: f64
}

#[derive(Debug, Clone)]
pub struct ProjectileConfig {
    pub mass: f64,
    pub drag_coefficient: f64,
    pub area: f64
}

impl State{
    pub fn new(position:Vector3<f64>,velocity:Vector3<f64>,time:f64)->Self{
        Self{position,velocity,time}
    }
}

impl Environment{
    pub fn new(gravity:Vector3<f64>,air_density:f64)->Self{
        Self{gravity,air_density}
    }
}

impl ProjectileConfig{
    pub fn new(mass:f64,drag_coefficient:f64,area:f64)->Self{
        Self{mass,drag_coefficient,area}
    }
}
