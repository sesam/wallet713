use std::sync::Arc;

use grin_util::Mutex;
use grin_keychain::Keychain;
use grin_wallet::libwallet::types::{NodeClient, WalletBackend};
use grin_wallet::libwallet::Error;

use super::api::{Wallet713ForeignAPI, Wallet713OwnerAPI};

pub fn owner_single_use<F, T: ?Sized, C, K>(wallet: Arc<Mutex<T>>, f: F) -> Result<(), Error>
    where
        T: WalletBackend<C, K>,
        F: FnOnce(&mut Wallet713OwnerAPI<T, C, K>) -> Result<(), Error>,
        C: NodeClient,
        K: Keychain,
{
    f(&mut Wallet713OwnerAPI::new(wallet.clone()))?;
    Ok(())
}

pub fn foreign_single_use<F, T: ?Sized, C, K>(wallet: Arc<Mutex<T>>, f: F) -> Result<(), Error>
    where
        T: WalletBackend<C, K>,
        F: FnOnce(&mut Wallet713ForeignAPI<T, C, K>) -> Result<(), Error>,
        C: NodeClient,
        K: Keychain,
{
    f(&mut Wallet713ForeignAPI::new(wallet.clone()))?;
    Ok(())
}
