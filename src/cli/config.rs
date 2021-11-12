use std::process;
use clap::{Arg, ArgMatches, App, SubCommand};
//use app::{App, Cmd, Opt, }; //Args, OptTypo, OptValue, OptValueParse};

/*  not needed for now but might re-introduce later
#[derive(Debug, Default)]
pub struct Config{
    verbose: u8,
}
*/

pub fn parse(){
    //let mut config = Config::default();

    let matches = App::new("rust_cv")
        .version("1.0.0")
        .arg(
            Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Set level of logging verbosity")
        )
        .subcommand(SubCommand::with_name("filter")
            .about("Filter input image  by threshold u8 amount")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .required(true)
                .takes_value(true)
                .help("Image file to filter")
            )
            .arg(Arg::with_name("threshold")
                .short("t")
                .long("threshold")
                .required(false)
                .takes_value(true)
                .default_value("128")
                .help("Set threshold value to filter upon")
            )
        )   
        .subcommand(SubCommand::with_name("centroid")
            .about("Show centroids of input image")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .required(true)
                .takes_value(true)
                .help("Image file to target")
            )
        )   
        .get_matches();

    if let Err(e) = run(&matches){
        println!("Application error: {}", e);
        process::exit(1);
    }

    process_subcommands(&matches);

}

fn run(matches: &ArgMatches) -> Result<(), String> {    
    let min_log_level = match matches.occurrences_of("verbose") {
        0 => "WARN",
        1 => "INFO",
        2 => "DEBUG",
        3 | _ => "TRACE",
    };
    std::env::set_var("RUST_LOG", min_log_level);
    env_logger::init();
    Ok(())
}

pub fn process_subcommands(args: &ArgMatches){

    if let Some(config_matches) = args.subcommand_matches("filter"){
        //commands::show_config::exec(&config_matches);
        let t = config_matches.value_of("threshold").unwrap_or_default().parse().expect("t param missing");
        let file = config_matches.value_of("file").expect("file param missing");
        crate::filter::run_filter(&file, t).unwrap();
        std::process::exit(0);
    }   

    if let Some(config_matches) = args.subcommand_matches("centroid"){
        //commands::show_config::exec(&config_matches);
        let file = config_matches.value_of("file").expect("file param missing");
        crate::centroid::show_centriods(&file).unwrap();
        std::process::exit(0);
    }   


}
