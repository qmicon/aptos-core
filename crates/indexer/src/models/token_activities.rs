// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::unused_unit)]

use crate::{
    schema::token_activities as token_activitys,
    util::{hash_str, u64_to_bigdecimal},
};
use anyhow::Context;
use aptos_api_types::Event as APIEvent;
use bigdecimal::BigDecimal;
use diesel_derive_enum::DbEnum;
use field_count::FieldCount;
use serde::Serialize;

enum TokenActivityType {
    Mint,
    Claim,
    Offer,
    Transfer,
    CancelOffer,
    List,
    Buy,
}

#[derive(Debug, Deserialize, FieldCount, Identifiable, Insertable, Queryable, Serialize)]
#[primary_key(creator_address, collection_name_hash, name_hash, transaction_version)]
#[diesel(table_name = "token_activities")]
pub struct TokenActivity {
    pub creator_address: String,
    pub collection_name_hash: String,
    pub name_hash: String,
    pub property_version: bigdecimal::BigDecimal,
    pub event_key: String,
    pub event_sequence_number: i64,
    pub collection_name: String,
    pub name: String,
    pub transaction_version: i64,
    pub transfer_type: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: bigdecimal::BigDecimal,
    // Default time columns
    pub inserted_at: chrono::NaiveDateTime,
}

impl TokenActivity {
    pub fn from_event(
        event: &APIEvent,
        txn_version: i64,
    ) -> anyhow::Result<Option<Self>> {
        let event.type 
        let key = &table_item_data.key;
        let value = &table_item_data.value;
        let token_data_id = Self::get_token_data_id_from_table_item_key(key, txn_version)?;

        let collection_name_hash = hash_str(&token_data_id.collection_name);
        let name_hash = hash_str(&token_data_id.name);

        Ok(Some(Self {
            creator_address: token_data_id.creator_address,
            collection_name_hash,
            name_hash,
            collection_name: token_data_id.collection_name,
            name: token_data_id.name,
            transaction_version: txn_version,
            maximum: value["maximum"]
                .as_str()
                .map(|s| -> anyhow::Result<BigDecimal> { Ok(u64_to_bigdecimal(s.parse::<u64>()?)) })
                .context(format!(
                    "version {} failed! maximum missing from token data {:?}",
                    txn_version, value
                ))?
                .context(format!(
                    "version {} failed! failed to parse maximum {:?}",
                    txn_version, value["maximum"]
                ))?,
            supply: value["supply"]
                .as_str()
                .map(|s| -> anyhow::Result<BigDecimal> { Ok(u64_to_bigdecimal(s.parse::<u64>()?)) })
                .context(format!(
                    "version {} failed! supply missing from token data {:?}",
                    txn_version, value
                ))?
                .context(format!(
                    "version {} failed! failed to parse supply {:?}",
                    txn_version, value["maximum"]
                ))?,
            largest_property_version: value["largest_property_version"]
                .as_str()
                .map(|s| -> anyhow::Result<BigDecimal> { Ok(u64_to_bigdecimal(s.parse::<u64>()?)) })
                .context(format!(
                    "version {} failed! largest_property_version missing from token data {:?}",
                    txn_version, value
                ))?
                .context(format!(
                    "version {} failed! failed to parse largest_property_version {:?}",
                    txn_version, value["maximum"]
                ))?,
            metadata_uri: value["uri"]
                .as_str()
                .map(|s| s.to_string())
                .context(format!(
                    "version {} failed! uri missing from token data {:?}",
                    txn_version, value
                ))?,
            payee_address: value["royalty"]["payee_address"]
                .as_str()
                .map(|s| s.to_string())
                .context(format!(
                    "version {} failed! royalty.payee_address missing {:?}",
                    txn_version, value
                ))?,
            royalty_points_numerator: value["royalty"]["royalty_points_numerator"]
                .as_str()
                .map(|s| -> anyhow::Result<BigDecimal> { Ok(u64_to_bigdecimal(s.parse::<u64>()?)) })
                .context(format!(
                    "version {} failed! royalty.royalty_points_numerator missing {:?}",
                    txn_version, value
                ))?
                .context(format!(
                    "version {} failed! failed to parse royalty_points_numerator {:?}",
                    txn_version, value["royalty"]["royalty_points_numerator"]
                ))?,
            royalty_points_denominator: value["royalty"]["royalty_points_denominator"]
                .as_str()
                .map(|s| -> anyhow::Result<BigDecimal> { Ok(u64_to_bigdecimal(s.parse::<u64>()?)) })
                .context(format!(
                    "version {} failed! royalty.royalty_points_denominator missing {:?}",
                    txn_version, value
                ))?
                .context(format!(
                    "version {} failed! failed to parse royalty_points_denominator {:?}",
                    txn_version, value["royalty"]["royalty_points_denominator"]
                ))?,
            maximum_mutable: value["mutability_config"]["maximum"]
                .as_bool()
                .context(format!(
                    "version {} failed! mutability_config.maximum missing {:?}",
                    txn_version, value
                ))?,
            uri_mutable: value["mutability_config"]["uri"]
                .as_bool()
                .context(format!(
                    "version {} failed! mutability_config.uri missing {:?}",
                    txn_version, value
                ))?,
            description_mutable: value["mutability_config"]["description"]
                .as_bool()
                .context(format!(
                    "version {} failed! mutability_config.description missing {:?}",
                    txn_version, value
                ))?,
            properties_mutable: value["mutability_config"]["properties"].as_bool().context(
                format!(
                    "version {} failed! mutability_config.properties missing {:?}",
                    txn_version, value
                ),
            )?,
            royalty_mutable: value["mutability_config"]["royalty"]
                .as_bool()
                .context(format!(
                    "version {} failed! mutability_config.royalty missing {:?}",
                    txn_version, value
                ))?,
            default_properties: value["default_properties"].clone(),
            inserted_at: chrono::Utc::now().naive_utc(),
        }))
    }
}

impl TokenActivityType {
    fn as_str(&self) -> &'static str {
        match self {
            TokenActivityType::Mint => "mint",
            TokenActivityType::Offer => "offer",
            TokenActivityType::CancelOffer => "cancel_offer",
            TokenActivityType::Claim => "claim",
            TokenActivityType::Transfer => "transfer",
            TokenActivityType::List => "list",
            TokenActivityType::Buy => "buy",
        }
    }
}

