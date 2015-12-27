pub fn get_extension(uri: &String) -> Option<&str> {
    uri.split(".").last()
}
