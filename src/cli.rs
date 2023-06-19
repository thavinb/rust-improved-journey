// fqr [reads.fq.gz ...] [-h]
//
//
//

pub mod cli {
    use clap::Parser;
    use clap::builder::PossibleValuesParser;
    use std::path::Path;
    use log::info;

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    pub struct Cli {
        // Positonal arg(s)
        // Compressed (gz) fastq input(s)
        #[arg(value_parser = validate_file_path)]
        pub fastq: Vec<String>,

        // Toggle debugging info
        #[arg(short, long, value_parser = PossibleValuesParser::new(&["error", "warn", "info", "debug", "trace"]), required(false), default_value = "info")]
        pub debug: String,
        // #[arg(short,long,action = clap::ArgAction::Count)]

        // #[command(subcommand)]
        // command: Option<Commands>,
    }

    fn validate_file_path(path: &str) -> Result<String,String> {

       if !Path::new(&path).exists() {
            Err(format!("{path} not found"))
        } else if !&path.ends_with(".gz"){
            Err(format!("{path} is not gzipped"))
        } else {
            info!("{:?}: existed!",&path);
            Ok(path.to_string())
        }


    }
}

