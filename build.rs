fn main() {
    // บอก Cargo ว่าถ้าไฟล์ UI เปลี่ยน ให้ Rebuild ใหม่นะ
    println!("cargo:rerun-if-changed=src/drafting.gresource.xml");
    println!("cargo:rerun-if-changed=src/window.ui");

    glib_build_tools::compile_resources(
        &["src"],
        "src/drafting.gresource.xml",
        "drafting.gresource",
    );
}