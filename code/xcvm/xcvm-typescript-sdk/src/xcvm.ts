import {loadSync, Type, Message, Enum} from "protobufjs";
import { resolve } from 'path';
import {utils, BigNumber} from "ethers";

export class XCVM {
  root: any;
  ProgramMessage: Type;
  InstructionMessage: Type;
  InstructionsMessage: Type;
  TransferMessage: Type;
  AssetIdMessage: Type;
  AccountMessage: Type;
  AssetMessage: Type;
  AbsoluteMessage: Type;
  BalanceMessage: Type;
  BindingsMessage: Type;
  BindingMessage: Type;
  RatioMessage: Type;
  UnitMessage: Type;
  AssetAmountMessage: Type;
  SaltMessage: Type;
  NetworkMessage: Type;
  SpawnMessage: Type;
  CallMessage: Type;
  BridgeSecurityEnum: Enum;
  BindingValueMessage: Type
  Uint128Message: Type
  SelfMessage: Type
  RelayerMessage: Type
  messageTypeLookUp: { [k: string]: any } = {};

  constructor() {
    this.root = loadSync(resolve(__dirname, './interpreter.proto'));
    this.ProgramMessage = this.root.lookupType("interpreter.Program");
    this.InstructionMessage = this.root.lookupType("interpreter.Instruction");
    this.InstructionsMessage = this.root.lookupType("interpreter.Instructions");
    this.TransferMessage = this.root.lookupType("interpreter.Transfer");
    this.AssetIdMessage = this.root.lookupType("interpreter.AssetId");
    this.AccountMessage = this.root.lookupType("interpreter.Account");
    this.AssetMessage = this.root.lookupType("interpreter.Asset");
    this.AbsoluteMessage = this.root.lookupType("interpreter.Absolute");
    this.BalanceMessage = this.root.lookupType("interpreter.Balance");
    this.BindingMessage = this.root.lookupType("interpreter.Binding");
    this.BindingsMessage = this.root.lookupType("interpreter.Bindings");
    this.AssetAmountMessage = this.root.lookupType("interpreter.AssetAmount");
    this.UnitMessage = this.root.lookupType("interpreter.Unit");
    this.RatioMessage = this.root.lookupType("interpreter.Ratio");
    this.NetworkMessage = this.root.lookupType("interpreter.Network");
    this.SpawnMessage = this.root.lookupType("interpreter.Spawn");
    this.SaltMessage = this.root.lookupType("interpreter.Salt");
    this.CallMessage = this.root.lookupType("interpreter.Call");
    this.BindingValueMessage = this.root.lookupType("interpreter.BindingValue");
    this.Uint128Message =  this.root.lookupType("interpreter.Uint128");
    this.SelfMessage = this.root.lookupType("interpreter.Self");
    this.RelayerMessage = this.root.lookupType("interpreter.Relayer");
    this.BridgeSecurityEnum = this.root.lookupEnum("interpreter.BridgeSecurity");

    this.messageTypeLookUp['Program'] = this.ProgramMessage;
    this.messageTypeLookUp['Instruction'] = this.InstructionMessage;
    this.messageTypeLookUp['Instructions'] = this.InstructionsMessage;
    this.messageTypeLookUp['Transfer'] = this.TransferMessage;
    this.messageTypeLookUp['AssetId'] = this.AssetIdMessage;
    this.messageTypeLookUp['Account'] = this.AccountMessage;
    this.messageTypeLookUp['Asset'] = this.AssetMessage;
    this.messageTypeLookUp['Absolute'] = this.AbsoluteMessage;
    this.messageTypeLookUp['Balance'] = this.BalanceMessage;
    this.messageTypeLookUp['Binding'] = this.BindingMessage;
    this.messageTypeLookUp['BindingValue'] = this.BindingValueMessage;
    this.messageTypeLookUp['Bindings'] = this.BindingsMessage;
    this.messageTypeLookUp['AssetAmount'] = this.AssetAmountMessage;
    this.messageTypeLookUp['Unit'] = this.UnitMessage;
    this.messageTypeLookUp['Ratio'] = this.RatioMessage;
    this.messageTypeLookUp['Network'] = this.NetworkMessage;
    this.messageTypeLookUp['Salt'] = this.SaltMessage;
    this.messageTypeLookUp['Call'] = this.CallMessage;
    this.messageTypeLookUp['Spawn'] = this.SpawnMessage;
    this.messageTypeLookUp['BridgeSecurity'] = this.BridgeSecurityEnum;
    this.messageTypeLookUp['Uint128'] = this.Uint128Message;
    this.messageTypeLookUp['Self'] = this.SelfMessage;
    this.messageTypeLookUp['Relayer'] = this.RelayerMessage;
  }

