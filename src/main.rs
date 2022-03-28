mod args;
use args::Args;
use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() {
    let a = Args::new();
    let (image1, image1_format) = find_image_from_path(a.image1);
    let (image2, image2_format) = find_image_from_path(a.image2);

    println!("{}", (1, 2) == (1, 2))
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader = Reader::open(path).unwrap();
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();
    (image, image_format)
}

fn get_smallest_dim(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    let pix1 = dim1.0 * dim1.1;
    let pix2 = dim2.0 * dim2.1;

    return if pix1 < pix2 { dim1 } else { dim2 };
}

fn standardize_size(img1: DynamicImage, img2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dim(img1.dimensions(), img2.dimensions());

    if img1.dimensions() != (width, height) {
        img1.resize_exact(width, height, Triangle);
    }
    if img2.dimensions() != (width, height) {
        img2.resize_exact(width, height, Triangle);
    }
    (img1, img2)
}
