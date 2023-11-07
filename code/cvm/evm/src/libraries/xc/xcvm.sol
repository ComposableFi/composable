// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.10;
import "yui-ibc/proto/ProtoBufRuntime.sol";
import "yui-ibc/proto/GoogleProtobufAny.sol";
import "./common.sol";

library CvmXcvmPacketAsset {


  //struct definition
  struct Data {
    CvmCommonUint128.Data asset_id;
    CvmCommonUint128.Data amount;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_asset_id(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_amount(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_asset_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmCommonUint128.Data memory x, uint256 sz) = _decode_CvmCommonUint128(p, bs);
    r.asset_id = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_amount(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmCommonUint128.Data memory x, uint256 sz) = _decode_CvmCommonUint128(p, bs);
    r.amount = x;
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmCommonUint128(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmCommonUint128.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmCommonUint128.Data memory r, ) = CvmCommonUint128._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmCommonUint128._encode_nested(r.asset_id, pointer, bs);
    
    
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmCommonUint128._encode_nested(r.amount, pointer, bs);
    
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmCommonUint128._estimate(r.asset_id));
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmCommonUint128._estimate(r.amount));
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    CvmCommonUint128.store(input.asset_id, output.asset_id);
    CvmCommonUint128.store(input.amount, output.amount);

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmPacketAsset

library CvmXcvmPacket {


  //struct definition
  struct Data {
    bytes interpreter;
    CvmXcvmUserOrigin.Data user_origin;
    bytes salt;
    CvmXcvmProgram.Data program;
    CvmXcvmPacketAsset.Data[] assets;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint[6] memory counters;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_interpreter(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_user_origin(pointer, bs, r);
      } else
      if (fieldId == 3) {
        pointer += _read_salt(pointer, bs, r);
      } else
      if (fieldId == 4) {
        pointer += _read_program(pointer, bs, r);
      } else
      if (fieldId == 5) {
        pointer += _read_unpacked_repeated_assets(pointer, bs, nil(), counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    pointer = offset;
    if (counters[5] > 0) {
      require(r.assets.length == 0);
      r.assets = new CvmXcvmPacketAsset.Data[](counters[5]);
    }

    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 5) {
        pointer += _read_unpacked_repeated_assets(pointer, bs, r, counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }
    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_interpreter(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (bytes memory x, uint256 sz) = ProtoBufRuntime._decode_bytes(p, bs);
    r.interpreter = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_user_origin(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmUserOrigin.Data memory x, uint256 sz) = _decode_CvmXcvmUserOrigin(p, bs);
    r.user_origin = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_salt(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (bytes memory x, uint256 sz) = ProtoBufRuntime._decode_bytes(p, bs);
    r.salt = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_program(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmProgram.Data memory x, uint256 sz) = _decode_CvmXcvmProgram(p, bs);
    r.program = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @param counters The counters for repeated fields
   * @return The number of bytes decoded
   */
  function _read_unpacked_repeated_assets(
    uint256 p,
    bytes memory bs,
    Data memory r,
    uint[6] memory counters
  ) internal pure returns (uint) {
    /**
     * if `r` is NULL, then only counting the number of fields.
     */
    (CvmXcvmPacketAsset.Data memory x, uint256 sz) = _decode_CvmXcvmPacketAsset(p, bs);
    if (isNil(r)) {
      counters[5] += 1;
    } else {
      r.assets[r.assets.length - counters[5]] = x;
      counters[5] -= 1;
    }
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmUserOrigin(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmUserOrigin.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmUserOrigin.Data memory r, ) = CvmXcvmUserOrigin._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmProgram(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmProgram.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmProgram.Data memory r, ) = CvmXcvmProgram._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmPacketAsset(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmPacketAsset.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmPacketAsset.Data memory r, ) = CvmXcvmPacketAsset._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    uint256 i;
    if (r.interpreter.length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_bytes(r.interpreter, pointer, bs);
    }
    
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmUserOrigin._encode_nested(r.user_origin, pointer, bs);
    
    if (r.salt.length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      3,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_bytes(r.salt, pointer, bs);
    }
    
    pointer += ProtoBufRuntime._encode_key(
      4,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmProgram._encode_nested(r.program, pointer, bs);
    
    if (r.assets.length != 0) {
    for(i = 0; i < r.assets.length; i++) {
      pointer += ProtoBufRuntime._encode_key(
        5,
        ProtoBufRuntime.WireType.LengthDelim,
        pointer,
        bs)
      ;
      pointer += CvmXcvmPacketAsset._encode_nested(r.assets[i], pointer, bs);
    }
    }
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;uint256 i;
    e += 1 + ProtoBufRuntime._sz_lendelim(r.interpreter.length);
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmUserOrigin._estimate(r.user_origin));
    e += 1 + ProtoBufRuntime._sz_lendelim(r.salt.length);
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmProgram._estimate(r.program));
    for(i = 0; i < r.assets.length; i++) {
      e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmPacketAsset._estimate(r.assets[i]));
    }
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (r.interpreter.length != 0) {
    return false;
  }

  if (r.salt.length != 0) {
    return false;
  }

  if (r.assets.length != 0) {
    return false;
  }

    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    output.interpreter = input.interpreter;
    CvmXcvmUserOrigin.store(input.user_origin, output.user_origin);
    output.salt = input.salt;
    CvmXcvmProgram.store(input.program, output.program);

    for(uint256 i5 = 0; i5 < input.assets.length; i5++) {
      output.assets.push(input.assets[i5]);
    }
    

  }


  //array helpers for Assets
  /**
   * @dev Add value to an array
   * @param self The in-memory struct
   * @param value The value to add
   */
  function addAssets(Data memory self, CvmXcvmPacketAsset.Data memory value) internal pure {
    /**
     * First resize the array. Then add the new element to the end.
     */
    CvmXcvmPacketAsset.Data[] memory tmp = new CvmXcvmPacketAsset.Data[](self.assets.length + 1);
    for (uint256 i = 0; i < self.assets.length; i++) {
      tmp[i] = self.assets[i];
    }
    tmp[self.assets.length] = value;
    self.assets = tmp;
  }


  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmPacket

library CvmXcvmUserOrigin {


  //struct definition
  struct Data {
    uint32 network_id;
    bytes account;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_network_id(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_account(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_network_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (uint32 x, uint256 sz) = ProtoBufRuntime._decode_uint32(p, bs);
    r.network_id = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_account(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (bytes memory x, uint256 sz) = ProtoBufRuntime._decode_bytes(p, bs);
    r.account = x;
    return sz;
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    if (r.network_id != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_uint32(r.network_id, pointer, bs);
    }
    if (r.account.length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_bytes(r.account, pointer, bs);
    }
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;
    e += 1 + ProtoBufRuntime._sz_uint32(r.network_id);
    e += 1 + ProtoBufRuntime._sz_lendelim(r.account.length);
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (r.network_id != 0) {
    return false;
  }

  if (r.account.length != 0) {
    return false;
  }

    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    output.network_id = input.network_id;
    output.account = input.account;

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmUserOrigin

library CvmXcvmProgram {


  //struct definition
  struct Data {
    bytes tag;
    CvmXcvmInstruction.Data[] instructions;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint[3] memory counters;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_tag(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_unpacked_repeated_instructions(pointer, bs, nil(), counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    pointer = offset;
    if (counters[2] > 0) {
      require(r.instructions.length == 0);
      r.instructions = new CvmXcvmInstruction.Data[](counters[2]);
    }

    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 2) {
        pointer += _read_unpacked_repeated_instructions(pointer, bs, r, counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }
    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_tag(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (bytes memory x, uint256 sz) = ProtoBufRuntime._decode_bytes(p, bs);
    r.tag = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @param counters The counters for repeated fields
   * @return The number of bytes decoded
   */
  function _read_unpacked_repeated_instructions(
    uint256 p,
    bytes memory bs,
    Data memory r,
    uint[3] memory counters
  ) internal pure returns (uint) {
    /**
     * if `r` is NULL, then only counting the number of fields.
     */
    (CvmXcvmInstruction.Data memory x, uint256 sz) = _decode_CvmXcvmInstruction(p, bs);
    if (isNil(r)) {
      counters[2] += 1;
    } else {
      r.instructions[r.instructions.length - counters[2]] = x;
      counters[2] -= 1;
    }
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmInstruction(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmInstruction.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmInstruction.Data memory r, ) = CvmXcvmInstruction._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    uint256 i;
    if (r.tag.length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_bytes(r.tag, pointer, bs);
    }
    if (r.instructions.length != 0) {
    for(i = 0; i < r.instructions.length; i++) {
      pointer += ProtoBufRuntime._encode_key(
        2,
        ProtoBufRuntime.WireType.LengthDelim,
        pointer,
        bs)
      ;
      pointer += CvmXcvmInstruction._encode_nested(r.instructions[i], pointer, bs);
    }
    }
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;uint256 i;
    e += 1 + ProtoBufRuntime._sz_lendelim(r.tag.length);
    for(i = 0; i < r.instructions.length; i++) {
      e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmInstruction._estimate(r.instructions[i]));
    }
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (r.tag.length != 0) {
    return false;
  }

  if (r.instructions.length != 0) {
    return false;
  }

    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    output.tag = input.tag;

    for(uint256 i2 = 0; i2 < input.instructions.length; i2++) {
      output.instructions.push(input.instructions[i2]);
    }
    

  }


  //array helpers for Instructions
  /**
   * @dev Add value to an array
   * @param self The in-memory struct
   * @param value The value to add
   */
  function addInstructions(Data memory self, CvmXcvmInstruction.Data memory value) internal pure {
    /**
     * First resize the array. Then add the new element to the end.
     */
    CvmXcvmInstruction.Data[] memory tmp = new CvmXcvmInstruction.Data[](self.instructions.length + 1);
    for (uint256 i = 0; i < self.instructions.length; i++) {
      tmp[i] = self.instructions[i];
    }
    tmp[self.instructions.length] = value;
    self.instructions = tmp;
  }


  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmProgram

library CvmXcvmInstruction {


  //struct definition
  struct Data {
    CvmXcvmTransfer.Data transfer;
    CvmXcvmSpawn.Data spawn;
    CvmXcvmCall.Data call;
    CvmXcvmExchange.Data exchange;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_transfer(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_spawn(pointer, bs, r);
      } else
      if (fieldId == 3) {
        pointer += _read_call(pointer, bs, r);
      } else
      if (fieldId == 4) {
        pointer += _read_exchange(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_transfer(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmTransfer.Data memory x, uint256 sz) = _decode_CvmXcvmTransfer(p, bs);
    r.transfer = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_spawn(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmSpawn.Data memory x, uint256 sz) = _decode_CvmXcvmSpawn(p, bs);
    r.spawn = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_call(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmCall.Data memory x, uint256 sz) = _decode_CvmXcvmCall(p, bs);
    r.call = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_exchange(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmExchange.Data memory x, uint256 sz) = _decode_CvmXcvmExchange(p, bs);
    r.exchange = x;
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmTransfer(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmTransfer.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmTransfer.Data memory r, ) = CvmXcvmTransfer._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmSpawn(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmSpawn.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmSpawn.Data memory r, ) = CvmXcvmSpawn._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmCall(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmCall.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmCall.Data memory r, ) = CvmXcvmCall._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmExchange(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmExchange.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmExchange.Data memory r, ) = CvmXcvmExchange._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmTransfer._encode_nested(r.transfer, pointer, bs);
    
    
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmSpawn._encode_nested(r.spawn, pointer, bs);
    
    
    pointer += ProtoBufRuntime._encode_key(
      3,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmCall._encode_nested(r.call, pointer, bs);
    
    
    pointer += ProtoBufRuntime._encode_key(
      4,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmExchange._encode_nested(r.exchange, pointer, bs);
    
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmTransfer._estimate(r.transfer));
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmSpawn._estimate(r.spawn));
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmCall._estimate(r.call));
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmExchange._estimate(r.exchange));
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    CvmXcvmTransfer.store(input.transfer, output.transfer);
    CvmXcvmSpawn.store(input.spawn, output.spawn);
    CvmXcvmCall.store(input.call, output.call);
    CvmXcvmExchange.store(input.exchange, output.exchange);

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmInstruction

library CvmXcvmRatio {


  //struct definition
  struct Data {
    uint64 nominator;
    uint64 denominator;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_nominator(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_denominator(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_nominator(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (uint64 x, uint256 sz) = ProtoBufRuntime._decode_uint64(p, bs);
    r.nominator = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_denominator(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (uint64 x, uint256 sz) = ProtoBufRuntime._decode_uint64(p, bs);
    r.denominator = x;
    return sz;
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    if (r.nominator != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_uint64(r.nominator, pointer, bs);
    }
    if (r.denominator != 0) {
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_uint64(r.denominator, pointer, bs);
    }
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;
    e += 1 + ProtoBufRuntime._sz_uint64(r.nominator);
    e += 1 + ProtoBufRuntime._sz_uint64(r.denominator);
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (r.nominator != 0) {
    return false;
  }

  if (r.denominator != 0) {
    return false;
  }

    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    output.nominator = input.nominator;
    output.denominator = input.denominator;

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmRatio

library CvmXcvmUnit {


  //struct definition
  struct Data {
    CvmCommonUint128.Data integer;
    CvmXcvmRatio.Data ratio;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_integer(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_ratio(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_integer(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmCommonUint128.Data memory x, uint256 sz) = _decode_CvmCommonUint128(p, bs);
    r.integer = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_ratio(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmRatio.Data memory x, uint256 sz) = _decode_CvmXcvmRatio(p, bs);
    r.ratio = x;
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmCommonUint128(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmCommonUint128.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmCommonUint128.Data memory r, ) = CvmCommonUint128._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmRatio(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmRatio.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmRatio.Data memory r, ) = CvmXcvmRatio._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmCommonUint128._encode_nested(r.integer, pointer, bs);
    
    
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmRatio._encode_nested(r.ratio, pointer, bs);
    
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmCommonUint128._estimate(r.integer));
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmRatio._estimate(r.ratio));
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    CvmCommonUint128.store(input.integer, output.integer);
    CvmXcvmRatio.store(input.ratio, output.ratio);

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmUnit

library CvmXcvmAbsolute {


  //struct definition
  struct Data {
    CvmCommonUint128.Data value;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_value(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_value(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmCommonUint128.Data memory x, uint256 sz) = _decode_CvmCommonUint128(p, bs);
    r.value = x;
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmCommonUint128(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmCommonUint128.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmCommonUint128.Data memory r, ) = CvmCommonUint128._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmCommonUint128._encode_nested(r.value, pointer, bs);
    
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmCommonUint128._estimate(r.value));
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    CvmCommonUint128.store(input.value, output.value);

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmAbsolute

library CvmXcvmBalance {


  //struct definition
  struct Data {
    CvmXcvmRatio.Data ratio;
    CvmXcvmAbsolute.Data absolute;
    CvmXcvmUnit.Data unit;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_ratio(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_absolute(pointer, bs, r);
      } else
      if (fieldId == 3) {
        pointer += _read_unit(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_ratio(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmRatio.Data memory x, uint256 sz) = _decode_CvmXcvmRatio(p, bs);
    r.ratio = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_absolute(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmAbsolute.Data memory x, uint256 sz) = _decode_CvmXcvmAbsolute(p, bs);
    r.absolute = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_unit(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmUnit.Data memory x, uint256 sz) = _decode_CvmXcvmUnit(p, bs);
    r.unit = x;
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmRatio(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmRatio.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmRatio.Data memory r, ) = CvmXcvmRatio._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmAbsolute(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmAbsolute.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmAbsolute.Data memory r, ) = CvmXcvmAbsolute._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmUnit(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmUnit.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmUnit.Data memory r, ) = CvmXcvmUnit._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmRatio._encode_nested(r.ratio, pointer, bs);
    
    
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmAbsolute._encode_nested(r.absolute, pointer, bs);
    
    
    pointer += ProtoBufRuntime._encode_key(
      3,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmUnit._encode_nested(r.unit, pointer, bs);
    
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmRatio._estimate(r.ratio));
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmAbsolute._estimate(r.absolute));
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmUnit._estimate(r.unit));
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    CvmXcvmRatio.store(input.ratio, output.ratio);
    CvmXcvmAbsolute.store(input.absolute, output.absolute);
    CvmXcvmUnit.store(input.unit, output.unit);

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmBalance

library CvmXcvmAsset {


  //struct definition
  struct Data {
    CvmCommonUint128.Data asset_id;
    CvmXcvmBalance.Data balance;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_asset_id(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_balance(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_asset_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmCommonUint128.Data memory x, uint256 sz) = _decode_CvmCommonUint128(p, bs);
    r.asset_id = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_balance(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmBalance.Data memory x, uint256 sz) = _decode_CvmXcvmBalance(p, bs);
    r.balance = x;
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmCommonUint128(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmCommonUint128.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmCommonUint128.Data memory r, ) = CvmCommonUint128._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmBalance(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmBalance.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmBalance.Data memory r, ) = CvmXcvmBalance._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmCommonUint128._encode_nested(r.asset_id, pointer, bs);
    
    
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmBalance._encode_nested(r.balance, pointer, bs);
    
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmCommonUint128._estimate(r.asset_id));
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmBalance._estimate(r.balance));
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    CvmCommonUint128.store(input.asset_id, output.asset_id);
    CvmXcvmBalance.store(input.balance, output.balance);

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmAsset

library CvmXcvmAssetAmount {


  //struct definition
  struct Data {
    CvmCommonUint128.Data asset_id;
    CvmXcvmBalance.Data balance;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_asset_id(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_balance(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_asset_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmCommonUint128.Data memory x, uint256 sz) = _decode_CvmCommonUint128(p, bs);
    r.asset_id = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_balance(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmBalance.Data memory x, uint256 sz) = _decode_CvmXcvmBalance(p, bs);
    r.balance = x;
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmCommonUint128(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmCommonUint128.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmCommonUint128.Data memory r, ) = CvmCommonUint128._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmBalance(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmBalance.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmBalance.Data memory r, ) = CvmXcvmBalance._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmCommonUint128._encode_nested(r.asset_id, pointer, bs);
    
    
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmBalance._encode_nested(r.balance, pointer, bs);
    
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmCommonUint128._estimate(r.asset_id));
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmBalance._estimate(r.balance));
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    CvmCommonUint128.store(input.asset_id, output.asset_id);
    CvmXcvmBalance.store(input.balance, output.balance);

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmAssetAmount

library CvmXcvmBindingValue {


  //struct definition
  struct Data {
    XCVM_PROTO_GLOBAL_ENUMS.Register register;
    CvmCommonUint128.Data asset_id;
    CvmXcvmAssetAmount.Data asset_amount;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_register(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_asset_id(pointer, bs, r);
      } else
      if (fieldId == 3) {
        pointer += _read_asset_amount(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_register(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (int64 tmp, uint256 sz) = ProtoBufRuntime._decode_enum(p, bs);
    XCVM_PROTO_GLOBAL_ENUMS.Register x = XCVM_PROTO_GLOBAL_ENUMS.decode_Register(tmp);
    r.register = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_asset_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmCommonUint128.Data memory x, uint256 sz) = _decode_CvmCommonUint128(p, bs);
    r.asset_id = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_asset_amount(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmAssetAmount.Data memory x, uint256 sz) = _decode_CvmXcvmAssetAmount(p, bs);
    r.asset_amount = x;
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmCommonUint128(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmCommonUint128.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmCommonUint128.Data memory r, ) = CvmCommonUint128._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmAssetAmount(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmAssetAmount.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmAssetAmount.Data memory r, ) = CvmXcvmAssetAmount._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    if (uint(r.register) != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    int32 _enum_register = XCVM_PROTO_GLOBAL_ENUMS.encode_Register(r.register);
    pointer += ProtoBufRuntime._encode_enum(_enum_register, pointer, bs);
    }
    
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmCommonUint128._encode_nested(r.asset_id, pointer, bs);
    
    
    pointer += ProtoBufRuntime._encode_key(
      3,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmAssetAmount._encode_nested(r.asset_amount, pointer, bs);
    
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;
    e += 1 + ProtoBufRuntime._sz_enum(XCVM_PROTO_GLOBAL_ENUMS.encode_Register(r.register));
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmCommonUint128._estimate(r.asset_id));
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmAssetAmount._estimate(r.asset_amount));
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (uint(r.register) != 0) {
    return false;
  }

    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    output.register = input.register;
    CvmCommonUint128.store(input.asset_id, output.asset_id);
    CvmXcvmAssetAmount.store(input.asset_amount, output.asset_amount);

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmBindingValue

library CvmXcvmBinding {


  //struct definition
  struct Data {
    uint32 position;
    CvmXcvmBindingValue.Data binding_value;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_position(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_binding_value(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_position(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (uint32 x, uint256 sz) = ProtoBufRuntime._decode_uint32(p, bs);
    r.position = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_binding_value(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmBindingValue.Data memory x, uint256 sz) = _decode_CvmXcvmBindingValue(p, bs);
    r.binding_value = x;
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmBindingValue(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmBindingValue.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmBindingValue.Data memory r, ) = CvmXcvmBindingValue._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    if (r.position != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_uint32(r.position, pointer, bs);
    }
    
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmBindingValue._encode_nested(r.binding_value, pointer, bs);
    
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;
    e += 1 + ProtoBufRuntime._sz_uint32(r.position);
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmBindingValue._estimate(r.binding_value));
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (r.position != 0) {
    return false;
  }

    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    output.position = input.position;
    CvmXcvmBindingValue.store(input.binding_value, output.binding_value);

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmBinding

library CvmXcvmTransfer {


  //struct definition
  struct Data {
    bytes account;
    CvmXcvmTip.Data tip;
    CvmXcvmAsset.Data[] assets;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint[4] memory counters;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_account(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_tip(pointer, bs, r);
      } else
      if (fieldId == 3) {
        pointer += _read_unpacked_repeated_assets(pointer, bs, nil(), counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    pointer = offset;
    if (counters[3] > 0) {
      require(r.assets.length == 0);
      r.assets = new CvmXcvmAsset.Data[](counters[3]);
    }

    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 3) {
        pointer += _read_unpacked_repeated_assets(pointer, bs, r, counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }
    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_account(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (bytes memory x, uint256 sz) = ProtoBufRuntime._decode_bytes(p, bs);
    r.account = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_tip(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmTip.Data memory x, uint256 sz) = _decode_CvmXcvmTip(p, bs);
    r.tip = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @param counters The counters for repeated fields
   * @return The number of bytes decoded
   */
  function _read_unpacked_repeated_assets(
    uint256 p,
    bytes memory bs,
    Data memory r,
    uint[4] memory counters
  ) internal pure returns (uint) {
    /**
     * if `r` is NULL, then only counting the number of fields.
     */
    (CvmXcvmAsset.Data memory x, uint256 sz) = _decode_CvmXcvmAsset(p, bs);
    if (isNil(r)) {
      counters[3] += 1;
    } else {
      r.assets[r.assets.length - counters[3]] = x;
      counters[3] -= 1;
    }
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmTip(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmTip.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmTip.Data memory r, ) = CvmXcvmTip._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmAsset(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmAsset.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmAsset.Data memory r, ) = CvmXcvmAsset._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    uint256 i;
    if (r.account.length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_bytes(r.account, pointer, bs);
    }
    
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmTip._encode_nested(r.tip, pointer, bs);
    
    if (r.assets.length != 0) {
    for(i = 0; i < r.assets.length; i++) {
      pointer += ProtoBufRuntime._encode_key(
        3,
        ProtoBufRuntime.WireType.LengthDelim,
        pointer,
        bs)
      ;
      pointer += CvmXcvmAsset._encode_nested(r.assets[i], pointer, bs);
    }
    }
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;uint256 i;
    e += 1 + ProtoBufRuntime._sz_lendelim(r.account.length);
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmTip._estimate(r.tip));
    for(i = 0; i < r.assets.length; i++) {
      e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmAsset._estimate(r.assets[i]));
    }
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (r.account.length != 0) {
    return false;
  }

  if (r.assets.length != 0) {
    return false;
  }

    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    output.account = input.account;
    CvmXcvmTip.store(input.tip, output.tip);

    for(uint256 i3 = 0; i3 < input.assets.length; i3++) {
      output.assets.push(input.assets[i3]);
    }
    

  }


  //array helpers for Assets
  /**
   * @dev Add value to an array
   * @param self The in-memory struct
   * @param value The value to add
   */
  function addAssets(Data memory self, CvmXcvmAsset.Data memory value) internal pure {
    /**
     * First resize the array. Then add the new element to the end.
     */
    CvmXcvmAsset.Data[] memory tmp = new CvmXcvmAsset.Data[](self.assets.length + 1);
    for (uint256 i = 0; i < self.assets.length; i++) {
      tmp[i] = self.assets[i];
    }
    tmp[self.assets.length] = value;
    self.assets = tmp;
  }


  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmTransfer

library CvmXcvmTip {


  //struct definition
  struct Data {
    bool x;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
    }
    return (r, sz);
  }

  // field readers


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory /* r */
  ) internal pure returns (uint) {
    uint256 e;
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {

  }



  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmTip

library CvmXcvmExchange {


  //struct definition
  struct Data {
    CvmCommonUint128.Data exchange_id;
    CvmXcvmAsset.Data[] give;
    CvmXcvmAsset.Data[] want;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint[7] memory counters;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_exchange_id(pointer, bs, r);
      } else
      if (fieldId == 5) {
        pointer += _read_unpacked_repeated_give(pointer, bs, nil(), counters);
      } else
      if (fieldId == 6) {
        pointer += _read_unpacked_repeated_want(pointer, bs, nil(), counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    pointer = offset;
    if (counters[5] > 0) {
      require(r.give.length == 0);
      r.give = new CvmXcvmAsset.Data[](counters[5]);
    }
    if (counters[6] > 0) {
      require(r.want.length == 0);
      r.want = new CvmXcvmAsset.Data[](counters[6]);
    }

    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 5) {
        pointer += _read_unpacked_repeated_give(pointer, bs, r, counters);
      } else
      if (fieldId == 6) {
        pointer += _read_unpacked_repeated_want(pointer, bs, r, counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }
    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_exchange_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmCommonUint128.Data memory x, uint256 sz) = _decode_CvmCommonUint128(p, bs);
    r.exchange_id = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @param counters The counters for repeated fields
   * @return The number of bytes decoded
   */
  function _read_unpacked_repeated_give(
    uint256 p,
    bytes memory bs,
    Data memory r,
    uint[7] memory counters
  ) internal pure returns (uint) {
    /**
     * if `r` is NULL, then only counting the number of fields.
     */
    (CvmXcvmAsset.Data memory x, uint256 sz) = _decode_CvmXcvmAsset(p, bs);
    if (isNil(r)) {
      counters[5] += 1;
    } else {
      r.give[r.give.length - counters[5]] = x;
      counters[5] -= 1;
    }
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @param counters The counters for repeated fields
   * @return The number of bytes decoded
   */
  function _read_unpacked_repeated_want(
    uint256 p,
    bytes memory bs,
    Data memory r,
    uint[7] memory counters
  ) internal pure returns (uint) {
    /**
     * if `r` is NULL, then only counting the number of fields.
     */
    (CvmXcvmAsset.Data memory x, uint256 sz) = _decode_CvmXcvmAsset(p, bs);
    if (isNil(r)) {
      counters[6] += 1;
    } else {
      r.want[r.want.length - counters[6]] = x;
      counters[6] -= 1;
    }
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmCommonUint128(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmCommonUint128.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmCommonUint128.Data memory r, ) = CvmCommonUint128._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmAsset(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmAsset.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmAsset.Data memory r, ) = CvmXcvmAsset._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    uint256 i;
    
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmCommonUint128._encode_nested(r.exchange_id, pointer, bs);
    
    if (r.give.length != 0) {
    for(i = 0; i < r.give.length; i++) {
      pointer += ProtoBufRuntime._encode_key(
        5,
        ProtoBufRuntime.WireType.LengthDelim,
        pointer,
        bs)
      ;
      pointer += CvmXcvmAsset._encode_nested(r.give[i], pointer, bs);
    }
    }
    if (r.want.length != 0) {
    for(i = 0; i < r.want.length; i++) {
      pointer += ProtoBufRuntime._encode_key(
        6,
        ProtoBufRuntime.WireType.LengthDelim,
        pointer,
        bs)
      ;
      pointer += CvmXcvmAsset._encode_nested(r.want[i], pointer, bs);
    }
    }
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;uint256 i;
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmCommonUint128._estimate(r.exchange_id));
    for(i = 0; i < r.give.length; i++) {
      e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmAsset._estimate(r.give[i]));
    }
    for(i = 0; i < r.want.length; i++) {
      e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmAsset._estimate(r.want[i]));
    }
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (r.give.length != 0) {
    return false;
  }

  if (r.want.length != 0) {
    return false;
  }

    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    CvmCommonUint128.store(input.exchange_id, output.exchange_id);

    for(uint256 i5 = 0; i5 < input.give.length; i5++) {
      output.give.push(input.give[i5]);
    }
    

    for(uint256 i6 = 0; i6 < input.want.length; i6++) {
      output.want.push(input.want[i6]);
    }
    

  }


  //array helpers for Give
  /**
   * @dev Add value to an array
   * @param self The in-memory struct
   * @param value The value to add
   */
  function addGive(Data memory self, CvmXcvmAsset.Data memory value) internal pure {
    /**
     * First resize the array. Then add the new element to the end.
     */
    CvmXcvmAsset.Data[] memory tmp = new CvmXcvmAsset.Data[](self.give.length + 1);
    for (uint256 i = 0; i < self.give.length; i++) {
      tmp[i] = self.give[i];
    }
    tmp[self.give.length] = value;
    self.give = tmp;
  }

  //array helpers for Want
  /**
   * @dev Add value to an array
   * @param self The in-memory struct
   * @param value The value to add
   */
  function addWant(Data memory self, CvmXcvmAsset.Data memory value) internal pure {
    /**
     * First resize the array. Then add the new element to the end.
     */
    CvmXcvmAsset.Data[] memory tmp = new CvmXcvmAsset.Data[](self.want.length + 1);
    for (uint256 i = 0; i < self.want.length; i++) {
      tmp[i] = self.want[i];
    }
    tmp[self.want.length] = value;
    self.want = tmp;
  }


  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmExchange

library CvmXcvmSpawn {


  //struct definition
  struct Data {
    uint32 network_id;
    bytes salt;
    CvmXcvmProgram.Data program;
    CvmXcvmAsset.Data[] assets;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint[6] memory counters;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_network_id(pointer, bs, r);
      } else
      if (fieldId == 3) {
        pointer += _read_salt(pointer, bs, r);
      } else
      if (fieldId == 4) {
        pointer += _read_program(pointer, bs, r);
      } else
      if (fieldId == 5) {
        pointer += _read_unpacked_repeated_assets(pointer, bs, nil(), counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    pointer = offset;
    if (counters[5] > 0) {
      require(r.assets.length == 0);
      r.assets = new CvmXcvmAsset.Data[](counters[5]);
    }

    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 5) {
        pointer += _read_unpacked_repeated_assets(pointer, bs, r, counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }
    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_network_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (uint32 x, uint256 sz) = ProtoBufRuntime._decode_uint32(p, bs);
    r.network_id = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_salt(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (bytes memory x, uint256 sz) = ProtoBufRuntime._decode_bytes(p, bs);
    r.salt = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_program(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (CvmXcvmProgram.Data memory x, uint256 sz) = _decode_CvmXcvmProgram(p, bs);
    r.program = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @param counters The counters for repeated fields
   * @return The number of bytes decoded
   */
  function _read_unpacked_repeated_assets(
    uint256 p,
    bytes memory bs,
    Data memory r,
    uint[6] memory counters
  ) internal pure returns (uint) {
    /**
     * if `r` is NULL, then only counting the number of fields.
     */
    (CvmXcvmAsset.Data memory x, uint256 sz) = _decode_CvmXcvmAsset(p, bs);
    if (isNil(r)) {
      counters[5] += 1;
    } else {
      r.assets[r.assets.length - counters[5]] = x;
      counters[5] -= 1;
    }
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmProgram(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmProgram.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmProgram.Data memory r, ) = CvmXcvmProgram._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }

  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmAsset(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmAsset.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmAsset.Data memory r, ) = CvmXcvmAsset._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    uint256 i;
    if (r.network_id != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_uint32(r.network_id, pointer, bs);
    }
    if (r.salt.length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      3,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_bytes(r.salt, pointer, bs);
    }
    
    pointer += ProtoBufRuntime._encode_key(
      4,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += CvmXcvmProgram._encode_nested(r.program, pointer, bs);
    
    if (r.assets.length != 0) {
    for(i = 0; i < r.assets.length; i++) {
      pointer += ProtoBufRuntime._encode_key(
        5,
        ProtoBufRuntime.WireType.LengthDelim,
        pointer,
        bs)
      ;
      pointer += CvmXcvmAsset._encode_nested(r.assets[i], pointer, bs);
    }
    }
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;uint256 i;
    e += 1 + ProtoBufRuntime._sz_uint32(r.network_id);
    e += 1 + ProtoBufRuntime._sz_lendelim(r.salt.length);
    e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmProgram._estimate(r.program));
    for(i = 0; i < r.assets.length; i++) {
      e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmAsset._estimate(r.assets[i]));
    }
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (r.network_id != 0) {
    return false;
  }

  if (r.salt.length != 0) {
    return false;
  }

  if (r.assets.length != 0) {
    return false;
  }

    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    output.network_id = input.network_id;
    output.salt = input.salt;
    CvmXcvmProgram.store(input.program, output.program);

    for(uint256 i5 = 0; i5 < input.assets.length; i5++) {
      output.assets.push(input.assets[i5]);
    }
    

  }


  //array helpers for Assets
  /**
   * @dev Add value to an array
   * @param self The in-memory struct
   * @param value The value to add
   */
  function addAssets(Data memory self, CvmXcvmAsset.Data memory value) internal pure {
    /**
     * First resize the array. Then add the new element to the end.
     */
    CvmXcvmAsset.Data[] memory tmp = new CvmXcvmAsset.Data[](self.assets.length + 1);
    for (uint256 i = 0; i < self.assets.length; i++) {
      tmp[i] = self.assets[i];
    }
    tmp[self.assets.length] = value;
    self.assets = tmp;
  }


  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmSpawn

library CvmXcvmCall {


  //struct definition
  struct Data {
    bytes payload;
    CvmXcvmBinding.Data[] bindings;
  }

  // Decoder section

  /**
   * @dev The main decoder for memory
   * @param bs The bytes array to be decoded
   * @return The decoded struct
   */
  function decode(bytes memory bs) internal pure returns (Data memory) {
    (Data memory x, ) = _decode(32, bs, bs.length);
    return x;
  }

  /**
   * @dev The main decoder for storage
   * @param self The in-storage struct
   * @param bs The bytes array to be decoded
   */
  function decode(Data storage self, bytes memory bs) internal {
    (Data memory x, ) = _decode(32, bs, bs.length);
    store(x, self);
  }
  // inner decoder

  /**
   * @dev The decoder for internal usage
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param sz The number of bytes expected
   * @return The decoded struct
   * @return The number of bytes decoded
   */
  function _decode(uint256 p, bytes memory bs, uint256 sz)
    internal
    pure
    returns (Data memory, uint)
  {
    Data memory r;
    uint[3] memory counters;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_payload(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_unpacked_repeated_bindings(pointer, bs, nil(), counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    pointer = offset;
    if (counters[2] > 0) {
      require(r.bindings.length == 0);
      r.bindings = new CvmXcvmBinding.Data[](counters[2]);
    }

    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 2) {
        pointer += _read_unpacked_repeated_bindings(pointer, bs, r, counters);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }
    }
    return (r, sz);
  }

  // field readers

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_payload(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (bytes memory x, uint256 sz) = ProtoBufRuntime._decode_bytes(p, bs);
    r.payload = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @param counters The counters for repeated fields
   * @return The number of bytes decoded
   */
  function _read_unpacked_repeated_bindings(
    uint256 p,
    bytes memory bs,
    Data memory r,
    uint[3] memory counters
  ) internal pure returns (uint) {
    /**
     * if `r` is NULL, then only counting the number of fields.
     */
    (CvmXcvmBinding.Data memory x, uint256 sz) = _decode_CvmXcvmBinding(p, bs);
    if (isNil(r)) {
      counters[2] += 1;
    } else {
      r.bindings[r.bindings.length - counters[2]] = x;
      counters[2] -= 1;
    }
    return sz;
  }

  // struct decoder
  /**
   * @dev The decoder for reading a inner struct field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The decoded inner-struct
   * @return The number of bytes used to decode
   */
  function _decode_CvmXcvmBinding(uint256 p, bytes memory bs)
    internal
    pure
    returns (CvmXcvmBinding.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (CvmXcvmBinding.Data memory r, ) = CvmXcvmBinding._decode(pointer, bs, sz);
    return (r, sz + bytesRead);
  }


  // Encoder section

  /**
   * @dev The main encoder for memory
   * @param r The struct to be encoded
   * @return The encoded byte array
   */
  function encode(Data memory r) internal pure returns (bytes memory) {
    bytes memory bs = new bytes(_estimate(r));
    uint256 sz = _encode(r, 32, bs);
    assembly {
      mstore(bs, sz)
    }
    return bs;
  }
  // inner encoder

  /**
   * @dev The encoder for internal usage
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    uint256 offset = p;
    uint256 pointer = p;
    uint256 i;
    if (r.payload.length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_bytes(r.payload, pointer, bs);
    }
    if (r.bindings.length != 0) {
    for(i = 0; i < r.bindings.length; i++) {
      pointer += ProtoBufRuntime._encode_key(
        2,
        ProtoBufRuntime.WireType.LengthDelim,
        pointer,
        bs)
      ;
      pointer += CvmXcvmBinding._encode_nested(r.bindings[i], pointer, bs);
    }
    }
    return pointer - offset;
  }
  // nested encoder

  /**
   * @dev The encoder for inner struct
   * @param r The struct to be encoded
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @return The number of bytes encoded
   */
  function _encode_nested(Data memory r, uint256 p, bytes memory bs)
    internal
    pure
    returns (uint)
  {
    /**
     * First encoded `r` into a temporary array, and encode the actual size used.
     * Then copy the temporary array into `bs`.
     */
    uint256 offset = p;
    uint256 pointer = p;
    bytes memory tmp = new bytes(_estimate(r));
    uint256 tmpAddr = ProtoBufRuntime.getMemoryAddress(tmp);
    uint256 bsAddr = ProtoBufRuntime.getMemoryAddress(bs);
    uint256 size = _encode(r, 32, tmp);
    pointer += ProtoBufRuntime._encode_varint(size, pointer, bs);
    ProtoBufRuntime.copyBytes(tmpAddr + 32, bsAddr + pointer, size);
    pointer += size;
    delete tmp;
    return pointer - offset;
  }
  // estimator

  /**
   * @dev The estimator for a struct
   * @param r The struct to be encoded
   * @return The number of bytes encoded in estimation
   */
  function _estimate(
    Data memory r
  ) internal pure returns (uint) {
    uint256 e;uint256 i;
    e += 1 + ProtoBufRuntime._sz_lendelim(r.payload.length);
    for(i = 0; i < r.bindings.length; i++) {
      e += 1 + ProtoBufRuntime._sz_lendelim(CvmXcvmBinding._estimate(r.bindings[i]));
    }
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (r.payload.length != 0) {
    return false;
  }

  if (r.bindings.length != 0) {
    return false;
  }

    return true;
  }


  //store function
  /**
   * @dev Store in-memory struct to storage
   * @param input The in-memory struct
   * @param output The in-storage struct
   */
  function store(Data memory input, Data storage output) internal {
    output.payload = input.payload;

    for(uint256 i2 = 0; i2 < input.bindings.length; i2++) {
      output.bindings.push(input.bindings[i2]);
    }
    

  }


  //array helpers for Bindings
  /**
   * @dev Add value to an array
   * @param self The in-memory struct
   * @param value The value to add
   */
  function addBindings(Data memory self, CvmXcvmBinding.Data memory value) internal pure {
    /**
     * First resize the array. Then add the new element to the end.
     */
    CvmXcvmBinding.Data[] memory tmp = new CvmXcvmBinding.Data[](self.bindings.length + 1);
    for (uint256 i = 0; i < self.bindings.length; i++) {
      tmp[i] = self.bindings[i];
    }
    tmp[self.bindings.length] = value;
    self.bindings = tmp;
  }


  //utility functions
  /**
   * @dev Return an empty struct
   * @return r The empty struct
   */
  function nil() internal pure returns (Data memory r) {
    assembly {
      r := 0
    }
  }

  /**
   * @dev Test whether a struct is empty
   * @param x The struct to be tested
   * @return r True if it is empty
   */
  function isNil(Data memory x) internal pure returns (bool r) {
    assembly {
      r := iszero(x)
    }
  }
}
//library CvmXcvmCall

library XCVM_PROTO_GLOBAL_ENUMS {

  //enum definition
  // Solidity enum definitions
  enum Register {
    IP,
    TIP,
    THIS,
    RESULT
  }


  // Solidity enum encoder
  function encode_Register(Register x) internal pure returns (int32) {
    
    if (x == Register.IP) {
      return 0;
    }

    if (x == Register.TIP) {
      return 1;
    }

    if (x == Register.THIS) {
      return 2;
    }

    if (x == Register.RESULT) {
      return 3;
    }
    revert();
  }


  // Solidity enum decoder
  function decode_Register(int64 x) internal pure returns (Register) {
    
    if (x == 0) {
      return Register.IP;
    }

    if (x == 1) {
      return Register.TIP;
    }

    if (x == 2) {
      return Register.THIS;
    }

    if (x == 3) {
      return Register.RESULT;
    }
    revert();
  }


  /**
   * @dev The estimator for an packed enum array
   * @return The number of bytes encoded
   */
  function estimate_packed_repeated_Register(
    Register[] memory a
  ) internal pure returns (uint256) {
    uint256 e = 0;
    for (uint i = 0; i < a.length; i++) {
      e += ProtoBufRuntime._sz_enum(encode_Register(a[i]));
    }
    return e;
  }
}
//library XCVM_PROTO_GLOBAL_ENUMS