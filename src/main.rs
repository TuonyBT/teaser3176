fn main() {

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
                        println!("RESULT!! p: {}, a: {}, b: {}, x: {}, y: {}", p, a, b, x, y);
                    }
                }
            }
        }
    }

}
