fn main() {
    let screen_width = 62;
    let screen_height = 31;
    let exponent = 2.0;
    let max_iterations = 10;
    render_looped(screen_width, screen_height, exponent, max_iterations);
    println!("\n");
    render_recursive(screen_width, screen_height, exponent, max_iterations)
}
#[derive(Copy, Clone)]
struct ComplexNumber {
    real: f64,
    imaginary: f64,
}

impl ComplexNumber {
    fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }
    fn pow(self, exponent: f64) -> Self {
        let angle = self.imaginary.atan2(self.real);
        let len = self.real.hypot(self.imaginary);

        let new_angle = angle * exponent;
        let new_len = len.powf(exponent);

        Self {
            real: new_angle.cos() * new_len,
            imaginary: new_angle.sin() * new_len,
        }
    }
}

impl std::ops::Add for ComplexNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

fn mandelbrot_recursive(
    z: ComplexNumber,
    exponent: f64,
    c: ComplexNumber,
    iterations: u32,
    max_iterations: u32,
) -> u32 {
    if iterations >= max_iterations {
        return max_iterations;
    }
    let new_point = z.pow(exponent) + c;
    if new_point.real.hypot(new_point.imaginary) > 2.0 {
        return iterations;
    }

    mandelbrot_recursive(new_point, exponent, c, iterations + 1, max_iterations)
}

fn mandelbrot_looped(
    z: ComplexNumber,
    exponent: f64,
    c: ComplexNumber,
    max_iterations: u32,
) -> u32 {
    let mut current_point = z;

    for iterations in 1..=max_iterations {
        if current_point.real.hypot(current_point.imaginary) > 2.0 {
            return iterations;
        }
        current_point = current_point.pow(exponent) + c;
    }

    max_iterations
}

fn render_recursive(screen_width: u32, screen_height: u32, exponent: f64, max_iterations: u32) {
    let mut output = String::new();
    for y in 0..screen_height {
        for x in 0..screen_width {
            let centered_and_normalized = ComplexNumber::new(
                ((x as f64) - ((screen_width as f64 - 1.0) / 2.0))
                    / ((screen_width as f64 - 1.0) / 2.0),
                ((y as f64) - ((screen_height as f64 - 1.0) / 2.0))
                    / ((screen_height as f64 - 1.0) / 2.0),
            );
            let coordinate = ComplexNumber::new(
                centered_and_normalized.real * 1.5,
                centered_and_normalized.imaginary * 1.5,
            ) + ComplexNumber::new(-0.5, 0.0);
            let iters = mandelbrot_recursive(coordinate, exponent, coordinate, 1, max_iterations);
            if iters == max_iterations {
                output.push('#')
            } else {
                output.push('.')
            }
        }
        output.push('\n')
    }
    println!("{output}")
}

fn render_looped(screen_width: u32, screen_height: u32, exponent: f64, max_iterations: u32) {
    let mut output = String::new();
    for y in 0..screen_height {
        for x in 0..screen_width {
            let centered_and_normalized = ComplexNumber::new(
                ((x as f64) - ((screen_width as f64 - 1.0) / 2.0))
                    / ((screen_width as f64 - 1.0) / 2.0),
                ((y as f64) - ((screen_height as f64 - 1.0) / 2.0))
                    / ((screen_height as f64 - 1.0) / 2.0),
            );
            let coordinate = ComplexNumber::new(
                centered_and_normalized.real * 1.5,
                centered_and_normalized.imaginary * 1.5,
            ) + ComplexNumber::new(-0.5, 0.0);
            let iters = mandelbrot_looped(coordinate, exponent, coordinate, max_iterations);
            if iters == max_iterations {
                output.push('#')
            } else {
                output.push('.')
            }
        }
        output.push('\n')
    }
    println!("{output}")
}
