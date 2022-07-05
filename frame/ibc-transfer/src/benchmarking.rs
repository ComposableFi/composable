//! Benchmarking setup for ibc-transfer

#[allow(unused)]
use super::*;
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	defi::DeFiComposableConfig,
	xcm::assets::{RemoteAssetRegistryMutate, XcmAssetLocation},
};
use core::str::FromStr;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::traits::fungibles::{Inspect, Mutate};
use frame_system::RawOrigin;
use ibc::{
	applications::transfer::{
		acknowledgement::ACK_ERR_STR,
		denom::{Amount, Coin, PrefixedDenom},
		packet::PacketData,
		VERSION,
	},
	core::{
		ics04_channel::{
			channel::{ChannelEnd, Counterparty, Order, State},
			msgs::acknowledgement::Acknowledgement,
			packet::Packet,
			Version,
		},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
		ics26_routing::context::{AsAnyMut, Module, OnRecvPacketAck},
	},
	handler::HandlerOutputBuilder,
	timestamp::Timestamp,
	Height,
};
use ibc_trait::{get_channel_escrow_address, ibc_denom_to_foreign_asset_id, OpenChannelParams};
use primitives::currency::CurrencyId;
use sp_runtime::{
	traits::{IdentifyAccount, Zero},
	AccountId32,
};

