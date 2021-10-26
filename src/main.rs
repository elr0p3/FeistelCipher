mod feistel;
mod formatdata;

use std::io::Write;
use std::fs::{self, File};

use feistel::{
	feistelcipher,
	Mode
};
use formatdata::fdata;

use clap::{
	Arg,
	App,
	SubCommand,
};


fn main() {
	let matches = App::new("Feistel Cipher Program")
		.version("1.0")
		.author("elr0p3")
		.about("Encrypt text and files")
		.arg(
			Arg::with_name("text")
				.value_name("TEXT")
				// .short("t")
				// .long("text")
				.takes_value(true)
				// .required_unless("file")
				.help("Text to encrypt")
		)
		.subcommand(
			SubCommand::with_name("file")
				.about("Let (e)Encrypt (d)Decrypt files")
				.version("1.0")
				.author("elr0p3 <someone_else@other.com>")
				// .required_unless("text")
				// .help("File to encrypt/decrypt")
				.arg(
					Arg::with_name("encrypt")
						.short("e")
						.long("encrypt")
						.takes_value(true)
						.required_unless("decrypt")
						.help("File to encrypt")
				)
				.arg(
					Arg::with_name("decrypt")
						.short("d")
						.long("decrypt")
						.takes_value(true)
						.required_unless("encrypt")
						.help("File to decrypt")
				)
				.arg(
					Arg::with_name("output")
						.short("o")
						.long("output")
						.takes_value(true)
						.required(true)
						.help("File where encrypted or decrypted information is stored")
				)
		)
		.get_matches();

	if matches.is_present("text") {
		let content = matches.value_of("text").unwrap();
		text(content);
	} else if let Some(sub_matches) = matches.subcommand_matches("file") {
		let output_filename = sub_matches.value_of("output").unwrap();
		let input_filename = if sub_matches.is_present("encrypt") {
			sub_matches.value_of("encrypt").unwrap()
		} else {
			sub_matches.value_of("decrypt").unwrap()
		};
		file(input_filename);
	} else {
		text("Crypto");
	}
}

fn text(content: &str) {
	// Convert input text into a BigInt and then into a binary String
	println!("PlainText (ascii)\t: {}", content);
	let int_content = fdata::str_to_int(content);
	let binary_str_content = fdata::bigint_to_binary_string(&int_content);
	println!("PlainText (ascii)\t: {}", binary_str_content);

	sep();

	// Encrypt the binary String
	// Apply Feistel encryption in blocks of 8 bits
	let ciphered_binary_string = feistelcipher::encrypt_decrypt(
		Mode::Encrypt, &binary_str_content
	);
	println!("Ciphered (binary)\t: {}", ciphered_binary_string);
	let ciphered_int = fdata::binary_str_to_bigint(&ciphered_binary_string);
	let ciphered_text = fdata::bigint_to_str(&ciphered_int);
	println!("Ciphered (ascii)\t: {}", ciphered_text);
	
	sep();

	// Decrypt the binary String
	let decrypted_binary_string = feistelcipher::encrypt_decrypt(
		Mode::Decrypt, &ciphered_binary_string
	);
	println!("Deciphered (binary)\t: {}", decrypted_binary_string);
	let deciphered_int = fdata::binary_str_to_bigint(&decrypted_binary_string);
	let deciphered_text = fdata::bigint_to_str(&deciphered_int);
	println!("Deciphered (ascii)\t: {}", deciphered_text);
}

fn file_enc() {

}

fn file_dec() {

}

fn file(filename: &str) {
	println!("FILE - {}", filename);
	// let ciph_extension = "ciph";
	let encrypted_filename = "encrypted.ciph";
	let decrypted_filename = "decrypted.result";
	let content_file = fs::read(filename).unwrap();

	if let Some("ciph") = get_file_extension(filename) {
		let content_bigint = fdata::slice_to_bigint(&content_file);
		let content_binary_string = fdata::bigint_to_binary_string(&content_bigint);
		let deciphered_binary_string = feistelcipher::encrypt_decrypt(
			Mode::Decrypt, &content_binary_string
		);
		let deciphered_int = fdata::binary_str_to_bigint(&deciphered_binary_string);
		let deciphered_content = fdata::bigint_to_vec(&deciphered_int);

		let mut result_file = File::create(decrypted_filename).unwrap();
		result_file.write_all(&deciphered_content).unwrap();
	} else {
		let content_bigint = fdata::slice_to_bigint(&content_file);
		let content_binary_string = fdata::bigint_to_binary_string(&content_bigint);
		let ciphered_binary_string = feistelcipher::encrypt_decrypt(
			Mode::Encrypt, &content_binary_string
		);
		let ciphered_int = fdata::binary_str_to_bigint(&ciphered_binary_string);
		let ciphered_content = fdata::bigint_to_vec(&ciphered_int);

		let mut result_file = File::create(encrypted_filename).unwrap();
		result_file.write_all(&ciphered_content).unwrap();
	}
}


fn sep() {
	let separator = std::iter::repeat("-  ").take(32).collect::<String>();
	println!("{}", separator);
}

fn get_file_extension(filename: &str) -> Option<&str> {
	use std::path::Path;
	use std::ffi::OsStr;

	Path::new(filename)        
		.extension()        
		.and_then(OsStr::to_str)
}
