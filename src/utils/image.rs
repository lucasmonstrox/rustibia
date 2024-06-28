use image::GenericImageView;
use ndarray::ArrayView3;
use std::error::Error;

pub fn load_image(path: &str) -> Result<ArrayView3<'static, u8>, Box<dyn Error>> {
    let img = image::open(path)?;
    let (width, height) = img.dimensions();
    let img_bytes = img.as_bytes().to_vec();
    let img_bytes_boxed: Box<[u8]> = img_bytes.into_boxed_slice();
    let img_bytes_static: &'static [u8] = Box::leak(img_bytes_boxed);
    let array_view =
        ArrayView3::from_shape((height as usize, width as usize, 4), img_bytes_static)?;
    Ok(array_view)
}
