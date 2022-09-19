// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::unused_unit)]

use crate::{schema::token_datas, util::hash_str};
use anyhow::Context;
use aptos_api_types::WriteTableItem as APIWriteTableItem;
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

use super::token_utils::TokenWriteSet;

#[derive(Debug, Deserialize, FieldCount, Identifiable, Insertable, Queryable, Serialize)]
#[primary_key(creator_address, collection_name_hash, name_hash, transaction_version)]
#[diesel(table_name = "token_datas")]
pub struct TokenData {
    pub creator_address: String,
    pub collection_name_hash: String,
    pub name_hash: String,
    pub collection_name: String,
    pub name: String,
    pub transaction_version: i64,
    pub maximum: bigdecimal::BigDecimal,
    pub supply: bigdecimal::BigDecimal,
    pub largest_property_version: bigdecimal::BigDecimal,
    pub metadata_uri: String,
    pub payee_address: String,
    pub royalty_points_numerator: bigdecimal::BigDecimal,
    pub royalty_points_denominator: bigdecimal::BigDecimal,
    pub maximum_mutable: bool,
    pub uri_mutable: bool,
    pub description_mutable: bool,
    pub properties_mutable: bool,
    pub royalty_mutable: bool,
    pub default_properties: serde_json::Value,
    // Default time columns
    pub inserted_at: chrono::NaiveDateTime,
}

impl TokenData {
    pub fn from_write_table_item(
        table_item: &APIWriteTableItem,
        txn_version: i64,
    ) -> anyhow::Result<Option<Self>> {
        let table_item_data = table_item.data.as_ref().unwrap();

        let maybe_token_data = match TokenWriteSet::from_table_item_type(
            table_item_data.value_type.as_str(),
            &table_item_data.value,
            txn_version,
        )? {
            Some(TokenWriteSet::TokenData(inner)) => Some(inner),
            _ => None,
        };

        if let Some(token_data) = maybe_token_data {
            let token_data_id = match TokenWriteSet::from_table_item_type(
                table_item_data.key_type.as_str(),
                &table_item_data.key,
                txn_version,
            )? {
                Some(TokenWriteSet::TokenDataId(inner)) => Some(inner),
                _ => None,
            }
            .context(format!(
                "Could not get token data id from table item key_type: {}, key: {:?} version: {}",
                table_item_data.key_type, table_item_data.key, txn_version
            ))?;
            let collection_name_hash = hash_str(&token_data_id.collection);
            let name_hash = hash_str(&token_data_id.name);
            Ok(Some(Self {
                creator_address: token_data_id.creator,
                collection_name_hash,
                name_hash,
                collection_name: token_data_id.collection,
                name: token_data_id.name,
                transaction_version: txn_version,
                maximum: token_data.maximum,
                supply: token_data.supply,
                largest_property_version: token_data.largest_property_version,
                metadata_uri: token_data.uri,
                payee_address: token_data.royalty.payee_address,
                royalty_points_numerator: token_data.royalty.royalty_points_numerator,
                royalty_points_denominator: token_data.royalty.royalty_points_denominator,
                maximum_mutable: token_data.mutability_config.maximum,
                uri_mutable: token_data.mutability_config.uri,
                description_mutable: token_data.mutability_config.description,
                properties_mutable: token_data.mutability_config.properties,
                royalty_mutable: token_data.mutability_config.royalty,
                default_properties: token_data.default_properties,
                inserted_at: chrono::Utc::now().naive_utc(),
            }))
        } else {
            Ok(None)
        }
    }
}
