fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "GitTerm");
        res.set(
            "FileDescription",
            "Git status viewer with integrated terminal",
        );
        res.set("CompanyName", "GitTerm");
        res.compile().unwrap();
    }

    #[cfg(target_os = "macos")]
    {
        // Embed Info.plist so macOS grants microphone permission to the binary
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let plist_path = std::path::Path::new(&manifest_dir).join("Info.plist");
        if plist_path.exists() {
            println!(
                "cargo:rustc-link-arg=-Wl,-sectcreate,__TEXT,__info_plist,{}",
                plist_path.display()
            );
        }
    }
}