  public encodeMessage(message: Message) {
    let messageType: Type = this.messageTypeLookUp[message.$type.name];
    return messageType.encode(message).finish();
  }

  public getTypeError(messageName: string, type: string) {
    return messageName + " is not type of " + type;
  }

  public convertUint128(n: Number | string): Message<{}> {
      const bn = BigNumber.from(n);
      const hexBytes = bn.toHexString().slice(2);
      let highBits;
      let lowBits;
      if (hexBytes.length > 32) {
        throw ("Number is bigger than uint128 max value");
      } else if (hexBytes.length > 16) {
        highBits = BigNumber.from("0x" + hexBytes.slice(0, hexBytes.length-16)).toString()
        lowBits = BigNumber.from("0x" + hexBytes.slice(hexBytes.length-16, hexBytes.length)).toString()
      } else{
        highBits = BigNumber.from("0").toString()
        lowBits = BigNumber.from("0x" + hexBytes).toString();
      }
      return this.Uint128Message.create({highBits: highBits, lowBits: lowBits});
  }

  public createRatio(nominator: Number | string, denominator: Number | string): Message<{}> {
    return this.RatioMessage.create({nominator: this.convertUint128(nominator), denominator: this.convertUint128(denominator)})
  }

  public createUnit(integer: Number | string, ratioMessage: Message): Message<{}> {
    if (ratioMessage.$type.name != "Ratio") {
      throw this.getTypeError("ratioMessage", "ratio")
    }
    return this.UnitMessage.create({integer: this.convertUint128(integer), ratio: ratioMessage})
  }

  public createAbsolute(absoluteValue: Number | string): Message<{}> {
    return this.AbsoluteMessage.create({value: this.convertUint128(absoluteValue)})
  }

  public createBalance(balanceTypeMessage: Message): Message<{}> {
    if (balanceTypeMessage.$type.name == "Absolute") {
      return this.BalanceMessage.create({absolute: balanceTypeMessage});
    } else if (balanceTypeMessage.$type.name == "Unit") {
      return this.BalanceMessage.create({unit: balanceTypeMessage});
    } else if (balanceTypeMessage.$type.name == "Ratio") {
      return this.BalanceMessage.create({ratio: balanceTypeMessage});
    } else {
      throw ("balance type message incorrect");
    }
  }

  public createAssetId(id: Number): Message<{}> {
    return this.AssetIdMessage.create({assetId: this.convertUint128(id)});
  }

  public createAsset(assetIdMessage: Message, balanceMessage: Message): Message<{}> {
    if (assetIdMessage.$type.name != "AssetId") {
      throw this.getTypeError("assetIdMessage", "assetId")
    }
    if (balanceMessage.$type.name != "Balance") {
      throw this.getTypeError("balanceMessage", "balance")
    }
    return this.AssetMessage.create({assetId: assetIdMessage, balance: balanceMessage});
  }

  public createAccount(address: string): Message<{}> {
    return this.AccountMessage.create({
      account: utils.arrayify(address),
    });
  }


  public createTransfer(accountOrRelayerMessage: Message, assets: Array<Message>): Message<{}> {
    if (accountOrRelayerMessage.$type.name != "Account" && accountOrRelayerMessage.$type.name != "Relayer") {
      throw this.getTypeError("accountOrRelayerMessage", "balance or relayer");
    }
    for (let i = 0; i < assets.length; i++) {
      if (assets[i].$type.name != "Asset") {
        throw this.getTypeError("assets[" + i + "]", "asset")
      }
    }
    if (accountOrRelayerMessage.$type.name == "Account") {
      return this.TransferMessage.create({account: accountOrRelayerMessage, assets: assets})
    } else {
      return this.TransferMessage.create({relayer: accountOrRelayerMessage, assets: assets})
    }
  }

  public createInstruction(typedInstruction: Message): Message<{}> {
    const typeName = typedInstruction.$type.name;
    if (typeName != "Transfer"
      && typeName != "Spawn"
      && typeName != "Call"
      && typeName != "Query"
    ) {
      throw this.getTypeError("typedInstruction", "Transfer, Spawn, Call or Quey");
    }
    if (typeName == "Transfer") {
      return this.InstructionMessage.create({transfer: typedInstruction})
    } else if (typeName == "Spawn") {
      return this.InstructionMessage.create({spawn: typedInstruction})
    } else if (typeName == "Call") {
      return this.InstructionMessage.create({call: typedInstruction})
    } else {
      return this.InstructionMessage.create({query: typedInstruction})
    }
  }

