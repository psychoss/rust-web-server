use rustc_serialize::json::Json;
pub struct JsonParser;
impl JsonParser {
    fn read_json(buf: &Vec<u8>) -> Result<Json, ()> {
        let j = ok!(str::from_utf8(buf));
        println!("{:?}", j);
        let data = ok!(Json::from_str(j));
        Ok(data)
        // let v = match data.as_object() {
        //     Some(v) =>Ok(v),
        //     None => Err("".to_string()),
        // };

    }
}
