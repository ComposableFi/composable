const { task, types } = require("hardhat/config");

task("mosaic_vault_config_whitelist_token", "Whitelist a new token in the Vault")
  .addParam("tokenaddress", "Token Contract Address")
  .addParam("mintransferamount", "Minimum amount a user can transfer")
  .addParam("maxtransferamount", "Maximum amount a user can transfer")
  .setAction(async (taskArgs, hre) => {
    const { getNamedAccounts, ethers, deployments } = hre;
    const { deployer } = await getNamedAccounts();
    const owner = await ethers.getSigner(deployer);

    const mosaicVaultConfig = await deployments.getOrNull("MosaicVaultConfig");
    if (!mosaicVaultConfig) {
      throw "Deploy the MosaicVaultConfig before";
    }

    const mosaicVaultConfigContract = await ethers.getContractAt(
      "MosaicVaultConfig",
      mosaicVaultConfig.address
    );

    console.log("Whitelisting tokens..");

    const whitelistedToken = await mosaicVaultConfigContract.whitelistedTokens(
      taskArgs.tokenaddress
    );

    if (whitelistedToken.underlyingReceiptAddress === ethers.constants.AddressZero) {
      console.log(`Adding token ${taskArgs.tokenaddress}....`);

      await mosaicVaultConfigContract
        .connect(owner)
        .addWhitelistedToken(
          taskArgs.tokenaddress,
          taskArgs.mintransferamount,
          taskArgs.maxtransferamount,
          { gasLimit: 9000000 }
        );

      console.log(`Token ${taskArgs.tokenaddress} has been added to the whitelist`);
    } else {
      console.log(`Token ${taskArgs.tokenaddress} already in the whitelist`);
    }

    console.log("Finished!");
  });

task("mosaic_vault_config_add_token_in_network", "Assign remote token address")
  .addParam("tokenaddress", "Token Contract Address")
  .addParam("tokenaddressremote", "Remote Token Contract Address")
  .addParam("remotenetwork", "ID of the remote network")
  .addOptionalParam("ratio", "Ratio of the token", 1000, types.int)
  .setAction(async (taskArgs, hre) => {
    const { getNamedAccounts, ethers, deployments } = hre;
    const { deployer } = await getNamedAccounts();
    const owner = await ethers.getSigner(deployer);

    const mosaicVaultConfig = await deployments.getOrNull("MosaicVaultConfig");
    if (!mosaicVaultConfig) {
      throw "Deploy the MosaicVaultConfig before";
    }

    const mosaicVaultConfigContract = await ethers.getContractAt(
      "MosaicVaultConfig",
      mosaicVaultConfig.address
    );

    console.log("Adding tokens...");

    await mosaicVaultConfigContract
      .connect(owner)
      .addTokenInNetwork(
        taskArgs.tokenaddress,
        taskArgs.tokenaddressremote,
        taskArgs.remotenetwork,
        taskArgs.ratio,
        { gasLimit: 9000000 }
      );

    console.log("Finished!");
  });

task("sushiswap_quote_price", "Get price quote of token B")
  .addParam("tokenAddressA", "Token address A")
  .addParam("amountTokenA", "Amount of token A want to invest")
  .addParam("tokenAddressB", "Token address B")
  .setAction(async (taskArgs, { deployments, ethers }) => {
    const { tokenAddressA, tokenAddressB, amountTokenA } = taskArgs;
    if (ethers.BigNumber.from(tokenAddressA).lt(ethers.BigNumber.from(tokenAddressB))) {
      throw "Token pair need to be sorted";
    }

    const sushiswapLPStrategyDeployment = await deployments.get("SushiswapLiquidityProvider");
    const sushiswapLPStrategyContract = await ethers.getContractAt(
      "SushiswapLiquidityProvider",
      sushiswapLPStrategyDeployment.address
    );
    const quote = await sushiswapLPStrategyContract.callStatic.getTokenPrice(
      amountTokenA,
      tokenAddressA,
      tokenAddressB
    );
    console.log("Quote: ", quote.toString());
  });

