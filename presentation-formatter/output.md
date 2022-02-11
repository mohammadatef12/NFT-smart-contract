# NFT Smart contract
## Variables and Mutability
In rust language, variables by default are immutable, that means if you didn't put the word (mut) before the name of the variable, it will be read only variable and you cannot manipulate its value 
```rust=
fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }
```
the value of the builder is initialized with default, and then a value is assigned to it, so we had to put the word (mut) before the name of the variable (build)

----

```rust=
let token_id = "0".to_string();

```
the value of the variable token_id cannot be changed as it is not mut variable

----

## Constants
```rust=
const MINT_STORAGE_COST: u128 = 5870000000000000000000;
```
there is a difference between the not mutable variable and a const,the main differences are that  the const cannot be mutable, and the data type must be annotated, and its value must be a const expression, to know more about variables and constants check this <a href="https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html">link!</a>

----

## Functions
```rust=
fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}
```
this is how we define functions in rust language, to know more about functions in rust check this <a href="https://doc.rust-lang.org/book/ch03-03-how-functions-work.html">link!</a>

----

## Ownership
Ownership is a set of rules that governs how a Rust program manages memory. 
```rust=
let token_id = "0".to_string();
contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());
// alice approves bob
testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
contract.nft_approve(token_id.clone(), accounts(1), None);
```
In Rust, some simple types are "implicitly copyable" and when you assign them or pass them as arguments, the receiver will get a copy, leaving the original value in place. These types do not require allocation to copy and do not have finalizers (i.e. they do not contain owned boxes or implement Drop), so the compiler considers them cheap and safe to copy. For other types copies must be made explicitly, by convention implementing the Clone trait and calling the clone method, so in the above example, if we didn't clone the variable, we cannot use the token id again in the contract nft approve method, as it will be consumed in the contract nft mint method, if you want to know more about the ownership and how th rust is dealing with memory check this <a href="https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html">link!</a>

----

## References and Borrowing
Ownership is a set of rules that governs how a Rust program manages memory. 
```rust=
pub fn nft_mint(
    &mut self,
    token_id: TokenId,
    token_owner_id: AccountId,
    token_metadata: TokenMetadata,
) -> Token {
    assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");
    self.tokens.internal_mint(token_id, token_owner_id, Some(token_metadata))
}
```
the first parameter here is &self, if we'd like to access data without taking ownership over it. To accomplish this, Rust uses a borrowing mechanism. Instead of passing objects by value (T), objects can be passed by reference (&T).

The compiler statically guarantees (via its borrow checker) that references always point to valid objects. That is, while references to an object exist, the object cannot be destroyed. for more details check this <a href="https://doc.rust-lang.org/rust-by-example/scope/borrow.html">link!</a>

----

## Structs
A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group. 
```rust=
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}
```
here we define our custom data type which includes two variables, Token from the NonFungibleToken data type, and metadata from LazyOption<NFTContractMetadata> data type, to know more about structs in rust check this <a href="https://doc.rust-lang.org/book/ch05-00-structs.html">link!</a>

----

## Enumerations
A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group. 
```rust=
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}
```
An enumeration, also referred to as an enum, is a simultaneous definition of a nominal enumerated type as well as a set of constructors, that can be used to create or pattern-match values of the corresponding enumerated type. to know more about enums in rust check this  <a href="https://doc.rust-lang.org/book/ch06-00-enums.html">link!</a>

----

## Modules
Modules let us organize code within a crate into groups for readability and easy reuse.
```rust=
mod tests {
	..
	..
}
```
TA module item is a module, surrounded in braces, named, and prefixed with the keyword mod. A module item introduces a new, named module into the tree of modules making up a crate. Modules can nest arbitrarily. <a href="https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html">link!</a>

----

## Paths
Modules let us organize code within a crate into groups for readability and easy reuse.
```rust=
let mut contract = Contract::new_default_meta(accounts(0).into());
env::account_balance()
```
To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. If we want to call a function, we need to know its path.

. to know more about paths in rust check this  <a href="https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html">link!</a>

----

## Separating Modules into Different Files
to implement the NFT contract, we didn't only use the methods defined in this code, but we used other methods from other crates (libraries).
```rust=
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
```
so we can know from these line above, we can know that inside near_contract_standards there is non_fungible_token, and inside it there is NonFungibleToken. to know more about separating our code check this <a href="https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html">link!</a>

----

## Bringing paths into scope
```rust=
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
```
You can bring an item from other scopes into scope with use and a relative path. to learn more about bringing paths check this <a href="https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html">link!</a>

----

## Unrecoverable Errors with panic!
```rust=
if let Some(token) = contract.nft_token(token_id.clone()) {
            assert_eq!(token.token_id, token_id);
            assert_eq!(token.owner_id, accounts(1));
            assert_eq!(token.metadata.unwrap(), sample_token_metadata());
            assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
        } else {
            panic!("token not correctly created, or not found by nft_token");
        }
```
Sometimes, bad things happen in your code, and there’s nothing you can do about it. In these cases, Rust has the panic! macro. When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit. to learn more about unrecoverable errors check this <a href="https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html">link!</a>

----

## Recoverable Errors with Result
```rust=
fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
```
Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to. For example, if you try to open a file and that operation fails because the file does not exist, you might want to create the file instead of terminating the process. we can know that the function used here (get) the return type of it is result, as we used the method unwrap() with it, the unwrap method Give me the result of the computation, and if there was an error, panic and stop the program, so we decided here to act as (panic) in the case of failure, as the program demands so, but we had the option to do whatever we want in the case of Error. to learn more about recoverable errors check this <a href="https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html">link!</a>

----

## Tests
What are macros in Rust?
Macros enable you to write code that writes other code, which is known as metaprogramming
to learn more about macros check this <a https://doc.rust-lang.org/book/ch19-06-macros.html">link!</a>

----

```rust=
assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");

```
Asserts that two expressions are equal to each other (using PartialEq).

On panic, this macro will print the values of the expressions with their debug representations.

----

```rust=
#[near_bindgen]
```
The #[near_bindgen] macro is used on a struct and the function implementations to generate the necessary code to be a valid NEAR contract and expose the intended functions to be able to be called externally.

----

```rust=
#[init]

```
Mark a function to run before main

----

```rust=
#[payable]

```
Payable methods. We can allow methods to accept token transfer together with the function call. This is done so that contracts can define a fee in tokens that needs to be payed when they are used. By the default the methods are not payable and they will panic if someone will attempt to transfer tokens to them during the invocation. This is done for safety reason, in case someone accidentally transfers tokens during the function call.

----

```rust=
#[cfg(all(test, not(target_arch = "wasm32")))]

```
this line disables all test if the target architecture is wasm32

----

```rust=
#[should_panic(expected = "The contract is not initialized")]

```
adding the macro should panic, This test will now succeed if we panic! and fail if we complete, and the message after expect will appear in the debug info

----

