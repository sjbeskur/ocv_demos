
#[derive(Debug)] 
pub struct Camera{
    focal_length: f32,
}


#[allow(dead_code)]
impl Camera {
     pub fn get_type() -> String{
         "default".to_string()
     }   
}

impl Default for Camera {
    fn default() -> Self{
        Self{
            focal_length: 1.1,
        }
    }
}


trait PinholeCamera{
    fn get_extrinsic(&self) -> f32;
}


impl PinholeCamera for Camera{

    fn get_extrinsic(&self) -> f32{
        return self.focal_length + 242.0
    }        
}

