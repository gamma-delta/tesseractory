pub type Vec4 = wedged::subspace::SimpleVec4<f32>;
/// Mostly used for the camera
pub type Vec3 = wedged::subspace::SimpleVec3<f32>;
pub type Vec2 = wedged::subspace::SimpleVec2<f32>;

pub type UnitVec4 = wedged::subspace::UnitVec4<f32>;
pub type UnitVec3 = wedged::subspace::UnitVec3<f32>;

pub type Rotor4 = wedged::subspace::Rotor4<f32>;
pub type Pseudoscalar4 = wedged::subspace::SimpleQuadVec4<f32>;

/// Glam types are used for quick-and-dirty things because it's easier
/// to just use them as lists of numbers
pub type GVec4 = glam::Vec4;
pub type GVec3 = glam::Vec3;
pub type GVec2 = glam::Vec2;
pub type GMat4 = glam::Mat4;

pub type Color = godot::prelude::Color;
