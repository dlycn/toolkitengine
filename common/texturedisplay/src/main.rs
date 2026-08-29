use image::{GenericImage, GenericImageView, ImageReader, RgbaImage, imageops::resize};
use tokio;

#[tokio::main]
async fn main() {
    let id = 320;
    let path = format!("test/sprites/DefineSprite_{id}/");
    let input = path.clone();
    let mut pnglist: Vec<u32> = Vec::new();
    let mut entries = tokio::fs::read_dir(&input).await.unwrap();
    while let Some(entry) = entries.next_entry().await.unwrap(){
        let file_type = entry.file_name().into_string().unwrap();
        if file_type.ends_with(".png"){
            if let Some(Ok(idnum))= file_type.split('.').next().map(|s| s.parse::<u32>()){pnglist.push(idnum)};
        }
    }
    if pnglist.len() == pnglist.iter().map(|i|{i.clone() as usize}).max().unwrap_or(0){
        let num = pnglist.len() as u32;
        let col = 8;
        let row = (num - 1) / col + 1;
        let (width, height) = ImageReader::open(format!("{}/{}.png", path, pnglist[0] as usize)).unwrap().decode().unwrap().dimensions();

        let mut outimg = RgbaImage::new(width * col, height * row);
        for i in 0..num{
            let (x, y) = (i % col, i / col);
            let img = ImageReader::open(format!("{}/{}.png", path, pnglist[i as usize] as usize)).unwrap().decode().unwrap();
            outimg.copy_from(&img, x * width, y * height).unwrap();
        }
        let nw = 2048;
        let nh = ((height * row * nw/width/col)>>2)*4;
        outimg = resize(&outimg,  nw, nh, image::imageops::FilterType::Lanczos3);
        outimg.save(format!("{}/out.png", path)).unwrap();
    }

}
