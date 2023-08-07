#[test]
fn test_memo() {
	let encoded_address = "centauri17ak6lczt2c4gxja432enf870lv8y99qvs84g9k";
	println!("start: {}", encoded_address);
	let (hrp, data, v) = bech32_no_std::decode(encoded_address).unwrap();
	println!("Decoded Variant: {:?}", v);
	assert!(v == bech32_no_std::Variant::Bech32);

	println!("Decoded HRP: {}", hrp);
	println!("Decoded Data: {:?}", data);
	println!("Decoded Data len: {:?}", data.len());
	let mut decoded_data = Vec::new();
	for byte in data.clone() {
		decoded_data.push(u8::try_from(byte).unwrap());
	}
	print!("Decoded Data: {:?}\n", decoded_data.clone());
	let res = bech32_no_std::encode(&hrp, data.clone(), bech32_no_std::Variant::Bech32).unwrap();
	print!("Encided back Data: {:?}\n", res);
	assert_eq!(encoded_address, res);

	let address: [u8; 32] = [
		30, 29, 22, 26, 31, 24, 2, 11, 10, 24, 21, 8, 6, 18, 29, 21, 17, 10, 25, 19, 9, 7, 30, 15,
		31, 12, 7, 4, 5, 5, 0, 12,
	];
	assert_eq!(decoded_data, address);
	let name = "centauri";
	let data1: Vec<bech32_no_std::u5> = address
		.into_iter()
		.map(|byte| bech32_no_std::u5::try_from_u8(byte).unwrap())
		.collect::<Vec<bech32_no_std::u5>>();
	assert_eq!(data, data1);
	println!("data1 : {:?}", data1.clone());
	// println!("data : {:?}", data.clone());
	let name = String::from_utf8(name.into()).unwrap();
	let res = bech32_no_std::encode(&name, data1.clone(), bech32_no_std::Variant::Bech32).unwrap();
	println!("res : {}", res);
	assert_eq!(encoded_address, res);
}
