use clap::{command, Parser};
use rand::Rng;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    outfile: Option<String>,

    #[arg(short, long)]
    blur: Option<f32>,

    #[arg(short, long)]
    shine: Option<i32>,

    #[arg(short, long)]
    crop: Option<String>,

    #[arg(short, long)]
    rotate: Option<String>,

    #[arg(short, long)]
    invert: Option<bool>,

    // this command will make the image black and white
    #[arg(short, long)]
    monochrome: Option<bool>,

    #[arg(short, long)]
    fractal: Option<bool>,

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

    // forgive the repetition
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    let infile = args.remove(0);
    let mut outfile = String::from("outfile.png");

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
        Some(width_height) => {
            let wh: Vec<&str> = width_height.split("-").collect();
            let width: u32 = wh[0].parse().unwrap_or_else(|_| 0);
            let height: u32 = wh[1].parse().unwrap_or_else(|_| 0);
            let crop_params = CropParams {
                x: 0,
                y: 0,
                width,
                height,
            };
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
    /*
             let mut args: Vec<String> = std::env::args().skip(1).collect();

        if args.is_empty() {
            print_usage_and_exit();
        }

        if (args.len() >= 3) {
            let mut args_ = args.clone();
            for arg in args.iter_mut() {
                let subcommand = args_.remove(0);
                let infile = args_.remove(0);
                let outfile = args_.remove(0);
                match subcommand.as_str() {
                    "blur" => {
                        let blur_amount: f32;

                        blur_amount = args_
                            .remove(0)
                            .parse()
                            .expect("Failed to parse Blur Amount");

                        blur(infile, outfile, blur_amount);
                    }

                    "brighten" => {
                        if args_.len() < 2 || args_.len() > 3 {
                            print_usage_and_exit();
                        }
                        let infile = args_.remove(0);
                        let outfile = args_.remove(0);
                        let inc_amt: i32;
                        if args_.len() > 0 {
                            inc_amt = args_
                                .remove(0)
                                .parse()
                                .expect("Failed to parse brightness amount");
                        } else {
                            inc_amt = 2;
                        }
                        brighten(infile, outfile, inc_amt);
                    }

                    "crop" => {
                        if args_.len() != 6 {
                            print_usage_and_exit();
                        }
                        let infile = args_.remove(0);
                        let outfile = args_.remove(0);

                        let crop_params = CropParams {
                            x: args_.remove(0).parse().expect("Failed to parse x"),
                            y: args_.remove(0).parse().expect("Failed to parse x"),
                            width: args_.remove(0).parse().expect("Failed to parse x"),
                            height: args_.remove(0).parse().expect("Failed to parse x"),
                        };

                        crop(infile, outfile, &crop_params);
                    }
                    // Rotate -- see the rotate() function below
                    "rotate" => {
                        if args_.len() < 2 || args_.len() > 3 {
                            print_usage_and_exit();
                        }

                        let infile = args_.remove(0);
                        let outfile = args_.remove(0);
                        let direction: String;
                        if args_.len() > 0 {
                            direction = args_.remove(0);
                        } else {
                            direction = String::from("right");
                        }

                        rotate(infile, outfile, &direction);
                    }

                    "invert" => {
                        if args_.len() != 2 {
                            print_usage_and_exit();
                        }
                        let infile = args_.remove(0);
                        let outfile = args_.remove(0);

                        invert(infile, outfile);
                    }

                    "grayscale" => {
                        if args_.len() != 2 {
                            print_usage_and_exit();
                        }
                        let infile = args_.remove(0);
                        let outfile = args_.remove(0);

                        grayscale(infile, outfile);
                    }

                    _ => {
                        print_usage_and_exit();
                    }
                }
                // args = args_;
            }
        } else {
            let subcommand = args.remove(0);
            match subcommand.as_str() {
                "fractal" => {
                    if args.len() != 1 {
                        print_usage_and_exit();
                    }
                    let outfile = args.remove(0);
                    fractal(outfile);
                }

                "generate" => {
                    let color: [u8; 3];
                    let outfile = args.remove(0);
                    if args.len() == 0 {
                        let mut rng = rand::thread_rng();
                        color = [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()];

                        generate(outfile, &color);
                    }
                }
                _ => {}
            }
        }
    */

    // let subcommand = args.remove(0);
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
