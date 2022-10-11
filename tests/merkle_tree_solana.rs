pub mod solana {
    use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
    use serde::Deserialize;
    use solana_program::pubkey::Pubkey;
    use std::fs::File;
    use std::io::Read;
    use std::str::FromStr;

    #[derive(Clone)]
    pub struct SolanaSha256Algorithm {}

    impl Hasher for SolanaSha256Algorithm {
        type Hash = [u8; 32];

        fn hash(data: &[u8]) -> [u8; 32] {
            solana_program::keccak::hashv(&[data]).to_bytes()
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    struct WhiteListItem {
        handle: String,
        amount: u64,
    }

    #[test]
    pub fn solana_merkle_tree() {
        let mut file = File::open("client/src/whitelist-test.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let whitelist: Vec<WhiteListItem> = serde_json::from_str(&data).unwrap();

        let mut leafs = vec![];

        for item in whitelist.iter() {
            leafs.push(
                solana_program::keccak::hashv(&[
                    &[0x00],
                    &Pubkey::from_str(item.handle.as_str()).unwrap().to_bytes(),
                    &item.amount.to_le_bytes(),
                ])
                .to_bytes(),
            );
        }

        // need to hash the leaves before passing into the Merkle tree
        let merkle_tree = MerkleTree::<SolanaSha256Algorithm>::from_leaves(&leafs);

        println!("root hex: {:?}", merkle_tree.root_hex().unwrap())
    }
}
