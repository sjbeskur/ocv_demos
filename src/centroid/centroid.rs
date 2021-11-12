use std::time::Instant;
use opencv::core::Size;
use opencv::core::Moments;
use opencv::core::Mat;
use opencv::core::Scalar;
use opencv::{
    core::Point,
	imgproc,
	Result,
    imgcodecs,
    highgui,
//    imgproc::sobel,
//    imgproc::canny,
//	objdetect,
//	prelude::*,
//	types,
//	videoio,

};

pub fn show_centriods(file: &str) -> Result<()> {
    let mut img = imgcodecs::imread(file, -1).unwrap();

    //let mut gray_image = img.clone();
    //let rslt = imgproc::cvt_color(&img, &mut gray_image, imgproc::COLOR_RGB2GRAY, 0 )?;
    let now = Instant::now();

    let rslt = locate_centriods(&mut img);
    println!("Total centroid: {}(millis)", now.elapsed().as_millis());  

//    highgui::imshow("single centroid", &img)?;
    highgui::wait_key(0)?;
    highgui::imshow("mulit centroid", &rslt.unwrap())?;
    highgui::wait_key(0)?;

    Ok(())

/*
    let rect = core::Rect::new(100, 100, 500, 500);
    let scalar = core::Scalar::all(1.0);
    imgproc::rectangle(&mut gray_image, rect, scalar, 2, 1, 0).unwrap();
    highgui::imshow("hello", &gray_image)?;
*/
}





pub fn single_centriod(gray_image: &mut opencv::prelude::Mat) -> Result<()>{

    //https://learnopencv.com/find-center-of-blob-centroid-using-opencv-cpp-python/
    let mut thresh_img = gray_image.clone();

    imgproc::threshold(gray_image, &mut thresh_img,100.0,255.0, imgproc::THRESH_BINARY)?;

    //convert gs to binary image    
    let moments = imgproc::moments(&thresh_img, true)?;
    println!("moment: {:?}", moments);
    let m10 = moments.m10 as i32;
    let m01 = moments.m01 as i32;
    let m00 = moments.m00 as i32;
    let center_point = Point::new(m10/ m00, m01 / m00);

    println!("coords of centroid: {:?}", center_point);
    
    let radius = 40;
    let color = Scalar::new(64.0, 64.0, 0.0, 0.0);
    let thickness = 3;
    let line_type = 8;
    let shift = 0;
    imgproc::circle(gray_image, center_point, radius, color, thickness, line_type, shift)?;

    println!("point from moments: {:?}",  center_point);

    Ok(())

}


pub fn locate_centriods(gray_image: &opencv::prelude::Mat) -> Result<Mat, Box<dyn std::error::Error>>{
    let mut contours = opencv::types::VectorOfMat::new(); 
    let mut canny_output: Mat = gray_image.clone(); 
    let mut blured = gray_image.clone();
    let ksize = Size::new(2,2);
    let anchor = Point::new(-1,-1);
    imgproc::blur(&gray_image, &mut blured, ksize, anchor, opencv::core::BORDER_DEFAULT )?;
    
//    println!("[canny] Running canny Edge Detection.");
    imgproc::canny(&blured, &mut canny_output , 50.0, 150.0, 3, false)?;
    
//    println!("[contours] Finding contours...");
    let simple = imgproc::CHAIN_APPROX_SIMPLE;
    //let none = imgproc::CHAIN_APPROX_NONE;
    //let tc89_kcos = imgproc::CHAIN_APPROX_TC89_KCOS;
    //let l_one = imgproc::CHAIN_APPROX_TC89_L1;    
    imgproc::find_contours(&canny_output, &mut contours, imgproc::RETR_TREE, simple , Point::new(0,0))?;
//    println!("Contours found detected: {}.", contours.to_vec().len());

    // [moments] Get moments;
    let mut mu: Vec<Moments> =  Vec::new();     
    for c in contours.iter(){
        mu.push( imgproc::moments(&c, false)? );
    }

    // Get the centroid of figures.
    let mut mc : Vec<Point>= Vec::new(); 
    for (i, _) in contours.iter().enumerate() {
        let m10 = mu[i].m10 as i32;
        let m00 = mu[i].m00 as i32;
        let m01 = mu[i].m01 as i32;
        mc.push( Point::new( m10/m00, m01/m00) );
    }

    let mut drawing = gray_image.clone(); 
    let color = Scalar::new(64.0, 64.0, 0.0, 0.0);
    let thickness: i32 = 4;    
    let maxresult = 0;
    let zero_offset = Point::new(0, 0);
    for (i, _) in contours.iter().enumerate() {
        let contour_idx = i as i32;
        let  _ = imgproc::draw_contours(&mut drawing, &contours, contour_idx , color, thickness, imgproc::LINE_AA, &opencv::core::no_array(), maxresult, zero_offset)?;
        let _ = imgproc::circle(&mut drawing, mc[i], 40, color, 3, 8, 0 )?;
    }

    Ok(drawing.clone())
}
