use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str;

use nom::is_hex_digit;
use nom::is_space;

use error::Result;

pub struct RegisterWrite {
    register:	u64,
    timestamp:	String,
    value:	u64,
}

impl fmt::Debug for RegisterWrite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "timestamp: {:?} reg {:08x?} value {:08x?}",
               self.timestamp, self.register, self.value)
    }
}

named!(timestamp_parser,
       delimited!(
           tag!("[ "),
           take_until!("]"),
           tag!("]")
       )
);

#[test]
fn timestamp_parser_test() {
    let string = "[ 1.002540] 1ca0000.dsi: reg=0x42424242 val=0x84848484";
    let timestamp = "1.002540";
    let rest = " 1ca0000.dsi: reg=0x42424242 val=0x84848484";

    assert_eq!(timestamp_parser(string.as_bytes()),
               Ok((rest.as_bytes(), timestamp.as_bytes())));

    let string = "[ 1.002540]";
    let rest = "";
    assert_eq!(timestamp_parser(string.as_bytes()),
               Ok((rest.as_bytes(), timestamp.as_bytes())));

    let string = "[ 1.002540";
    assert_eq!(timestamp_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(1))));

    let string = "[";
    assert_eq!(timestamp_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(2))));

    let string = "";
    assert_eq!(timestamp_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(2))));
}

named!(hex_parser<&[u8], u64>,
       do_parse!(
           tag!("0x") >>
           _hex: take_while_m_n!(1, 8, is_hex_digit) >>
           (u64::from_str_radix(str::from_utf8(_hex).unwrap(), 16).unwrap())
       )
);

#[test]
fn hex_parser_test() {
    let string = "0x42424242\n";
    let rest = "\n";
    assert_eq!(hex_parser(string.as_bytes()),
               Ok((rest.as_bytes(), 0x42424242)));

    let string = "0x42424242";
    let rest = "";
    assert_eq!(hex_parser(string.as_bytes()),
               Ok((rest.as_bytes(), 0x42424242)));

    let string = "0x";
    assert_eq!(hex_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(1))));
}

named!(register_parser<&[u8], u64>,
       do_parse!(
           tag!("reg=") >>
           _hex: hex_parser >>
           (_hex)
       )
);

#[test]
fn register_parser_test() {
    let string = "reg=0x42424242";
    let rest = "";
    assert_eq!(register_parser(string.as_bytes()),
               Ok((rest.as_bytes(), 0x42424242)));

    let string = "reg=0x";
    assert_eq!(register_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(1))));

    let string = "reg=";
    assert_eq!(register_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(2))));

    let string = "";
    assert_eq!(register_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(4))));
}

named!(value_parser<&[u8], u64>,
       do_parse!(
           tag!("val=") >>
           _hex: hex_parser >>
           (_hex)
       )
);

#[test]
fn value_parser_test() {
    let string = "val=0x42424242";
    let rest = "";
    assert_eq!(value_parser(string.as_bytes()),
               Ok((rest.as_bytes(), 0x42424242)));

    let string = "val=0x";
    assert_eq!(value_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(1))));

    let string = "val=";
    assert_eq!(value_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(2))));

    let string = "";
    assert_eq!(value_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(4))));
}

named!(log_line_parser<&[u8], RegisterWrite>,
       do_parse!(
           _time: timestamp_parser >>
           take_while!(is_space) >>
           take_until_and_consume!(":") >>
           take_while!(is_space) >>
           _reg: register_parser >>
           take_while!(is_space) >>
           _val: value_parser >>
           (
               RegisterWrite {
                   register: _reg,
                   timestamp: String::from_utf8(_time.to_vec()).unwrap(),
                   value: _val,
               }
           )
       )
);

#[test]
fn log_line_parser_test() {
    let string = "[ 1.002540] 1ca0000.dsi: reg=0x42424242 val=0x84848484";
    let result = log_line_parser(string.as_bytes()).unwrap().1;

    assert_eq!(result.register, 0x42424242);
    assert_eq!(result.timestamp, "1.002540");
    assert_eq!(result.value, 0x84848484);
}

pub fn read_register_writes(filename: &str) -> Result<()> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        println!("{:?}", log_line_parser(line.as_bytes()));
    }

    Ok(())
}
