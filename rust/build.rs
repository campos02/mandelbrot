use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new().files(["src/render_thread.rs"]).build();
}
