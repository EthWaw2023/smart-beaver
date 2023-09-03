#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod psp22 {
    use ink::storage::Mapping;
    use ink::prelude::{
        vec,
        vec::Vec,
        string::String,
    };

    /// The PSP22 error type. Contract will throw one of this errors.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Custom error type for cases in which an implementation adds its own restrictions.
        Custom(String),
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
        /// Returned if recipient's address is zero.
        ZeroRecipientAddress,
        /// Returned if sender's address is zero.
        ZeroSenderAddress,
        /// Returned if a safe transfer check fails (e.g. if the receiving contract does not accept tokens).
        SafeTransferCheckFailed(String),
    }

    pub type PSP22Result<T> = core::result::Result<T, Error>;

    #[ink::trait_definition]
    pub trait Psp22Base {
        /// Returns the total token supply.
        #[ink(message)]
        fn total_supply(&self) -> Balance;

        /// Returns the account Balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance;

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set `0`.
        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

        /// Transfers `value` amount of tokens from the caller's account to account `to`
        /// with additional `data` in unspecified format.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account Balance.
        ///
        /// Returns `ZeroSenderAddress` error if sender's address is zero.
        ///
        /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> PSP22Result<()>;

        /// Transfers `value` tokens on the behalf of `from` to the account `to`
        /// with additional `data` in unspecified format.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        ///
        /// On success a `Transfer` and `Approval` events are emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// for the caller to withdraw from `from`.
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the the account Balance of `from`.
        ///
        /// Returns `ZeroSenderAddress` error if sender's address is zero.
        ///
        /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> PSP22Result<()>;

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with `value`.
        ///
        /// An `Approval` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `ZeroSenderAddress` error if sender's address is zero.
        ///
        /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> PSP22Result<()>;

        /// Atomically increases the allowance granted to `spender` by the caller.
        ///
        /// An `Approval` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `ZeroSenderAddress` error if sender's address is zero.
        ///
        /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
        #[ink(message)]
        fn increase_allowance(
            &mut self,
            spender: AccountId,
            delta_value: Balance,
        ) -> PSP22Result<()>;

        /// Atomically decreases the allowance granted to `spender` by the caller.
        ///
        /// An `Approval` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// by owner for `spender`.
        ///
        /// Returns `ZeroSenderAddress` error if sender's address is zero.
        ///
        /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
        #[ink(message)]
        fn decrease_allowance(
            &mut self,
            spender: AccountId,
            delta_value: Balance,
        ) -> PSP22Result<()>;
    }
    pub trait Psp22Internal {
        /// User must override those methods in their contract.
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _amount: Balance,
        );

        fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance);

        fn _total_supply(&self) -> Balance;

        fn _balance_of(&self, owner: &AccountId) -> Balance;

        fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance;

        fn _transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> PSP22Result<()>;

        fn _approve_from_to(
            &mut self,
            owner: AccountId,
            spender: AccountId,
            amount: Balance,
        ) -> PSP22Result<()>;

        fn _mint_to(&mut self, account: AccountId, amount: Balance) -> PSP22Result<()>;

        fn _burn_from(&mut self, account: AccountId, amount: Balance) -> PSP22Result<()>;

        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> PSP22Result<()>;

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> PSP22Result<()>;
    }
    #[ink::trait_definition]
    pub trait Psp22Burnable {
        //Burns `amount` of tokens from `account`
        #[ink(message)]
        #[cfg(feature = "pausable")]
        fn burn(&mut self, account: AccountId, amount: Balance) -> PSP22Result<()>;
    }
    /*&&%*/
    #[ink::trait_definition]
    pub trait Psp22Mintable {
        //Mints tokens to the given account
        #[ink(message)]
        #[cfg(feature = "mintable")]
        fn mint(&mut self, account: AccountId, amount: Balance) -> PSP22Result<()>;
    }
    /*&&%*/
    #[ink::trait_definition]
    pub trait Psp22Pausable {
        //Toggle token pausable state
        #[ink(message)]
        #[cfg(feature = "pausable")]
        fn toggle_is_paused(&mut self) -> PSP22Result<()>;
    }
    /*&&%*/
    #[ink::trait_definition]
    pub trait Psp22Metadata {
        #[ink(message)]
        #[cfg(feature = "metadata")]
        fn token_name(&self) -> Option<String>;
        #[ink(message)]
        #[cfg(feature = "metadata")]
        fn token_symbol(&self) -> Option<String>;
        #[ink(message)]
        #[cfg(feature = "metadata")]
        fn token_decimals(&self) -> u8;
    }
    /*&&%*/
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Psp22 {
        /// Total token supply.
        total_supply: Balance,
        /// Mapping from owner to number of owned token.
        balances: Mapping<AccountId, Balance>,
        /// Mapping of the token amount which an account is allowed to withdraw
        /// from another account.
        allowances: Mapping<(AccountId, AccountId), Balance>,

        #[cfg(feature = "modifier")]
        owner: AccountId,
        /*&&%*/
        #[cfg(feature = "pausable")]
        is_paused: bool,
        /*&&%*/
        #[cfg(feature = "capped")]
        cap: u128,
        /*&&%*/
        #[cfg(feature = "metadata")]
        pub name: Option<String>,
        #[cfg(feature = "metadata")]
        pub symbol: Option<String>,
        #[cfg(feature = "metadata")]
        pub decimals: u8,
        /*&&%*/
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl Psp22 {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(
            total_supply: Balance,
            cap: u128,
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8,
        ) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });
            Self {
                total_supply,
                balances,
                allowances: Default::default(),
                #[cfg(feature = "modifier")]
                owner: caller,
                /*&&%*/
                #[cfg(feature = "pausable")]
                is_paused: false,
                /*&&%*/
                #[cfg(feature = "capped")]
                cap: cap,
                /*&&%*/
                #[cfg(feature = "metadata")]
                name: name,
                #[cfg(feature = "metadata")]
                symbol: symbol,
                #[cfg(feature = "metadata")]
                decimals: decimals,
                /*&&%*/
            }
        }
    }
    impl Psp22Burnable for Psp22 {
        #[ink(message)]
        #[cfg(feature = "pausable")]
        fn burn(&mut self, account: AccountId, amount: Balance) -> PSP22Result<()> {
            #[cfg(feature = "pausable")]
            assert!(!self.is_paused, "Token is paused");
            /*&&%*/
            assert_eq!(Self::env().caller(), self.owner, "Only owner can burn");
            Psp22Internal::_burn_from(self, account, amount)
        }
    }
    /*&&%*/
    impl Psp22Mintable for Psp22 {
        #[ink(message)]
        #[cfg(feature = "mintable")]
        fn mint(&mut self, account: AccountId, amount: Balance) -> PSP22Result<()> {
            #[cfg(feature = "pausable")]
            assert!(!self.is_paused, "Token is paused");
            /*&&%*/
            #[cfg(feature = "capped")]
            assert!(self.cap < (self.total_supply + amount), "Max cap exeeded");
            /*&&%*/
            assert_eq!(Self::env().caller(), self.owner, "Only owner can mint");
            Psp22Internal::_mint_to(self, account, amount)
        }
    }
    /*&&%*/
    impl Psp22Pausable for Psp22 {
        #[ink(message)]
        #[cfg(feature = "pausable")]
        fn toggle_is_paused(&mut self) -> PSP22Result<()> {
            assert_eq!(Self::env().caller(), self.owner, "Only owner can mint");
            self.is_paused = !self.is_paused;
            Ok(())
        }
    }
    /*&&%*/

    impl Psp22Metadata for Psp22 {
        #[ink(message)]
        #[cfg(feature = "metadata")]
        fn token_name(&self) -> Option<String> {
            self.name.clone()
        }
        #[ink(message)]
        #[cfg(feature = "metadata")]
        fn token_symbol(&self) -> Option<String> {
            self.symbol.clone()
        }
        #[ink(message)]
        #[cfg(feature = "metadata")]
        fn token_decimals(&self) -> u8 {
            self.decimals
        }
    }
    /*&&%*/

    impl Psp22Internal for Psp22 {
        /// User must override those methods in their contract.
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _amount: Balance,
        ) {
            ink::codegen::EmitEvent::emit_event(
                ink::codegen::Env::env(self),
                Transfer {
                    from: _from,
                    to: _to,
                    value: _amount,
                },
            );
        }

        fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
            ink::codegen::EmitEvent::emit_event(
                ink::codegen::Env::env(self),
                Approval {
                    owner: _owner,
                    spender: _spender,
                    value: _amount,
                },
            );
        }

        fn _total_supply(&self) -> Balance {
            self.total_supply
        }

        fn _balance_of(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        fn _transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> PSP22Result<()> {
            let from_balance = Psp22Internal::_balance_of(self, &from);

            if from_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            Psp22Internal::_before_token_transfer(self, Some(&from), Some(&to), &amount)?;

            self.balances.insert(&from, &(from_balance - amount));

            let to_balance = Psp22Internal::_balance_of(self, &to);
            self.balances.insert(&to, &(to_balance + amount));

            Psp22Internal::_after_token_transfer(self, Some(&from), Some(&to), &amount)?;
            Psp22Internal::_emit_transfer_event(self, Some(from), Some(to), amount);

            Ok(())
        }

        fn _approve_from_to(
            &mut self,
            owner: AccountId,
            spender: AccountId,
            amount: Balance,
        ) -> PSP22Result<()> {
            self.allowances.insert((&owner, &spender), &amount);
            Psp22Internal::_emit_approval_event(self, owner, spender, amount);
            Ok(())
        }

        fn _mint_to(&mut self, account: AccountId, amount: Balance) -> PSP22Result<()> {
            Psp22Internal::_before_token_transfer(self, None, Some(&account), &amount)?;
            let mut new_balance = Psp22Internal::_balance_of(self, &account);
            new_balance += amount;
            self.balances.insert(&account, &new_balance);

            let new_supply = self.total_supply + amount;
            self.total_supply = new_supply;

            Psp22Internal::_after_token_transfer(self, None, Some(&account), &amount)?;
            Psp22Internal::_emit_transfer_event(self, None, Some(account), amount);

            Ok(())
        }

        fn _burn_from(&mut self, account: AccountId, amount: Balance) -> PSP22Result<()> {
            let mut from_balance = Psp22Internal::_balance_of(self, &account);

            if from_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            Psp22Internal::_before_token_transfer(self, Some(&account), None, &amount)?;

            from_balance -= amount;
            self.balances.insert(&account, &from_balance);

            let new_supply = self.total_supply - amount;
            self.total_supply = new_supply;

            Psp22Internal::_after_token_transfer(self, Some(&account), None, &amount)?;
            Psp22Internal::_emit_transfer_event(self, Some(account), None, amount);

            Ok(())
        }

        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> PSP22Result<()> {
            Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> PSP22Result<()> {
            Ok(())
        }
    }

    impl Psp22Base for Psp22 {
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            Psp22Internal::_balance_of(self, &owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            Psp22Internal::_allowance(self, &owner, &spender)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> PSP22Result<()> {
            #[cfg(feature = "pausable")]
            assert!(!self.is_paused, "Token is paused");
            /*&&%*/
            let from = self.env().caller();
            Psp22Internal::_transfer_from_to(self, from, to, value, data)
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`
        /// with additional `data` in unspecified format.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        ///
        /// On success a `Transfer` and `Approval` events are emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// for the caller to withdraw from `from`.
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the the account Balance of `from`.
        ///
        /// Returns `ZeroSenderAddress` error if sender's address is zero.
        ///
        /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> PSP22Result<()> {
            #[cfg(feature = "pausable")]
            assert!(!self.is_paused, "Token is paused");
            /*&&%*/
            let caller = Self::env().caller();
            let allowance = self._allowance(&from, &caller);

            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            Psp22Internal::_approve_from_to(self, from, caller, allowance - value)?;
            Psp22Internal::_transfer_from_to(self, from, to, value, data)?;
            Ok(())
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with `value`.
        ///
        /// An `Approval` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `ZeroSenderAddress` error if sender's address is zero.
        ///
        /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> PSP22Result<()> {
            let from = self.env().caller();
            Psp22Internal::_approve_from_to(self, from, spender, value)
        }

        /// Atomically increases the allowance granted to `spender` by the caller.
        ///
        /// An `Approval` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `ZeroSenderAddress` error if sender's address is zero.
        ///
        /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
        #[ink(message)]
        fn increase_allowance(
            &mut self,
            spender: AccountId,
            delta_value: Balance,
        ) -> PSP22Result<()> {
            let owner = Self::env().caller();
            Psp22Internal::_approve_from_to(
                self,
                owner,
                spender,
                self._allowance(&owner, &spender) + delta_value,
            )
        }

        /// Atomically decreases the allowance granted to `spender` by the caller.
        ///
        /// An `Approval` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// by owner for `spender`.
        ///
        /// Returns `ZeroSenderAddress` error if sender's address is zero.
        ///
        /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
        #[ink(message)]
        fn decrease_allowance(
            &mut self,
            spender: AccountId,
            delta_value: Balance,
        ) -> PSP22Result<()> {
            let owner = Self::env().caller();
            Psp22Internal::_approve_from_to(
                self,
                owner,
                spender,
                self._allowance(&owner, &spender) - delta_value,
            )
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        use ink::primitives::{Clear, Hash};

        type Event = <Psp22 as ::ink::reflect::ContractEventBase>::Type;

        fn assert_transfer_event(
            event: &ink::env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
                assert_eq!(from, expected_from, "encountered invalid Transfer.from");
                assert_eq!(to, expected_to, "encountered invalid Transfer.to");
                assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
            } else {
                panic!("encountered unexpected event kind: expected a Transfer event")
            }
            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"Psp22::Transfer",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Psp22::Transfer::from",
                    value: &expected_from,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Psp22::Transfer::to",
                    value: &expected_to,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Psp22::Transfer::value",
                    value: &expected_value,
                }),
            ];

            let topics = event.topics.clone();
            for (n, (actual_topic, expected_topic)) in
            topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash = Hash::CLEAR_HASH;
                let len = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);

                assert_eq!(
                    topic_hash, expected_topic,
                    "encountered invalid topic at {n}"
                );
            }
        }

        /// The default constructor does its job.
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let _psp22 = Psp22::new(
                100,
                1000,
                Some("name".to_string()),
                Some("name".to_string()),
                18,
            );

            // Transfer event triggered during initial construction.
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());

            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        /// The total supply was applied.
        #[ink::test]
        fn total_supply_works() {
            // Constructor works.
            let psp22 = Psp22::new(
                100,
                1000,
                Some("name".to_string()),
                Some("name".to_string()),
                18,
            );
            // Transfer event triggered during initial construction.
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Get the token total supply.
            assert_eq!(psp22.total_supply, 100);
        }

        /// Get the actual balance of an account.
        #[ink::test]
        fn balance_of_works() {
            // Constructor works
            let psp22 = Psp22::new(
                100,
                1000,
                Some("name".to_string()),
                Some("name".to_string()),
                18,
            );
            // Transfer event triggered during initial construction
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Alice owns all the tokens on contract instantiation
            assert_eq!(psp22.balance_of(accounts.alice), 100);
            // Bob does not owns tokens
            assert_eq!(psp22.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            // Constructor works.
            let mut psp22 = Psp22::new(
                100,
                1000,
                Some("name".to_string()),
                Some("name".to_string()),
                18,
            );
            // Transfer event triggered during initial construction.
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(psp22.balance_of(accounts.bob), 0);
            // Alice transfers 10 tokens to Bob.
            assert_eq!(psp22.transfer(accounts.bob, 10, vec![]), Ok(()));
            // Bob owns 10 tokens.
            assert_eq!(psp22.balance_of(accounts.bob), 10);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            // Check first transfer event related to ERC-20 instantiation.
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Check the second transfer event relating to the actual trasfer.
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            // Constructor works.
            let mut psp22 = Psp22::new(
                100,
                1000,
                Some("name".to_string()),
                Some("name".to_string()),
                18,
            );
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(psp22.balance_of(accounts.bob), 0);

            // Set the contract as callee and Bob as caller.
            let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(contract);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Bob fails to transfers 10 tokens to Eve.
            assert_eq!(
                psp22.transfer(accounts.eve, 10, vec![]),
                Err(Error::InsufficientBalance)
            );
            // Alice owns all the tokens.
            assert_eq!(psp22.balance_of(accounts.alice), 100);
            assert_eq!(psp22.balance_of(accounts.bob), 0);
            assert_eq!(psp22.balance_of(accounts.eve), 0);

            // Transfer event triggered during initial construction.
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        #[ink::test]
        fn transfer_from_works() {
            // Constructor works.
            let mut psp22 = Psp22::new(
                100,
                1000,
                Some("name".to_string()),
                Some("name".to_string()),
                18,
            );
            // Transfer event triggered during initial construction.
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Bob fails to transfer tokens owned by Alice.
            assert_eq!(
                psp22.transfer_from(accounts.alice, accounts.eve, 10, vec![]),
                Err(Error::InsufficientAllowance)
            );
            // Alice approves Bob for token transfers on her behalf.
            assert_eq!(psp22.approve(accounts.bob, 10), Ok(()));

            // The approve event takes place.
            assert_eq!(ink::env::test::recorded_events().count(), 2);

            // Set the contract as callee and Bob as caller.
            let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(contract);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Bob transfers tokens from Alice to Eve.
            assert_eq!(
                psp22.transfer_from(accounts.alice, accounts.eve, 10, vec![]),
                Ok(())
            );
            // Eve owns tokens.
            assert_eq!(psp22.balance_of(accounts.eve), 10);

            // Check all transfer events that happened during the previous calls:
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 4);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // The second event `emitted_events[1]` is an Approve event that we skip
            // checking.
            assert_transfer_event(
                &emitted_events[3],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }

        /**
         * Not working due to this issue
         * https://github.com/paritytech/ink/issues/1778
         */
        //#[ink::test]
        fn allowance_must_not_change_on_failed_transfer() {
            let mut psp22 = Psp22::new(
                100,
                1000,
                Some("name".to_string()),
                Some("name".to_string()),
                18,
            );
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice approves Bob for token transfers on her behalf.
            let alice_balance = psp22.balance_of(accounts.alice);
            let initial_allowance = alice_balance + 2;
            assert_eq!(psp22.approve(accounts.bob, initial_allowance), Ok(()));

            // Get contract address.
            let callee = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(callee);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(
                psp22.allowance(accounts.alice, accounts.bob),
                initial_allowance
            );
            // Bob tries to transfer tokens from Alice to Eve.
            let emitted_events_before = ink::env::test::recorded_events().count();
            assert_eq!(
                psp22.transfer_from(accounts.alice, accounts.eve, alice_balance + 1, vec![]),
                Err(Error::InsufficientBalance)
            );
            // Allowance must have stayed the same
            assert_eq!(
                psp22.allowance(accounts.alice, accounts.bob),
                initial_allowance
            );
            // No more events must have been emitted
            assert_eq!(
                emitted_events_before,
                ink::env::test::recorded_events().count()
            )
        }

        /// For calculating the event topic hash.
        struct PrefixedValue<'a, 'b, T> {
            pub prefix: &'a [u8],
            pub value: &'b T,
        }

        impl<X> scale::Encode for PrefixedValue<'_, '_, X>
            where
                X: scale::Encode,
        {
            #[inline]
            fn size_hint(&self) -> usize {
                self.prefix.size_hint() + self.value.size_hint()
            }

            #[inline]
            fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
                self.prefix.encode_to(dest);
                self.value.encode_to(dest);
            }
        }

        fn encoded_into_hash<T>(entity: &T) -> Hash
            where
                T: scale::Encode,
        {
            use ink::{
                env::hash::{Blake2x256, CryptoHash, HashOutput},
                primitives::Clear,
            };

            let mut result = Hash::CLEAR_HASH;
            let len_result = result.as_ref().len();
            let encoded = entity.encode();
            let len_encoded = encoded.len();
            if len_encoded <= len_result {
                result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                return result;
            }
            let mut hash_output = <<Blake2x256 as HashOutput>::Type as Default>::default();
            <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
            let copy_len = core::cmp::min(hash_output.len(), len_result);
            result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
            result
        }
    }
}