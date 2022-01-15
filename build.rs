#[cfg(windows)] use winres::WindowsResource;

fn main() {
    #[cfg(windows)] {
        WindowsResource::new()
            .set_icon("icon/icon_win.ico")
            .compile()
            .unwrap();
    }
}