pub fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	where_clause {
		where AccountId32: From<T::AccountId>,
			  T::AccountId: From<AccountId32>,
			  CurrencyId: From<<T as DeFiComposableConfig>::MayBeAssetId>,
			  <T as DeFiComposableConfig>::Balance: From<u128>,
			  <T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
			  <<T as Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetId:
						From<<T as DeFiComposableConfig>::MayBeAssetId>,
			<<T as Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetNativeLocation:
						From<XcmAssetLocation>,
			T: Send + Sync
	}
	transfer {
		let caller: T::AccountId = whitelisted_caller();
		let client_id = <T as Config>::IbcHandler::create_client().unwrap();
		let connection_id = ConnectionId::new(0);
		<T as Config>::IbcHandler::create_connection(client_id, connection_id.clone()).unwrap();
		let port_id = PortId::transfer();
		let counterparty = Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let channel_end = ChannelEnd::new(
			State::Init,
			Order::Unordered,
			counterparty,
			vec![connection_id],
			Version::new(VERSION.to_string()),
		);

		let balance = 100000 * CurrencyId::milli::<u128>();
		let channel_id = <T as Config>::IbcHandler::open_channel(port_id, channel_end).unwrap();
		let denom = "transfer/channel-15/uatom";
		let foreign_asset_id = ibc_denom_to_foreign_asset_id(denom);
		let asset_id = <T as Config>::CurrencyFactory::create(
			RangeId::IBC_ASSETS,
			<T as DeFiComposableConfig>::Balance::zero(),
		).unwrap();
		<T as Config>::AssetRegistry::set_reserve_location(
			asset_id.into(),
			foreign_asset_id.into(),
			None,
			None,
		).unwrap();
		<<T as Config>::MultiCurrency as Mutate<T::AccountId>>::mint_into(
			asset_id.into(),
			&caller,
			balance.into(),
		).unwrap();


		let transfer_params = TransferParams {
			to:  "bob".to_string().as_bytes().to_vec(),
			source_channel: channel_id.to_string().as_bytes().to_vec(),
			timeout_timestamp: 1690894363u64.saturating_mul(1000000000),
			timeout_height: 2000,
			revision_number: None
		};

		Pallet::<T>::resgister_asset_id(asset_id.into(), denom.as_bytes().to_vec());
		<Params<T>>::put(PalletParams {
			send_enabled: true,
			receive_enabled: true
		});

		let amt = 1000 * CurrencyId::milli::<u128>();

	}:_(RawOrigin::Signed(caller.clone()), transfer_params, asset_id.into(), amt.into())
	verify {
		assert_eq!(<<T as Config>::MultiCurrency as Inspect<T::AccountId>>::balance(
			asset_id.into(),
			&caller
		), (balance - amt).into());
	}

	open_channel {
		let client_id = <T as Config>::IbcHandler::create_client().unwrap();
		let connection_id = ConnectionId::new(0);
		<T as Config>::IbcHandler::create_connection(client_id, connection_id.clone()).unwrap();
		let port_id = PortId::transfer();
		let open_channel_params = OpenChannelParams {
			connection_id: connection_id.as_bytes().to_vec(),
			order: 1,
			counterparty_port_id: port_id.clone().as_bytes().to_vec(),
			version: vec![]
		};
	}:_(RawOrigin::Root, open_channel_params)
	verify {
		assert_last_event::<T>(Event::<T>::ChannelOpened {
			channel_id: ChannelId::new(0).to_string().as_bytes().to_vec(),
			port_id: port_id.as_bytes().to_vec()
		}.into())
	}

	set_pallet_params {
		let pallet_params = PalletParams {
			send_enabled: true,
			receive_enabled: true
		};

	}:_(RawOrigin::Root, pallet_params)
	verify {
		assert_last_event::<T>(Event::<T>::PalletParamsUpdated {
			send_enabled: true,
			receive_enabled: true
		}.into())
	}

	on_chan_open_init {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let counterparty = Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let connection_hops = vec![ConnectionId::new(0)];
		let version = Version::new(VERSION.to_string());
		let order = Order::Unordered;
		let channel_id = ChannelId::new(0);
		let mut handler = IbcCallbackHandler::<T>::default();
	}:{
		handler.on_chan_open_init(&mut output, order, &connection_hops, &port_id, &channel_id, &counterparty, &version).unwrap();
	}

	on_chan_open_try {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let counterparty = Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let connection_hops = vec![ConnectionId::new(0)];
		let version = Version::new(VERSION.to_string());
		let order = Order::Unordered;
		let channel_id = ChannelId::new(0);
		let mut handler = IbcCallbackHandler::<T>::default();
	}:{
		handler.on_chan_open_try(&mut output, order, &connection_hops, &port_id, &channel_id, &counterparty, &version, &version).unwrap();
	}

	on_chan_open_ack {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let version = Version::new(VERSION.to_string());
		let channel_id = ChannelId::new(0);
		let mut handler = IbcCallbackHandler::<T>::default();
	}:{
		handler.on_chan_open_ack(&mut output, &port_id, &channel_id, &version).unwrap();
	}
	verify {
		assert_eq!(ChannelIds::<T>::get().len(), 1)
	}

	on_chan_open_confirm {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let channel_id = ChannelId::new(0);
		let mut handler = IbcCallbackHandler::<T>::default();
	}:{
		handler.on_chan_open_confirm(&mut output, &port_id, &channel_id).unwrap();
	}
	verify {
		assert_eq!(ChannelIds::<T>::get().len(), 1)
	}

	on_chan_close_init {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let channel_id = ChannelId::new(0);
		let channel_ids = vec![channel_id.to_string().as_bytes().to_vec()];
		ChannelIds::<T>::put(channel_ids);
		let mut handler = IbcCallbackHandler::<T>::default();
	}:{
		handler.on_chan_close_init(&mut output, &port_id, &channel_id).unwrap();
	}
	verify {
		assert_eq!(ChannelIds::<T>::get().len(), 0)
	}

	on_chan_close_confirm {
		let mut output = HandlerOutputBuilder::new();
		let port_id = PortId::transfer();
		let channel_id = ChannelId::new(0);
		let channel_ids = vec![channel_id.to_string().as_bytes().to_vec()];
		ChannelIds::<T>::put(channel_ids);
		let mut handler = IbcCallbackHandler::<T>::default();
	}:{
		handler.on_chan_close_confirm(&mut output, &port_id, &channel_id).unwrap();
	}
	verify {
		assert_eq!(ChannelIds::<T>::get().len(), 0)
	}

	on_recv_packet {
		let caller: T::AccountId = whitelisted_caller();
		let client_id = <T as Config>::IbcHandler::create_client().unwrap();
		let connection_id = ConnectionId::new(0);
		<T as Config>::IbcHandler::create_connection(client_id, connection_id.clone()).unwrap();
		let port_id = PortId::transfer();
		let counterparty = Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let channel_end = ChannelEnd::new(
			State::Init,
			Order::Unordered,
			counterparty,
			vec![connection_id],
			Version::new(VERSION.to_string()),
		);


		let balance = 100000 * CurrencyId::milli::<u128>();
		let channel_id = <T as Config>::IbcHandler::open_channel(port_id.clone(), channel_end).unwrap();
		let denom = "transfer/channel-1/PICA";
		let channel_escrow_address = get_channel_escrow_address(&port_id, channel_id).unwrap();
		let channel_escrow_address = <T as Config>::AccountIdConversion::try_from(channel_escrow_address).map_err(|_| ()).unwrap();
		let channel_escrow_address: T::AccountId = channel_escrow_address.into_account();

		<<T as Config>::MultiCurrency as Mutate<T::AccountId>>::mint_into(
			CurrencyId::PICA.into(),
			&channel_escrow_address,
			balance.into(),
		).unwrap();


		<Params<T>>::put(PalletParams {
			send_enabled: true,
			receive_enabled: true
		});

		let raw_user: AccountId32 =  caller.clone().into();
		let raw_user: &[u8] = raw_user.as_ref();
		let mut hex_string = hex::encode_upper(raw_user.to_vec());
		hex_string.insert_str(0, "0x");
		let prefixed_denom = PrefixedDenom::from_str(denom).unwrap();
		let amt = 1000 * CurrencyId::milli::<u128>();
		let coin = Coin {
			denom: prefixed_denom,
			amount: Amount::from_str(&format!("{:?}", amt)).unwrap()
		};
		let packet_data = PacketData {
			token: coin,
			sender: Signer::from_str("alice").unwrap(),
			receiver: Signer::from_str(&hex_string).unwrap(),
		};

		let data = serde_json::to_vec(&packet_data).unwrap();
		let packet = Packet {
			sequence: 0u64.into(),
			source_port: port_id.clone(),
			source_channel: ChannelId::new(1),
			destination_port: port_id,
			destination_channel: ChannelId::new(0),
			data,
			timeout_height: Height::new(2000, 5),
			timeout_timestamp: Timestamp::from_nanoseconds(1690894363u64.saturating_mul(1000000000))
				.unwrap(),
		 };
		 let mut handler = IbcCallbackHandler::<T>::default();
		 let mut output = HandlerOutputBuilder::new();
		 let signer = Signer::from_str("relayer").unwrap();
	}:{

		let res = handler.on_recv_packet(&mut output, &packet, &signer);
		match res {
			OnRecvPacketAck::Successful(_, write_fn) => {
				write_fn(handler.as_any_mut()).unwrap()
			}
			_ => panic!("Expected successful execution")
		}

	 }
	verify {
		assert_eq!(<<T as Config>::MultiCurrency as Inspect<T::AccountId>>::balance(
			CurrencyId::PICA.into(),
			&caller
		), amt.into());
	}

	on_acknowledgement_packet {
		let caller: T::AccountId = whitelisted_caller();
		let client_id = <T as Config>::IbcHandler::create_client().unwrap();
		let connection_id = ConnectionId::new(0);
		<T as Config>::IbcHandler::create_connection(client_id, connection_id.clone()).unwrap();
		let port_id = PortId::transfer();
		let counterparty = Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let channel_end = ChannelEnd::new(
			State::Init,
			Order::Unordered,
			counterparty,
			vec![connection_id],
			Version::new(VERSION.to_string()),
		);


		let balance = 100000 * CurrencyId::milli::<u128>();
		let channel_id = <T as Config>::IbcHandler::open_channel(port_id.clone(), channel_end).unwrap();
		let denom = "PICA";
		let channel_escrow_address = get_channel_escrow_address(&port_id, channel_id).unwrap();
		let channel_escrow_address = <T as Config>::AccountIdConversion::try_from(channel_escrow_address).map_err(|_| ()).unwrap();
		let channel_escrow_address: T::AccountId = channel_escrow_address.into_account();

		<<T as Config>::MultiCurrency as Mutate<T::AccountId>>::mint_into(
			CurrencyId::PICA.into(),
			&channel_escrow_address,
			balance.into(),
		).unwrap();


		<Params<T>>::put(PalletParams {
			send_enabled: true,
			receive_enabled: true
		});

		let raw_user: AccountId32 =  caller.clone().into();
		let raw_user: &[u8] = raw_user.as_ref();
		let mut hex_string = hex::encode_upper(raw_user.to_vec());
		hex_string.insert_str(0, "0x");
		let prefixed_denom = PrefixedDenom::from_str(denom).unwrap();
		let amt = 1000 * CurrencyId::milli::<u128>();
		let coin = Coin {
			denom: prefixed_denom,
			amount: Amount::from_str(&format!("{:?}", amt)).unwrap()
		};
		let packet_data = PacketData {
			token: coin,
			sender: Signer::from_str(&hex_string).unwrap(),
			receiver: Signer::from_str("alice").unwrap(),
		};

		let data = serde_json::to_vec(&packet_data).unwrap();
		let packet = Packet {
			sequence: 0u64.into(),
			source_port: port_id.clone(),
			source_channel: ChannelId::new(0),
			destination_port: port_id,
			destination_channel: ChannelId::new(1),
			data,
			timeout_height: Height::new(2000, 5),
			timeout_timestamp: Timestamp::from_nanoseconds(1690894363u64.saturating_mul(1000000000))
				.unwrap(),
		 };
		 let mut handler = IbcCallbackHandler::<T>::default();
		 let mut output = HandlerOutputBuilder::new();
		 let signer = Signer::from_str("relayer").unwrap();
		 let ack: Acknowledgement = ACK_ERR_STR.to_string().as_bytes().to_vec().into();
	}:{
	   handler.on_acknowledgement_packet(&mut output, &packet, &ack, &signer).unwrap();
	}
	verify {
		assert_eq!(<<T as Config>::MultiCurrency as Inspect<T::AccountId>>::balance(
			CurrencyId::PICA.into(),
			&caller
		), amt.into());
	}

	on_timeout_packet {
		let caller: T::AccountId = whitelisted_caller();
		let client_id = <T as Config>::IbcHandler::create_client().unwrap();
		let connection_id = ConnectionId::new(0);
		<T as Config>::IbcHandler::create_connection(client_id, connection_id.clone()).unwrap();
		let port_id = PortId::transfer();
		let counterparty = Counterparty::new(port_id.clone(), Some(ChannelId::new(1)));
		let channel_end = ChannelEnd::new(
			State::Init,
			Order::Unordered,
			counterparty,
			vec![connection_id],
			Version::new(VERSION.to_string()),
		);


		let balance = 100000 * CurrencyId::milli::<u128>();
		let channel_id = <T as Config>::IbcHandler::open_channel(port_id.clone(), channel_end).unwrap();
		let denom = "PICA";
		let channel_escrow_address = get_channel_escrow_address(&port_id, channel_id).unwrap();
		let channel_escrow_address = <T as Config>::AccountIdConversion::try_from(channel_escrow_address).map_err(|_| ()).unwrap();
		let channel_escrow_address: T::AccountId = channel_escrow_address.into_account();

		<<T as Config>::MultiCurrency as Mutate<T::AccountId>>::mint_into(
			CurrencyId::PICA.into(),
			&channel_escrow_address,
			balance.into(),
		).unwrap();


		<Params<T>>::put(PalletParams {
			send_enabled: true,
			receive_enabled: true
		});

		let raw_user: AccountId32 =  caller.clone().into();
		let raw_user: &[u8] = raw_user.as_ref();
		let mut hex_string = hex::encode_upper(raw_user.to_vec());
		hex_string.insert_str(0, "0x");
		let prefixed_denom = PrefixedDenom::from_str(denom).unwrap();
		let amt = 1000 * CurrencyId::milli::<u128>();
		let coin = Coin {
			denom: prefixed_denom,
			amount: Amount::from_str(&format!("{:?}", amt)).unwrap()
		};
		let packet_data = PacketData {
			token: coin,
			sender: Signer::from_str(&hex_string).unwrap(),
			receiver: Signer::from_str("alice").unwrap(),
		};

		let data = serde_json::to_vec(&packet_data).unwrap();
		let packet = Packet {
			sequence: 0u64.into(),
			source_port: port_id.clone(),
			source_channel: ChannelId::new(0),
			destination_port: port_id,
			destination_channel: ChannelId::new(1),
			data,
			timeout_height: Height::new(2000, 5),
			timeout_timestamp: Timestamp::from_nanoseconds(1690894363u64.saturating_mul(1000000000))
				.unwrap(),
		 };
		 let mut handler = IbcCallbackHandler::<T>::default();
		 let mut output = HandlerOutputBuilder::new();
		 let signer = Signer::from_str("relayer").unwrap();
	}:{
		handler.on_timeout_packet(&mut output, &packet, &signer).unwrap();
	}
	verify {
		assert_eq!(<<T as Config>::MultiCurrency as Inspect<T::AccountId>>::balance(
			CurrencyId::PICA.into(),
			&caller
		), amt.into());
	}
}
