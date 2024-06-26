use std::future::IntoFuture;

use crate::error::Error::{AddressHasNoOwnerKeys, AddressNotFound};
use crate::error::Result;

use radix_common::network::NetworkDefinition;
use radix_gateway_sdk::generated::model::{
    ResourceAggregationLevel::Vault, StateEntityDetailsOptIns,
};
use radix_gateway_sdk::{Client, Network};

pub struct GatewayService {
    client: Client,
}

impl GatewayService {
    pub fn new(
        network_definition: NetworkDefinition,
        dapp_definition: String,
        application_name: String,
    ) -> Result<Self> {
        Ok(Self {
            client: Client::new(
                Network::from_network_id(network_definition.id).unwrap(),
                Some(application_name),
                Some(dapp_definition),
            )?,
        })
    }

    pub async fn get_entity_owner_keys(&self, address: String) -> Result<String> {
        let opt_ins = StateEntityDetailsOptIns {
            ancestor_identities: None,
            component_royalty_config: None,
            component_royalty_vault_balance: None,
            explicit_metadata: Some(vec!["owner_keys".to_string()]),
            non_fungible_include_nfids: None,
            package_royalty_vault_balance: None,
        };

        self.client
            .get_inner_client()
            .state_entity_details(&[address.as_str()], Vault, opt_ins)
            .into_future()
            .await?
            .items
            .first()
            .ok_or_else(|| AddressNotFound)
            .and_then(|item| {
                item.metadata
                    .items
                    .iter()
                    .find(|item| item.key == "owner_keys")
                    .ok_or_else(|| AddressHasNoOwnerKeys)
                    .map(|item| item.value.raw_hex.clone().to_string())
            })
    }
}