task("add_sushi_lp", "Provide liquidity into SUSHI LP strategy")
  .addParam("tokenAddressA", "Token address A")
  .addParam("amountTokenA", "Amount of token A want to invest")
  .addParam("tokenAddressB", "Token address B")
  .addParam("amountTokenB", "Amount of token B want to invest")
  .addParam("deadline", "Offer availability time in seconds")
  .setAction(async (taskArgs, { deployments, ethers }) => {
    const { tokenAddressA, tokenAddressB, amountTokenA, amountTokenB, deadline } = taskArgs;
    if (ethers.BigNumber.from(tokenAddressA).lt(ethers.BigNumber.from(tokenAddressB))) {
      throw "Token pair need to be sorted";
    }
    const investments = [
      { token: tokenAddressA, amount: amountTokenA },
      { token: tokenAddressB, amount: amountTokenB },
    ];
    const blockNum = await ethers.provider.getBlockNumber();
    const currentBlock = await ethers.provider.getBlock(blockNum);
    const data = ethers.utils.defaultAbiCoder.encode(
      ["uint256", "uint256", "uint256"],
      [currentBlock.timestamp + deadline, amountTokenA, amountTokenB]
    );
    const sushiswapLPContract = await deployments.get("SushiswapLiquidityProvider");
    const mosaicHoldingDeployment = await deployments.get("MosaicHolding");
    const mosaicHoldingContract = await ethers.getContractAt(
      "MosaicHolding",
      mosaicHoldingDeployment.address
    );
    await mosaicHoldingContract.invest(investments, sushiswapLPContract.address, data);

    console.log("Liquidity successfully added");
  });

task("summoner_set_relayer", "Set the relayer address on Summoner")
  .addParam("relayer", "Relayer address")
  .setAction(async (taskArgs, { deployments, ethers, getNamedAccounts }) => {
    const { relayer } = taskArgs;
    const { deployer } = await getNamedAccounts();
    const owner = await ethers.getSigner(deployer);
    const Summoner = await deployments.get("Summoner");
    const summoner = await ethers.getContractAt("Summoner", Summoner.address);
    console.log("setting relayer on summoner..");
    let tx = await summoner.connect(owner).setRelayer(relayer);
    console.log("tx id: " + tx.hash);
    await tx.wait();
  });

task("summoner_set_fee_token", "Set the fee token on the summoner config")
  .addParam("remotenetwork", "the remote network id")
  .addParam("token", "fee token address")
  .addParam("amount", "fee amount")
  .setAction(async (taskArgs, { deployments, ethers, getNamedAccounts }) => {
    const { remotenetwork, token, amount } = taskArgs;
    const { deployer } = await getNamedAccounts();
    const owner = await ethers.getSigner(deployer);
    const SummonerConfig = await deployments.get("SummonerConfig");
    const summonerConfig = await ethers.getContractAt("SummonerConfig", SummonerConfig.address);
    console.log("setting the fee token on the summoner config..");
    let tx = await summonerConfig.connect(owner).setFeeToken(remotenetwork, token, amount);
    console.log("tx id: " + tx.hash);
    await tx.wait();
  });

task("mosaic_holding_set_role", "set the role on mosaic holding")
  .addParam("role", "role name")
  .addParam("address", "role address")
  .setAction(async (taskArgs, { deployments, ethers, getNamedAccounts }) => {
    const { role, address } = taskArgs;
    const { deployer } = await getNamedAccounts();
    const owner = await ethers.getSigner(deployer);
    const MosaicHolding = await deployments.get("MosaicHolding");
    const mosaicHolding = await ethers.getContractAt("MosaicHolding", MosaicHolding.address);
    console.log("setting the role on mosaicHolding..");
    let tx = await mosaicHolding.connect(owner).setUniqRole(ethers.utils.id(role), address);
    console.log("tx id: " + tx.hash);
    await tx.wait();
  });

task(
  "mosaic_holding_set_rebalancing_threshold",
  "set rebalancing threshold for tokens on mosaic holding"
)
  .addParam("token", "the token address")
  .addParam("amount", "amount to set")
  .setAction(async (taskArgs, { deployments, ethers, getNamedAccounts }) => {
    const { token, amount } = taskArgs;
    const { deployer } = await getNamedAccounts();
    const owner = await ethers.getSigner(deployer);
    const MosaicHolding = await deployments.get("MosaicHolding");
    const mosaicHolding = await ethers.getContractAt("MosaicHolding", MosaicHolding.address);
    console.log("setting the token rebalancing threshold..");
    let tx = await mosaicHolding.connect(owner).setRebalancingThreshold(token, amount);
    console.log("tx id: " + tx.hash);
    await tx.wait();
  });

task("mosaic_vault_set_relayer", "Set the relayer address on MosaicVault")
  .addParam("relayer", "Relayer address")
  .setAction(async (taskArgs, { deployments, ethers, getNamedAccounts }) => {
    const { relayer } = taskArgs;
    const { deployer } = await getNamedAccounts();
    const owner = await ethers.getSigner(deployer);
    const MosaicVault = await deployments.get("MosaicVault");
    const mosaicVault = await ethers.getContractAt("MosaicVault", MosaicVault.address);
    console.log("setting relayer on MosaicVault..");
    let tx = await mosaicVault.connect(owner).setRelayer(relayer);
    console.log("tx id: " + tx.hash);
    await tx.wait();
  });

