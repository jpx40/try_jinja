use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
// Example custom build script.
fn main() {
    build();
}

fn clear_css() {
    if Path::new("static/style/bundle.css").exists() {
        fs::remove_file("static/style/bundle.css").expect("failed to remove bundle.css");
    }
}

fn css_build() {
    //    println!("cargo:rerun-if-changed=css/input.css");
    // if Path::new("static/style/bundle.css").exists() {
    //     clear_css();
    // }

    let home_path = Path::new("/home/jonas/")
        .canonicalize()
        .unwrap()
        .to_owned()
        .to_str()
        .unwrap()
        .to_string();
    let code_path = "code/rust/web/try_jinja";

    let path: String = home_path + code_path;

    let input = path.clone() + "style/*";
    let output = path.clone() + "static/style/bundle.css";
    println!("building css");
    let cmd = "npx lightningcss --minify --bundle --targets '>= 0.25%' ./style/* --output-file ./static/style/bundle.css";
    std::process::Command::new("sh")
        .args([
            "npx",
            "lightningcss",
            "--minify",
            "--bundle",
            "--targets",
            "'>= 0.25%''",
            &input,
            "--output-file",
            &output,
        ])
        .status()
        .expect("failed to execute npm run build");
}

fn build() {
    //clear_css();
    css_build();
    js_build();
}

fn build_watch() {}

fn js_build() {
    let cmd = "esbuild js/* --bundle --minify  --outdir=./static/lib";

    let home_path = Path::new("/home/jonas/")
        .canonicalize()
        .unwrap()
        .to_owned()
        .to_str()
        .unwrap()
        .to_string();
    let code_path = "code/rust/web/try_jinja";

    let path: String = home_path + code_path;
    println!("{:?}", path);
    println!("building js");

    let input: String = path.clone() + "js/*";
    let output: String = "--outdir=".to_string() + &path + "static/lib";
    Command::new("sh")
        .args(["esbuild", &input, "--bundle", "--minify", &output])
        .status()
        .expect("esbuild failed");
}
