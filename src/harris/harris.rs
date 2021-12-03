use opencv::core::Point;
use opencv::core::Scalar;
use opencv::core::Mat;
use opencv::{
    prelude::*,
	  imgproc,
	  Result,
    imgcodecs,
    highgui,
//	videoio,
};

pub fn show_corners(file: &str) -> Result<()> {
  let img = imgcodecs::imread(file, -1).unwrap();
  let mut gray_image = img.clone();
  let _rslt = imgproc::cvt_color(&img, &mut gray_image, imgproc::COLOR_RGB2GRAY, 0 )?;
  harris_corners(&mut gray_image, 200.0 )?;
  Ok(())
}

#[allow(dead_code)]
pub fn harris_corners(gray_image: &mut opencv::prelude::Mat, thresh: f32) -> Result<()>{

  highgui::imshow(" gray corners", gray_image)?;

    let sz = gray_image.mat_size();
    let rows = gray_image.rows();
    let cols = gray_image.cols();
    let dims = sz.dims();
    let mut dst = Mat::zeros(rows,cols,opencv::core::CV_32FC1)?.to_mat()?; 
    
    // Mat::new_rows_cols(rows,cols, opencv::core::CV_32FC1)?;
    // Mat::new_nd(sz.dims(), , );
    //Mat::default();
    
    let mut dst_norm = Mat::zeros(rows,cols,opencv::core::CV_32FC1)?.to_mat()?;
    let mut dst_norm_scaled = Mat::zeros(rows,cols,opencv::core::CV_32FC1)?.to_mat()?;
    let mask = Mat::default(); // = Mat::zeros(rows,cols,opencv::core::CV_32FC1)?.to_mat()?;

    let block_size: i32 = 2;
    let aperture_size: i32 = 3;
    let k = 0.04; 

    imgproc::corner_harris(gray_image, &mut dst, block_size, aperture_size, k, opencv::core::BORDER_DEFAULT)?;
    opencv::core::normalize(&dst, &mut dst_norm, 0.0 , 255.0, opencv::core::NORM_MINMAX , opencv::core::CV_32FC1, &mask )?;
    opencv::core::convert_scale_abs(&dst_norm, &mut dst_norm_scaled, 1.0, 0.0)?;
    println!("3");

    let color = Scalar::new(64.0, 64.0, 0.0, 0.0);
    let thickness = 3;
    let line_type = 8;
    let shift = 0;

    for j in 0..dst_norm.rows() { 
      for  i in 0..dst_norm.cols() {
        let r = dst_norm.at_2d::<f32>(i,j);
        if  r.unwrap() > &thresh {
          imgproc::circle(&mut dst_norm_scaled, Point::new(i,j), 5, color, thickness, line_type, shift)?;
        }
      
      }
    }

    highgui::imshow("wip: harris corners", &dst_norm_scaled)?;
    highgui::wait_key(0)?;

    Ok(())
}

/*  

  Mat dst, dst_norm, dst_norm_scaled;
  dst = Mat::zeros( src.size(), CV_32FC1 );

  int blockSize = 2;
  int apertureSize = 3;
  double k = 0.04;

  cornerHarris( src_gray, dst, blockSize, apertureSize, k, BORDER_DEFAULT );

  normalize( dst, dst_norm, 0, 255, NORM_MINMAX, CV_32FC1, Mat() );
  convertScaleAbs( dst_norm, dst_norm_scaled );

  for( int j = 0; j < dst_norm.rows ; j++ )
     { for( int i = 0; i < dst_norm.cols; i++ )
          {
            if( (int) dst_norm.at<float>(j,i) > thresh )
              {
               circle( dst_norm_scaled, Point( i, j ), 5,  Scalar(0), 2, 8, 0 );
              }
          }
     }
  namedWindow( corners_window, WINDOW_AUTOSIZE );
  imshow( corners_window, dst_norm_scaled );
*/