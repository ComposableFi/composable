# solves using convex optimization

import itertools
import numpy as np
import cvxpy as cp

MAX_RESERVE = 1e10


# simulate denom paths to and from chains, with center node
def populate_chain_dict(chains: dict[str, list[str]], center_node: str):
    # Add tokens with denom to Center Node
    # Basic IBC transfer
    for key, value in chains.items():
        if key != center_node:
            chains[center_node].extend(f"{key}/{token}" for token in value)

    # Add tokens from Center Node to outers
    # Simulate IBC transfer through Centuri
    for key, value in chains.items():
        if key != center_node:
            chains[key].extend(
                f"{center_node}/{token}"
                for token in chains[center_node]
                if f"{key}/" not in token
            )

def solve(
    all_tokens: list[str],
    all_cfmms: list[tuple[str, str]],
    reserves: list[np.ndarray[np.float64]],
    cfmm_tx_cost: list[float],
    fees: list[float],
    ibc_pools: int,
    origin_token: str,
    number_of_init_tokens: int,
    obj_token: str,
    force_eta: list[float] = None,
):
    # Build local-global matrices
    count_tokens = len(all_tokens)
    count_cfmms = len(all_cfmms)

    current_assets = np.zeros(count_tokens)  # Inital assets
    current_assets[all_tokens.index(origin_token)] = number_of_init_tokens

    A = []
    for cfmm in all_cfmms:
        n_i = len(cfmm)  # Number of tokens in pool (default to 2)
        A_i = np.zeros((count_tokens, n_i))
        for i, token in enumerate(cfmm):
            A_i[all_tokens.index(token), i] = 1
        A.append(A_i)

    # Build variables
    deltas = [cp.Variable(len(l), nonneg=True) for l in all_cfmms]
    lambdas = [cp.Variable(len(l), nonneg=True) for l in all_cfmms]
    eta = cp.Variable(
        count_cfmms, nonneg=True
    )  # Binary value, indicates tx or not for given pool

    psi = cp.sum([A_i @ (L - D) for A_i, D, L in zip(A, deltas, lambdas)])

    # Objective is to trade number_of_init_tokens of asset origin_token for a maximum amount of asset objective_token
    obj = cp.Maximize(psi[all_tokens.index(obj_token)] - eta @ cfmm_tx_cost)

    # Reserves after trade
    new_reserves = [
        R + gamma_i * D - L for R, gamma_i, D, L in zip(reserves, fees, deltas, lambdas)
    ]

    # Trading function constraints
    constrains = [
        psi + current_assets >= 0,
    ]

    # Pool constraint (Uniswap v2 like)
    for i in range(count_cfmms - ibc_pools):
        constrains.append(cp.geo_mean(new_reserves[i]) >= cp.geo_mean(reserves[i]))

    # Pool constraint for IBC transfer (constant sum)
    # NOTE: Ibc pools are at the bottom of the cfmm list
    for i in range(count_cfmms - ibc_pools, count_cfmms):
        constrains.append(cp.sum(new_reserves[i]) >= cp.sum(reserves[i]))
        constrains.append(new_reserves[i] >= 0)

    # Enforce deltas depending on pass or not pass variable
    # MAX_RESERVE should be big enough so delta <<< MAX_RESERVE
    for i in range(count_cfmms):
        constrains.append(deltas[i] <= eta[i] * MAX_RESERVE)
        if force_eta:
            constrains.append(eta[i] == force_eta[i])

    # Set up and solve problem
    prob = cp.Problem(obj, constrains)
    prob.solve(verbose= True)

    print(
        f"\033[1;91mTotal amount out: {psi.value[all_tokens.index(obj_token)]}\033[0m"
    )

    for i in range(count_cfmms):
        print(
            f"Market {all_cfmms[i][0]}<->{all_cfmms[i][1]}, delta: {deltas[i].value}, lambda: {lambdas[i].value}, eta: {eta[i].value}",
        )
    return deltas, lambdas, psi, eta
