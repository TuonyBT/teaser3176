
use std::collections::BTreeSet;
use itertools::Itertools;

fn main() {

//  METHOD 1: loop p first, then grow outwards from veg plot

//  consider a large rectangle, dimensions x * y, with one corner on the origin of two axes
//  inside this rectangle is a smaller one, dimensions a * b, also with one corner on the origin
//  the bisector of the smaller rectangle intercepts the y-axis at point p; 
//  it must therefore also intercept the right-hand side of the small rectangle at point (b - p)
//  and the right-hand side of the large rectangle at point (y - p)

//  this line forms a triangle within each rectangle, about which we can say the following:
//  the angle formed between the bisector and the x-axis (BOA) has a tangent (b - 2p) / a (small rectangle)
//  the angle formed between the bisector and the x-axis (YOX) has a tangent (y - 2p) / x (large rectangle)
//  this latter triangle also has a hypotenuse of 30 and the other two sides are x and (y - 2p)
//  also, b > 2p, a + b = 30 /2, so p < b/2: this defines the search space in terms of a, b and p

    for p in 1_u32..8 {
        for b in (2 * p + 1)..15 {
            let a = 15 - b;


//  we also know that x > a and y > b (by definition)
            let mut x = a + 1;
            let mut y = b;
//  so we grow the larger rectangle, keeping the tangents of BOA and YOX equal until the hypotenuse is too large
            while x.pow(2) + (y - 2 * p).pow(2) <= 900 {
                y += 1;
                if a * (y - 2 * p) % (b - 2 * p) == 0 {
                    x = a * (y - 2 * p) / (b - 2 * p);
//  a valid solution must have an integer value for x

// once the smaller triangle has an area less than 7% of the larger one, the solution has a hypotenuse of exactly 30
                    if 100 * a * b < 7 * x * y && x.pow(2) + (y - 2 * p).pow(2) == 900 {
                        println!("Loop p first RESULT!! p: {}, a: {}, b: {}, x: {}, y: {}", p, a, b, x, y);
                    }
                }
            }
        }
    }


//  METHOD 2: evaluate all Pythagorean triads for (x, y-2p, 30) and (a, b-2p, c) and build rectangles from congruent pairs

//  Smallest possible value for b is 14 (and a = 1), so smallest possible value for b - 2p is 2
//  Smallest possible value for c^2 = 1^2 + 2^2 = 5
    let n_min: u32 = 5.0_f64.sqrt().floor() as u32;

//  Evaluate all Pythagorean traids smaller than the one that fits in the lawn (a, b-2p, c)
    let veg_triads = (n_min..30).map(|n| pythag_triplets(n))
                                                            .filter(|s| !s.is_empty())
                                                            .flatten()
                                                            .collect::<Vec<[u32; 3]>>();

//  Evaluate all Pythagorean triads that fit the lawn (x, y-2p, 30)                                                   
    let lawn_triads = (30..31).map(|n| pythag_triplets(n))
                                                            .filter(|s| !s.is_empty())
                                                            .flatten()
                                                            .collect::<Vec<[u32; 3]>>();

//  Collect pairs of (veg, lawn) triads that form congruent triangles
    for ([a, b_, _c], [x, y_, _z]) in veg_triads.iter().cartesian_product(&lawn_triads) {
        if a * y_ != b_ * x || a + b_ > 14 || (a + b_) % 2 == 0 {continue}

//  Find p that matches the veg patch triangle and check the ratio of the areas is less than 7%
        let p = (15 - a - b_) / 2_;
        let veg_area = a * (b_ + 2 * p);
        let lawn_area = x * (y_ + 2 * p);
        if veg_area * 100 < lawn_area * 7 {
            println!("Compare Pythag triads RESULT!! p: {}, a: {}, b: {}, x: {}, y: {}", p, a, b_ + 2 * p, x, y_ + 2 * p);
        }
    }

//  METHOD 3: evaluate all Pythagorean triads for (x, y-2p, 30) and find the smallest congruent triangle for the veg patch

    for [x, y_, _z] in lawn_triads.iter() {
        let x_facs = prime_factor(*x as usize);
        let y_facs = prime_factor(*y_ as usize);

        let cfs = x_facs.iter().cartesian_product(&y_facs)
                                                        .filter(|(x, y)| x[0] == y[0])
                                                        .map(|(x, y)|[x[0], x[1].min(y[1])])
                                                        .collect::<Vec<[usize; 2]>>();

//  Highest common factor of the perpendicular sides of the lawn triangle
        let hcf = cfs.iter().map(|z| (z[0] as u32).pow(z[1] as u32)).product::<u32>();

//  Perpendicular sides of the smallest possible veg triangle with integer lengths: 
        let a = x / hcf;
        let b_ = y_ / hcf;
        if (15 - a - b_) % 2 != 0 {continue;}; // just in case we have a scenario where p is not an integer

//  Find p that matches the veg patch triangle and check the ratio of the areas is less than 7%
        let p = (15 - a - b_) / 2;
        let veg_area = a * (b_ + 2 * p);
        let lawn_area = x * (y_ + 2 * p);
        if veg_area * 100 < lawn_area * 7 {
            println!("Smallest congruent triangle RESULT!! p: {}, a: {}, b: {}, x: {}, y: {}", p, a, b_ + 2 * p, x, y_ + 2 * p);
        }
    }





}

//  Return all Pythagorean triplets with hypotenuse n
pub fn pythag_triplets(c: u32) -> BTreeSet<[u32; 3]> {

    let mut triplets = BTreeSet::<[u32; 3]>::new();
    let mut b = c - 1;
    let mut a = 1u32;

    while a < b {
        let a_sq = c.pow(2) - b.pow(2);
        let a_ = (a_sq as f64).sqrt();
        a = a_.floor() as u32;
        if a.pow(2) + b.pow(2) == c.pow(2) {
            triplets.insert([a, b, c]);
        }
        b -= 1;
    }
    triplets
} 


//  Prime factor finder ported from Jim Randell's Enigma Python library
//  Wheel factorisation using wheels of circumference 30
pub fn prime_factor(m: usize) -> Vec<[usize; 2]> {
    let mut factors: Vec<[usize; 2]> = Vec::new();
    if m > 1 {
        let mut n = m;
        let mut i = 2;
        let ds = [1, 2, 2, 4, 2, 4, 2, 4, 6, 2, 6];
        let js = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 3];
        let mut j = 0;
        while i*i <= n {
            let mut e = 0;
            loop {
                let (d, r) = (&n/&i, &n%&i);

                if r > 0 {
                    break;
                }
            e += 1;
            n = d;
            }
            if e > 0 {
                factors.push([i, e]);
            }
            i += ds[j];
            j = js[j];
        }
        if n > 1 {
            factors.push([n, 1]);
        }
    }
    factors
}
