/// BribeDAO interface
pub trait Bribe {
	fn create_request(request: BribeRequest) {}

	fn take_bribe(request: TakeBribeRequest) {}
}

pub struct BribeRequest {}

pub struct TakeBribeRequest {}
