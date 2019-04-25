use lucet_idl::cache::*;
use lucet_idl::config::*;
use lucet_idl::data_description_helper::*;
use lucet_idl::errors::*;
use lucet_idl::generators::*;
use lucet_idl::parser::*;
use lucet_idl::pretty_writer::*;
use lucet_idl::validate::*;

use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Default, Clone, Debug)]
pub struct ExeConfig {
    pub input_path: PathBuf,
    pub output_path: Option<PathBuf>,
    pub config: Config,
}

impl ExeConfig {
    pub fn parse() -> Result<Self, IDLError> {
        let matches = App::new("lucet-idl")
            .version("1.0")
            .about("lucet_idl code generator")
            .arg(
                Arg::with_name("input_file")
                    .short("i")
                    .long("input")
                    .takes_value(true)
                    .required(true)
                    .help("Path to the input file"),
            )
            .arg(
                Arg::with_name("target")
                    .short("t")
                    .long("target")
                    .default_value("Generic")
                    .takes_value(true)
                    .required(false)
                    .help("Target, one of: x86, x86_64, x86_64_32, generic"),
            )
            .arg(
                Arg::with_name("backend")
                    .short("b")
                    .long("backend")
                    .default_value("c")
                    .takes_value(true)
                    .required(false)
                    .help("Backend, one of: c, rust"),
            )
            .arg(
                Arg::with_name("zero-native-pointers")
                    .short("z")
                    .long("zero-native-pointers")
                    .takes_value(false)
                    .required(false)
                    .help("Do not serialize native pointers"),
            )
            .get_matches();
        let input_path = PathBuf::from(
            matches
                .value_of("input_file")
                .ok_or(IDLError::UsageError("Input file required"))?,
        );
        let config = Config::parse(
            matches.value_of("target").unwrap(),
            matches.value_of("backend").unwrap(),
            matches.is_present("zero-native-pointers"),
        );
        Ok(ExeConfig {
            input_path,
            output_path: None,
            config,
        })
    }
}
fn doit() -> Result<(), IDLError> {
    let exe_config = ExeConfig::parse()?;
    let mut source = String::new();
    File::open(&exe_config.input_path)?.read_to_string(&mut source)?;
    let mut parser = Parser::new(&source);
    let decls = parser.match_decls()?;
    let data_description = DataDescription::validate(&decls)?;
    let deps = data_description
        .ordered_dependencies()
        .map_err(|_| IDLError::InternalError("Unable to resolve dependencies"))?;
    let data_description_helper = DataDescriptionHelper { data_description };
    let mut cache = Cache::default();
    let mut generator = Generators::c(&exe_config.config);
    let mut pretty_writer = PrettyWriter::new(io::stdout());
    generator.gen_prelude(&mut pretty_writer)?;
    for id in deps {
        data_description_helper.gen_for_id(&mut generator, &mut cache, &mut pretty_writer, id)?;
    }
    Ok(())
}

fn main() {
    doit().unwrap();
}
