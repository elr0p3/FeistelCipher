// pub struct FeistelCipher {
//     rounds: u32,
//     keys: Vec<String>,
// }

pub enum Mode {
	Encrypt,
	Decrypt,
}

// impl FeistelCipher {
pub mod feistelcipher {

	const ROUNDS: u32 = 4;
	const KEYS: [&'static str; 16] = [
		"1110", "0100", "1101", "0001",
		"0010", "1111", "1011", "1000",
		"0011", "1010", "0110", "1100",
		"0101", "1001", "0000", "0111",
	];
	// fn convert_keys_to_string() -> Vec<String> {
	//     Self::KEYS.iter()
	//         .map(|&s| String::from(s))
	//         .collect::<Vec<String>>()
	// }

	// #[allow(dead_code)]
	// pub fn new() -> Self {
	//     let keys = Self::convert_keys_to_string();
	//     Self {
	//         rounds: 4,
	//         keys,
	//     }
	// }

	// #[allow(dead_code)]
	// pub fn from_rounds(rounds: u32) -> Self {
	//     let keys = Self::convert_keys_to_string();
	//     Self {
	//         rounds,
	//         keys,
	//     }
	// }


	pub fn encrypt_decrypt(mode: super::Mode, message: &str) -> String {
		let mut ciphered_binary_string = String::new();
		let mut i: usize = 0;
		while i < message.len() {
			let slice = match mode {
				super::Mode::Encrypt => encrypt_bytes(&message[i..i + 8]),
				super::Mode::Decrypt => decrypt_bytes(&message[i..i + 8]),
			};
			ciphered_binary_string += &slice;
			i += 8;
		}

		ciphered_binary_string
	}

	fn encrypt_bytes(byte: &str) -> String {
		let middle_message = byte.len() / 2;
		let mut left = String::from(&byte[..middle_message]);
		let mut right = String::from(&byte[middle_message..]);

		for i in 0..ROUNDS {
			let tmp = right.clone();
			let function_text = and(&right, get_sub_key(i));
			right = xor(&left, &function_text);
			left = tmp;
		}

		left.to_string() + &right
	}

	fn decrypt_bytes(byte: &str) -> String {
		let middle_message = byte.len() / 2;
		let mut left = String::from(&byte[..middle_message]);
		let mut right = String::from(&byte[middle_message..]);

		for i in 0..ROUNDS {
			let tmp = left.clone();
			let function_text = and(&left, get_sub_key(ROUNDS - i - 1));
			left = xor(&right, &function_text);
			right = tmp;
		}

		left.to_string() + &right
	}


	fn get_sub_key(i: u32) -> &'static str {
		KEYS[i as usize % 16]
	}

	fn and(left: &str, right: &str) -> String {
		let mut content = String::new();
		for i in 0..left.len() {
			let l = left.as_bytes()[i] - '0' as u8;
			let r = right.as_bytes()[i] - '0' as u8;
			content += &(l & r).to_string();
		}
		content
	}

	fn xor(left: &str, right: &str) -> String {
		let mut content = String::new();
		for i in 0..left.len() {
			let l = left.as_bytes()[i] - '0' as u8;
			let r = right.as_bytes()[i] - '0' as u8;
			content += &(l ^ r).to_string();
		}
		content
	}
}
