pub struct MediaType;

impl MediaType {
    pub fn get_media_type(&self, path: &str) -> Result<Vec<u8>, ()> {
        match path.split(".").last() {
            Some(v) => Ok(self.parse(v).as_bytes().to_vec()),
            None => Err(()),
        }
    }
    pub fn parse(&self, path: &str) -> &str {
        match path {
            "css" => "text/css",
            "js" => "application/javascript",
            "ico" => "image/x-icon",
            _ => "",
        }
    }
}
