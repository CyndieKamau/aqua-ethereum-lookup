//tests for the infura provider

#[cfg(test)]
mod tests {
    use ethereum_types::H512;

    use crate::{
        constants::constants::{
            UrlProvider, FAILED_TO_PARSE_TRANSACTION_HASH, WRONG_BLOCK_TIMESTAMP, WRONG_INPUT,
        },
        provider::get_tx_data,
    };

    #[tokio::test]
    async fn test_get_tx_data_success() {
        // Chain ID and transaction hash for a successful scenario (Sepolia)
        let chain_id = 0xaa36a7;
        let tx_hash = "0xd82cb4b91a83124fdd2aa367256c22b94276cbc046d1cf56379035fb13a9dd00";
        let correct_input: H512 = "0xe41d6466e2f1deb48afd31993a6b6e84b50185d2f30b399d97a801b0cf82e35764d52b39920ac39f11e518fc3f482d68d04e3ebaff91081dad13d80ac41c069a".parse().unwrap();

        // Call the function and unwrap the result
        let result = get_tx_data(chain_id, tx_hash, UrlProvider::Infura)
            .await
            .unwrap();

        // Verify the result
        assert_eq!(result.0, correct_input, "{}", WRONG_INPUT);
        assert_eq!(result.1, 1717611456, "{}", WRONG_BLOCK_TIMESTAMP);
    }

    #[tokio::test]
    async fn test_get_tx_data_invalid_chain_id() {
        // Invalid chain ID but valid transaction hash
        let chain_id = 0x2;
        let tx_hash = "0xd82cb4b91a83124fdd2aa367256c22b94276cbc046d1cf56379035fb13a9dd00";

        // Call the function and expect an error
        let result = get_tx_data(chain_id, tx_hash, UrlProvider::Infura).await;

        // Verify that the result is an error
        assert!(result.is_err());

        // Optionally, check the specific error message or type

        assert_eq!(
            format!("{}", result.err().unwrap()),
            "Unsupported chain ID: 2",
            "Invalid chain ID test failed"
        );
    }

    #[tokio::test]
    async fn test_get_tx_data_invalid_tx_hash() {
        // Valid chain ID but invalid transaction hash
        let chain_id = 0xaa36a7;
        let tx_hash = "invalid_tx_hash";

        // Call the function and expect an error
        let result = get_tx_data(chain_id, tx_hash, UrlProvider::Infura).await;

        // Verify that the result is an error
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.err().unwrap()),
            FAILED_TO_PARSE_TRANSACTION_HASH,
            "Invalid transaction hash test failed"
        );
    }
}
