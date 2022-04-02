use convert_case::{Case, Casing};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

const LIB_SLUG: &str = include_str!("build/lib_slug.rs");
const SLUG: &str = include_str!("build/slug.rs");

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrObject {
    String(String),
    Object(HashMap<String, String>),
}

fn main() {
    println!("cargo:rerun-if-changed=build/");
    println!("cargo:rerun-if-changed=package-lock.json");

    Command::new(if cfg!(target_os = "windows") {
        "npm.cmd"
    } else {
        "npm"
    })
    .args(["ci", "--audit=false"])
    .status()
    .expect("Failed to get NPM's status code")
    .success()
    .then(|| ())
    .expect("Failed to install NPM's dependencies");

    let collection: HashMap<String, Vec<Vec<StringOrObject>>> =
        serde_json::from_reader(
            File::open(PathBuf::from(format!(
                "{}/node_modules/lucide-static/icon-nodes.json",
                env::current_dir().unwrap().to_str().unwrap()
            )))
            .unwrap(),
        )
        .unwrap();

    let lib_content = [
        LIB_SLUG.to_owned(),
        collection
            .iter()
            .map(|(name, markup)| {
                let safe_snake_name =
                    format!("r#{}", name.to_case(Case::Snake));

                let mut tags = String::new();
                for markup in markup {
                    tags.push_str("<");
                    for markup in markup {
                        match markup {
                            StringOrObject::String(s) => tags.push_str(&s),
                            StringOrObject::Object(o) => {
                                for (k, v) in o {
                                    tags.push_str(&k);
                                    tags.push_str("=\"");
                                    tags.push_str(&v);
                                    tags.push_str("\" ");
                                }
                            }
                        }
                        tags.push(' ');
                    }
                    tags.push_str("/>");
                }

                let markup = SLUG
                    .replace("__ComponentName", &name.to_case(Case::Pascal))
                    .replace("__component_func", &safe_snake_name)
                    .replace("__component_markup", &tags);

                format!("\nmod {module_name} {{\n{markup}\n}}\npub use {module_name}::{pascal_name};", module_name = safe_snake_name, markup = markup, pascal_name = &name.to_case(Case::Pascal))
            })
            .collect::<String>(),
    ]
    .concat();

    fs::write("src/lib.rs", lib_content.as_bytes())
        .expect("can write to lib.rs file");
}
