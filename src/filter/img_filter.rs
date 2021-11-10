use opencv::core::Mat;
use std::time::Instant;

use opencv::{
	core,
    core::Vector,
	Result,
    imgcodecs,    
};

pub fn run_filter(file: &str, threshold: u8) -> Result<()>{
    let img = imgcodecs::imread(file, -1).unwrap();
    filter(&img, threshold)?;
    Ok(())
}

pub fn filter(img: &Mat, threshold: u8) -> Result<()>{
    let _rect = core::Rect::new(100, 100, 500, 500);
    let _scalar = core::Scalar::all(1.0);

    let mut buffer = Vector::new();

    let mut now = Instant::now();
    imgcodecs::imencode(".png", &img, &mut buffer, &Vector::new())?;
    let read_ms = now.elapsed().as_millis();

    now = Instant::now();
    threshold_filter(&buffer, threshold);
    let filter_ms = now.elapsed().as_millis();
    
    println!("Image buffer size: {:?} bytes", buffer.len());
    println!("Image read time: {}(millis)", read_ms);  
    println!("Image filter time: {}(millis)", filter_ms);  
    Ok(())
}


fn threshold_filter(bytes: &Vector<u8>, threshold: u8){
  
    // filter the image u8 pixels and retrieve only the index and values 
    // where the values are greater than threshold
    let iv: Vec<_> = bytes.into_iter()
                    .enumerate()
                    .filter(|&(_,val)| val > threshold)
                    .map(|(index,val)| (index, val))
                    .collect();
    println!("Filtered image bytes: {}", iv.len());
}