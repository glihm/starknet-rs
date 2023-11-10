//! Expands the contract first implementation with
//! default configuration for provider and account, if any.
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use super::utils;

pub struct CairoContract;

impl CairoContract {
    pub fn expand(contract_name: Ident) -> TokenStream2 {
        let reader = utils::str_to_ident(format!("{}Reader", contract_name).as_str());
        let q = quote! {

            #[derive(Debug)]
            pub struct #contract_name<A: starknet::accounts::ConnectedAccount + Sync> {
                pub address: starknet::core::types::FieldElement,
                pub account: A,
            }

            impl<A: starknet::accounts::ConnectedAccount + Sync> #contract_name<A> {
                pub fn new(address: starknet::core::types::FieldElement, account: A) -> Self {
                    Self { address, account }
                }

                pub fn reader(&self) -> #reader<A::Provider> {
                    #reader::new(self.address, self.account.provider())
                }
            }

            #[derive(Debug)]
            pub struct #reader<'a, P: starknet::providers::Provider + Sync> {
                pub address: starknet::core::types::FieldElement,
                pub provider: &'a P,
                call_block_id: starknet::core::types::BlockId,
            }

            impl<'a, P: starknet::providers::Provider + Sync> #reader<'a, P> {
                pub fn new(
                    address: starknet::core::types::FieldElement,
                    provider: &'a P,
                ) -> Self {
                    let call_block_id = starknet::core::types::BlockId::Tag(starknet::core::types::BlockTag::Pending);
                    Self { address, provider, call_block_id }
                }

                pub fn set_call_block_id(&mut self, block_id: starknet::core::types::BlockId) {
                    self.call_block_id = block_id;
                }

                pub fn get_call_block_id(&self) -> starknet::core::types::BlockId {
                    self.call_block_id
                }
            }
        };

        q
    }
}