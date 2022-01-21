fn main() {
    //https://sixtyfps.io/releases/0.1.5/docs/rust/sixtyfps/docs/widgets/index.html
    sixtyfps_build::compile("ui/main.60").unwrap();
    embed_resource::compile("ui/icon.rc");
}