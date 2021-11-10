use app::{App, Cmd, Opt, }; //Args, OptTypo, OptValue, OptValueParse};

#[derive(Debug, Default)]
pub struct Config{
    filter: FilterImg,
    centroid: CentriodCmd,
}


impl Config {


    pub fn parse(){
        //let mut thresh :i32 = 0;
        let mut config = Config::default();

        let helper = {
            App::new("rust_cv")
                .version("1.0.0")
                .cmd(
                    Cmd::new("filter")
                    .short("f")
                    .desc("filter input image")
                    .opt(
                        Opt::new("threshold", &mut config.filter.threshold )
                        .short('t')
                        .long("threshold")
                        .optional()
                        .help("seth threshold value")
                    )
                    .opt(
                        Opt::new("fileName", &mut config.filter.file_name )
                        .short('f')
                        .long("fileName")
                        .help("image file to filter")
                    )                    
                )
                .cmd(
                    Cmd::new("centroid")
                    .short("c")
                    .desc("show centroid of input image")
                    .opt(
                        Opt::new("fileName", &mut config.centroid.file_name )
                        .short('f')
                        .long("fileName")
                        .help("image file to target")
                    )                    
                )

                .parse_args()
        };

        if *helper.args_len() == 0 {
            helper.help_exit(0);
        }

        config
            .check_and_call(helper.current_cmd_str())
            .map_err(|e| helper.help_cmd_err_exit(helper.current_cmd_ref(), e, 1))
            .unwrap(); // map_err alrendy exit if it is err, so unwrap is safe.        
    }


    fn check_and_call(self, cmd: Option<&str>) -> Result<(), String> {
        println!("Match Cmd: {:?}", cmd);
        match cmd {
            Some("filter") => {
                self.filter.check()?;
            },
            Some("centroid") => {
                self.centroid.check()?;
            }

            _ => unreachable!(),
        }
        Ok(())
    }    
 
}



#[derive(Debug)]
pub struct FilterImg {
    pub threshold: u8, 
    pub file_name: String,

}

impl FilterImg{
    fn check(&self) -> Result<(), String> {
        println!("Running Filter: threshold: {}, file: {}", self.threshold, self.file_name);
        crate::filter::run_filter(&self.file_name,self.threshold).unwrap();
        Ok(())
    }
}

impl Default for FilterImg{
    fn default() -> Self{        
        Self{ threshold: 128, file_name: "".to_owned() }
    }
} 

#[derive(Debug,Default)]
pub struct CentriodCmd{
    pub file_name: String,
}

impl CentriodCmd{
    fn check(&self) -> Result<(), String> {
        println!("Running Filter: threshold: file: {}",self.file_name);
        crate::centroid::show_centriods(&self.file_name).unwrap();
        Ok(())
    }
}
