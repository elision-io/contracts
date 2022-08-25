
# Elision Options Protocol
`Elision Options Protocol` (EOP) is a decentralized options trading platform that provides users with access to various trading strategies. The platform is built on the Near blockchain.

## Peer-to-Pool Model
The `peer-to-pool` model enables the liquidity pool to assume the counterparty risk. Pools possess the following abilities in a non-custodial fashion:

* Secure liquidity as underlying assets to write options
* Sell options contracts to buyers
* Aggregate and allocate premiums to the liquidity pool providers
* Settle option contracts on-chain

Users deposit an underlying asset into a liquidity pool and issue options contracts backed by the provded assets. This allows the users to collectively act as a seller of the options contract without owning a large amount of capital. Think of the option contract as being fractionally owned by the liquidity providers of the underlying asset.

Buyer of the option contracts send the required price and premium in order to mint a new option contract. The pool receives the necessary amount from the buyer and then sends a newly minted [Non-Fungible Token](https://github.com/near/NEPs/blob/master/neps/nep-0171.md). The token will have an expiration published in its [metadata](https://github.com/near/NEPs/blob/master/neps/nep-0177.md). Other core functionality will be built into the token functionality e.g. exercising the token for the asset.

## Derivative Calculations

### **Black-Scholes Pricing Model (BSM)**
The Black-Scholes Model is a differential equation used to price options contracts. The model makes the following assumptions:
* No dividends are paid out during the life of the option.
* Markets are random (i.e., market movements cannot be predicted).
* There are no transaction costs in buying the option.
* The risk-free rate and volatility of the underlying asset are known and constant.
* The returns of the underlying asset are normally distributed.
* The option is European and can only be exercised at expiration.


**Mathematical Notation**

$${C=SN(d1)− Ke^{-rt}N(d_2)} $$

**where**
$$ d_1 = {{ln_k^s + (r + {\sigma_v^2 \over 2}t)} \over \sigma_s \sqrt t} $$

**and**
$$ d_2 = d_1 - \sigma_s \sqrt t $$

**and where:**
* $C$ = Call option price
* $S$ = Underlying asset price
* $K$ = Strike price
* $r$ = Time to maturity
* $N$ = Normal Distribution


### **Binomial Option Pricing Model (BOPM)**
Binomial Option Pricing Model is a pricing model that is utilized in the calculation of American options.
This model takes an iterative approach with the result of moving up or moving down a [binomial tree](https://www.investopedia.com/terms/b/binomial_tree.asp).

Assumptions
* At every point in time, the price can go to only two possible ways - up or down
* The underlying asset pays no dividends
* The interest rate (discount factor) is a constant throughout the period
* Investors are risk-neutral, indifferent to risk; 
* The risk-free rate remains constant.


If we set the current (spot) price of an option as $s$, then the price can either go up to $s+$ or down to $s-$.
$$ u = {s+ \over s} $$
$$ d = {s- \over s} $$

**Call Option Contracts**

Call options entitle the holder to purchase the underlying asset at exercise price $(Px)$.

A call option is "in-the-money" when the spot price is above the exercise price $(S > Px)$

When upward price movement occurs, the payoff of the call option is max value between zero and the spot price, multiplied by the up factor and reduced with the exercise price.
$$ C+ = max(O,uS - Px) $$
$$ C- = max(O,dS - Px) $$
    
**Put Option Contracts**
A put option entitles the holder to sell at the exercise price Px.

When the price goes down or up, we calculate a put option like below
$$ P+ = max(Px - uS,O) $$
$$ P- = max(Px - dS,O) $$

### Collateralization Ratio (CR)
This is the ratio of the value of the collateral to the value of the asset being collaterlized.
The Minimum Collateralization Ratio (MCR) is the minimum required value by the Collateralization Ratio.

$$ CR = C * C^{rc} / A * A^rc $$

Where:

* $C$    = Units of collateral assets
* $A$    = Units of underlying assets
* $C^{rc}$ = Price of a single collateral asset unit in fiat currency (USD)
* $A^{rc}$ = Price of a single underlying asset unit in fiat currency (USD)


## Elision Token Model ($ELX)
Liquidity providers be distributed rewards based on the proportion of the liquidity provided to the total liquidity of the pool.

The holders of the option contracts will recieve rewards for posessing the options and utilizing the protocol.

### **Staking Syndicate**
Elision allows users to stake a speicific amount of ELX tokens in order to become a part of the *Elision Syndicate*. 

Each *Syndic* will recieve distributed rewards from the 1% settlment fee when paid out in the underlying asset.

These methods enable the users of the protocol to become the owners of the protocol since both the liquidity providers, contract holders, and syndic members all contribute to the stability of the Elision Protocol.

## Terms
* **Put Option:** Contract that gives the buyer the right to sell the underlying asset at a certain price during a fixed, speicifed duration Obligates the seller to buy assets within the time period.
* **Call Option:** Contract that gives the buyer the right to buy the underlying asset at a certain price for during a fixed, specified duration. Obligates the writer to sell an asset within the time period.
* **Buyer:** Purchaser and holder of an options contract.
* **Seller:** Individual(s) that write and sell the option contract. Most likely multiple liquidity providers fractionally own the contract.
* **Amount:** The size of the options contract e.g. 1 ETH.
* **Price:** The current market price of the asset.
* **Strike:** The exercise price of the option contract.
* **Period:** Time period where the option contract is valid.
* **Rate:** Predifined percentage needed to open a options contract; impacts the premium.
* **Premium:** Fee paid by the buyer of an option contract for the right to buy or sell the underlying asset at a speicific price within the time period of the contract.
* **Counterparty Risk:** Risk that the seller will not sell or buy the underlying asset as agreed; circumvented with the peer-to-pool model.
* **Asset Availability:** Amount of funds available for holding and executing option contracts.
* **Expiration:** The block or epoch that invalidates the option contract.
* **Intrinsic Value:** Difference between underlying asset price and stike price.
* **Time Value:** Difference between the contract's premium and intrinsic value.
* **Settlement Fee:** Cost of executing an options contract that is paid in advance by the option buyer.
* **Elision Syndicate:** Group consisting of users that have staked a specified amount of the Elision Token ($ELX). In return, the 1% Settlement Fee is distributed to the users within the Syndicate.
* **Syndic:** Individual member of the Elision Syndicate
* **Strategy:** Specific type of trades that are used to target a particular amount of risk in order to profit from the price movements of the underlying asset.
* **Implied Volatility:** Market's forecast of the likely movement of a particular asset. This variable is used to price option's contracts.
