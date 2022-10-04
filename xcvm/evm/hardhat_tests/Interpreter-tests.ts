import {ethers, network} from "hardhat";
import {expect} from "chai";
import {XCVM} from "xcvm-typescript-sdk";

const protobuf = require("protobufjs");

describe("Interpreter", function () {
  let gateway: any;
  let interpreter: any;
  let owner: any;
  let user1: any;
  let user2: any;
  let accounts: any;
  let interpreterAddress: any;
  let erc20: any;
  beforeEach(async function () {
    accounts = await ethers.getSigners();
    [owner, user1, user2] = accounts;
    const Interpreter = await ethers.getContractFactory(
      "Interpreter"
    );
    const Gateway = await ethers.getContractFactory("Gateway");
    gateway = await Gateway.deploy();
    //register owner as the bridge
    await gateway.registerBridge(owner.address, 1, 1);

    await gateway.createInterpreter({
      networkId: 1,
      account: owner.address,
    });
    console.log(interpreterAddress);
    interpreterAddress = await gateway.userInterpreter(
      1,
      owner.address,
    )
    ;
    const ERC20Mock = await ethers.getContractFactory("ERC20Mock");
    erc20 = await ERC20Mock.deploy(
      "test",
      "test",
      interpreterAddress,
      ethers.utils.parseEther("10000000000000000")
    );
    await gateway.registerAsset(erc20.address, 1);
  });

  describe("interpreter with protobuf", function () {
    it("test user protobuf transfer with absolute", async function () {
      let root = await protobuf.load("./interpreter.proto");
      let ProgramMessage = root.lookupType("interpreter.Program");
      let InstructionMessage = root.lookupType(
        "interpreter.Instruction"
      );
      let InstructionsMessage = root.lookupType(
        "interpreter.Instructions"
      );
      let TransferMessage = root.lookupType("interpreter.Transfer");
      let AssetIdMessage = root.lookupType("interpreter.AssetId");
      let AccountMessage = root.lookupType("interpreter.Account");
      let AssetMessage = root.lookupType("interpreter.Asset");
      let AbsoluteMessage = root.lookupType("interpreter.Absolute");
      let BalanceMessage = root.lookupType("interpreter.Balance");

      let transferAmount = 100;

      let accountMessage = AccountMessage.create({
        account: ethers.utils.arrayify(owner.address),
      });
      let assetIdMessage = AssetIdMessage.create({assetId: 1});
      let absoluteMessage = AbsoluteMessage.create({
        value: transferAmount,
      });
      let balanceMessage = BalanceMessage.create(
        {absolute: absoluteMessage},
      );

      console.log("owner", owner.address.toString("hex"));
      let asset = AssetMessage.create({
        assetId: assetIdMessage,
        balance: balanceMessage,
      });
      console.log(
        "assets",
        AssetMessage.encode(asset).finish().toString("hex")
      );
      let transferMessage = TransferMessage.create({
        account: accountMessage,
        assets: [asset],
      });
      console.log(
        "account",
        AccountMessage.encode(accountMessage).finish().toString("hex")
      );
      console.log(
        "transfer",
        TransferMessage.encode(transferMessage)
          .finish()
          .toString("hex")
      );
      let instructionMessage = InstructionMessage.create(
        {transfer: transferMessage},
      );
      console.log(
        "instruction",
        InstructionMessage.encode(instructionMessage)
          .finish()
          .toString("hex")
      );
      let instructionsMessage = InstructionsMessage.create({
        instructions: [instructionMessage],
      });
      console.log(
        "instructions",
        InstructionsMessage.encode(instructionsMessage)
          .finish()
          .toString("hex")
      );
      console.log(owner.address);
      console.log(InstructionMessage.fromObject(instructionMessage));
      let programMessage = ProgramMessage.create({
        instructions: instructionsMessage,
      });
      console.log(ProgramMessage.fromObject(programMessage));
      let encodedProgram =
        "0x" +
        ProgramMessage.encode(programMessage)
          .finish()
          .toString("hex");
      encodedProgram = ProgramMessage.encode(programMessage).finish();
      console.log(encodedProgram);

      await gateway.runProgram(
        {networkId: 1, account: owner.address},
        encodedProgram,
        [],
        []
      );
      expect(
        (await erc20.balanceOf(owner.address)).toString()
      ).to.be.equal(transferAmount.toString());
    });

    it("test user protobuf transfer with ratio", async function () {
      let root = await protobuf.load("./interpreter.proto");
      let ProgramMessage = root.lookupType("interpreter.Program");
      let InstructionMessage = root.lookupType(
        "interpreter.Instruction"
      );
      let InstructionsMessage = root.lookupType(
        "interpreter.Instructions"
      );
      let TransferMessage = root.lookupType("interpreter.Transfer");
      let AssetIdMessage = root.lookupType("interpreter.AssetId");
      let AccountMessage = root.lookupType("interpreter.Account");
      let AssetMessage = root.lookupType("interpreter.Asset");
      let BalanceMessage = root.lookupType("interpreter.Balance");
      let RatioMessage = root.lookupType("interpreter.Ratio");

      let accountMessage = AccountMessage.create({
        account: ethers.utils.arrayify(owner.address),
      });
      let assetIdMessage = AssetIdMessage.create({assetId: 1});
      // half of the interpreter balance
      let ratioMessage = RatioMessage.create({
        nominator: 1,
        denominator: 2,
      });
      let balanceMessage = BalanceMessage.create(
        {ratio: ratioMessage},
        {oneOfs: true}
      );
      console.log(
        "balance",
        BalanceMessage.encode(balanceMessage).finish().toString("hex")
      );
      console.log("owner", owner.address.toString("hex"));
      let asset = AssetMessage.create({
        assetId: assetIdMessage,
        balance: balanceMessage,
      });
      console.log(
        "assets",
        AssetMessage.encode(asset).finish().toString("hex")
      );
      let transferMessage = TransferMessage.create({
        account: accountMessage,
        assets: [asset],
      });
      console.log(
        "account",
        AccountMessage.encode(accountMessage).finish().toString("hex")
      );
      console.log(
        "transfer",
        TransferMessage.encode(transferMessage)
          .finish()
          .toString("hex")
      );
      let instructionMessage = InstructionMessage.create(
        {transfer: transferMessage},
        {oneofs: true}
      );
      console.log(
        "instruction",
        InstructionMessage.encode(instructionMessage)
          .finish()
          .toString("hex")
      );
      let instructionsMessage = InstructionsMessage.create({
        instructions: [instructionMessage],
      });
      console.log(
        "instructions",
        InstructionsMessage.encode(instructionsMessage)
          .finish()
          .toString("hex")
      );
      console.log(owner.address);
      console.log(InstructionMessage.fromObject(instructionMessage));
      let programMessage = ProgramMessage.create({
        instructions: instructionsMessage,
      });
      console.log(ProgramMessage.fromObject(programMessage));
      let encodedProgram =
        "0x" +
        ProgramMessage.encode(programMessage)
          .finish()
          .toString("hex");
      encodedProgram = ProgramMessage.encode(programMessage).finish();
      console.log(encodedProgram);

      let oldBalance = await erc20.balanceOf(interpreterAddress);
      await gateway.runProgram(
        {networkId: 1, account: owner.address},
        encodedProgram,
        [],
        []
      );
      let newBalance = await erc20.balanceOf(interpreterAddress);
      expect((oldBalance / 2).toString()).to.be.equal(
        newBalance.toString()
      );
      expect(
        (await erc20.balanceOf(owner.address)).toString()
      ).to.be.equal((oldBalance / 2).toString());
    });

    it("test user protobuf transfer with unit", async function () {
      let root = await protobuf.load("./interpreter.proto");
      let ProgramMessage = root.lookupType("interpreter.Program");
      let InstructionMessage = root.lookupType(
        "interpreter.Instruction"
      );
      let InstructionsMessage = root.lookupType(
        "interpreter.Instructions"
      );
      let TransferMessage = root.lookupType("interpreter.Transfer");
      let AssetIdMessage = root.lookupType("interpreter.AssetId");
      let AccountMessage = root.lookupType("interpreter.Account");
      let AssetMessage = root.lookupType("interpreter.Asset");
      let BalanceMessage = root.lookupType("interpreter.Balance");
      let RatioMessage = root.lookupType("interpreter.Ratio");
      let UnitMessage = root.lookupType("interpreter.Unit");

      let accountMessage = AccountMessage.create({
        account: ethers.utils.arrayify(owner.address),
      });
      let assetIdMessage = AssetIdMessage.create({assetId: 1});
      // half of the interpreter balance
      let ratioMessage = RatioMessage.create({
        nominator: 1,
        denominator: 2,
      });
      // 1.5 unit of tokens
      console.log(
        "ratio",
        RatioMessage.encode(ratioMessage).finish().toString("hex")
      );
      let unitMessage = UnitMessage.create({
        integer: 1,
        ratio: ratioMessage,
      });
      console.log(
        "unit",
        UnitMessage.encode(unitMessage).finish().toString("hex")
      );
      let balanceMessage = BalanceMessage.create(
        {unit: unitMessage},
        {oneofs: true}
      );
      console.log(
        "balance",
        BalanceMessage.encode(balanceMessage).finish().toString("hex")
      );
      console.log("owner", owner.address.toString("hex"));
      let asset = AssetMessage.create({
        assetId: assetIdMessage,
        balance: balanceMessage,
      });
      console.log(
        "assets",
        AssetMessage.encode(asset).finish().toString("hex")
      );
      let transferMessage = TransferMessage.create({
        account: accountMessage,
        assets: [asset],
      });
      console.log(
        "account",
        AccountMessage.encode(accountMessage).finish().toString("hex")
      );
      console.log(
        "transfer",
        TransferMessage.encode(transferMessage)
          .finish()
          .toString("hex")
      );
      let instructionMessage = InstructionMessage.create(
        {transfer: transferMessage},
        {oneofs: true}
      );
      console.log(
        "instruction",
        InstructionMessage.encode(instructionMessage)
          .finish()
          .toString("hex")
      );
      let instructionsMessage = InstructionsMessage.create({
        instructions: [instructionMessage],
      });
      console.log(
        "instructions",
        InstructionsMessage.encode(instructionsMessage)
          .finish()
          .toString("hex")
      );
      console.log(owner.address);
      console.log(InstructionMessage.fromObject(instructionMessage));
      let programMessage = ProgramMessage.create({
        instructions: instructionsMessage,
      });
      console.log(ProgramMessage.fromObject(programMessage));
      let encodedProgram =
        "0x" +
        ProgramMessage.encode(programMessage)
          .finish()
          .toString("hex");
      encodedProgram = ProgramMessage.encode(programMessage).finish();
      console.log(encodedProgram);

      await gateway.runProgram(
        {networkId: 1, account: owner.address},
        encodedProgram,
        [],
        []
      );
      // 1.5 token with 18 decimals
      expect(
        (await erc20.balanceOf(owner.address)).toString()
      ).to.be.equal("1500000000000000000");
    });

    it("test user protobuf call instruction", async function () {
      let root = await protobuf.load("./interpreter.proto");
      let ProgramMessage = root.lookupType("interpreter.Program");
      let InstructionMessage = root.lookupType(
        "interpreter.Instruction"
      );
      let InstructionsMessage = root.lookupType(
        "interpreter.Instructions"
      );
      let CallMessage = root.lookupType("interpreter.Call");
      let BindingMessage = root.lookupType("interpreter.Binding");

      console.log(user1.address);


      let functionSignature = erc20.interface.encodeFunctionData("transfer", [
        user1.address,
        100,
      ]);
      const abiCoder = ethers.utils.defaultAbiCoder;
      const payload = ethers.utils.concat([abiCoder.encode(["address"], [erc20.address]), ethers.utils.arrayify(functionSignature)])
      console.log(payload);
      console.log("owner", owner.address.toString("hex"));
      let callMessage = CallMessage.create({
        payload: ethers.utils.arrayify(payload),
        bindings: [],
      });
      console.log(
        "callMessage",
        CallMessage.encode(callMessage).finish().toString("hex")
      );
      console.log(CallMessage.fromObject(callMessage));
      let instructionMessage = InstructionMessage.create(
        {call: callMessage},
        {oneofs: true}
      );
      console.log(
        "instruction",
        InstructionMessage.encode(instructionMessage)
          .finish()
          .toString("hex")
      );
      let instructionsMessage = InstructionsMessage.create({
        instructions: [instructionMessage],
      });
      console.log(
        "instructions",
        InstructionsMessage.encode(instructionsMessage)
          .finish()
          .toString("hex")
      );
      console.log(owner.address);
      console.log(InstructionMessage.fromObject(instructionMessage));
      let programMessage = ProgramMessage.create({
        instructions: instructionsMessage,
      });
      console.log(ProgramMessage.fromObject(programMessage));
      let encodedProgram =
        "0x" +
        ProgramMessage.encode(programMessage)
          .finish()
          .toString("hex");
      encodedProgram = ProgramMessage.encode(programMessage).finish();
      console.log(encodedProgram);
      await gateway.runProgram(
        {networkId: 1, account: owner.address},
        encodedProgram,
        [],
        []
      );
      expect((await erc20.balanceOf(user1.address)).toString()).to.be.equal('100');
    });

    it("test user protobuf call instruction using parameter interpolation", async function () {
      let root = await protobuf.load("./interpreter.proto");
      let ProgramMessage = root.lookupType("interpreter.Program");
      let InstructionMessage = root.lookupType(
        "interpreter.Instruction"
      );
      let InstructionsMessage = root.lookupType(
        "interpreter.Instructions"
      );


      let CallMessage = root.lookupType("interpreter.Call");
      let BindingMessage = root.lookupType("interpreter.Binding");
      let BindingValueMessage = root.lookupType("interpreter.BindingValue");
      let AssetIdMessage = root.lookupType("interpreter.AssetId");
      //let BalanceMessage = root.lookupType("interpreter.Balance");
      let RatioMessage = root.lookupType("interpreter.Ratio");
      let AssetAmount = root.lookupType("interpreter.AssetAmount");
      let AbsoluteMessage = root.lookupType("interpreter.Absolute");
      let BindingsMessage = root.lookupType("interpreter.Bindings");

      let assetIdMessage = AssetIdMessage.create({assetId: 1});
      let ratioMessage = RatioMessage.create({
        nominator: 1,
        denominator: 2,
      });
      console.log("ratio message", RatioMessage.encode(ratioMessage).finish().toString("hex"));

      let assetAmountMessage = AssetAmount.create({assetId: assetIdMessage, ratio: ratioMessage})
      console.log("asset amount message", AssetAmount.encode(assetAmountMessage).finish().toString("hex"));

      console.log("asset id", AssetIdMessage.encode(assetIdMessage).finish().toString("hex"))
      let addressBindingValueMessage = BindingValueMessage.create({assetId: assetIdMessage}, {oneofs: true});
      console.log("addressBindingValueMessage", BindingValueMessage.encode(addressBindingValueMessage).finish().toString("hex"))
      // first byte. the erc20 contract address
      let addressBindingMessage = BindingMessage.create({position: 0, bindingValue: addressBindingValueMessage})
      console.log("address binding", BindingMessage.encode(addressBindingMessage).finish().toString("hex"))

      let assetAmountBindingValueMessage = BindingValueMessage.create({assetAmount: assetAmountMessage}, {oneofs: true});
      console.log("assetAmountBindingValueMessage", BindingValueMessage.encode(assetAmountBindingValueMessage).finish().toString("hex"))
      // bingdingValuePosition(1 byte) + function signature (4bytes) + address(32bytes, its encoded) = 37 => balanceValuePosition
      let balanceBindingMessage = BindingMessage.create({position: 37, bindingValue: assetAmountBindingValueMessage})
      console.log("balanceBinding", BindingMessage.encode(balanceBindingMessage).finish().toString("hex"))
      let bindingsMessage = BindingsMessage.create({bindings: [addressBindingMessage, balanceBindingMessage]});

      console.log("bindingsMessage", BindingsMessage.encode(bindingsMessage).finish().toString("hex"))
      //let bindingsMessage = BindingsMessage.create({bindings: [addressBinding, balanceBinding]});

      console.log("address", user1.address);
      let expectedPayload = erc20.interface.encodeFunctionData("transfer", [
        user1.address,
        100,
      ]);
      console.log("expected payload", expectedPayload);
      let functionSignature = erc20.interface.getSighash("transfer(address,uint256)")

      const abi = ethers.utils.defaultAbiCoder;
      const abiCoder = ethers.utils.defaultAbiCoder;

      // placehold 1 and 2
      const payload = ethers.utils.concat([ethers.utils.arrayify("0x01"), ethers.utils.arrayify(functionSignature), abiCoder.encode(['address'], [user1.address]), ethers.utils.arrayify("0x02")])
      console.log("payload", ethers.utils.hexlify(payload))

      console.log("owner", owner.address.toString("hex"));
      let callMessage = CallMessage.create({
        payload: payload,
        bindings: bindingsMessage,
      });
      console.log(
        "callMessage",
        CallMessage.encode(callMessage).finish().toString("hex")
      );
      console.log(CallMessage.fromObject(callMessage));
      let instructionMessage = InstructionMessage.create(
        {call: callMessage},
        {oneofs: true}
      );
      console.log(
        "instruction",
        InstructionMessage.encode(instructionMessage)
          .finish()
          .toString("hex")
      );
      let instructionsMessage = InstructionsMessage.create({
        instructions: [instructionMessage],
      });
      console.log(
        "instructions",
        InstructionsMessage.encode(instructionsMessage)
          .finish()
          .toString("hex")
      );
      console.log(owner.address);
      console.log(InstructionMessage.fromObject(instructionMessage));
      let programMessage = ProgramMessage.create({
        instructions: instructionsMessage,
      });
      console.log(ProgramMessage.fromObject(programMessage));
      let encodedProgram;
      encodedProgram = ProgramMessage.encode(programMessage).finish();
      console.log(encodedProgram);
      const oldBalance = await erc20.balanceOf(interpreterAddress)
      await gateway.runProgram(
        {networkId: 1, account: owner.address},
        encodedProgram,
        [],
        []
      );
      // check if the fund was sent to user
      expect(
        (await erc20.balanceOf(user1.address)).toString()
      ).to.be.equal((oldBalance / 2).toString());
    });

    it.only("test program using sdk", async function () {
      let xcvm = new XCVM();
      let data =
        xcvm.createProgram(
          xcvm.createInstructions(
            [
              xcvm.createInstruction(
                xcvm.createTransfer(
                  xcvm.createAccount(owner.address),
                  [
                    xcvm.createAsset(
                      xcvm.createAssetId(1),
                      xcvm.createBalance(
                        xcvm.createAbsolut('1000000000000000000000000000')
                      )
                    )
                  ]
                )
              )
            ]
          )
        )
      console.log(xcvm.encodeMessage(data));

      let cc = xcvm.ProgramMessage.decode(xcvm.encodeMessage(data))

      console.log(333, JSON.stringify(cc.toJSON(), null, 2))

      await gateway.runProgram(
        {networkId: 1, account: owner.address},
        xcvm.encodeMessage(data),
        [],
        []
      );
      expect((await erc20.balanceOf(owner.address)).toString()).to.be.equal('1000000000000000000000000000')
    });

    it.only("test call function using sdk", async function () {
      const abi = ethers.utils.defaultAbiCoder;
      const abiCoder = ethers.utils.defaultAbiCoder;

      let functionSignature = erc20.interface.getSighash("transfer(address,uint256)")
      // placehold 1 and 2
      const payload = ethers.utils.concat([ethers.utils.arrayify("0x01"), ethers.utils.arrayify(functionSignature), abiCoder.encode(['address'], [user1.address]), ethers.utils.arrayify("0x02")])
      console.log("payload", ethers.utils.hexlify(payload))


      let xcvm = new XCVM();
      let a = xcvm.createInstruction(xcvm.createCall(payload, xcvm.createBindings([
          xcvm.createBinding(
            0,
            xcvm.createBindingValue(
              xcvm.createAssetId(1)
            )
          ),
          // bingdingValuePosition(1 byte) + function signature (4bytes) + address(32bytes, its encoded) = 37 => balanceValuePosition
          xcvm.createBinding(
            37,
            xcvm.createBindingValue(
              xcvm.createAssetAmount(
                xcvm.createAssetId(1),
                xcvm.createRatio(1, 2)
              )
            )
          )
        ]
      )))
      let programMessage =
        xcvm.createProgram(
          xcvm.createInstructions(
            [
              xcvm.createInstruction(
                xcvm.createCall(payload, xcvm.createBindings([
                      xcvm.createBinding(
                        0,
                        xcvm.createBindingValue(
                          xcvm.createAssetId(1)
                        )
                      ),
                      // bingdingValuePosition(1 byte) + function signature (4bytes) + address(32bytes, its encoded) = 37 => balanceValuePosition
                      xcvm.createBinding(
                        37,
                        xcvm.createBindingValue(
                          xcvm.createAssetAmount(
                            xcvm.createAssetId(1),
                            xcvm.createRatio(1, 2)
                          )
                        )
                      )
                    ]
                  )
                )
              )
            ]
          )
        )

      let encodedProgram = xcvm.encodeMessage(programMessage);
      console.log(3333, encodedProgram);
      await gateway.runProgram(
        {networkId: 1, account: owner.address},
        encodedProgram,
        [],
        []
      );
      expect((await erc20.balanceOf(user1.address)).toString()).to.be.equal(ethers.utils.parseEther("5000000000000000").toString());
    })

    it.only("test spawn program using sdk", async function () {
      let xcvm = new XCVM();
      let programMessage =
        xcvm.createProgram(
          xcvm.createInstructions(
            [
              xcvm.createInstruction(
                xcvm.createTransfer(
                  xcvm.createAccount(owner.address),
                  [
                    xcvm.createAsset(
                      xcvm.createAssetId(1),
                      xcvm.createBalance(
                        xcvm.createAbsolut("100")
                      )
                    )
                  ]
                )
              )
            ]
          )
        )

      let data =
        xcvm.createProgram(
          xcvm.createInstructions(
            [
              xcvm.createInstruction(
                xcvm.createSpawn(
                  xcvm.createNetwork(1),
                  xcvm.createSalt(1),
                  1,
                  programMessage,
                  [
                    xcvm.createAsset(
                      xcvm.createAssetId(1),
                      xcvm.createBalance(
                        xcvm.createAbsolut(200)
                      )
                    )
                  ]
                )
              )
            ]
          )
        )

      console.log(ethers.utils.hexlify(xcvm.encodeMessage(programMessage)));
      await expect(gateway.runProgram(
        {networkId: 1, account: owner.address},
        xcvm.encodeMessage(data),
        [],
        []
      )).to.emit(gateway, "Spawn").withArgs(owner.address.toLowerCase(), 1, 1, 1, ethers.utils.hexlify(xcvm.encodeMessage(programMessage)), [erc20.address], [200]);
      expect((await erc20.balanceOf(owner.address)).toString()).to.be.equal('200')
    });
  });
});
