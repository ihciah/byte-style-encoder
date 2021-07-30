use byte_style_encoder::{encode_to_byte_style, decode_from_byte_style};
use clap::{App, Arg};

fn main() {
    let matches = App::new("Byte Style Encoder")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("字节范编解码器：我们只说字节范!")
        .setting(clap::AppSettings::TrailingVarArg)
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .arg(Arg::with_name("decode")
            .short("d")
            .long("decode")
            .takes_value(false)
            .help("切换为解码模式（默认为编码模式） / Switch to decoder mode"))
        .arg(
            Arg::with_name("content")
                .required(true)
                .takes_value(true)
                .multiple(true)
                .help("需要编解码的内容 / Content to be encoded or decoded")
        )
        .get_matches();

    let decode = matches.is_present("decode");
    let content = matches.values_of("content").expect("没有待编解码的内容！ / No content provided!").collect::<Vec<_>>();

    if decode {
        match decode_from_byte_style(&content.join(" ")) {
            Ok(decoded) => {println!("{}", decoded);}
            Err(e) => {println!("Decode failed: {}", e)}
        }
        return;
    }
    match encode_to_byte_style(&content.join(" ")) {
        Ok(encoded) => {println!("{}", encoded);}
        Err(e) => {println!("Encode failed: {}", e)}
    }
}
