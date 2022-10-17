use frame_support::dispatch::Weight;

pub trait WeightInfo {
	fn transfer() -> Weight;
	fn open_channel() -> Weight;
	fn set_pallet_params() -> Weight;
	fn on_chan_open_init() -> Weight;
	fn on_chan_open_try() -> Weight;
	fn on_chan_open_ack() -> Weight;
	fn on_chan_open_confirm() -> Weight;
	fn on_chan_close_init() -> Weight;
	fn on_chan_close_confirm() -> Weight;
	fn on_recv_packet() -> Weight;
	fn on_acknowledgement_packet() -> Weight;
	fn on_timeout_packet() -> Weight;
}

impl WeightInfo for () {
	fn transfer() -> Weight {
		0
	}

	fn open_channel() -> Weight {
		0
	}

	fn set_pallet_params() -> Weight {
		0
	}

	fn on_chan_open_init() -> Weight {
		0
	}

	fn on_chan_open_try() -> Weight {
		0
	}

	fn on_chan_open_ack() -> Weight {
		0
	}

	fn on_chan_open_confirm() -> Weight {
		0
	}

	fn on_chan_close_init() -> Weight {
		0
	}

	fn on_chan_close_confirm() -> Weight {
		0
	}

	fn on_recv_packet() -> Weight {
		0
	}

	fn on_acknowledgement_packet() -> Weight {
		0
	}

	fn on_timeout_packet() -> Weight {
		0
	}
}
