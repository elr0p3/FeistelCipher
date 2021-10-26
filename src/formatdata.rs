pub mod fdata {

	use num_bigint::{
		BigInt,
		Sign,
	};
	use num_traits::Num;

	pub fn str_to_int(content: &str) -> BigInt {
		let array_content = content.as_bytes();
		println!("bytes  -> {:x?}", array_content);
		let bi = BigInt::from_bytes_be(Sign::Plus, array_content);
		println!("bigint -> {}", bi);
		bi
	}
	
	/// Add 0 bits in the beginning to complete 8-bit blocks (left-padding)
	pub fn bigint_to_binary_string(number: &BigInt) -> String {
		// let mut binary_str_content = format!("{:b}", number);
		let mut binary_str_content = BigInt::to_str_radix(&number, 2);
		while binary_str_content.len() % 8 != 0 {
			binary_str_content.insert(0, '0');
		}
		binary_str_content
	}
	
	pub fn bigint_to_str(number: &BigInt) -> String {
		println!("bigint -> {}", number);
		let (_, content) = number.to_bytes_be();
		println!("bytes  -> {:x?}", content);
		String::from_utf8_lossy(&content).to_string()
	}

	pub fn binary_str_to_bigint(bi_str: &str) -> BigInt {
		BigInt::from_str_radix(bi_str, 2).unwrap()
	}

	pub fn slice_to_bigint(content: &[u8]) -> BigInt {
		BigInt::from_bytes_be(Sign::Plus, content)
	}
	
	pub fn bigint_to_vec(number: &BigInt) -> Vec<u8> {
		number.to_signed_bytes_be()
	}
}


	// let content = String::from("tonto");
	// let array_content = content.as_bytes();
	// let big = BigInt::from_bytes_be(Sign::Plus, array_content);
	// let (_, second_array) = big.to_bytes_be();
	// let final_content = String::from_utf8(second_array.clone()).unwrap();

	// println!("CONTENT\t>>> {}", content);
	// println!("ARRAY_C\t>>> {:?}", array_content);
	// println!("ARRAY_C\t>>> {:x?}", array_content);
	// println!("BIGINT\t>>> {}", big);
	// println!("BIGINTX\t>>> {:x}", big);
	// println!("BIGINTB\t>>> {:b}", big);

	// let sep = std::iter::repeat("-  ").take(16).collect::<String>();
	// println!("{}", sep);
	// println!("ARRAY_B\t>>> {:?}", second_array);
	// println!("FINAL\t>>> {}", final_content);
