# Genesis

This document outlines the general architecture and purpose of the `staking-v1` set of programs.

## Introduction

The `staking-v1` set of programs aim to give `Dorse` users the capability to **signal** that a candidate
is a good match for a certain role. Staking on a candidate's application implies risks, which is what
makes the signal reliable. Otherwise, if staking was free, anyone could just stake on any candidate, giving
no valuable insights from the process.

What is staking in this context? It's the process of attaching a sum of capital in the form of a token to
a candidate's application over a period of time. There is a reward linked to staking in the event that
a candidate is successfully placed on the role. Nevertheless, in the even in which the candidate does not
get the role, the staked amount can be redemeed in full by the staker once the role is closed.

The fact that the amount staked is locked for a period of time is what makes the process of staking costly.
There's an opportunity cost associated to locking capital for some time. And on the other hand, staking on
a token (which may or may not be a stablecoin) has some currency risks.

## Design decisions for v1

The first version aims to bring an innovative mechanism to live that can potentially revolutionize the way
people hire and look for a role. Our top prioritity for now is to have a set of smart contracts allowing
this to happen ASAP. By choosing to move fast, there will be some features that will be delayed for next versions.

For the first version, the only token allowed for staking will be USDC. This is rather a arbitrary decision. But it
simplifies our life as smart contract developers while still keeps the core idea of the program. In the future
we will allow multiple tokens to be staked, or at least to be used and converted right away. And in particular, we
are interested in having our own token for this.

## Formulae

**Parameter definitions** (everything is denominated in USDC):

```
w_i: total amount staked right now on application i
W: max amount allowed to stake on an application
R: max reward that will be paid # given as a parameter
a: multiplier for tier 1 # given as a parameter OR adjustable? (validate this! - in case we want to set the APR)
b: multiplier for tier 2 # given as a parameter
c: multiplier for tier 3 # given as a parameter
k: amount of tokens that the user wants to stake on an application
v: amount of vouchers that a user gets in exchange for staking
```

function definitions

```
mul_mapping(x) =
    {
        a if x = 1,
        b if x = 2,
        c if x = 3,
    }
```

```
def num_vouchers(w_i, W, k):
    # calculate and updates amounts to be staked on each tier
    k_tier_1 = 0
    k_tier_2 = 0
    k_tier_3 = 0

    k_tier_1 = min(k, (min(W/3, max(0, W*1/3-w)))
    k -= k_tier_1

    k_tier_2 = min(k, (min(W/3, max(0, W*2/3-w)))
    k -= k_tier_2

    k_tier_3 = min(k, (min(W/3, max(0, W*3/3-w)))
    k -= k_tier_3

    return a * k_tier_1 + b * k_tier_2  + c * k_tier_3
```

set the W

```
# we know that
R = a * W/3 + b * W/3 + c * W/3
R = W / 3 * (a + b + c)
# therefore
W = R * 3 / (a + b + c)
```