  public createInstructions(instructionsMessage: Array<Message>): Message<{}> {
    for (let i = 0; i < instructionsMessage.length; i++) {
      if (instructionsMessage[i].$type.name != "Instruction") {
        throw this.getTypeError("instructions[" + i + "]", "instruction")
      }
    }
    return this.InstructionsMessage.create({instructions: instructionsMessage})
  }

  public createProgram(tag: string, instructionsMessage: Message) {
    if (instructionsMessage.$type.name != "Instructions") {
      throw this.getTypeError("instructionsMessage", "Instructions")
    }
    return this.ProgramMessage.create({
      tag: utils.arrayify(tag),
      instructions: instructionsMessage,
    });
  }


  public createNetwork(networkId: Number): Message<{}> {
    return this.NetworkMessage.create({networkId: this.convertUint128(networkId)});
  }

  public createSalt(salt: string): Message<{}> {
    return this.SaltMessage.create({salt: utils.arrayify(salt)});
  }


  //public createBridgeSecurity(security: Number): Enum {
  //  return security;
  //}

  public createSpawn(networkMessage: Message, saltMessage: Message, security: Number, programMessage: Message, assetsMessage: Array<Message>): Message<{}> {
    if (networkMessage.$type.name != "Network") {
      throw this.getTypeError("networkMessage", "network")
    }
    if (saltMessage.$type.name != "Salt") {
      throw this.getTypeError("saltMessage", "salt")
    }
    if (programMessage.$type.name != "Program") {
      throw this.getTypeError("programMessage", "program")
    }
    for (let i = 0; i < assetsMessage.length; i++) {
      if (assetsMessage[i].$type.name != "Asset") {
        throw this.getTypeError("assets[" + i + "]", "asset")
      }
    }

    return this.SpawnMessage.create({
      network: networkMessage,
      salt: saltMessage,
      security: security,
      program: programMessage,
      assets: assetsMessage
    })
  }

  public createCall(payload: Uint8Array, bindingsMessage: Message): Message<{}> {
    if (bindingsMessage.$type.name != "Bindings") {
      throw this.getTypeError("bindingsMessage", "bindings")
    }
    return this.CallMessage.create({payload: payload, bindings: bindingsMessage});
  }

  public createAssetAmount(assetIdMessage: Message, balanceMessage: Message): Message<{}> {
    if (assetIdMessage.$type.name != "AssetId") {
      throw this.getTypeError("assetIdMessage", "assetId")
    }
    if (balanceMessage.$type.name != "Balance") {
      throw this.getTypeError("balanceMessage", "balance")
    }
    return this.AssetAmountMessage.create({assetId: assetIdMessage, balance: balanceMessage});
  }

  public createSelf(): Message<{}> {
    return this.SelfMessage.create({self: 1});
  }

  public createRelayer(): Message<{}> {
    return this.RelayerMessage.create({relayer: 1});
  }

  public createBindingValue(bindingValueType: any): Message<{}> {
    if (bindingValueType.$type.name == "Self") {
      return this.BindingValueMessage.create({self: bindingValueType});
    } else if (bindingValueType.$type.name == "Relayer") {
      return this.BindingValueMessage.create({relayer: bindingValueType});
    } else if (bindingValueType.$type.name == "AssetAmount") {
      return this.BindingValueMessage.create({assetAmount: bindingValueType});
    } else if (bindingValueType.$type.name == "AssetId") {
      return this.BindingValueMessage.create({assetId: bindingValueType});
    } else if (bindingValueType.isNumeric()) {
      // type 3
      return this.BindingValueMessage.create({result: bindingValueType});
    } else {
      throw ("Binding value type message incorrect");
    }
  }

  public createBinding(position: Number, bindingValueMessage: Message): Message<{}> {
    if (bindingValueMessage.$type.name != "BindingValue") {
      throw this.getTypeError("bindingValueMessage", "bindingValue");
    }
    return this.BindingMessage.create({position: position, bindingValue: bindingValueMessage});
  }

  public createBindings(bindingsMessage: Array<Message>): Message<{}> {
    for (let i = 0; i < bindingsMessage.length; i++) {
      if (bindingsMessage[i].$type.name != "Binding") {
        throw this.getTypeError("bindings[" + i + "]", "binding")
      }
    }
    return this.BindingsMessage.create({bindings: bindingsMessage});
  }

  //public decodeProgram(programMessage: Message): Json {
  //  let jsonData = this.ProgramMessage.decode(this.encodeMessage(this.ProgramMessage.encode(programMessage))
  //  //return JSON.stringify(jsonData.toJSON(), null, 2))
  //}

}
