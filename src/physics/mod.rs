
pub mod solver;

mod constant {
    pub const K: f32 = 9E9;
}

mod statics {
    use std::sync::LazyLock;
    use std::sync::Mutex;

    pub static ID_COUNTER: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
}

#[derive(Clone, PartialEq, Debug)]
pub struct Position(pub f32, pub f32);
#[derive(Clone, PartialEq, Debug)]
pub struct Velocity(pub f32, pub f32);
#[derive(Clone, PartialEq, Debug)]
pub struct Acceleration(pub f32, pub f32);
impl Acceleration {
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0.0;
        self.1 = 0.0;
    
        self
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct Force(pub f32, pub f32);

#[derive(Clone, PartialEq, Debug)]
pub struct PointBody {
    pub mass: f32,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,

    force_list: Vec<Force>,

    pub id: i32,
}
impl Default for PointBody {
    fn default() -> Self {
        let mut id_counter = statics::ID_COUNTER.lock().unwrap();
        *id_counter += 1;

        Self {
            position: Position(0.0, 0.0),
            velocity: Velocity(0.0, 0.0),
            acceleration: Acceleration(0.0, 0.0),
            force_list: Vec::new(),
            mass: 1.0,
            id: *id_counter,
        }
    }
}
impl PointBody {
    // Constructors.
    pub fn new_from_mass(mass: f32) -> Self {
        let mut id_counter = statics::ID_COUNTER.lock().unwrap();
        *id_counter += 1;
        let id_counter_num = *id_counter;
        drop(id_counter);

        Self {
            mass: mass,
            id: id_counter_num,

            ..Default::default()
        }
    }

    /// Motion integration.
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
        self.acceleration.clear();

        for force in &self.force_list {
            self.acceleration.0 += force.0 / self.mass;
            self.acceleration.1 += force.1 / self.mass;
        }

        self
    }
    pub fn apply_force(&mut self, force: Force) -> &mut Self {
        self.force_list.push(force);
        self
    }

    /// Distance from another PointBody object.    
    pub fn distance(&self, other: &PointBody) -> f32 {
        let x_distance = other.position.0 - self.position.0;
        let y_distance = other.position.1 - self.position.1;

        f32::sqrt(x_distance.powi(2) + y_distance.powi(2))
    }
    /// Angle between self and another PointBody object.
    pub fn angle(&self, other: &PointBody) -> f32 {
        let x_distance = other.position.0 - self.position.0;
        let y_distance = other.position.1 - self.position.1;

        f32::atan2(y_distance, x_distance)
    }
    pub fn acceleration_angle(&self, other: &PointBody) -> f32 {
        let x_acceleration = self.acceleration.0 - other.acceleration.0;
        let y_acceleration = self.acceleration.1 - other.acceleration.1;

        f32::atan2(y_acceleration, x_acceleration)
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
    pub fn new_from_position(charge: f32, mass: f32, position: Position) -> Self {
        Self {
            charge: charge,
            body: PointBody {
                mass: mass,
                position: position,
                ..Default::default()
            },

            ..Default::default()
        }
    }
    /// Returns a `Force` tuple vector in the direction of `other`.
    pub fn electric_force(&self, other: &Charge) -> Force {
        let electric_force_magnitude = -1.0 * constant::K * self.charge * other.charge
            / self.body.distance(&other.body).powi(2);
        let angle = self.body.angle(&other.body);

        Force(
            electric_force_magnitude * f32::cos(angle),
            electric_force_magnitude * f32::sin(angle),
        )
    }
}
