// Example custom build script.
fn main() {
    css_build();
}

fn css_build() {
    //    println!("cargo:rerun-if-changed=css/input.css");
    std::process::Command::new("sh")
        .args([
            "npx ightningcss --minify --bundle -targets '>= 0.25%' style/* -o static/style/bundle.css",
        ])
        .status()
        .expect("failed to execute npm run build");
}
