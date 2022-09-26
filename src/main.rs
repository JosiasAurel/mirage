use rand::Rng;

struct CropParams {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);

    match subcommand.as_str() {
        "blur" => {
            if args.len() < 2 || args.len() > 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let blur_amount: f32;
            if args.len() > 0 {
                blur_amount = args.remove(0).parse().expect("Failed to parse Blur Amount");
            } else {
                blur_amount = 2.0;
            }
            blur(infile, outfile, blur_amount);
        }

        "brighten" => {
            if args.len() < 2 || args.len() > 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let inc_amt: i32;
            if args.len() > 0 {
                inc_amt = args
                    .remove(0)
                    .parse()
                    .expect("Failed to parse brightness amount");
            } else {
                inc_amt = 2;
            }
            brighten(infile, outfile, inc_amt);
        }

        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            let crop_params = CropParams {
                x: args.remove(0).parse().expect("Failed to parse x"),
                y: args.remove(0).parse().expect("Failed to parse x"),
                width: args.remove(0).parse().expect("Failed to parse x"),
                height: args.remove(0).parse().expect("Failed to parse x"),
            };

            crop(infile, outfile, &crop_params);
        }
        // Rotate -- see the rotate() function below
        "rotate" => {
            if args.len() < 2 || args.len() > 3 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);
            let direction: String;
            if args.len() > 0 {
                direction = args.remove(0);
            } else {
                direction = String::from("right");
            }

            rotate(infile, outfile, &direction);
        }

        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            invert(infile, outfile);
        }

        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            grayscale(infile, outfile);
        }
        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
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
            } else if args.len() == 3 {
                let red: u8 = args.remove(0).parse().expect("Failed to parse red value");
                let green: u8 = args.remove(0).parse().expect("Failed to parse green value");
                let blue: u8 = args.remove(0).parse().expect("Failed to parse blue value");
                color = [red, green, blue];

                generate(outfile, &color);
            }
        }

        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
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

        let x_: i32 = if x <= 400 { (x as i32)-400 } else { 400-(x as i32) };
        let y_: i32 = if x <= 400 { (y as i32)-400 } else { 400-(y as i32) };
        let angle: f32 = (y_ as f32/x_ as f32).atan();
        let r = (x_ as f32)/angle.cos();
        
        // println!("a = {}, r = {}, fn = {}, x= {}, y = {}", a1, r, trig_fn, x, y);
        let mut light_pixel = false;
        match trig_fn {
            1 => {
                let r_c = 400.0*(a1*angle).cos();
                if (r_c - r).abs() < 5.0 {
                    light_pixel = true;
                }
            }
            2 => {
                let r_c = 400.0*(a1*angle).sin();
                if (r_c - r).abs() < 5.0 {
                    light_pixel = true;
                }
            }
            3 => {
                let r_c = 400.0*(a1*angle).tan();
                if (r_c - r).abs() < 5.0 {
                    light_pixel = true;
                }
            }
            _ => {}
        }

        if  light_pixel {
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