task("mosaic_vault_provide_liquidity", "Provide liquidity on MosaicVault")
  .addParam("amount", "Amount to provide")
  .addParam("tokenaddress", "Token address")
  .addParam("blocksforactiveliq", "Blocks for active liquidity")
  .setAction(async (taskArgs, { deployments, ethers, getNamedAccounts }) => {
    const { amount, tokenaddress, blocksforactiveliq } = taskArgs;
    const { deployer } = await getNamedAccounts();
    const owner = await ethers.getSigner(deployer);
    const MosaicVault = await deployments.get("MosaicVault");
    const mosaicVault = await ethers.getContractAt("MosaicVault", MosaicVault.address);
    const erc20 = await ethers.getContractAt("ERC20", tokenaddress);
    console.log("provide liquidity on MosaicVault..");

    const tokenAllowance = await erc20.connect(owner).allowance(owner.address, MosaicVault.address);
    if (tokenAllowance < amount) {
      let tx = await erc20
        .connect(owner)
        .increaseAllowance(MosaicVault.address, amount - tokenAllowance);
      await tx.wait();
    }

    if (blocksforactiveliq == 0) {
      let tx = await mosaicVault
        .connect(owner)
        .providePassiveLiquidity(amount, tokenaddress, { gasLimit: 9000000 });
      console.log("tx id: " + tx.hash);
      await tx.wait();
    } else {
      let tx = await mosaicVault
        .connect(owner)
        .provideActiveLiquidity(amount, tokenaddress, blocksforactiveliq, { gasLimit: 9000000 });
      console.log("tx id: " + tx.hash);
      await tx.wait();
    }
  });

task("mosaic_vault_withdraw_liquidity_request", "Withdraw liquidity request on MosaicVault")
  .addParam("amount", "Amount to provide")
  .addParam("tokenaddress", "Token address")
  .addParam("networkid", "Network ID")
  .setAction(async (taskArgs, { deployments, ethers, getNamedAccounts }) => {
    const { amount, tokenaddress, networkid } = taskArgs;
    const { deployer } = await getNamedAccounts();
    const owner = await ethers.getSigner(deployer);
    const MosaicVault = await deployments.get("MosaicVault");
    const mosaicVault = await ethers.getContractAt("MosaicVault", MosaicVault.address);
    const MosaicVaultConfig = await deployments.get("MosaicVaultConfig");
    const mosaicVaultConfig = await ethers.getContractAt(
      "MosaicVaultConfig",
      MosaicVaultConfig.address
    );
    const EMPTY_BYTES = ethers.utils.toUtf8Bytes("");
    console.log("withdraw liquidity request on MosaicVault..");

    const whitelistedUnderlyingIOUAddress = await mosaicVaultConfig.getUnderlyingIOUAddress(
      tokenaddress
    );

    let tx = await mosaicVault
      .connect(owner)
      .withdrawLiquidityRequest(
        whitelistedUnderlyingIOUAddress,
        amount,
        tokenaddress,
        owner.address,
        0,
        EMPTY_BYTES,
        networkid,
        [0, 0, false],
        { gasLimit: 9000000 }
      );
    console.log("tx id: " + tx.hash);
    await tx.wait();
  });

task("mosaic_vault_withdraw_liquidity", "Withdraw liquidity on MosaicVault")
  .addParam("amount", "Amount to provide")
  .addParam("tokenaddress", "Token address")
  .addParam("feepercentage", "Fee percentage")
  .addParam("basefee", "Base fee")
  .addParam("transid", "Transaction ID")
  .addParam("ammid", "AMM id")
  .setAction(async (taskArgs, { deployments, ethers, getNamedAccounts }) => {
    const { amount, tokenaddress, feepercentage, basefee, transid, ammid } = taskArgs;
    const { deployer } = await getNamedAccounts();
    const owner = await ethers.getSigner(deployer);
    const MosaicVault = await deployments.get("MosaicVault");
    const mosaicVault = await ethers.getContractAt("MosaicVault", MosaicVault.address);
    const EMPTY_BYTES = ethers.utils.toUtf8Bytes("");
    console.log("withdraw liquidity on MosaicVault..");

    let tx = await mosaicVault.connect(owner).withdrawLiquidity(
      owner.address,
      tokenaddress,
      tokenaddress,
      amount,
      0,
      {
        feePercentage: feepercentage,
        baseFee: basefee,
        investmentStrategy: ethers.constants.AddressZero,
        investmentStrategies: [],
        ammId: ammid,
        id: transid,
        amountToSwapToNative: 0,
        minAmountOutNative: 0,
        nativeSwapperId: 0,
      },
      EMPTY_BYTES,
      { gasLimit: 9000000 }
    );
    console.log("tx id: " + tx.hash);
    await tx.wait();
  });
