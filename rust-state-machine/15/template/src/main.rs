mod balances;
mod system;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
	/*
		TODO: Move your type definitions for `AccountId` and `Balance` here.
	*/
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet,
	/* TODO: Use your type definitions for your new generic `balances::Pallet`. */
	balances: balances::Pallet,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	runtime.balances.set_balance(&"alice", 100);

	// start emulating a block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	runtime.system.inc_nonce(&"alice");
	let _res = runtime.balances.transfer(&"alice", &"bob", 30).map_err(|e| eprintln!("{}", e));

	// second transaction
	runtime.system.inc_nonce(&"alice");
	let _res = runtime
		.balances
		.transfer(&"alice", &"charlie", 20)
		.map_err(|e| eprintln!("{}", e));

	println!("{:#?}", runtime);
}