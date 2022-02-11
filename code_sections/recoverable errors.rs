fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }