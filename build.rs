fn main() {
    if std::env::var("TARGET").unwrap_or_default().contains("windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "Aksharamale");
        res.set("FileDescription", "Aksharamale - Open-source Kannada Unicode Typing Engine");
        res.set("LegalCopyright", "Copyright (c) 2026 Aksharamale Contributors");
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resource: {:?}", e);
        }
    }
}
