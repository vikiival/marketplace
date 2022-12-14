// Copyright (c) 2022 Astar Network
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::impls::marketplace::types::{
    Data,
    MarketplaceError,
};
pub use crate::traits::marketplace::MarketplaceSale;
use ink_env::Hash;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::*,
    },
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
        String,
    },
};

impl<T> MarketplaceSale for T
where
    T: Storage<Data> + Storage<ownable::Data>,
{
    default fn factory(&mut self, hash: Hash, ipfs: String) -> Result<(), MarketplaceError> {
        Ok(())
    }

    default fn list(
        &mut self,
        contract_address: AccountId,
        token_id: Id,
        price: Balance,
    ) -> Result<(), MarketplaceError> {
        let caller = Self::env().caller();
        if let Some(token_owner) = PSP34Ref::owner_of(&contract_address, token_id.clone()) {
            if caller != token_owner {
                return Err(MarketplaceError::NotOwner)
            }
        } else {
            return Err(MarketplaceError::ItemNotFound)
        }
        
        self.data::<Data>().items.insert(&(contract_address, token_id), &price);
        Ok(())
    }

    default fn unlist(
        &mut self,
        contract_address: AccountId,
        token_id: Id,
    ) -> Result<(), MarketplaceError> {
        Ok(())
    }

    default fn buy(
        &mut self,
        contract_address: AccountId,
        token_id: u64,
    ) -> Result<(), MarketplaceError> {
        Ok(())
    }

    default fn register(&mut self, contract_address: AccountId) -> Result<(), MarketplaceError> {
        Ok(())
    }

    // TODO return owner check
    // #[modifiers(only_owner)]
    default fn set_marketplace_fee(&mut self, fee: u16) -> Result<(), MarketplaceError> {
        // TODO check max fee
        self.data::<Data>().fee = fee;
        Ok(())
    }

    default fn get_marketplace_fee(&self) -> u16 {
        self.data::<Data>().fee
    }

    default fn is_listed(&self, contract_address: AccountId, token_id: Id) -> Option<u16> {
        None
    }

    default fn set_contract_metadata(&mut self, ipfs: String) -> Result<(), MarketplaceError> {
        Ok(())
    }

    // fn check_owner(
    //     &mut self,
    //     contract_address: AccountId,
    //     token_id: Id,
    // ) -> Result<(), MarketplaceError> {
    //     let caller = Self::env().caller();
    //     if let Some(token_owner) = PSP34Ref::owner_of(&contract_address, token_id.clone()) {
    //         if caller != token_owner {
    //             return Err(MarketplaceError::NotOwner)
    //         }
    //     } else {
    //         return Err(MarketplaceError::ItemNotFound)
    //     }
        
    //     Ok(())
    // }
}
