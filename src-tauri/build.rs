extern crate embed_resource;

fn main() {
  tauri_build::build();
  embed_resource::compile("calypso-manifest.rc");
}
