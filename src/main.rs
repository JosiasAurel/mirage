use clap::{command, Parser};
use rand::Rng;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    infile: Option<String>,

    /// Name of file where to store the result
    #[arg(short, long)]
    outfile: Option<String>,

    /// Specify blur amount in decimal
    #[arg(short, long)]
    blur: Option<f32>,

    /// Positive whole number ot brighten and negative whole number to dim the image
    #[arg(short, long)]
    shine: Option<i32>,

    /// Provide the width and height by which to crop image. Starts at x= 0 & y = 0
    #[arg(short, long)]
    crop: Option<Vec<u32>>,

    /// rotates image _left_, _right_ or reverse
    #[arg(short, long)]
    rotate: Option<String>,

    /// Invert the image colors
    #[arg(short, long)]
    invert: Option<bool>,

    /// Make the image black and white
    #[arg(short, long)]
    monochrome: Option<bool>,

    /// Generates a fractal
    #[arg(short, long)]
    fractal: Option<bool>,

    /// Generates a random diagram using polar coordinates
    #[arg(short, long)]
    generate: Option<bool>,
}

struct CropParams {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn main() {
    let commands = Args::parse();

    let mut infile = String::from("infile.png");
    let mut outfile = String::from("outfile.png");

    match commands.infile {
        Some(inf) => {
            infile = inf;
            // println!("Infile = {}", &inf);
        }
        None => {}
    }

    match commands.outfile {
        Some(out) => outfile = out.clone(),

        None => {}
    }

    match commands.blur {
        Some(amount) => {
            blur(infile.clone(), outfile.to_string(), amount);
        }
        None => {}
    }

    match commands.shine {
        Some(amount) => {
            brighten(infile.clone(), outfile.to_string(), amount);
        }
        None => {}
    }

    match commands.crop {
        Some(crop_params_) => {
            let mut crop_params: CropParams = CropParams {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };

            if crop_params_.len() == 2 {
                crop_params = CropParams {
                    x: 0,
                    y: 0,
                    width: crop_params_[0],
                    height: crop_params_[1],
                };
            } else if crop_params_.len() == 4 {
                crop_params = CropParams {
                    x: crop_params_[0],
                    y: crop_params_[1],
                    width: crop_params_[2],
                    height: crop_params_[3],
                }
            } else {
                println!("Arguments missing when tryping to crop image");
                std::process::exit(-1);
            }
            crop(infile.clone(), outfile.clone(), &crop_params);
        }
        None => {}
    }

    match commands.rotate {
        Some(dir) => {
            rotate(infile.clone(), outfile.clone(), &dir);
        }
        None => {}
    }

    match commands.invert {
        Some(_) => {
            invert(infile.clone(), outfile.clone());
        }
        None => {}
    }

    match commands.monochrome {
        Some(_) => {
            grayscale(infile.clone(), outfile.clone());
        }
        None => {}
    }

    match commands.fractal {
        Some(_) => {
            fractal(outfile.clone());
        }
        None => {}
    }

    match commands.generate {
        Some(_) => {
            let mut rng = rand::thread_rng();
            let color: [u8; 3] = [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()];
            generate(outfile.clone(), &color);
        }
        None => {}
    }
}

fn blur(infile: String, outfile: String, blur_amount: f32) {
    // open image
    let img = image::open(infile).expect("Failed to open INFILE.");

    // blur image
    let blurred_image = img.blur(blur_amount);

    // save image file
    blurred_image
        .save(outfile)
        .expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, inc_amt: i32) {
    let img = image::open(infile).expect("Failed to open INFILE");

    let brightened_image = img.brighten(inc_amt);

    brightened_image
        .save(outfile)
        .expect("Failed to write to OUTFILE");
}

fn crop(infile: String, outfile: String, crop_params: &CropParams) {
    let mut img = image::open(infile).expect("Failed to open INFILE");

    let cropped_img = img.crop(
        crop_params.x,
        crop_params.y,
        crop_params.width,
        crop_params.height,
    );

    cropped_img.save(outfile).expect("Failed to crop image");
}
fn rotate(infile: String, outfile: String, direction: &String) {
    let img = image::open(infile).expect("Failed to read INFILE");

    let rotated_img;
    match direction.as_str() {
        "right" => {
            rotated_img = img.rotate90();
        }
        "left" => {
            rotated_img = img.rotate270();
        }
        "reverse" => {
            rotated_img = img.rotate180();
        }
        _ => {
            println!("Unknown option");
            return;
        }
    }

    rotated_img.save(outfile).expect("Failed to save image")
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE");

    img.invert();

    img.save(outfile).expect("Failed to save image");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE");

    let grayscaled_img = img.grayscale();

    grayscaled_img.save(outfile).expect("Failed to save image");
}

fn generate(outfile: String, color: &[u8; 3]) {
    let mut imgbuf = image::ImageBuffer::new(800, 800);

    let mut rng = rand::thread_rng();
    let trig_fn = rng.gen_range(1..=3);
    let a1: f32 = rng.gen_range(1.0..5.0) as f32;
    let point_color = [
        rng.gen_range(1..=255),
        rng.gen_range(1..=255),
        rng.gen_range(1..=255),
    ];

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let x_: i32 = if x <= 400 {
            (x as i32) - 400
        } else {
            400 - (x as i32)
        };
        let y_: i32 = if x <= 400 {
            (y as i32) - 400
        } else {
            400 - (y as i32)
        };
        let angle: f32 = (y_ as f32 / x_ as f32).atan();
        let r = (x_ as f32) / angle.cos();

        // println!("a = {}, r = {}, fn = {}, x= {}, y = {}", a1, r, trig_fn, x, y);
        let mut light_pixel = false;
        match trig_fn {
            1 => {
                let r_c = 400.0 * (a1 * angle).cos();
                if (r_c - r).abs() < 5.0 {
                    light_pixel = true;
                }
            }
            2 => {
                let r_c = 400.0 * (a1 * angle).sin();
                if (r_c - r).abs() < 5.0 {
                    light_pixel = true;
                }
            }
            3 => {
                let r_c = 400.0 * (a1 * angle).tan();
                if (r_c - r).abs() < 5.0 {
                    light_pixel = true;
                }
            }
            _ => {}
        }

        if light_pixel {
            *pixel = image::Rgb(point_color);
        } else {
            *pixel = image::Rgb(*color);
        }
    }

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}
