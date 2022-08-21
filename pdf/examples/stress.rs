// Stressful PDF files from https://www.pdfa.org/stressful-pdf-corpus/
//
//   Run with `cargo run --example stress /path/to/pdfs`


use std::env;
use std::process;
use std::fs::metadata;
use env_logger::Env;
use walkdir::WalkDir;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use colored::*;



fn test_file(pathname: &str) {
    println!("Testing opening of {}", pathname);
    match pdf::file::File::<Vec<u8>>::open(pathname) {
        Ok(pdf) => {
            let page_count = pdf.pages().count();
            if page_count < 1 {
                eprintln!("  page count < 1");
            }
            for mpage in pdf.pages() {
                match mpage {
                    Ok(page) => {
                        if let Some(annots) = &page.annots {
                            if annots.len() < 1 {
                                eprintln!("  annotation vec length < 1");
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error accessing page: {:?}", e);
                    },
                }
            }
        },
        Err(e) => {
            eprintln!("{} opening {:?}: {:?}", "Error".red(), pathname, e);
        }
    }
}

fn main () {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pdf-file-or-directory>", &args[0]);
        process::exit(1);
    }

    if let Ok(m) = metadata(&args[1]) {
        if m.is_file() {
            test_file(&args[1]);
        }
        if m.is_dir() {
            for entry in WalkDir::new(&args[1]).into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension() == Some(OsStr::from_bytes(b"pdf")))
            {
                test_file(entry.path().to_str().unwrap());
            }
        }
    } else {
        eprintln!("no such path {}", &args[1]);
        process::exit(2);
    }
}
