# SMART BEAVER
Are you tired of using OpenBrush? Are you frustrated with their custom library, lack of updates, and limited features?

Do you dream of having a wizard that can easily configure and deploy contracts written in **clean ink!**?

Welcome to Smart Beaver! We specialize in building contracts as **sturdy as dams.**


## Inspiration

This time, our goal was to work on a project related to architecture and tools. 
When we talked to the Aleph Zero team, we got inspired. 
We asked them if there was a tool to easily set up popular smart contract standards for Polkadot, like the one from OpenZeppelin. 
They mentioned OpenBrush, but it relies on their library. 
We all agreed that creating a simple wizard to make smart contracts in clean Ink! would be a great idea with lots of potential.

## Challenges we ran into

Our main challenge was coming up with an idea for allowing users to configure the contract with different extensions in clean Ink! while ensuring they can view and understand the generated code.

We decided against following the OpenBrush approach, as it wouldn't bring any added value to the ecosystem. Instead, we opted to write a single comprehensive contract in clean Ink! and then employ Rust macros to enable or disable extensions based on the users' preferences.

Our next step was to run the preprocessor and present the generated source code to the user. However, we encountered an issue where it expanded all the traits and macros, including those not declared by us. 
This resulted in the source code being less readable and comprehensible. 
To address this problem, we resolved to create our own parser, allowing us to hide and display the smart contract extensions in the source code, providing users with a more understandable representation.

At the same time, all your private keys are safely stored in the wallet, this means that we don’t have access to it. The user is not required to trust us that we won’t steal his private key.
## What we learned
We've gained a lot of knowledge about Ink! and Rust macros, 
and we also have a better understanding of how to write contracts that adhere to the PSP. 
Overall, it was a valuable experience."
## What's next for Smart Beaver

First, we need to clean the contract code, perhaps reconsider the design, and write tests to achieve comprehensive test coverage. Afterward, we can incorporate support for additional standards, such as PSP34.

The ultimate objective is to create a wizard that will prove useful for both experienced Polkadot developers and non-technical individuals seeking a straightforward way to configure and deploy standard contracts.
