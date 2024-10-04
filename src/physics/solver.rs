use super::Charge;

#[derive(Default)]
pub struct Solver {
    pub charge_list: Vec<Charge>,
    pub charge1: Charge,
    pub charge2: Charge,
}
impl Solver {
    pub fn new() -> Solver {
        Solver::default()
    }

    pub fn solve(&mut self, dt: f32) {
        // let charge_list_ref = self.charge_list.clone();

        // for other in &charge_list_ref {
        //     for charge in &mut self.charge_list {
        //         if charge != other {
        //             charge.body.apply_force(charge.electric_force(other));
        //         }
        //     }

        // for charge in &mut self.charge_list {
        //     charge.body.update_motion(dt);
        // }

        self.charge1
            .body
            .apply_force(self.charge1.electric_force(&self.charge2));
        self.charge2
            .body
            .apply_force(self.charge2.electric_force(&self.charge1));

        self.charge1.body.update_motion(dt);
        self.charge2.body.update_motion(dt);
    }

    pub fn add_charge(&mut self, charge: Charge) -> &mut Self {
        self.charge_list.push(charge);
        self
    }
}
