let token_id = "0".to_string();
contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());
// alice approves bob
testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
contract.nft_approve(token_id.clone(), accounts(1), None);