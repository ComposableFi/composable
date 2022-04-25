use crate::{Any, Config};
use core::marker::PhantomData;
use frame_support::pallet_prelude::Weight;
use ibc::core::{
	ics02_client::msgs as client_msgs, ics03_connection::msgs as conn_msgs,
	ics04_channel::msgs as chan_msgs,
};
use ibc_trait::CallbackWeight;
use sp_std::prelude::*;

pub trait WeightInfo {
	fn create_client() -> Weight;
	fn update_client() -> Weight;
	fn connection_init() -> Weight;
	fn create_channel() -> Weight;
}

impl WeightInfo for () {
	fn create_client() -> Weight {
		0
	}

	fn update_client() -> Weight {
		0
	}

	fn connection_init() -> Weight {
		0
	}

	fn create_channel() -> Weight {
		0
	}
}

pub struct WeightRouter<T: Config>(PhantomData<T>);

impl<T: Config> WeightRouter<T> {
	pub fn new() -> Self {
		Self(PhantomData::default())
	}

	pub fn get_weight(port_id: &str) -> Box<dyn CallbackWeight> {
		match port_id {
			pallet_ibc_ping::PORT_ID => Box::new(pallet_ibc_ping::WeightHandler::<T>::new()),
			_ => panic!("Invalid route"),
		}
	}
}

pub fn deliver<T: Config>(msgs: Vec<Any>) -> Weight {
	let _router = WeightRouter::<T>::new();
	msgs.into_iter().fold(Weight::default(), |acc, msg| {
		// Decode message type and get port_id
		// Add benchmarked weight for that message type
		// Add benchmarked weight for module callback
		acc
	})
}
