fn main() {
    // Unix/Linux 시스템에서 X11 라이브러리 링크
    #[cfg(unix)]
    {
        println!("cargo:rustc-link-lib=X11");
    }
}