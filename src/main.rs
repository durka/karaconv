#[macro_use] extern crate structopt;
#[macro_use] extern crate failure;
extern crate serde;
extern crate chrono;
extern crate regex;
extern crate serde_json;
extern crate serde_xml_rs as serde_xml;

extern crate karaconv;

use failure::Error;
use regex::Regex;
use structopt::StructOpt;

use karaconv::{xml, json};
use json::KeyOrButtonConv;

use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Input file (Karabiner XML)
    #[structopt(short="i", long="input", parse(from_os_str))]
    infile: PathBuf,

    /// Output file (Karabiner-Elements JSON)
    #[structopt(short="o", long="output", parse(from_os_str))]
    outfile: PathBuf,

    /// Dry run
    #[structopt(short="n")]
    dry_run: bool,
}

fn try_main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let inxml: xml::Karabiner = serde_xml::deserialize(File::open(&opt.infile)?)?;
    let mut outjson: json::Karabiner = serde_json::from_reader(File::open(&opt.outfile)?)?;

    for item in inxml.items {
        print!("Converting {}... ", item.name);
        let mut rule = json::Rule {
            description: item.name,
            manipulators: vec![]
        };
        
        for key in item.keys {
            let type_regex = Regex::new(r"--(?P<type>[a-zA-Z]+)-- (?P<contents>.*)")?;
            if let Some(caps) = type_regex.captures(&key) {
                let mut keys = karaconv::collect_keys(&caps["contents"])?;

                match &caps["type"] {
                    "KeyToKey" => {
                        let (fromkey, frommod) = keys.remove(0);
                        rule.manipulators.push(json::Manipulator {
                            type_: "basic".into(),
                            from: json::From::conv(fromkey, frommod),
                            to: keys.into_iter().map(|(k, m)| json::To::conv(k, m)).collect(),
                            to_if_alone: vec![],
                        });
                    }

                    "KeyOverlaidModifier" => {
                        let (fromkey, frommod) = keys.remove(0);
                        let (tokey, tomod) = keys.remove(0);
                        rule.manipulators.push(json::Manipulator {
                            type_: "basic".into(),
                            from: json::From::conv(fromkey, frommod),
                            to: vec![json::To::conv(tokey, tomod)],
                            to_if_alone: keys.into_iter().map(|(k, m)| json::To::conv(k, m)).collect(),
                        });
                    }

                    otherwise => bail!("Unsupported autogen type: {}", otherwise)
                }
            } else {
                bail!("Unparseable autogen: {}", key);
            }
        }

        let mut done = false;
        if let Some(existing_rule) = outjson.profiles[0].complex_modifications.rules.iter_mut().find(|r| r.description == rule.description) {
            println!("replacing existing rule");
            existing_rule.manipulators = rule.manipulators.clone();
            done = true;
        }
        if !done {
            println!("adding new rule");
            outjson.profiles[0].complex_modifications.rules.push(rule);
        }
    }

    if opt.dry_run {
        println!("{}", serde_json::to_string_pretty(&outjson)?);
    } else {
        let ext = format!("{}.bak.{}", opt.outfile.extension().unwrap().to_str().unwrap(), chrono::Local::now().format("%Y%m%d"));
        fs::copy(&opt.outfile, opt.outfile.with_extension(ext))?;
        serde_json::to_writer_pretty(File::create(&opt.outfile)?, &outjson)?;
    }

    Ok(())
}

fn main() {
    try_main().unwrap();
}

