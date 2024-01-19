mod args;

use std::{io::BufReader, fs::File};

use args::Args;
use image::{DynamicImage, ImageFormat, io::Reader, GenericImageView, imageops::FilterType::Triangle};


struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = 3_655_744;
        let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
        FloatingImage {
            width,
            height,
            data: buffer, 
            name
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }

        self.data = data;
        Ok(())
    }
}



#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    println!("{:?}", args);

    let (image_one, image_one_format) = find_image_from_path(args.image_one);
    let (image_two, image_two_format) = find_image_from_path(args.image_two);

    if image_one_format != image_two_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_one, image_two) = standardise_size(image_one, image_two);

    let mut output = FloatingImage::new(image_one.width(), image_one.height(), args.output);

    let combined_data = combine_images(image_one, image_two);

    output.set_data(combined_data)?;

    image::save_buffer_with_format(
        output.name, 
        &output.data, 
        output.width, 
        output.height, 
        image::ColorType::Rgb8,
        image_one_format,
    ).unwrap();

    Ok(())
    
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn standardise_size(image_one: DynamicImage, image_two: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (widith, height) = get_smallest_dimensions(image_one.dimensions(), image_two.dimensions());
    println!("widith: {}, heigth: {}", widith, height);

    if image_two.dimensions() == (widith, height) {
        (image_one.resize_exact(widith, height, Triangle), image_two)
    } else {
        (image_one, image_two.resize_exact(widith, height, Triangle))
    }
}

fn combine_images(image_one: DynamicImage, image_two: DynamicImage) -> Vec<u8> {
    let vec_one = image_one.to_rgb8().into_vec();
    let vec_two = image_two.to_rgb8().into_vec();

    alternate_pixels(vec_one, vec_two)
}

fn alternate_pixels(vec_one: Vec<u8>, vec_two: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_one.len()];

    let mut i = 0;
    while i < vec_one.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_one, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_two, i, i + 3));
        }

        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}
