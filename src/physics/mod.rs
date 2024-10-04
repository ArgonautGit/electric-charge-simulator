use std::any::Any;

pub mod solver;

mod constant {
    pub const K: f32 = 400000.0;
}

mod unit {
    pub type Distance = f32;
    pub type Angle = f32;
}

#[derive(Clone, PartialEq, Debug)]
pub struct Position(pub f32, pub f32);
#[derive(Clone, PartialEq, Debug)]
pub struct Velocity(pub f32, pub f32);
#[derive(Clone, PartialEq, Debug)]
pub struct Acceleration(pub f32, pub f32);
#[derive(Clone, PartialEq, Debug)]
pub struct Force(pub f32, pub f32);

#[derive(Clone, PartialEq, Debug)]
pub struct PointBody {
    pub position: Position,
    pub velocity: Velocity,
    acceleration: Acceleration,
    force_list: Vec<Force>,
    force: Force,

    pub mass: f32,
}
impl Default for PointBody {
    fn default() -> Self {
        Self {
            position: Position(0.1, 0.1),
            velocity: Velocity(0.0, 0.0),
            acceleration: Acceleration(0.0, 0.0),
            force_list: Vec::new(),
            force: Force(0.0, 0.0),

            mass: 0.0,
        }
    }
}
impl PointBody {
    // Constructors.
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_from_mass(mass: f32) -> Self {
        Self {
            mass: mass,

            ..Default::default()
        }
    }

    // Motion integration.
    pub fn update_motion(&mut self, dt: f32) -> &mut Self {
        self.update_acceleration()
        .update_velocity(dt)
        .update_position(dt);

        self.force_list.clear();

        self
    }
    pub fn update_position(&mut self, dt: f32) -> &mut Self {
        self.position.0 += self.velocity.0 * dt;
        self.position.1 += self.velocity.1 * dt;
        self
    }
    pub fn update_velocity(&mut self, dt: f32) -> &mut Self {
        self.velocity.0 += self.acceleration.0 * dt;
        self.velocity.1 += self.acceleration.1 * dt;
        self
    }
    pub fn update_acceleration(&mut self) -> &mut Self {
        // for force in &self.force_list {
        println!("mass: {}, force: {:?}", self.mass, self.force);
        self.acceleration.0 = self.force.0 / self.mass;
        self.acceleration.1 = self.force.1 / self.mass;
        // }
        self
    }
    pub fn apply_force(&mut self, force: Force) -> &mut Self {
        // self.force_list.push(force);
        self.force = force;
        self
    }

    /// Distance from another PointBody object.    
    pub fn distance(&self, other: &PointBody) -> unit::Distance {
        let x_distance = other.position.0 - self.position.0;
        let y_distance = other.position.1 - self.position.1;

        f32::sqrt(x_distance.powi(2) + y_distance.powi(2))
    }
    /// Angle between self and another PointBody object.
    pub fn angle(&self, other: &PointBody) -> unit::Angle {
        let x_distance = other.position.0 - self.position.0;
        let y_distance = other.position.1 - self.position.1;

        f32::atan2(y_distance, x_distance)
    }
    pub fn force_angle(&self) -> unit::Angle {
        f32::atan2(self.force.1, self.force.0)
    }
}
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Charge {
    pub body: PointBody,
    pub charge: f32,
}
impl Charge {
    // Constructors
    pub fn new(charge: f32, mass: f32) -> Self {
        Self {
            charge: charge,
            body: PointBody::new_from_mass(mass),

            ..Default::default()
        }
    }
    /// Returns a `Force` tuple vector in the direction of `other`.
    pub fn electric_force(&self, other: &Charge) -> Force {
        let electric_force_magnitude =
            constant::K * self.charge * other.charge / self.body.distance(&other.body).powi(2);
        let angle = self.body.angle(&other.body);

        Force(
            electric_force_magnitude * f32::cos(angle),
            electric_force_magnitude * f32::sin(angle),
        )
    }
}
