

use {super::*, sp_runtime::traits::*};


    #[test]
    fn aaas() {
        	let asset_id = AssetId::Concrete(
		MultiLocation::new(
			0, 
			X1(
				Junction::AccountId32 { network: Any, id : AccountId::from(DEFAULT).into()}
			)
		)
	);
        let asset_id = CurrencyIdConvert::convert(asset_id);
    }