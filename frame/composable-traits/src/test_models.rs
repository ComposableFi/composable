fn compound_v1() {
    let borrows_current = 100.;
    let cash = 10.;
    let wrapped_token = borrows_current + cash;
    let reserves_current = 1.;
    let index_start = 1.;
    let utilization_factor  = borrows_current / (cash + borrows_current);
    let borrow_interest_rate = 0.025 + utilization_factor * 0.2;
    let exchange_rate = (cash + borrows_current - reserves)/ wrapped_token;
    let mut index_current = index_start;
    let passed_blocks = 3.0;
    let borrow_interest_rate_per_block = borrow_interest_rate / passed_blocks;
    let index_next = index_current * (1.0 + borrow_interest_rate_per_block * passed_blocks);
    let borrows_next = borrows_current * (1.0 + borrow_interest_rate_per_block * passed_blocks);
    let reserves_next =  reserves_current +  borrows_current * borrow_interest_rate_per_block * passed_blocks;
    let assets = 30.;
    let oracle_asset_price = 1.3;
    let oracle_collateral_price =  16;
    let collateral_factor = 0.66;
    let normalized_asset = assets * oracle_asset_price;
    let normalized_collateral = collaterals * oracle_collateral_price;
    let collateral_needed = normalized_asset * collateral_factor / oracle_collateral_price;
}

