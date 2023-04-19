/*! The MIT License (MIT)
 ** 
 ** Copyright (c) 2022 Muqiu Han
 ** 
 ** Permission is hereby granted, free of charge, to any person obtaining a copy
 ** of this software and associated documentation files (the "Software"), to deal
 ** in the Software without restriction, including without limitation the rights
 ** to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 ** copies of the Software, and to permit persons to whom the Software is
 ** furnished to do so, subject to the following conditions:
 ** 
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 ** 
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 ** FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 ** AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 ** LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 ** OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 ** SOFTWARE. */



use opencv::core::{Mat, Scalar};
use opencv::imgcodecs::{imread, imwrite};
use opencv::prelude::{MatTrait, MatTraitConst};
use std::path::PathBuf;
use std::time::Instant;
use structopt::{self, StructOpt};

const COLOR_RANGE: u8 = 20;

#[derive(Debug, StructOpt)]
#[structopt(name = "crp", about = "image color replace tool")]
pub struct Args {
    #[structopt(long)]
    pub input_image: PathBuf,

    #[structopt(long)]
    pub target_image: PathBuf,

    #[structopt(long)]
    pub origin_color: String,

    #[structopt(long)]
    pub target_color: String,
}

#[derive(Debug)]
struct Config {
    pub input_image: String,
    pub target_image: String,
    pub origin_color: (u8, u8, u8),
    pub target_color: (u8, u8, u8),
}

fn parse_args() -> Config {
    let args = Args::from_args();

    if !args.input_image.exists() || !args.input_image.is_file() {
        panic!("Please input image!")
    }

    let get_color_rgb = |color_arg: String| -> (u8, u8, u8) {
        let color_rgb: Vec<u8> = color_arg
            .split(' ')
            .map(|rgb| rgb.parse::<u8>().unwrap())
            .collect();
        (color_rgb[0], color_rgb[1], color_rgb[2])
    };

    Config {
        input_image: String::from(args.input_image.to_str().unwrap()),
        target_image: String::from(args.target_image.to_str().unwrap()),
        origin_color: get_color_rgb(args.origin_color),
        target_color: get_color_rgb(args.target_color),
    }
}

fn core(input_image: &Mat, target_image: &mut Mat, config: &Config) -> Mat {
    for y in 0..input_image.rows() {
        for x in 0..input_image.cols() {
            let pixel = input_image.at_2d::<opencv::core::Vec3b>(y, x).unwrap();
            if pixel[2].abs_diff(config.origin_color.0) < COLOR_RANGE
                && pixel[1].abs_diff(config.origin_color.1) < COLOR_RANGE
                && pixel[0].abs_diff(config.origin_color.2) < COLOR_RANGE
            {
                target_image
                    .at_2d_mut::<opencv::core::Vec3b>(y, x)
                    .unwrap()
                    .copy_from_slice(&[
                        config.target_color.2,
                        config.target_color.1,
                        config.target_color.0,
                    ]);
            } else {
                target_image
                    .at_2d_mut::<opencv::core::Vec3b>(y, x)
                    .unwrap()
                    .copy_from_slice(&pixel.as_slice());
            }
        }
    }

    target_image.clone()
}

fn main() {
    let config = parse_args();
    let input_image = imread(config.input_image.as_str(), opencv::imgcodecs::IMREAD_COLOR).unwrap();
    let mut target_image = Mat::new_rows_cols_with_default(
        input_image.rows(),
        input_image.cols(),
        input_image.typ(),
        Scalar::new(0.0, 0.0, 0.0, 0.0),
    )
    .unwrap();
    let start_time = Instant::now();
    core(&input_image, &mut target_image, &config);

    println!("{}ms", (Instant::now() - start_time).subsec_millis());

    imwrite(
        &config.target_image,
        &target_image,
        &opencv::core::Vector::new(),
    )
    .unwrap();
}
