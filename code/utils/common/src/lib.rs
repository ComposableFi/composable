/// Concrete event type for verbose event asserts in tests.
#[allow(clippy::large_enum_variant)]
#[derive(derive_more::From)]
pub enum AllRuntimeEvents {
	/// Picasso runtime events
	Picasso(picasso_runtime::Event),
	/// Dali runtime events
	Dali(dali_runtime::Event),
	/// Composable runtime events
	Composable(composable_runtime::Event),
}

/// Convenience method to match on [`AllRuntimeEvents`]
#[macro_export]
macro_rules! match_event {
	($ev:expr, $event:ident, $sub_ev:pat) => {{
		matches!(
			$ev,
			AllRuntimeEvents::Picasso(picasso_runtime::Event::$event($sub_ev)) |
				AllRuntimeEvents::Dali(dali_runtime::Event::$event($sub_ev)) |
				AllRuntimeEvents::Composable(composable_runtime::Event::$event($sub_ev))
		)
	}};
}
