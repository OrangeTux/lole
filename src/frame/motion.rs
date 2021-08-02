// This struct is around 1300 bytes. Is that maybe to large to implement `Clone()`?
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MotionBody {
    pub car_motion: [CarMotion; 22],
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CarMotion {
    pub world_position_x: f32,
    pub world_position_y: f32,
    pub world_position_z: f32,

    pub world_velocity_x: f32,
    pub world_velocity_y: f32,
    pub world_velocity_z: f32,

    pub world_forward_direction_x: u16,
    pub world_forward_direction_y: u16,
    pub world_forward_direction_z: u16,

    pub world_right_direction_x: u16,
    pub world_right_direction_y: u16,
    pub world_right_direction_z: u16,

    pub g_force_lateral: f32,
    pub g_force_longitudinal: f32,
    pub g_force_vertical: f32,

    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl CarMotion {
    // One could think: "Why not implementing the trait std::default::Default()?". Well, a
    // CarMotion filled with all zeroes doesn't make sense semantically.  That is why a `default()`
    // method isn't part of the public API. As trait methods are `pub` always, a custom `default()`
    // method has been implemented with limited visibility.
    pub(crate) fn default() -> Self {
        CarMotion {
            world_position_x: std::default::Default::default(),
            world_position_y: std::default::Default::default(),
            world_position_z: std::default::Default::default(),
            world_velocity_x: std::default::Default::default(),
            world_velocity_y: std::default::Default::default(),
            world_velocity_z: std::default::Default::default(),
            world_forward_direction_x: std::default::Default::default(),
            world_forward_direction_y: std::default::Default::default(),
            world_forward_direction_z: std::default::Default::default(),
            world_right_direction_x: std::default::Default::default(),
            world_right_direction_y: std::default::Default::default(),
            world_right_direction_z: std::default::Default::default(),
            g_force_lateral: std::default::Default::default(),
            g_force_longitudinal: std::default::Default::default(),
            g_force_vertical: std::default::Default::default(),
            yaw: std::default::Default::default(),
            pitch: std::default::Default::default(),
            roll: std::default::Default::default(),
        }
    }
}
