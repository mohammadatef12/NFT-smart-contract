pub fn nft_mint(
    &mut self,
    token_id: TokenId,
    token_owner_id: AccountId,
    token_metadata: TokenMetadata,
) -> Token {
    assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");
    self.tokens.internal_mint(token_id, token_owner_id, Some(token_metadata))
}