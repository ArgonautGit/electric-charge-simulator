pub mod entry {
    use crate::physics::solver::Solver;
    use crate::physics::Charge;

    use std::sync::{LazyLock, Mutex};

    const CHARGE_SIZE: f32 = 10.0;

    static SOLVER_MUTEX: LazyLock<Mutex<Solver>> = LazyLock::new(|| Mutex::new(Solver::new()));

    pub fn entry(ui: &mut egui::Ui, ctx: &egui::Context) {
        let mut solver = SOLVER_MUTEX.lock().unwrap();
        // let mut initial_pos = INITIAL_POS.lock().unwrap();
        // let mut charge_id = CHARGE_ID.lock().unwrap();

        if button::add_charge(ui).clicked() {
            if solver.charge_list.len() == 0 {
                let mut large_charge = Charge::new(0.25, 1E3);
                large_charge.body.position.0 = 200.0;
                large_charge.body.position.1 = 200.0;

                solver.add_charge(large_charge);
            }

            let mut charge = Charge::new(-0.005, 1.0);

            charge.body.position.0 = 200.0;
            charge.body.position.1 = 150.0;
            charge.body.velocity.0 = -500.0;


            solver.add_charge(charge);
        }

        if button::clear_charges(ui).clicked() {
            solver.charge_list.clear();
        }

        if solver.charge_list.len() > 0 { // if *charge_id == 2 {
            solver.solve(ui.input(|i| i.stable_dt));
            ctx.request_repaint();
        }

        // Draw each charge.
        for charge in &solver.charge_list {
            let x = charge.body.position.0;
            let y = charge.body.position.1;

            super::draw::circle_filled(&ui, egui::pos2(x, y), CHARGE_SIZE, egui::Color32::BLUE);
            for other in &solver.charge_list {
                if charge.body.id != other.body.id {
                    let x = charge.body.position.0;
                    let y = charge.body.position.1;
                    let angle = charge.body.acceleration_angle(&other.body);
                    super::draw::line(&ui, x, y, angle);
                }
            }
        }
    }

    mod button {
        pub fn add_charge(ui: &mut egui::Ui) -> egui::Response {
            ui.button("Add Charge")
        }

        pub fn clear_charges(ui: &mut egui::Ui) -> egui::Response {
            ui.button("Clear Charges")
        }
    }
}

pub mod draw {
    use egui::Color32;
    use egui::Painter;
    use egui::Pos2;

    pub fn circle_filled(
        ui: &egui::Ui,
        center: Pos2,
        radius: f32,
        fill_color: Color32,
    ) -> &Painter {
        ui.painter().circle_filled(center, radius, fill_color);
        ui.painter()
    }

    pub fn line(ui: &egui::Ui, x: f32, y: f32, angle: f32) -> &Painter {
        let start = egui::pos2(x, y);
        let end = egui::pos2(x + angle.cos() * 10.0, y + angle.sin() * 10.0);
        let stroke_width = 2.0;
        let stroke_color = egui::Color32::GREEN;

        ui.painter()
            .line_segment([start, end], (stroke_width, stroke_color));
        ui.painter()
    }
}
