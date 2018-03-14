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
                let mut parts = caps["contents"].split(',').map(str::trim).peekable();
                match &caps["type"] {
                    "KeyToKey" => {
                        let fromkey = karaconv::conv_key(parts.next().unwrap())?;
                        let frommod = if !parts.peek().unwrap().starts_with("Key") { Some(karaconv::conv_mod(parts.next().unwrap())?) } else { None };

                        let mut manip = json::Manipulator {
                            type_: "basic".into(),
                            from: json::From::conv(fromkey, frommod),
                            to: vec![],
                            to_if_alone: None,
                        };

                        while parts.peek().is_some() {
                            let tokey = karaconv::conv_key(parts.next().unwrap())?;
                            let tomod = if parts.peek().is_some() {
                                if !parts.peek().unwrap().starts_with("Key") { Some(karaconv::conv_mod(parts.next().unwrap())?) } else { None }
                            } else {
                                None
                            };
                            manip.to.push(json::To::conv(tokey, tomod));
                        }

                        rule.manipulators.push(manip);
                    }

                    "KeyOverlaidModifier" => {
                        let fromkey = karaconv::conv_key(parts.next().unwrap())?;
                        let frommod = if !parts.peek().unwrap().starts_with("Key") { Some(karaconv::conv_mod(parts.next().unwrap())?) } else { None };

                        let tokey = karaconv::conv_key(parts.next().unwrap())?;
                        let tomod = if !parts.peek().unwrap().starts_with("Key") { Some(karaconv::conv_mod(parts.next().unwrap())?) } else { None };

                        let mut manip = json::Manipulator {
                            type_: "basic".into(),
                            from: json::From::conv(fromkey, frommod),
                            to: vec![json::To::conv(tokey, tomod)],
                            to_if_alone: None,
                        };

                        let evtkey = karaconv::conv_key(parts.next().unwrap())?;
                        let evtmod = if parts.peek().is_some() {
                            if !parts.peek().unwrap().starts_with("Key") { Some(karaconv::conv_mod(parts.next().unwrap())?) } else { None }
                        } else {
                            None
                        };
                        let mut to_if_alone = vec![json::To::conv(evtkey, evtmod)];

                        while parts.peek().is_some() {
                            let evtkey = karaconv::conv_key(parts.next().unwrap())?;
                            let evtmod = if parts.peek().is_some() {
                                if !parts.peek().unwrap().starts_with("Key") { Some(karaconv::conv_mod(parts.next().unwrap())?) } else { None }
                            } else {
                                None
                            };
                            to_if_alone.push(json::To::conv(evtkey, evtmod));
                        }
                        manip.to_if_alone = Some(to_if_alone);

                        rule.manipulators.push(manip);
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
