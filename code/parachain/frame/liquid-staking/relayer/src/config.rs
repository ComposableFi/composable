use crate::{SovereignSubAccount, StorageKey};
pub fn get_config() -> Vec<SovereignSubAccount> {
	cfg_if::cfg_if! {
		if #[cfg(feature = "composable")] {
			println!("{}", "feature 'composable': Relayer for Composable network");
			let _sovereign_account_id = "13YMK2ecbyxtm4cmFs31PqzWmQ7gWVboJSmXbcA56DB94xB9";
			let sovereign_account_id_index_0 = "12x6QU4c9eRPxJMATFsRNFiZTMK5QgZkdZFFeu2QDKn4TR82";
			let sovereign_account_id_index_1 = "1461Z7Bm1bwQpz1PuYMQ8phj9bRpxNU7ZYsb7aXQRAUuNecG";
			let sovereign_account_id_index_2 = "15ySsNFkAhswdn9hSKkzoK7LhmJrj8bgyUZQAiM7Df9JpBUH";
			let sovereign_account_id_index_3 = "15s3DuzMeftBH7YdHykwPDUd2DBxdNbiyqgDfZDA3i5eRwUW";
			let sovereign_account_id_index_4 = "12uNvUSK39SDbHbqWuMhFdw2hHySrkbenVrHdS678fkj9BBb";
			let sovereign_account_id_index_5 = "14tDkT3U93Pc1wLrHEjfYuhPPnFpMwDr7o8phPCTwTRj5wfE";

			let p0 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc456d08aea5b028f73699523ae21709a815640ec97748f5b5da9a2298e830e8971df7908861e1710b957fe06f0703bca7d";
			let p1 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc491af1d8906a21795a98d84506b4216828886ca7474c66c027a9dc5d73901481568d551d01f13d0eb3bd36dd20ed2f13e";
			let p2 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4bc407118eef848de28bad224cc60c8fedbfdc1d87d5f65f94602d8e31d7426de7b7d0bb4a8e9223b72bb39231737377c";
			let p3 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc43802b39b0fa95ce7e37486987631cef3d71aafa64aa8d5d9164071a14106cdba71fe4b8fc08de39a7606c8d16f5b20f8";
			let p4 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4b8ac2e6f27924e06455d8e5c75b6a775542eca539ca4e92c4b4e0496bef934595597645e21eb29b5ce94bc38b7e45181";
			let p5 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4232647f67d6923193db6aedafe0d5ea3abc522231610ec74f6391785db08c68f3c07c1bb45e2193b413f66926dec5072";
		}
		else{
			println!("{}", "Default feature 'picasso': Relayer for Picasso network");
			let _sovereign_account_id = "F7fq1imhk2xwqqiQZav65eYKuzxSWWaF3HQp4fXkx6Sb3jY";
			let sovereign_account_id_index_0 = "5xi8ChU4B7KQhpu3VcAhiPHJwCB7ZRBGHrtkkWEc59dufxud";
			let sovereign_account_id_index_1 = "5vWm2EwDzaAx9m1zA4cn6L2kGpSqEodXCSboU3sC2L1Bap89";
			let sovereign_account_id_index_2 = "5yDkX5PDf3HrS3DCfJCQNmXbGhXS5expcsQRuHF9PHijnBP8";
			let sovereign_account_id_index_3 = "5uGqg72hYJwNbZsc8yUoosP6qKnaM9YmSLny97BiBWaFjVuF";
			let sovereign_account_id_index_4 = "5twVfsa9HMayg9ewDaoGvBaxjsUY5CoWCdAyMkyJxVfJZzhL";
			let sovereign_account_id_index_5 = "5xyCUmCz6X6Vt7BwqjD5TJrVztHa44SPbTsDbRFHtvNKoowG";

			let p0 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4ca5bb2d842b7645d5be486494c94ae9eb6e44e3cc5ecf446f64b424c4df5e6c8f131eea4643d888b6520601991cab3ed";
			let p1 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc46d27fba198fb3980b80b1cb2808485cb55c0d70b182f5eda4b3b431877942e51eed1e9f67605e3ff9796c37f990aa768";
			let p2 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4b8c748966862268009d367885803cc8fcd7cad008951936eedb5647e6fb1fcaabf026f5a8383909a18a9e460bbda98d5";
			let p3 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4ab8199cef4c3bcb677ed773063a7a59b1ee6d56d0cfb1c6fb2420245037c7e81f7502017ce171081faa51060c1893da2";
			let p4 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc495c7694204423897be092f9d03b39e281025d4e00dc0e913ac982b3367b9503cd14cf62850f2c0e9bccf39dcfa780043";
			let p5 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc436bc383c1b32aa2650599816d58630a2c263625eb7c0a0fc84845b51c699c67437e9fb698d2087cb7af53a0bfc8281cb";
		}
	}

	let s0 = StorageKey(hex::decode(p0).expect("Failed to decode hex string p0"));
	let s1 = StorageKey(hex::decode(p1).expect("Failed to decode hex string p1"));
	let s2 = StorageKey(hex::decode(p2).expect("Failed to decode hex string p2"));
	let s3 = StorageKey(hex::decode(p3).expect("Failed to decode hex string p3"));
	let s4 = StorageKey(hex::decode(p4).expect("Failed to decode hex string p4"));
	let s5 = StorageKey(hex::decode(p5).expect("Failed to decode hex string p5"));

	let sub_accounts = vec![
		SovereignSubAccount {
			address: sovereign_account_id_index_0.to_string(),
			storage_key: s0.clone(),
			derivative_index: 0,
		},
		SovereignSubAccount {
			address: sovereign_account_id_index_1.to_string(),
			storage_key: s1.clone(),
			derivative_index: 1,
		},
		SovereignSubAccount {
			address: sovereign_account_id_index_2.to_string(),
			storage_key: s2.clone(),
			derivative_index: 2,
		},
		SovereignSubAccount {
			address: sovereign_account_id_index_3.to_string(),
			storage_key: s3.clone(),
			derivative_index: 3,
		},
		SovereignSubAccount {
			address: sovereign_account_id_index_4.to_string(),
			storage_key: s4.clone(),
			derivative_index: 4,
		},
		SovereignSubAccount {
			address: sovereign_account_id_index_5.to_string(),
			storage_key: s5.clone(),
			derivative_index: 5,
		},
	];
	sub_accounts
}
