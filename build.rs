// Example custom build script.
fn main() {
    css_build();
}

fn css_build() {
    //    println!("cargo:rerun-if-changed=css/input.css");
    std::process::Command::new("sh")
        .args(&[
            "npx",
            "lightningcss",
            "--minify",
            " --bundle --targets '>= 0.25%' input.css",
        ])
        // .current_dir("css")
        .status()
        .expect("failed to execute npm run build");
}
