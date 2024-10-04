pub mod entry {
    use crate::physics::solver::Solver;
    use crate::physics::Charge;

    use std::sync::{LazyLock, Mutex};

    static SOLVER_MUTEX: LazyLock<Mutex<Solver>> = LazyLock::new(|| Mutex::new(Solver::new()));
    static INITIAL_POS: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(100.0));
    static CHARGE_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
    static DT: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(0.0));

    pub fn entry(ui: &mut egui::Ui, ctx: &egui::Context) {
        let mut solver = SOLVER_MUTEX.lock().unwrap();
        let mut initial_pos = INITIAL_POS.lock().unwrap();
        let mut charge_id = CHARGE_ID.lock().unwrap();
        let mut dt = DT.lock().unwrap();

        if button::add_charge(ui).clicked() {
            let mut charge = Box::new(Charge::new(0.1, 0.1));
            charge.body.position.0 = *initial_pos;
            charge.body.position.1 = *initial_pos;
            charge.body.velocity.0 = 30.0;

            *initial_pos += 15.0;

            let mut charge2 = Box::new(Charge::new(0.1, 0.1));
            charge2.body.position.0 = *initial_pos; 
            charge2.body.position.1 = 150.0;
            charge2.body.velocity.0 = 0.0;
            charge2.body.mass = 100000000.0;

            solver.charge1 = *charge;
            solver.charge2 = *charge2;

            *charge_id = 2;
        }

        if *charge_id == 2 {
            solver.solve(*dt);
            *dt += 0.001;
            super::draw::circle_filled(
                &ui,
                egui::pos2(
                    solver.charge1.body.position.0.to_owned(),
                    solver.charge1.body.position.1.to_owned(),
                ),
                10.0,
                egui::Color32::BLUE,
            );
            super::draw::circle_filled(
                &ui,
                egui::pos2(
                    solver.charge2.body.position.0.to_owned(),
                    solver.charge2.body.position.1.to_owned(),
                ),
                10.0,
                egui::Color32::RED,
            );
            
            let x1 = solver.charge1.body.position.0;
            let x2 = solver.charge2.body.position.0;
            let y1 = solver.charge1.body.position.1;
            let y2 = solver.charge2.body.position.1;
            super::draw::line(&ui, x1, y1, solver.charge1.body.force_angle());
            super::draw::line(&ui, x2, y2, solver.charge2.body.force_angle());    

            ctx.request_repaint();
        }

        // for charge in &solver.charge_list {
        //     let x = charge.body.position.0;
        //     let y = charge.body.position.1;

        //     super::draw::circle_filled(&ui, egui::pos2(x, y), 20.0, egui::Color32::BLUE);
        // }
    }

    mod button {
        pub fn add_charge(ui: &mut egui::Ui) -> egui::Response {
            ui.button("Add Charge")
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
