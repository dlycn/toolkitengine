use image::{GenericImageView, ImageReader, RgbaImage, imageops::resize};
use tokio;

#[tokio::main]
async fn main() {
    let mut path = "common/texturedisplay/".to_string();
    let input = "assets/imgs/output.png".to_string();
    let flex:[u8;2]=[8,4];
    let param:[u8;2]=[8,16];
    let image= ImageReader::open(input).unwrap()
    .decode().unwrap();
    let (width, height) = image.dimensions();
    let b: u32 = 2u32.pow(param[0] as u32);
    let w: u32 = b*flex[0] as u32;
    let d: f64 = (height as f64 / width as f64) * (flex[0] as f64 /flex[1] as f64) * param[1] as f64;
    let p: f64 = (d as u32) as f64 / param[1] as f64;
    let h: u32 = (p * (b * flex[1] as u32)as f64) as u32;
    let s = (w,h,p);
    println!("{:#?}", s);
    let mut resize = resize(&image, w, h, image::imageops::FilterType::CatmullRom);
    path.push_str("image.png");
    resize.save(path).unwrap();
}
