
# Elision
Elision Protocol smart contracts built on Near.

*__Elision Derivatives Protocol__* (EDP) is a derivatives trading platform that enables the decentralized exchange of various financial derivative smart contracts.
The contracts aggregate and secure liquidity, distribute rewards for liquidity providers, and implements the `peer-to-pool` model. 

*__Elision Leveraged Token Protocol__* (ELTP) provides a decentralized solution for creating and exchanging leveraged assets with the use of no-loss lending pools.
These no-loss lending pools provide users with the ability to earn interest in an easy, secure, and transparent way.


## Derivative Calculations
### Binomial Option Pricing Model (BOPM)
Binomial Option Pricing Model is a pricing model that is utilized in the calculation of American options.
This model takes an iterative approach with the result of moving up or moving down a [binomial tree](https://www.investopedia.com/terms/b/binomial_tree.asp).

Assumptions
1. At every point in time, the price can go to only two possible ways - up or down
2. The underlying asset pays no dividends
3. The interest rate (discount factor) is a constant throughout the period
4. Investors are risk-neutral, indifferent to risk; 
5. The risk-free rate remains constant.

```
Understanding the Model
- If we set the current (spot) price of an option as S, then the price can either go up to S+ or down to S-.
    u = s+ / s; 
    d = s- / s;
    
Call Option Contracts 
- Call options entitle the holder to purchase the underlying asset at exercise price (Px)
- A call option is "in-the-money" when the spot price is above the exercise price (S > Px)
- When upward price movement occurs, the payoff of the call option is max value between zero and
  the spot price, multiplied by the up factor and reduced with the exercise price.
    C+ = max(O,uS - Px);
    C- = max(O,dS - Px);
    
Put Option Contracts
- A put option entitles the holder to sell at the exercise price Px.
- When the price goes down or up, we calculate a put option like below
    P+ = max(Px - uS,O)
    P- = max(Px - dS,O)
```

### Collateralization Ratio (CR)
This is the ratio of the value of the collateral to the value of the asset being collaterlized.
The Minimum Collateralization Ratio (MCR) is the minimum required value by the Collateralization Ratio.
```
CR = C * C^rc / A * A^rc

Where:

C    = Units of collateral assets
A    = Units of underlying assets
C^rc = Price of a single collateral asset unit in fiat currency (USD)
A^rc = Price of a single underlying asset unit in fiat currency (USD)

```

### Operational Notes

```
- Locked tokens in a liquidity pool will act as the Market Maker
- Anytime the buyer purchases an option, the contract creates the option.
- If the option is exercized, the underlying assets will come from the LP.
- Option price will involve an oracle for price feed and use Black-Shoels model for calculation
- Premium is the price you pay to purchase the contract which will be recieved by the liquidity pool.

$ELX Token Model
- Liquidity providers be distributed rewards based on the proportion of wNEAR of the liqudity in the pool. 
- The holders of the option contracts will recieve rewards for posessing the options and utilizing the protocol.
- Elision will also enable its "Staking Syndicate". Each Syndic will recieve distributed rewards from the 1% settlment
  fee in $wNEAR for the execution of the option contracts. In order to become a Elision Syndic, a user must stake a
  large amount of $ELX.
- These methods enable the users of the protocol to become the owners of the protocol since both the liquidity providers,
  contract holders, and syndic members.

Notes on Opium
- Mint NFT (NEP-0171) that contains the expiration date, strike price, etc.
- Example Calculation of Settlement (Opium): 
    + Buyer bet price decrease by 15%
    + Seller bet price increase by 15%
    + Prices go down 10% at time of settlment
    + Seller gets 5% (15% - 10%)
    + Buyer gets 25% (15% + 10%)
- Fee Structure
   + 90% goes to the seller / 10% goes into the ecosystem through `PayPremium()` 

Notes on Opyn
- Uses the Uniswap Bonding Curve like MakerDAO, where you have collateral in a liquidity pool or vault. From there you can mint tokens to represent of ownership in a vault; these `oTokens` can be traded accordingly.
- `oTokens` represent the options contract in the system
- The model tends to not as revenue generating due to not much incentives for staking governance rewards


Hegic:
  - Use Case


Token Model
Value Flow
Value Accrual
```

