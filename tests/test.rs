#[macro_use] extern crate failure;
extern crate tempdir;

use failure::Error;
use tempdir::TempDir;

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

fn it_works_() -> Result<(), Error>{
    // copy private.xml and karabiner.before.json to a temporary folder
    let dir = TempDir::new("karaconv")?;
    fs::copy(Path::new(file!()).with_file_name("private.xml"),
             dir.path().join("private.xml"))?;
    fs::copy(Path::new(file!()).with_file_name("karabiner.before.json"),
             dir.path().join("karabiner.json"))?;
    
    // run conversion
    assert!(Command::new(Path::new(&env::var("CARGO_MANIFEST_DIR")?)
                              .join("target")
                              .join(&env::var("PROFILE")?)
                              .join("karaconv"))
                    .arg("-i").arg(dir.path().join("private.xml"))
                    .arg("-o").arg(dir.path().join("karabiner.json"))
                    .status()?
                    .success());
    
    // compare karabiner.json to karabiner.after.json and print diff
    fn run_jq(from: &Path, to: &Path) -> Result<PathBuf, Error> {
        let out = String::from_utf8(
            Command::new("jq")
                    // this incantation comes from https://stackoverflow.com/a/31933234/1114328
                    // it normalizes the JSON file by sorting, so we can do an order-less diff later
                    .arg("--argfile").arg("a").arg(from)
                    .arg("-n").arg("def post_recurse(f): def r: (f | select(. != null) | r), .; r; def post_recurse: post_recurse(.[]?); ($a | (post_recurse | arrays) |= sort) as $a | $a")
                    .output()?.stdout)?;
        let outpath = to.join(from.file_name().unwrap());
        File::create(&outpath)?.write_all(out.as_bytes())?;
        Ok(outpath)
    }
    let truth = run_jq(&Path::new(file!()).with_file_name("karabiner.after.json"), dir.path())?;
    let beauty = run_jq(&dir.path().join("karabiner.json"), dir.path())?;
    if !Command::new("diff")
                .arg("-u")
                .arg(&truth)
                .arg(&beauty)
                .status()?.success() {
        bail!("generated JSON was incorrect");
    }
    
    Ok(())
}

#[test]
fn it_works() {
    it_works_().unwrap();
}

