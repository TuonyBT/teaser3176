fn main() {

    for p in 1_u32..8 {
        for b in (2 * p + 1)..15 {
            let a = 15 - b;

            let y_max = 2 * b + ((900 - a.pow(2)) as f64).sqrt().ceil() as u32;

            for y in (b + 1)..(y_max + 1) {
                println!("p: {}, b: {}, y: {}", p, b, y);

                if a * (y - 2 * p) % (b - 2 * p) == 0 {
                    let x = a * (y - 2 * p) / (b - 2 * p);

                    if 100 * a * b < 7 * x * y && x.pow(2) + (y - 2 * p).pow(2) == 900 {
                        println!("RESULT!! p: {}, a: {}, b: {}, x: {}, y: {}", p, a, b, x, y);
                    }
                }
            }
        }
    }

}
