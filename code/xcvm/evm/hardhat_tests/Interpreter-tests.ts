import { ethers, network } from "hardhat";
import { expect } from "chai";
import { XCVM } from "xcvm-typescript-sdk";

const protobuf = require("protobufjs");

describe("Interpreter", function () {
  let router: any;
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
    const Interpreter = await ethers.getContractFactory("Interpreter");
    const Router = await ethers.getContractFactory("Router");
    router = await Router.deploy();
    //register owner as the bridge
    await router.registerBridge(owner.address, 1, 1);

    await router.createInterpreter({
      networkId: 1,
      account: owner.address,
    });
    interpreterAddress = await router.userInterpreter(1, owner.address);
    const ERC20Mock = await ethers.getContractFactory("ERC20Mock");
    erc20 = await ERC20Mock.deploy("test", "test", interpreterAddress, ethers.utils.parseEther("10000000000000000"));
    await router.registerAsset(erc20.address, 1);
  });

  describe("interpreter with protobuf", function () {
    it("test program using sdk: transfer unit", async function () {
      let xcvm = new XCVM();
      let data = xcvm.createProgram(
        "0x01",
        xcvm.createInstructions([
          xcvm.createInstruction(
            xcvm.createTransfer(xcvm.createAccount(owner.address), [
              xcvm.createAsset(
                xcvm.createAssetId(1),
                xcvm.createBalance(
                  // 1.5
                  xcvm.createUnit(1, xcvm.createRatio("1000", "2000"))
                )
              ),
            ])
          ),
        ])
      );
      await router.runProgram({ networkId: 1, account: owner.address }, xcvm.encodeMessage(data), [], []);
      // 1.5 units
      expect((await erc20.balanceOf(owner.address)).toString()).to.be.equal("1500000000000000000");
    });

    it("test program using sdk: transfer ratio", async function () {
      let xcvm = new XCVM();
      let data = xcvm.createProgram(
        "0x01",
        xcvm.createInstructions([
          xcvm.createInstruction(
            xcvm.createTransfer(xcvm.createAccount(owner.address), [
              xcvm.createAsset(
                xcvm.createAssetId(1),
                xcvm.createBalance(xcvm.createRatio("1000000000000000000000000000", "2000000000000000000000000000"))
              ),
            ])
          ),
        ])
      );
      await router.runProgram({ networkId: 1, account: owner.address }, xcvm.encodeMessage(data), [], []);
      expect((await erc20.balanceOf(owner.address)).toString()).to.be.equal("5000000000000000000000000000000000");
    });

    it("test program using sdk: transfer absolut", async function () {
      let xcvm = new XCVM();
      let data = xcvm.createProgram(
        "0x01",
        xcvm.createInstructions([
          xcvm.createInstruction(
            xcvm.createTransfer(xcvm.createAccount(owner.address), [
              xcvm.createAsset(
                xcvm.createAssetId(1),
                xcvm.createBalance(xcvm.createAbsolut("1000000000000000000000000000"))
              ),
            ])
          ),
        ])
      );
      let cc = xcvm.ProgramMessage.decode(xcvm.encodeMessage(data));
      await router.runProgram({ networkId: 1, account: owner.address }, xcvm.encodeMessage(data), [], []);
      expect((await erc20.balanceOf(owner.address)).toString()).to.be.equal("1000000000000000000000000000");
    });

    it("test call function using sdk", async function () {
      let functionSignature = erc20.interface.encodeFunctionData("transfer", [
        user1.address,
        ethers.utils.parseEther("100"),
      ]);
      const abiCoder = ethers.utils.defaultAbiCoder;
      const payload = ethers.utils.concat([
        ethers.utils.arrayify(abiCoder.encode(["address"], [erc20.address])),
        ethers.utils.arrayify(functionSignature),
      ]);
      let xcvm = new XCVM();
      let programMessage = xcvm.createProgram(
        "0x01",
        xcvm.createInstructions([xcvm.createInstruction(xcvm.createCall(payload, xcvm.createBindings([])))])
      );

      let encodedProgram = xcvm.encodeMessage(programMessage);
      await router.runProgram({ networkId: 1, account: owner.address }, encodedProgram, [], []);
      expect((await erc20.balanceOf(user1.address)).toString()).to.be.equal(ethers.utils.parseEther("100").toString());
    });

    it("test call function using sdk: call with late binding", async function () {
      const abiCoder = ethers.utils.defaultAbiCoder;

      let functionSignature = erc20.interface.getSighash("transfer(address,uint256)");
      // placehold 1 and 2
      const payload = ethers.utils.concat([
        ethers.utils.arrayify("0x01"),
        ethers.utils.arrayify(functionSignature),
        abiCoder.encode(["address"], [user1.address]),
        ethers.utils.arrayify("0x02"),
      ]);
      let xcvm = new XCVM();
      let programMessage = xcvm.createProgram(
        "0x01",
        xcvm.createInstructions([
          xcvm.createInstruction(
            xcvm.createCall(
              payload,
              xcvm.createBindings([
                xcvm.createBinding(0, xcvm.createBindingValue(xcvm.createAssetId(1))),
                // bingdingValuePosition(1 byte) + function signature (4bytes) + address(32bytes, its encoded) = 37 => balanceValuePosition
                xcvm.createBinding(
                  37,
                  xcvm.createBindingValue(xcvm.createAssetAmount(xcvm.createAssetId(1), xcvm.createRatio(1, 2)))
                ),
              ])
            )
          ),
        ])
      );

      let encodedProgram = xcvm.encodeMessage(programMessage);
      await router.runProgram({ networkId: 1, account: owner.address }, encodedProgram, [], []);
      expect((await erc20.balanceOf(user1.address)).toString()).to.be.equal(
        ethers.utils.parseEther("5000000000000000").toString()
      );
    });

    it("test spawn program using sdk", async function () {
      let xcvm = new XCVM();
      let programMessage = xcvm.createProgram(
        "0x01",
        xcvm.createInstructions([
          xcvm.createInstruction(
            xcvm.createTransfer(xcvm.createAccount(owner.address), [
              xcvm.createAsset(xcvm.createAssetId(1), xcvm.createBalance(xcvm.createAbsolut("100"))),
            ])
          ),
        ])
      );

      let data = xcvm.createProgram(
        "0x01",
        xcvm.createInstructions([
          xcvm.createInstruction(
            xcvm.createSpawn(xcvm.createNetwork(1), xcvm.createSalt("0x11"), 1, programMessage, [
              xcvm.createAsset(xcvm.createAssetId(1), xcvm.createBalance(xcvm.createAbsolut(200))),
            ])
          ),
        ])
      );

      await expect(router.runProgram({ networkId: 1, account: owner.address }, xcvm.encodeMessage(data), [], []))
        .to.emit(router, "Spawn")
        .withArgs(
          owner.address.toLowerCase(),
          1,
          1,
          "0x11",
          ethers.utils.hexlify(xcvm.encodeMessage(programMessage)),
          [erc20.address],
          [200]
        );
      expect((await erc20.balanceOf(owner.address)).toString()).to.be.equal("200");
    });

    it("test program with multiple instructions", async function () {
      const abiCoder = ethers.utils.defaultAbiCoder;

      let functionSignature = erc20.interface.getSighash("transfer(address,uint256)");
      // placehold 1 and 2
      const payload = ethers.utils.concat([
        ethers.utils.arrayify("0x01"),
        ethers.utils.arrayify(functionSignature),
        abiCoder.encode(["address"], [user1.address]),
        ethers.utils.arrayify("0x02"),
      ]);
      let xcvm = new XCVM();
      let programMessage = xcvm.createProgram(
        "0x01",
        xcvm.createInstructions([
          xcvm.createInstruction(
            xcvm.createCall(
              payload,
              xcvm.createBindings([
                xcvm.createBinding(0, xcvm.createBindingValue(xcvm.createAssetId(1))),
                // bingdingValuePosition(1 byte) + function signature (4bytes) + address(32bytes, its encoded) = 37 => balanceValuePosition
                xcvm.createBinding(
                  37,
                  xcvm.createBindingValue(xcvm.createAssetAmount(xcvm.createAssetId(1), xcvm.createRatio(1, 2)))
                ),
              ])
            )
          ),
          xcvm.createInstruction(
            xcvm.createTransfer(xcvm.createAccount(owner.address), [
              xcvm.createAsset(
                xcvm.createAssetId(1),
                xcvm.createBalance(
                  // 1.5
                  xcvm.createUnit(1, xcvm.createRatio("1000", "2000"))
                )
              ),
            ])
          ),
        ])
      );

      let encodedProgram = xcvm.encodeMessage(programMessage);
      await router.runProgram({ networkId: 1, account: owner.address }, encodedProgram, [], []);
      expect((await erc20.balanceOf(user1.address)).toString()).to.be.equal(
        ethers.utils.parseEther("5000000000000000").toString()
      );
      expect((await erc20.balanceOf(owner.address)).toString()).to.be.equal(ethers.utils.parseEther("1.5").toString());
    });
  });
});
