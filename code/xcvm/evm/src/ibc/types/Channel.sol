// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.9;
import "./ProtoBufRuntime.sol";
import "./GoogleProtobufAny.sol";
import "./Client.sol";

library Channel {

  //enum definition
  // Solidity enum definitions
  enum State {
    STATE_UNINITIALIZED_UNSPECIFIED,
    STATE_INIT,
    STATE_TRYOPEN,
    STATE_OPEN,
    STATE_CLOSED
  }


  // Solidity enum encoder
  function encode_State(State x) internal pure returns (int32) {
    
    if (x == State.STATE_UNINITIALIZED_UNSPECIFIED) {
      return 0;
    }

    if (x == State.STATE_INIT) {
      return 1;
    }

    if (x == State.STATE_TRYOPEN) {
      return 2;
    }

    if (x == State.STATE_OPEN) {
      return 3;
    }

    if (x == State.STATE_CLOSED) {
      return 4;
    }
    revert();
  }


  // Solidity enum decoder
  function decode_State(int64 x) internal pure returns (State) {
    
    if (x == 0) {
      return State.STATE_UNINITIALIZED_UNSPECIFIED;
    }

    if (x == 1) {
      return State.STATE_INIT;
    }

    if (x == 2) {
      return State.STATE_TRYOPEN;
    }

    if (x == 3) {
      return State.STATE_OPEN;
    }

    if (x == 4) {
      return State.STATE_CLOSED;
    }
    revert();
  }


  /**
   * @dev The estimator for an packed enum array
   * @return The number of bytes encoded
   */
  function estimate_packed_repeated_State(
    State[] memory a
  ) internal pure returns (uint256) {
    uint256 e = 0;
    for (uint i = 0; i < a.length; i++) {
      e += ProtoBufRuntime._sz_enum(encode_State(a[i]));
    }
    return e;
  }

  // Solidity enum definitions
  enum Order {
    ORDER_NONE_UNSPECIFIED,
    ORDER_UNORDERED,
    ORDER_ORDERED
  }


  // Solidity enum encoder
  function encode_Order(Order x) internal pure returns (int32) {
    
    if (x == Order.ORDER_NONE_UNSPECIFIED) {
      return 0;
    }

    if (x == Order.ORDER_UNORDERED) {
      return 1;
    }

    if (x == Order.ORDER_ORDERED) {
      return 2;
    }
    revert();
  }


  // Solidity enum decoder
  function decode_Order(int64 x) internal pure returns (Order) {
    
    if (x == 0) {
      return Order.ORDER_NONE_UNSPECIFIED;
    }

    if (x == 1) {
      return Order.ORDER_UNORDERED;
    }

    if (x == 2) {
      return Order.ORDER_ORDERED;
    }
    revert();
  }


  /**
   * @dev The estimator for an packed enum array
   * @return The number of bytes encoded
   */
  function estimate_packed_repeated_Order(
    Order[] memory a
  ) internal pure returns (uint256) {
    uint256 e = 0;
    for (uint i = 0; i < a.length; i++) {
      e += ProtoBufRuntime._sz_enum(encode_Order(a[i]));
    }
    return e;
  }

  //struct definition
  struct Data {
    Channel.State state;
    Channel.Order ordering;
    ChannelCounterparty.Data counterparty;
    string[] connection_hops;
    string version;
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
        pointer += _read_state(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_ordering(pointer, bs, r);
      } else
      if (fieldId == 3) {
        pointer += _read_counterparty(pointer, bs, r);
      } else
      if (fieldId == 4) {
        pointer += _read_unpacked_repeated_connection_hops(pointer, bs, nil(), counters);
      } else
      if (fieldId == 5) {
        pointer += _read_version(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    pointer = offset;
    if (counters[4] > 0) {
      require(r.connection_hops.length == 0);
      r.connection_hops = new string[](counters[4]);
    }

    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 4) {
        pointer += _read_unpacked_repeated_connection_hops(pointer, bs, r, counters);
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
  function _read_state(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (int64 tmp, uint256 sz) = ProtoBufRuntime._decode_enum(p, bs);
    Channel.State x = decode_State(tmp);
    r.state = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_ordering(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (int64 tmp, uint256 sz) = ProtoBufRuntime._decode_enum(p, bs);
    Channel.Order x = decode_Order(tmp);
    r.ordering = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_counterparty(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (ChannelCounterparty.Data memory x, uint256 sz) = _decode_ChannelCounterparty(p, bs);
    r.counterparty = x;
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
  function _read_unpacked_repeated_connection_hops(
    uint256 p,
    bytes memory bs,
    Data memory r,
    uint[6] memory counters
  ) internal pure returns (uint) {
    /**
     * if `r` is NULL, then only counting the number of fields.
     */
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    if (isNil(r)) {
      counters[4] += 1;
    } else {
      r.connection_hops[r.connection_hops.length - counters[4]] = x;
      counters[4] -= 1;
    }
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_version(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.version = x;
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
  function _decode_ChannelCounterparty(uint256 p, bytes memory bs)
    internal
    pure
    returns (ChannelCounterparty.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (ChannelCounterparty.Data memory r, ) = ChannelCounterparty._decode(pointer, bs, sz);
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
    if (uint(r.state) != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    int32 _enum_state = encode_State(r.state);
    pointer += ProtoBufRuntime._encode_enum(_enum_state, pointer, bs);
    }
    if (uint(r.ordering) != 0) {
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    int32 _enum_ordering = encode_Order(r.ordering);
    pointer += ProtoBufRuntime._encode_enum(_enum_ordering, pointer, bs);
    }
    
    pointer += ProtoBufRuntime._encode_key(
      3,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ChannelCounterparty._encode_nested(r.counterparty, pointer, bs);
    
    if (r.connection_hops.length != 0) {
    for(i = 0; i < r.connection_hops.length; i++) {
      pointer += ProtoBufRuntime._encode_key(
        4,
        ProtoBufRuntime.WireType.LengthDelim,
        pointer,
        bs)
      ;
      pointer += ProtoBufRuntime._encode_string(r.connection_hops[i], pointer, bs);
    }
    }
    if (bytes(r.version).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      5,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.version, pointer, bs);
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
    e += 1 + ProtoBufRuntime._sz_enum(encode_State(r.state));
    e += 1 + ProtoBufRuntime._sz_enum(encode_Order(r.ordering));
    e += 1 + ProtoBufRuntime._sz_lendelim(ChannelCounterparty._estimate(r.counterparty));
    for(i = 0; i < r.connection_hops.length; i++) {
      e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.connection_hops[i]).length);
    }
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.version).length);
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (uint(r.state) != 0) {
    return false;
  }

  if (uint(r.ordering) != 0) {
    return false;
  }

  if (r.connection_hops.length != 0) {
    return false;
  }

  if (bytes(r.version).length != 0) {
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
    output.state = input.state;
    output.ordering = input.ordering;
    ChannelCounterparty.store(input.counterparty, output.counterparty);
    output.connection_hops = input.connection_hops;
    output.version = input.version;

  }


  //array helpers for ConnectionHops
  /**
   * @dev Add value to an array
   * @param self The in-memory struct
   * @param value The value to add
   */
  function addConnectionHops(Data memory self, string memory value) internal pure {
    /**
     * First resize the array. Then add the new element to the end.
     */
    string[] memory tmp = new string[](self.connection_hops.length + 1);
    for (uint256 i = 0; i < self.connection_hops.length; i++) {
      tmp[i] = self.connection_hops[i];
    }
    tmp[self.connection_hops.length] = value;
    self.connection_hops = tmp;
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
//library Channel

library ChannelCounterparty {


  //struct definition
  struct Data {
    string port_id;
    string channel_id;
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
        pointer += _read_port_id(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_channel_id(pointer, bs, r);
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
  function _read_port_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.port_id = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_channel_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.channel_id = x;
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
    
    if (bytes(r.port_id).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.port_id, pointer, bs);
    }
    if (bytes(r.channel_id).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.channel_id, pointer, bs);
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
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.port_id).length);
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.channel_id).length);
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (bytes(r.port_id).length != 0) {
    return false;
  }

  if (bytes(r.channel_id).length != 0) {
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
    output.port_id = input.port_id;
    output.channel_id = input.channel_id;

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
//library ChannelCounterparty

library ChannelIdentifiedChannel {


  //struct definition
  struct Data {
    Channel.State state;
    Channel.Order ordering;
    ChannelCounterparty.Data counterparty;
    string[] connection_hops;
    string version;
    string port_id;
    string channel_id;
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
    uint[8] memory counters;
    uint256 fieldId;
    ProtoBufRuntime.WireType wireType;
    uint256 bytesRead;
    uint256 offset = p;
    uint256 pointer = p;
    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 1) {
        pointer += _read_state(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_ordering(pointer, bs, r);
      } else
      if (fieldId == 3) {
        pointer += _read_counterparty(pointer, bs, r);
      } else
      if (fieldId == 4) {
        pointer += _read_unpacked_repeated_connection_hops(pointer, bs, nil(), counters);
      } else
      if (fieldId == 5) {
        pointer += _read_version(pointer, bs, r);
      } else
      if (fieldId == 6) {
        pointer += _read_port_id(pointer, bs, r);
      } else
      if (fieldId == 7) {
        pointer += _read_channel_id(pointer, bs, r);
      } else
      {
        pointer += ProtoBufRuntime._skip_field_decode(wireType, pointer, bs);
      }

    }
    pointer = offset;
    if (counters[4] > 0) {
      require(r.connection_hops.length == 0);
      r.connection_hops = new string[](counters[4]);
    }

    while (pointer < offset + sz) {
      (fieldId, wireType, bytesRead) = ProtoBufRuntime._decode_key(pointer, bs);
      pointer += bytesRead;
      if (fieldId == 4) {
        pointer += _read_unpacked_repeated_connection_hops(pointer, bs, r, counters);
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
  function _read_state(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (int64 tmp, uint256 sz) = ProtoBufRuntime._decode_enum(p, bs);
    Channel.State x = Channel.decode_State(tmp);
    r.state = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_ordering(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (int64 tmp, uint256 sz) = ProtoBufRuntime._decode_enum(p, bs);
    Channel.Order x = Channel.decode_Order(tmp);
    r.ordering = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_counterparty(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (ChannelCounterparty.Data memory x, uint256 sz) = _decode_ChannelCounterparty(p, bs);
    r.counterparty = x;
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
  function _read_unpacked_repeated_connection_hops(
    uint256 p,
    bytes memory bs,
    Data memory r,
    uint[8] memory counters
  ) internal pure returns (uint) {
    /**
     * if `r` is NULL, then only counting the number of fields.
     */
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    if (isNil(r)) {
      counters[4] += 1;
    } else {
      r.connection_hops[r.connection_hops.length - counters[4]] = x;
      counters[4] -= 1;
    }
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_version(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.version = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_port_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.port_id = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_channel_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.channel_id = x;
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
  function _decode_ChannelCounterparty(uint256 p, bytes memory bs)
    internal
    pure
    returns (ChannelCounterparty.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (ChannelCounterparty.Data memory r, ) = ChannelCounterparty._decode(pointer, bs, sz);
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
    if (uint(r.state) != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    int32 _enum_state = Channel.encode_State(r.state);
    pointer += ProtoBufRuntime._encode_enum(_enum_state, pointer, bs);
    }
    if (uint(r.ordering) != 0) {
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    int32 _enum_ordering = Channel.encode_Order(r.ordering);
    pointer += ProtoBufRuntime._encode_enum(_enum_ordering, pointer, bs);
    }
    
    pointer += ProtoBufRuntime._encode_key(
      3,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ChannelCounterparty._encode_nested(r.counterparty, pointer, bs);
    
    if (r.connection_hops.length != 0) {
    for(i = 0; i < r.connection_hops.length; i++) {
      pointer += ProtoBufRuntime._encode_key(
        4,
        ProtoBufRuntime.WireType.LengthDelim,
        pointer,
        bs)
      ;
      pointer += ProtoBufRuntime._encode_string(r.connection_hops[i], pointer, bs);
    }
    }
    if (bytes(r.version).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      5,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.version, pointer, bs);
    }
    if (bytes(r.port_id).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      6,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.port_id, pointer, bs);
    }
    if (bytes(r.channel_id).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      7,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.channel_id, pointer, bs);
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
    e += 1 + ProtoBufRuntime._sz_enum(Channel.encode_State(r.state));
    e += 1 + ProtoBufRuntime._sz_enum(Channel.encode_Order(r.ordering));
    e += 1 + ProtoBufRuntime._sz_lendelim(ChannelCounterparty._estimate(r.counterparty));
    for(i = 0; i < r.connection_hops.length; i++) {
      e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.connection_hops[i]).length);
    }
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.version).length);
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.port_id).length);
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.channel_id).length);
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (uint(r.state) != 0) {
    return false;
  }

  if (uint(r.ordering) != 0) {
    return false;
  }

  if (r.connection_hops.length != 0) {
    return false;
  }

  if (bytes(r.version).length != 0) {
    return false;
  }

  if (bytes(r.port_id).length != 0) {
    return false;
  }

  if (bytes(r.channel_id).length != 0) {
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
    output.state = input.state;
    output.ordering = input.ordering;
    ChannelCounterparty.store(input.counterparty, output.counterparty);
    output.connection_hops = input.connection_hops;
    output.version = input.version;
    output.port_id = input.port_id;
    output.channel_id = input.channel_id;

  }


  //array helpers for ConnectionHops
  /**
   * @dev Add value to an array
   * @param self The in-memory struct
   * @param value The value to add
   */
  function addConnectionHops(Data memory self, string memory value) internal pure {
    /**
     * First resize the array. Then add the new element to the end.
     */
    string[] memory tmp = new string[](self.connection_hops.length + 1);
    for (uint256 i = 0; i < self.connection_hops.length; i++) {
      tmp[i] = self.connection_hops[i];
    }
    tmp[self.connection_hops.length] = value;
    self.connection_hops = tmp;
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
//library ChannelIdentifiedChannel

library Packet {


  //struct definition
  struct Data {
    uint64 sequence;
    string source_port;
    string source_channel;
    string destination_port;
    string destination_channel;
    bytes data;
    Height.Data timeout_height;
    uint64 timeout_timestamp;
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
        pointer += _read_sequence(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_source_port(pointer, bs, r);
      } else
      if (fieldId == 3) {
        pointer += _read_source_channel(pointer, bs, r);
      } else
      if (fieldId == 4) {
        pointer += _read_destination_port(pointer, bs, r);
      } else
      if (fieldId == 5) {
        pointer += _read_destination_channel(pointer, bs, r);
      } else
      if (fieldId == 6) {
        pointer += _read_data(pointer, bs, r);
      } else
      if (fieldId == 7) {
        pointer += _read_timeout_height(pointer, bs, r);
      } else
      if (fieldId == 8) {
        pointer += _read_timeout_timestamp(pointer, bs, r);
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
  function _read_sequence(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (uint64 x, uint256 sz) = ProtoBufRuntime._decode_uint64(p, bs);
    r.sequence = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_source_port(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.source_port = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_source_channel(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.source_channel = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_destination_port(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.destination_port = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_destination_channel(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.destination_channel = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_data(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (bytes memory x, uint256 sz) = ProtoBufRuntime._decode_bytes(p, bs);
    r.data = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_timeout_height(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (Height.Data memory x, uint256 sz) = _decode_Height(p, bs);
    r.timeout_height = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_timeout_timestamp(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (uint64 x, uint256 sz) = ProtoBufRuntime._decode_uint64(p, bs);
    r.timeout_timestamp = x;
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
  function _decode_Height(uint256 p, bytes memory bs)
    internal
    pure
    returns (Height.Data memory, uint)
  {
    uint256 pointer = p;
    (uint256 sz, uint256 bytesRead) = ProtoBufRuntime._decode_varint(pointer, bs);
    pointer += bytesRead;
    (Height.Data memory r, ) = Height._decode(pointer, bs, sz);
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
    
    if (r.sequence != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_uint64(r.sequence, pointer, bs);
    }
    if (bytes(r.source_port).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.source_port, pointer, bs);
    }
    if (bytes(r.source_channel).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      3,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.source_channel, pointer, bs);
    }
    if (bytes(r.destination_port).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      4,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.destination_port, pointer, bs);
    }
    if (bytes(r.destination_channel).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      5,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.destination_channel, pointer, bs);
    }
    if (r.data.length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      6,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_bytes(r.data, pointer, bs);
    }
    
    pointer += ProtoBufRuntime._encode_key(
      7,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += Height._encode_nested(r.timeout_height, pointer, bs);
    
    if (r.timeout_timestamp != 0) {
    pointer += ProtoBufRuntime._encode_key(
      8,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_uint64(r.timeout_timestamp, pointer, bs);
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
    e += 1 + ProtoBufRuntime._sz_uint64(r.sequence);
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.source_port).length);
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.source_channel).length);
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.destination_port).length);
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.destination_channel).length);
    e += 1 + ProtoBufRuntime._sz_lendelim(r.data.length);
    e += 1 + ProtoBufRuntime._sz_lendelim(Height._estimate(r.timeout_height));
    e += 1 + ProtoBufRuntime._sz_uint64(r.timeout_timestamp);
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (r.sequence != 0) {
    return false;
  }

  if (bytes(r.source_port).length != 0) {
    return false;
  }

  if (bytes(r.source_channel).length != 0) {
    return false;
  }

  if (bytes(r.destination_port).length != 0) {
    return false;
  }

  if (bytes(r.destination_channel).length != 0) {
    return false;
  }

  if (r.data.length != 0) {
    return false;
  }

  if (r.timeout_timestamp != 0) {
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
    output.sequence = input.sequence;
    output.source_port = input.source_port;
    output.source_channel = input.source_channel;
    output.destination_port = input.destination_port;
    output.destination_channel = input.destination_channel;
    output.data = input.data;
    Height.store(input.timeout_height, output.timeout_height);
    output.timeout_timestamp = input.timeout_timestamp;

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
//library Packet

library PacketState {


  //struct definition
  struct Data {
    string port_id;
    string channel_id;
    uint64 sequence;
    bytes data;
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
        pointer += _read_port_id(pointer, bs, r);
      } else
      if (fieldId == 2) {
        pointer += _read_channel_id(pointer, bs, r);
      } else
      if (fieldId == 3) {
        pointer += _read_sequence(pointer, bs, r);
      } else
      if (fieldId == 4) {
        pointer += _read_data(pointer, bs, r);
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
  function _read_port_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.port_id = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_channel_id(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (string memory x, uint256 sz) = ProtoBufRuntime._decode_string(p, bs);
    r.channel_id = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_sequence(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (uint64 x, uint256 sz) = ProtoBufRuntime._decode_uint64(p, bs);
    r.sequence = x;
    return sz;
  }

  /**
   * @dev The decoder for reading a field
   * @param p The offset of bytes array to start decode
   * @param bs The bytes array to be decoded
   * @param r The in-memory struct
   * @return The number of bytes decoded
   */
  function _read_data(
    uint256 p,
    bytes memory bs,
    Data memory r
  ) internal pure returns (uint) {
    (bytes memory x, uint256 sz) = ProtoBufRuntime._decode_bytes(p, bs);
    r.data = x;
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
    
    if (bytes(r.port_id).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      1,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.port_id, pointer, bs);
    }
    if (bytes(r.channel_id).length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      2,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_string(r.channel_id, pointer, bs);
    }
    if (r.sequence != 0) {
    pointer += ProtoBufRuntime._encode_key(
      3,
      ProtoBufRuntime.WireType.Varint,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_uint64(r.sequence, pointer, bs);
    }
    if (r.data.length != 0) {
    pointer += ProtoBufRuntime._encode_key(
      4,
      ProtoBufRuntime.WireType.LengthDelim,
      pointer,
      bs
    );
    pointer += ProtoBufRuntime._encode_bytes(r.data, pointer, bs);
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
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.port_id).length);
    e += 1 + ProtoBufRuntime._sz_lendelim(bytes(r.channel_id).length);
    e += 1 + ProtoBufRuntime._sz_uint64(r.sequence);
    e += 1 + ProtoBufRuntime._sz_lendelim(r.data.length);
    return e;
  }
  // empty checker

  function _empty(
    Data memory r
  ) internal pure returns (bool) {
    
  if (bytes(r.port_id).length != 0) {
    return false;
  }

  if (bytes(r.channel_id).length != 0) {
    return false;
  }

  if (r.sequence != 0) {
    return false;
  }

  if (r.data.length != 0) {
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
    output.port_id = input.port_id;
    output.channel_id = input.channel_id;
    output.sequence = input.sequence;
    output.data = input.data;

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
//library PacketState