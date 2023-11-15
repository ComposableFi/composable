import itertools
import numpy as np
import cvxpy as cp
from router import solve,populate_chain_dict


CENTER_NODE = "CENTAURI"  # Name of center Node

ORIGIN_TOKEN = "WETH"
OBJ_TOKEN = "ATOM"

chains: dict[str, list[str]] = {
    "ETHEREUM": ["WETH", "USDC", "SHIBA"],
    CENTER_NODE: [],
    "OSMOSIS": ["ATOM","SCRT"],
}

all_tokens = []
all_cfmms = []
reserves = []
fees = []
cfmm_tx_cost = []
ibc_pools = 0
tol = 1e-4

populate_chain_dict(chains,CENTER_NODE)

# simulate in chain CFMMS
for other_chain, other_tokens in chains.items():
    all_tokens.extend(other_tokens)
    all_cfmms.extend(itertools.combinations(other_tokens, 2))

# simulate reserves and gas costs to CFMMS
for cfmm in all_cfmms:
    reserves.append(np.random.uniform(9500, 10051, 2))
    cfmm_tx_cost.append(np.random.uniform(0, 20))

# simulate IBC "pools"
for token_on_center in chains[CENTER_NODE]:
    for other_chain, other_tokens in chains.items():
        if other_chain != CENTER_NODE:
            for other_token in other_tokens:
                # Check wether the chain has the token in centuri, or the other way around
                # Could cause problems if chainName == tokensName (for example OSMOSIS)
                if other_token in token_on_center or token_on_center in other_token:
                    all_cfmms.append((token_on_center, other_token))
                    reserves.append(np.random.uniform(10000, 11000, 2))
                    cfmm_tx_cost.append(np.random.uniform(0, 20))
                    ibc_pools += 1

# simulate random fees
fees.extend(np.random.uniform(0.97, 0.999) for _ in range(len(all_cfmms)))

print(chains)

for i, token in enumerate(all_tokens):
    print(i, token)

for i, cfmm in enumerate(all_cfmms):
    print(i, cfmm)

d, l, p, n = solve(
    all_tokens, 
    all_cfmms, 
    reserves, 
    cfmm_tx_cost, 
    fees, 
    ibc_pools, 
    ORIGIN_TOKEN,
    2000,
    OBJ_TOKEN
    )

to_look_n: list[float] = []
for i in range(len(all_cfmms)):
    to_look_n.append(n[i].value)

_max = 0
for t in sorted(to_look_n):
    try:
        d2, l2, p2, n2 =  solve(
            all_tokens,
            all_cfmms,
            reserves,
            cfmm_tx_cost,
            fees,
            ibc_pools,
            ORIGIN_TOKEN,
            2000,
            OBJ_TOKEN,
            [1 if value <= t else 0 for value in to_look_n],
        )
        if p.value[all_tokens.index(OBJ_TOKEN)] > _max:
            d_max, l_max, p_max, n_max = d2, l2, p2, n2 
        print("---")
    except:
        continue
eta = n_max
eta_change = True
print("---------")
lastp_value = p.value[all_tokens.index(OBJ_TOKEN)]
while eta_change:
    try:
        eta_change = False

        for idx, delta in enumerate(d_max):
            if all(delta_i.value < 1e-04 for delta_i in delta):
                n_max[idx] = 0
                eta_change = True
        d_max, l, p, eta = solve(
            all_tokens,
            all_cfmms,
            reserves,
            cfmm_tx_cost,
            fees,
            ibc_pools,
            ORIGIN_TOKEN,
            2000,
            OBJ_TOKEN,
            eta,
        )

    except:
        continue

print("---")
deltas, lambdas, p, eta = solve(
                all_tokens,
                all_cfmms,
                reserves,
                cfmm_tx_cost,
                fees,
                ibc_pools,
                ORIGIN_TOKEN,
                2000,
                OBJ_TOKEN,
                eta,
            )
m = len(all_cfmms)
for i in range(m):
    print(
        f"Market {all_cfmms[i][0]}<->{all_cfmms[i][1]}, delta: {deltas[i].value}, lambda: {lambdas[i].value}, eta: {eta[i].value}",
    )

print(p.value[all_tokens.index(OBJ_TOKEN)],lastp_value)
    
