fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "GitTerm");
        res.set("FileDescription", "Git status viewer with integrated terminal");
        res.set("CompanyName", "GitTerm");
        res.compile().unwrap();
    }
}
