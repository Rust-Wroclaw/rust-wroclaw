// correct would be `#[cfg(target_os = "linux")]` or `#cfg(unix)]`
#[cfg(linux)]
pub fn linux_only_method() {
    unimplemented!()
}
