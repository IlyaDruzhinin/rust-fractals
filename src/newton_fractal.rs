extern crate image;
extern crate num;
extern crate wasm_bindgen;

pub mod newtone_fractal {
    use complex;
    use complex::complex::Complex;
    use complex::complex::{abs, div, mul, scale, sub, sub_f64};
    use image;
    use logger;
    use rayon::iter::ParallelIterator;
    use rayon::prelude::*;
    use std::f64;
    use std::fs::File;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use wasm_bindgen::prelude::*;

    // #[wasm_bindgen]
    // pub fn choose_color(x: i32, y: i32, n: i32) -> [u8; 3] {
    //     [0, 0, 0]
    // }

    // F(x) = x^numRoots - 1
    // TODO: add pow
    pub fn zfunc(z: Complex) -> Complex {
        sub_f64(mul(z, mul(z, z)), 1.0)
    }

    // dF(x) = numRoots*x^(numRoots -1)
    // TODO: pow
    pub fn dfunc(z: Complex, pow_value: f64) -> Complex {
        scale(mul(z, z), pow_value as f64)
    }

    #[allow(dead_code)]
    pub fn draw(filename: &str, mx_input: i32, my_input: i32, iter: u32, z0: Complex, zn: Complex) {
        
        let tolerance = 0.1; // Work until the epsilon squared < this.

        let x0: i32 = 0;
        let xn: i32 = 0;
        let y0: i32 = 0;
        let yn: i32 = 0;

        let r1 = Complex { re: 1.0, im: 0.0 };
        let r2 = Complex {
            re: -0.5,
            im: 3.0_f64.sqrt() / 2.0,
        };
        let r3 = Complex {
            re: -0.5,
            im: -3.0_f64.sqrt() / 2.0,
        };

        let imgbuf = Arc::new(Mutex::new(image::RgbImage::new(
            mx_input as u32,
            my_input as u32,
        )));

        let mx = mx_input / 2;
        let my = my_input / 2;

        info!("start threads");

        (-my..my).into_par_iter().for_each(|y| {
            (-mx..mx).into_par_iter().for_each(|x| {
                let mut n = 0;

                let mut zxy = Complex {
                    re: x as f64 * 4.0 / (my_input - 2) as f64,
                    im: -(y as f64 * 4.0 / (mx_input + 2) as f64),
                };

                let imgbf = imgbuf.clone();

                while n < iter {
                    zxy = sub(zxy, div(zfunc(zxy), dfunc(zxy, 3 as f64)));
                    n = n + 1;
                }

                if abs(sub(zxy, r1)) < tolerance {
                    let mut ib = imgbf.lock().unwrap();
                    ib.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([255, 0, 0]));
                }

                if abs(sub(zxy, r2)) <= tolerance {
                    let mut ib = imgbf.lock().unwrap();
                    ib.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([0, 255, 0]));
                }

                if abs(sub(zxy, r3)) <= tolerance {
                    let mut ib = imgbf.lock().unwrap();
                    ib.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([0, 0, 255]));
                }
            });
        });

        info!("end threads");

        let imgbf = imgbuf.clone();
        let ib = imgbf.lock().unwrap();
        ib.save(filename).expect("error in creation PNG");
    }

    pub fn i_to_u(point: i32, canvas: i32) -> u32 {
        (point + canvas) as u32
    }
}
