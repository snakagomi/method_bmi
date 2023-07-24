struct Body {
    height: f64,
    weight: f64,
}

impl Body {
    fn calc_bmi(&self) -> f64 {
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }

    fn calc_per(&self) -> f64 {
        self.calc_bmi() / 22.0 * 100.0
    }
}

fn main() {
    let jiro = Body {
        height: 180.0,
        weight: 100.0,
    };
    println!("BMI={:.2}", jiro.calc_bmi());
    println!("乖離率={:.1}%", jiro.calc_per());
}
