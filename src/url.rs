
pub struct Encoder;

impl Encoder {
    pub fn encode(url:&String){

    }
}

pub struct Decoder;

impl Decoder {
    pub fn decode(){

    }
}

fn is_alpha_numeric(cc: u8) -> bool {
    if 97 <= cc && cc <= 122 {
        return true;
    }
    if 65 <= cc && cc <= 90 {
        return true;
    }
    if 48 <= cc && cc <= 57 {
        return true;
    }
    false
}

fn hex_value_of(code: u8) {
    // 0-9
    if code >= 48 && code <= 57 {
        return code - 48;
    }
    // A-F
    if code >= 65 && code <= 70 {
        return code - 55;
    }
    // a-f
    if code >= 97 && code <= 102 {
        return code - 87;
    }

    return -1;
}


#[test]
fn test_is_alpha_numeric() {
    assert_eq!(true, is_alpha_numeric('a'))
}
