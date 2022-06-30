'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

function _interopDefault (ex) { return (ex && (typeof ex === 'object') && 'default' in ex) ? ex['default'] : ex; }

function _interopNamespace(e) {
  if (e && e.__esModule) { return e; } else {
    var n = {};
    if (e) {
      Object.keys(e).forEach(function (k) {
        var d = Object.getOwnPropertyDescriptor(e, k);
        Object.defineProperty(n, k, d.get ? d : {
          enumerable: true,
          get: function () {
            return e[k];
          }
        });
      });
    }
    n['default'] = e;
    return n;
  }
}

var React = require('react');
var React__default = _interopDefault(React);
var create = _interopDefault(require('zustand'));
var produce = _interopDefault(require('immer'));
var api = require('@polkadot/api');
var rpcProvider = require('@polkadot/rpc-provider');

function _regeneratorRuntime() {
  /*! regenerator-runtime -- Copyright (c) 2014-present, Facebook, Inc. -- license (MIT): https://github.com/facebook/regenerator/blob/main/LICENSE */

  _regeneratorRuntime = function () {
    return exports;
  };

  var exports = {},
      Op = Object.prototype,
      hasOwn = Op.hasOwnProperty,
      $Symbol = "function" == typeof Symbol ? Symbol : {},
      iteratorSymbol = $Symbol.iterator || "@@iterator",
      asyncIteratorSymbol = $Symbol.asyncIterator || "@@asyncIterator",
      toStringTagSymbol = $Symbol.toStringTag || "@@toStringTag";

  function define(obj, key, value) {
    return Object.defineProperty(obj, key, {
      value: value,
      enumerable: !0,
      configurable: !0,
      writable: !0
    }), obj[key];
  }

  try {
    define({}, "");
  } catch (err) {
    define = function (obj, key, value) {
      return obj[key] = value;
    };
  }

  function wrap(innerFn, outerFn, self, tryLocsList) {
    var protoGenerator = outerFn && outerFn.prototype instanceof Generator ? outerFn : Generator,
        generator = Object.create(protoGenerator.prototype),
        context = new Context(tryLocsList || []);
    return generator._invoke = function (innerFn, self, context) {
      var state = "suspendedStart";
      return function (method, arg) {
        if ("executing" === state) throw new Error("Generator is already running");

        if ("completed" === state) {
          if ("throw" === method) throw arg;
          return doneResult();
        }

        for (context.method = method, context.arg = arg;;) {
          var delegate = context.delegate;

          if (delegate) {
            var delegateResult = maybeInvokeDelegate(delegate, context);

            if (delegateResult) {
              if (delegateResult === ContinueSentinel) continue;
              return delegateResult;
            }
          }

          if ("next" === context.method) context.sent = context._sent = context.arg;else if ("throw" === context.method) {
            if ("suspendedStart" === state) throw state = "completed", context.arg;
            context.dispatchException(context.arg);
          } else "return" === context.method && context.abrupt("return", context.arg);
          state = "executing";
          var record = tryCatch(innerFn, self, context);

          if ("normal" === record.type) {
            if (state = context.done ? "completed" : "suspendedYield", record.arg === ContinueSentinel) continue;
            return {
              value: record.arg,
              done: context.done
            };
          }

          "throw" === record.type && (state = "completed", context.method = "throw", context.arg = record.arg);
        }
      };
    }(innerFn, self, context), generator;
  }

  function tryCatch(fn, obj, arg) {
    try {
      return {
        type: "normal",
        arg: fn.call(obj, arg)
      };
    } catch (err) {
      return {
        type: "throw",
        arg: err
      };
    }
  }

  exports.wrap = wrap;
  var ContinueSentinel = {};

  function Generator() {}

  function GeneratorFunction() {}

  function GeneratorFunctionPrototype() {}

  var IteratorPrototype = {};
  define(IteratorPrototype, iteratorSymbol, function () {
    return this;
  });
  var getProto = Object.getPrototypeOf,
      NativeIteratorPrototype = getProto && getProto(getProto(values([])));
  NativeIteratorPrototype && NativeIteratorPrototype !== Op && hasOwn.call(NativeIteratorPrototype, iteratorSymbol) && (IteratorPrototype = NativeIteratorPrototype);
  var Gp = GeneratorFunctionPrototype.prototype = Generator.prototype = Object.create(IteratorPrototype);

  function defineIteratorMethods(prototype) {
    ["next", "throw", "return"].forEach(function (method) {
      define(prototype, method, function (arg) {
        return this._invoke(method, arg);
      });
    });
  }

  function AsyncIterator(generator, PromiseImpl) {
    function invoke(method, arg, resolve, reject) {
      var record = tryCatch(generator[method], generator, arg);

      if ("throw" !== record.type) {
        var result = record.arg,
            value = result.value;
        return value && "object" == typeof value && hasOwn.call(value, "__await") ? PromiseImpl.resolve(value.__await).then(function (value) {
          invoke("next", value, resolve, reject);
        }, function (err) {
          invoke("throw", err, resolve, reject);
        }) : PromiseImpl.resolve(value).then(function (unwrapped) {
          result.value = unwrapped, resolve(result);
        }, function (error) {
          return invoke("throw", error, resolve, reject);
        });
      }

      reject(record.arg);
    }

    var previousPromise;

    this._invoke = function (method, arg) {
      function callInvokeWithMethodAndArg() {
        return new PromiseImpl(function (resolve, reject) {
          invoke(method, arg, resolve, reject);
        });
      }

      return previousPromise = previousPromise ? previousPromise.then(callInvokeWithMethodAndArg, callInvokeWithMethodAndArg) : callInvokeWithMethodAndArg();
    };
  }

  function maybeInvokeDelegate(delegate, context) {
    var method = delegate.iterator[context.method];

    if (undefined === method) {
      if (context.delegate = null, "throw" === context.method) {
        if (delegate.iterator.return && (context.method = "return", context.arg = undefined, maybeInvokeDelegate(delegate, context), "throw" === context.method)) return ContinueSentinel;
        context.method = "throw", context.arg = new TypeError("The iterator does not provide a 'throw' method");
      }

      return ContinueSentinel;
    }

    var record = tryCatch(method, delegate.iterator, context.arg);
    if ("throw" === record.type) return context.method = "throw", context.arg = record.arg, context.delegate = null, ContinueSentinel;
    var info = record.arg;
    return info ? info.done ? (context[delegate.resultName] = info.value, context.next = delegate.nextLoc, "return" !== context.method && (context.method = "next", context.arg = undefined), context.delegate = null, ContinueSentinel) : info : (context.method = "throw", context.arg = new TypeError("iterator result is not an object"), context.delegate = null, ContinueSentinel);
  }

  function pushTryEntry(locs) {
    var entry = {
      tryLoc: locs[0]
    };
    1 in locs && (entry.catchLoc = locs[1]), 2 in locs && (entry.finallyLoc = locs[2], entry.afterLoc = locs[3]), this.tryEntries.push(entry);
  }

  function resetTryEntry(entry) {
    var record = entry.completion || {};
    record.type = "normal", delete record.arg, entry.completion = record;
  }

  function Context(tryLocsList) {
    this.tryEntries = [{
      tryLoc: "root"
    }], tryLocsList.forEach(pushTryEntry, this), this.reset(!0);
  }

  function values(iterable) {
    if (iterable) {
      var iteratorMethod = iterable[iteratorSymbol];
      if (iteratorMethod) return iteratorMethod.call(iterable);
      if ("function" == typeof iterable.next) return iterable;

      if (!isNaN(iterable.length)) {
        var i = -1,
            next = function next() {
          for (; ++i < iterable.length;) if (hasOwn.call(iterable, i)) return next.value = iterable[i], next.done = !1, next;

          return next.value = undefined, next.done = !0, next;
        };

        return next.next = next;
      }
    }

    return {
      next: doneResult
    };
  }

  function doneResult() {
    return {
      value: undefined,
      done: !0
    };
  }

  return GeneratorFunction.prototype = GeneratorFunctionPrototype, define(Gp, "constructor", GeneratorFunctionPrototype), define(GeneratorFunctionPrototype, "constructor", GeneratorFunction), GeneratorFunction.displayName = define(GeneratorFunctionPrototype, toStringTagSymbol, "GeneratorFunction"), exports.isGeneratorFunction = function (genFun) {
    var ctor = "function" == typeof genFun && genFun.constructor;
    return !!ctor && (ctor === GeneratorFunction || "GeneratorFunction" === (ctor.displayName || ctor.name));
  }, exports.mark = function (genFun) {
    return Object.setPrototypeOf ? Object.setPrototypeOf(genFun, GeneratorFunctionPrototype) : (genFun.__proto__ = GeneratorFunctionPrototype, define(genFun, toStringTagSymbol, "GeneratorFunction")), genFun.prototype = Object.create(Gp), genFun;
  }, exports.awrap = function (arg) {
    return {
      __await: arg
    };
  }, defineIteratorMethods(AsyncIterator.prototype), define(AsyncIterator.prototype, asyncIteratorSymbol, function () {
    return this;
  }), exports.AsyncIterator = AsyncIterator, exports.async = function (innerFn, outerFn, self, tryLocsList, PromiseImpl) {
    void 0 === PromiseImpl && (PromiseImpl = Promise);
    var iter = new AsyncIterator(wrap(innerFn, outerFn, self, tryLocsList), PromiseImpl);
    return exports.isGeneratorFunction(outerFn) ? iter : iter.next().then(function (result) {
      return result.done ? result.value : iter.next();
    });
  }, defineIteratorMethods(Gp), define(Gp, toStringTagSymbol, "Generator"), define(Gp, iteratorSymbol, function () {
    return this;
  }), define(Gp, "toString", function () {
    return "[object Generator]";
  }), exports.keys = function (object) {
    var keys = [];

    for (var key in object) keys.push(key);

    return keys.reverse(), function next() {
      for (; keys.length;) {
        var key = keys.pop();
        if (key in object) return next.value = key, next.done = !1, next;
      }

      return next.done = !0, next;
    };
  }, exports.values = values, Context.prototype = {
    constructor: Context,
    reset: function (skipTempReset) {
      if (this.prev = 0, this.next = 0, this.sent = this._sent = undefined, this.done = !1, this.delegate = null, this.method = "next", this.arg = undefined, this.tryEntries.forEach(resetTryEntry), !skipTempReset) for (var name in this) "t" === name.charAt(0) && hasOwn.call(this, name) && !isNaN(+name.slice(1)) && (this[name] = undefined);
    },
    stop: function () {
      this.done = !0;
      var rootRecord = this.tryEntries[0].completion;
      if ("throw" === rootRecord.type) throw rootRecord.arg;
      return this.rval;
    },
    dispatchException: function (exception) {
      if (this.done) throw exception;
      var context = this;

      function handle(loc, caught) {
        return record.type = "throw", record.arg = exception, context.next = loc, caught && (context.method = "next", context.arg = undefined), !!caught;
      }

      for (var i = this.tryEntries.length - 1; i >= 0; --i) {
        var entry = this.tryEntries[i],
            record = entry.completion;
        if ("root" === entry.tryLoc) return handle("end");

        if (entry.tryLoc <= this.prev) {
          var hasCatch = hasOwn.call(entry, "catchLoc"),
              hasFinally = hasOwn.call(entry, "finallyLoc");

          if (hasCatch && hasFinally) {
            if (this.prev < entry.catchLoc) return handle(entry.catchLoc, !0);
            if (this.prev < entry.finallyLoc) return handle(entry.finallyLoc);
          } else if (hasCatch) {
            if (this.prev < entry.catchLoc) return handle(entry.catchLoc, !0);
          } else {
            if (!hasFinally) throw new Error("try statement without catch or finally");
            if (this.prev < entry.finallyLoc) return handle(entry.finallyLoc);
          }
        }
      }
    },
    abrupt: function (type, arg) {
      for (var i = this.tryEntries.length - 1; i >= 0; --i) {
        var entry = this.tryEntries[i];

        if (entry.tryLoc <= this.prev && hasOwn.call(entry, "finallyLoc") && this.prev < entry.finallyLoc) {
          var finallyEntry = entry;
          break;
        }
      }

      finallyEntry && ("break" === type || "continue" === type) && finallyEntry.tryLoc <= arg && arg <= finallyEntry.finallyLoc && (finallyEntry = null);
      var record = finallyEntry ? finallyEntry.completion : {};
      return record.type = type, record.arg = arg, finallyEntry ? (this.method = "next", this.next = finallyEntry.finallyLoc, ContinueSentinel) : this.complete(record);
    },
    complete: function (record, afterLoc) {
      if ("throw" === record.type) throw record.arg;
      return "break" === record.type || "continue" === record.type ? this.next = record.arg : "return" === record.type ? (this.rval = this.arg = record.arg, this.method = "return", this.next = "end") : "normal" === record.type && afterLoc && (this.next = afterLoc), ContinueSentinel;
    },
    finish: function (finallyLoc) {
      for (var i = this.tryEntries.length - 1; i >= 0; --i) {
        var entry = this.tryEntries[i];
        if (entry.finallyLoc === finallyLoc) return this.complete(entry.completion, entry.afterLoc), resetTryEntry(entry), ContinueSentinel;
      }
    },
    catch: function (tryLoc) {
      for (var i = this.tryEntries.length - 1; i >= 0; --i) {
        var entry = this.tryEntries[i];

        if (entry.tryLoc === tryLoc) {
          var record = entry.completion;

          if ("throw" === record.type) {
            var thrown = record.arg;
            resetTryEntry(entry);
          }

          return thrown;
        }
      }

      throw new Error("illegal catch attempt");
    },
    delegateYield: function (iterable, resultName, nextLoc) {
      return this.delegate = {
        iterator: values(iterable),
        resultName: resultName,
        nextLoc: nextLoc
      }, "next" === this.method && (this.arg = undefined), ContinueSentinel;
    }
  }, exports;
}

function asyncGeneratorStep(gen, resolve, reject, _next, _throw, key, arg) {
  try {
    var info = gen[key](arg);
    var value = info.value;
  } catch (error) {
    reject(error);
    return;
  }

  if (info.done) {
    resolve(value);
  } else {
    Promise.resolve(value).then(_next, _throw);
  }
}

function _asyncToGenerator(fn) {
  return function () {
    var self = this,
        args = arguments;
    return new Promise(function (resolve, reject) {
      var gen = fn.apply(self, args);

      function _next(value) {
        asyncGeneratorStep(gen, resolve, reject, _next, _throw, "next", value);
      }

      function _throw(err) {
        asyncGeneratorStep(gen, resolve, reject, _next, _throw, "throw", err);
      }

      _next(undefined);
    });
  };
}

function _extends() {
  _extends = Object.assign ? Object.assign.bind() : function (target) {
    for (var i = 1; i < arguments.length; i++) {
      var source = arguments[i];

      for (var key in source) {
        if (Object.prototype.hasOwnProperty.call(source, key)) {
          target[key] = source[key];
        }
      }
    }

    return target;
  };
  return _extends.apply(this, arguments);
}

function _unsupportedIterableToArray(o, minLen) {
  if (!o) return;
  if (typeof o === "string") return _arrayLikeToArray(o, minLen);
  var n = Object.prototype.toString.call(o).slice(8, -1);
  if (n === "Object" && o.constructor) n = o.constructor.name;
  if (n === "Map" || n === "Set") return Array.from(o);
  if (n === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n)) return _arrayLikeToArray(o, minLen);
}

function _arrayLikeToArray(arr, len) {
  if (len == null || len > arr.length) len = arr.length;

  for (var i = 0, arr2 = new Array(len); i < len; i++) arr2[i] = arr[i];

  return arr2;
}

function _createForOfIteratorHelperLoose(o, allowArrayLike) {
  var it = typeof Symbol !== "undefined" && o[Symbol.iterator] || o["@@iterator"];
  if (it) return (it = it.call(o)).next.bind(it);

  if (Array.isArray(o) || (it = _unsupportedIterableToArray(o)) || allowArrayLike && o && typeof o.length === "number") {
    if (it) o = it;
    var i = 0;
    return function () {
      if (i >= o.length) return {
        done: true
      };
      return {
        done: false,
        value: o[i++]
      };
    };
  }

  throw new TypeError("Invalid attempt to iterate non-iterable instance.\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.");
}

var putTransactionData = function putTransactionData(transactions, txHash, data) {
  return produce(transactions, function (draft) {
    draft[txHash] = data;
  });
};
var putTrasactionStatus = function putTrasactionStatus(transactions, txHash, status) {
  return produce(transactions, function (draft) {
    if (draft[txHash]) {
      draft[txHash].status = status;
    }
  });
};
var putTransactionError = function putTransactionError(transactions, txHash, errorMessage) {
  return produce(transactions, function (draft) {
    if (draft[txHash]) {
      draft[txHash].status = 'Error';
      draft[txHash].dispatchError = errorMessage;
    }
  });
};
var putBlockHash = function putBlockHash(transactions, txHash, blockHash) {
  return produce(transactions, function (draft) {
    if (draft[txHash]) {
      draft[txHash].status = 'isInBlock';
      draft[txHash].blockHash = blockHash;
    }
  });
};

var createExtrinsicsSlice = function createExtrinsicsSlice(set) {
  return {
    extrinsics: {},
    addExtrinsic: function addExtrinsic(transactionHash, extrinsicCall) {
      return set(function (prev) {
        return {
          extrinsics: putTransactionData(prev.extrinsics, transactionHash, extrinsicCall)
        };
      });
    },
    addBlockHash: function addBlockHash(transactionHash, blockHash) {
      return set(function (prev) {
        return {
          extrinsics: putBlockHash(prev.extrinsics, transactionHash, blockHash)
        };
      });
    },
    updateExtrinsicStatus: function updateExtrinsicStatus(transactionHash, extrinsicStatus) {
      return set(function (prev) {
        return {
          extrinsics: putTrasactionStatus(prev.extrinsics, transactionHash, extrinsicStatus)
        };
      });
    },
    updateExtrinsicError: function updateExtrinsicError(transactionHash, errorMessage) {
      return set(function (prev) {
        return {
          extrinsics: putTransactionError(prev.extrinsics, transactionHash, errorMessage)
        };
      });
    }
  };
};

var useStore = /*#__PURE__*/create(function (set) {
  return _extends({}, createExtrinsicsSlice(set));
});

var useExtrinsics = function useExtrinsics() {
  var _useStore = useStore(),
      extrinsics = _useStore.extrinsics;

  return extrinsics;
};

function isPending(extrinsicStatus) {
  if (extrinsicStatus !== 'isFinalized' && extrinsicStatus !== 'Error') {
    return true;
  }

  return false;
}

var usePendingExtrinsic = function usePendingExtrinsic(method, section, sender) {
  var _useStore2 = useStore(),
      extrinsics = _useStore2.extrinsics;

  var _isPendingExtrinsic = React.useMemo(function () {
    var sortedTxs = Object.values(extrinsics).sort(function (a, b) {
      return a.timestamp - b.timestamp;
    });

    for (var _iterator = _createForOfIteratorHelperLoose(sortedTxs), _step; !(_step = _iterator()).done;) {
      var tx = _step.value;

      if (tx.method === method && section === tx.section && tx.sender === sender) {
        if (isPending(tx.status)) {
          return true;
        }
      }
    }

    return false;
  }, [extrinsics]);

  return _isPendingExtrinsic;
};
var useExtrinsicCalls = function useExtrinsicCalls(method, section, sender) {
  var _useStore3 = useStore(),
      extrinsics = _useStore3.extrinsics;

  var extrinsicCalls = React.useMemo(function () {
    var calls = [];

    for (var _i = 0, _Object$values = Object.values(extrinsics); _i < _Object$values.length; _i++) {
      var tx = _Object$values[_i];

      if (tx.method === method && section === tx.section && tx.sender === sender) {
        calls.push(_extends({}, tx));
      }
    }

    return calls;
  }, [extrinsics]);
  return extrinsicCalls;
};

var Executor = /*#__PURE__*/function () {
  function Executor(addExtrinsic, addBlockHash, updateExstrinsicStatus, updateExtrinsicError) {
    this.addExtrinsic = addExtrinsic;
    this.addBlockHash = addBlockHash;
    this.updateExstrinsicStatus = updateExstrinsicStatus;
    this.updateExtrinsicError = updateExtrinsicError;
  }
  /**
   * Execute an API Call (legacy or not?)
   * @param call a submittable extrinsic from Polkadot/api
   * @param sender address of the user
   * @param api polkadot api itself
   * @param signer signer from an extension wallet
   * @param onTxFinalized this should be optional
   */


  var _proto = Executor.prototype;

  _proto.execute =
  /*#__PURE__*/
  function () {
    var _execute = /*#__PURE__*/_asyncToGenerator( /*#__PURE__*/_regeneratorRuntime().mark(function _callee(call, sender, api, signer, onTxReady, onTxFinalized, onTxError) {
      var _this = this;

      var unsub;
      return _regeneratorRuntime().wrap(function _callee$(_context) {
        while (1) {
          switch (_context.prev = _context.next) {
            case 0:
              _context.next = 2;
              return call.signAndSend(sender, {
                signer: signer
              }, function (txResult) {
                var txHash = txResult.txHash.toString().toLowerCase();

                if (txResult.status.isReady) {
                  _this.onReady(call, txResult, sender, true);

                  if (onTxReady) {
                    onTxReady(txHash);
                  }
                }

                if (txResult.dispatchError) {
                  var error = _this.onDispatchError(txResult, api);

                  if (onTxError) onTxError(error);
                  unsub();
                }

                if (txResult.isFinalized) {
                  _this.onFinalized(txHash);

                  if (onTxFinalized) {
                    onTxFinalized(txHash, txResult.events);
                  }

                  unsub();
                }
              });

            case 2:
              unsub = _context.sent;

            case 3:
            case "end":
              return _context.stop();
          }
        }
      }, _callee);
    }));

    function execute(_x, _x2, _x3, _x4, _x5, _x6, _x7) {
      return _execute.apply(this, arguments);
    }

    return execute;
  }();

  _proto.executeUnsigned = /*#__PURE__*/function () {
    var _executeUnsigned = /*#__PURE__*/_asyncToGenerator( /*#__PURE__*/_regeneratorRuntime().mark(function _callee2(call, api, onTxReady, onTxFinalized) {
      var _this2 = this;

      var unsub;
      return _regeneratorRuntime().wrap(function _callee2$(_context2) {
        while (1) {
          switch (_context2.prev = _context2.next) {
            case 0:
              _context2.next = 2;
              return call.send(function (txResult) {
                var txHash = txResult.txHash.toString().toLowerCase();

                if (txResult.status.isReady) {
                  _this2.onReady(call, txResult, '', false);

                  if (onTxReady) onTxReady(txHash);
                }

                if (txResult.status.isInBlock) {
                  _this2.onBlockInclusion(txResult);
                }

                if (txResult.dispatchError) {
                  _this2.onDispatchError(txResult, api);

                  unsub();
                }

                if (txResult.isFinalized && !txResult.dispatchError) {
                  _this2.onFinalized(txHash);

                  if (onTxFinalized) onTxFinalized(txHash);
                  unsub();
                }
              });

            case 2:
              unsub = _context2.sent;

            case 3:
            case "end":
              return _context2.stop();
          }
        }
      }, _callee2);
    }));

    function executeUnsigned(_x8, _x9, _x10, _x11) {
      return _executeUnsigned.apply(this, arguments);
    }

    return executeUnsigned;
  }();

  _proto.onReady = /*#__PURE__*/function () {
    var _onReady = /*#__PURE__*/_asyncToGenerator( /*#__PURE__*/_regeneratorRuntime().mark(function _callee3(call, txResult, sender, isSigned) {
      var serialized, txHash, payload;
      return _regeneratorRuntime().wrap(function _callee3$(_context3) {
        while (1) {
          switch (_context3.prev = _context3.next) {
            case 0:
              serialized = call.toHuman();
              txHash = txResult.txHash.toString().toLowerCase();
              payload = {
                hash: txHash,
                method: serialized.method.method,
                section: serialized.method.section,
                sender: sender.toString(),
                args: serialized.method.args,
                dispatchError: undefined,
                status: 'isReady',
                isSigned: isSigned,
                timestamp: Date.now()
              };
              this.addExtrinsic(txHash, payload);

            case 4:
            case "end":
              return _context3.stop();
          }
        }
      }, _callee3, this);
    }));

    function onReady(_x12, _x13, _x14, _x15) {
      return _onReady.apply(this, arguments);
    }

    return onReady;
  }();

  _proto.onDispatchError = function onDispatchError(txResult, api) {
    var errorMessage = "";
    var txHash = txResult.txHash.toString().toLowerCase();

    if (txResult.dispatchError) {
      if (txResult.dispatchError.isModule) {
        var decoded = api.registry.findMetaError(txResult.dispatchError.asModule);
        var docs = decoded.docs,
            name = decoded.name,
            section = decoded.section;
        errorMessage = section + "." + name + ": " + docs.join(' ');
      } else {
        errorMessage = txResult.dispatchError.toString();
      }
    }

    this.updateExtrinsicError(txHash, errorMessage);
    return errorMessage;
  };

  _proto.onBlockInclusion = /*#__PURE__*/function () {
    var _onBlockInclusion = /*#__PURE__*/_asyncToGenerator( /*#__PURE__*/_regeneratorRuntime().mark(function _callee4(txResult) {
      var txHash, blockHash;
      return _regeneratorRuntime().wrap(function _callee4$(_context4) {
        while (1) {
          switch (_context4.prev = _context4.next) {
            case 0:
              txHash = txResult.txHash.toString().toLowerCase();
              blockHash = txResult.status.asInBlock.toString().toLowerCase();
              this.addBlockHash(txHash, blockHash);

            case 3:
            case "end":
              return _context4.stop();
          }
        }
      }, _callee4, this);
    }));

    function onBlockInclusion(_x16) {
      return _onBlockInclusion.apply(this, arguments);
    }

    return onBlockInclusion;
  }();

  _proto.onFinalized = /*#__PURE__*/function () {
    var _onFinalized = /*#__PURE__*/_asyncToGenerator( /*#__PURE__*/_regeneratorRuntime().mark(function _callee5(txHash) {
      return _regeneratorRuntime().wrap(function _callee5$(_context5) {
        while (1) {
          switch (_context5.prev = _context5.next) {
            case 0:
              this.updateExstrinsicStatus(txHash, 'isFinalized');

            case 1:
            case "end":
              return _context5.stop();
          }
        }
      }, _callee5, this);
    }));

    function onFinalized(_x17) {
      return _onFinalized.apply(this, arguments);
    }

    return onFinalized;
  }();

  return Executor;
}();

/**
 * As zustand useStore is a hook
 * we need to create a context and wrap
 * executor in a provider to be able to
 * use useStore methods via executor
 *
 * exectuor would expose execute and executeUnsigned
 * methods to be able to execute extrsinsic calls
 */

var ExecutorContext = /*#__PURE__*/React__default.createContext({
  executor: undefined
});
var ExecutorProvider = function ExecutorProvider(_ref) {
  var children = _ref.children;

  /**
   * Use store updaters
   * from zustand store
   */
  var _useStore = useStore(),
      addExtrinsic = _useStore.addExtrinsic,
      addBlockHash = _useStore.addBlockHash,
      updateExtrinsicStatus = _useStore.updateExtrinsicStatus,
      updateExtrinsicError = _useStore.updateExtrinsicError;
  /**
   * Create and memoize executor
   */


  var executor = React.useMemo(function () {
    return new Executor(addExtrinsic, addBlockHash, updateExtrinsicStatus, updateExtrinsicError);
  }, [addExtrinsic, addBlockHash, updateExtrinsicStatus, updateExtrinsicError]);
  return React__default.createElement(ExecutorContext.Provider, {
    value: {
      executor: executor
    }
  }, children);
};
/**
 * Hook that returns an extrinsics executor
 * @returns Executor
 */

var useExecutor = function useExecutor() {
  return React__default.useContext(ExecutorContext).executor;
};

var ParachainNetworks = {
  picasso: {
    name: 'Picasso',
    wsUrl: 'wss://picasso-rpc.composable.finance',
    tokenId: 'pica',
    prefix: 49,
    accountType: '*25519',
    subscanUrl: '',
    decimals: 12,
    color: '#B09A9F',
    symbol: 'PICA',
    logo: 'https://raw.githubusercontent.com/TalismanSociety/chaindata/2778d4b989407a2e9fca6ae897fe849561f74afe/assets/picasso/logo.svg',
    parachainId: 2087,
    relayChain: 'kusama'
  },
  karura: {
    name: 'Karura',
    wsUrl: 'wss://karura-rpc-0.aca-api.network',
    tokenId: 'kar',
    prefix: 8,
    accountType: '*25519',
    subscanUrl: 'https://karura.subscan.io/',
    decimals: 12,
    color: '#ff4c3b',
    symbol: 'KAR',
    logo: 'https://raw.githubusercontent.com/TalismanSociety/chaindata/2778d4b989407a2e9fca6ae897fe849561f74afe/assets/karura/logo.svg',
    parachainId: 2000,
    relayChain: 'kusama'
  }
};
var RelayChainNetworks = {
  kusama: {
    name: 'Kusama',
    color: '#000000',
    prefix: 2,
    logo: 'https://raw.githubusercontent.com/TalismanSociety/chaindata/2778d4b989407a2e9fca6ae897fe849561f74afe/assets/kusama/logo.svg',
    networkId: 'kusama',
    accountType: '*25519',
    wsUrl: 'wss://kusama-rpc.polkadot.io',
    subscanUrl: 'https://kusama.subscan.io/',
    decimals: 12,
    tokenId: 'ksm',
    symbol: 'KSM'
  },
  polkadot: {
    name: 'Polkadot',
    color: '#e6007a',
    prefix: 0,
    logo: 'https://raw.githubusercontent.com/TalismanSociety/chaindata/2778d4b989407a2e9fca6ae897fe849561f74afe/assets/polkadot/logo.svg',
    networkId: 'polkadot',
    accountType: '*25519',
    wsUrl: 'wss://rpc.polkadot.io',
    subscanUrl: 'https://polkadot.subscan.io/',
    decimals: 10,
    tokenId: 'dot',
    symbol: 'DOT'
  }
};
var getParachainNetwork = function getParachainNetwork(parachainId) {
  return ParachainNetworks[parachainId];
};
var getRelaychainNetwork = function getRelaychainNetwork(relaychainId) {
  return RelayChainNetworks[relaychainId];
};

var PARACHAIN_PROVIDERS_DEFAULT = /*#__PURE__*/Object.entries(ParachainNetworks).map(function (_ref) {
  var chainId = _ref[0],
      network = _ref[1];
  return {
    chainId: chainId,
    parachainApi: undefined,
    apiStatus: 'initializing',
    prefix: network.prefix,
    accounts: []
  };
}).reduce(function (acc, curr) {
  var _extends2;

  return _extends({}, acc, (_extends2 = {}, _extends2[curr.chainId] = curr, _extends2));
}, {});
var RELAYCHAIN_PROVIDERS_DEFAULT = /*#__PURE__*/Object.entries(RelayChainNetworks).map(function (_ref2) {
  var chainId = _ref2[0],
      network = _ref2[1];
  return {
    chainId: chainId,
    parachainApi: undefined,
    apiStatus: 'initializing',
    prefix: network.prefix,
    accounts: []
  };
}).reduce(function (acc, curr) {
  var _extends3;

  return _extends({}, acc, (_extends3 = {}, _extends3[curr.chainId] = curr, _extends3));
}, {});
var DotsamaContext = /*#__PURE__*/React.createContext({
  parachainProviders: PARACHAIN_PROVIDERS_DEFAULT,
  relaychainProviders: RELAYCHAIN_PROVIDERS_DEFAULT,
  extensionStatus: 'initializing',
  activate: undefined,
  selectedAccount: -1
});
var DotSamaContextProvider = function DotSamaContextProvider(_ref3) {
  var supportedParachains = _ref3.supportedParachains,
      children = _ref3.children,
      appName = _ref3.appName;

  var _useState = React.useState(PARACHAIN_PROVIDERS_DEFAULT),
      parachainProviders = _useState[0],
      setParachainProviders = _useState[1];

  var _useState2 = React.useState(RELAYCHAIN_PROVIDERS_DEFAULT),
      relaychainProviders = _useState2[0];

  var activate = /*#__PURE__*/function () {
    var _ref4 = _asyncToGenerator( /*#__PURE__*/_regeneratorRuntime().mark(function _callee() {
      var extensionExists, inectedExtesions, extensionPkg, _loop, i, _ret;

      return _regeneratorRuntime().wrap(function _callee$(_context2) {
        while (1) {
          switch (_context2.prev = _context2.next) {
            case 0:
              setExtension(function (s) {
                s.extensionStatus = 'connecting';
                return s;
              });
              extensionExists = true;
              _context2.prev = 2;
              _context2.next = 5;
              return new Promise(function (resolve) { resolve(_interopNamespace(require('@polkadot/extension-dapp'))); });

            case 5:
              extensionPkg = _context2.sent;
              _context2.next = 8;
              return extensionPkg.web3Enable(appName);

            case 8:
              inectedExtesions = _context2.sent;
              extensionExists = inectedExtesions.length !== 0;
              _context2.next = 16;
              break;

            case 12:
              _context2.prev = 12;
              _context2.t0 = _context2["catch"](2);
              console.error(_context2.t0);
              extensionExists = false;

            case 16:
              if (extensionExists) {
                _context2.next = 19;
                break;
              }

              setExtension(function (s) {
                s.extensionStatus = 'no_extension';
                return s;
              });
              return _context2.abrupt("return", inectedExtesions);

            case 19:
              setExtension(function (s) {
                s.extensionStatus = 'connected';
                return s;
              });
              _loop = /*#__PURE__*/_regeneratorRuntime().mark(function _loop(i) {
                var chainId, prefix, _extensionPkg, accounts;

                return _regeneratorRuntime().wrap(function _loop$(_context) {
                  while (1) {
                    switch (_context.prev = _context.next) {
                      case 0:
                        chainId = supportedParachains[i].chainId;
                        prefix = ParachainNetworks[chainId].prefix;
                        _context.prev = 2;
                        _context.next = 5;
                        return new Promise(function (resolve) { resolve(_interopNamespace(require('@polkadot/extension-dapp'))); });

                      case 5:
                        _extensionPkg = _context.sent;
                        _context.next = 8;
                        return _extensionPkg.web3Accounts({
                          ss58Format: prefix
                        });

                      case 8:
                        accounts = _context.sent;
                        setParachainProviders(function (s) {
                          s[chainId].accounts = accounts.map(function (x, i) {
                            var _x$meta$name;

                            return {
                              address: x.address,
                              name: (_x$meta$name = x.meta.name) != null ? _x$meta$name : i.toFixed()
                            };
                          });
                          return _extends({}, s);
                        }); // setting default account

                        setSelectedAccount(accounts.length ? 0 : -1);
                        _context.next = 17;
                        break;

                      case 13:
                        _context.prev = 13;
                        _context.t0 = _context["catch"](2);
                        console.error(_context.t0);
                        return _context.abrupt("return", "continue");

                      case 17:
                      case "end":
                        return _context.stop();
                    }
                  }
                }, _loop, null, [[2, 13]]);
              });
              i = 0;

            case 22:
              if (!(i < supportedParachains.length)) {
                _context2.next = 30;
                break;
              }

              return _context2.delegateYield(_loop(i), "t1", 24);

            case 24:
              _ret = _context2.t1;

              if (!(_ret === "continue")) {
                _context2.next = 27;
                break;
              }

              return _context2.abrupt("continue", 27);

            case 27:
              i++;
              _context2.next = 22;
              break;

            case 30:
              return _context2.abrupt("return", inectedExtesions);

            case 31:
            case "end":
              return _context2.stop();
          }
        }
      }, _callee, null, [[2, 12]]);
    }));

    return function activate() {
      return _ref4.apply(this, arguments);
    };
  }();

  var deactivate = /*#__PURE__*/function () {
    var _ref5 = _asyncToGenerator( /*#__PURE__*/_regeneratorRuntime().mark(function _callee2() {
      var _loop2, i, _ret2;

      return _regeneratorRuntime().wrap(function _callee2$(_context3) {
        while (1) {
          switch (_context3.prev = _context3.next) {
            case 0:
              setExtension(function (s) {
                s.extensionStatus = 'initializing';
                return s;
              });

              _loop2 = function _loop2(i) {
                setParachainProviders(function (s) {
                  var chainId = supportedParachains[i].chainId;
                  s[chainId].accounts = [];
                  return _extends({}, s);
                });
                setSelectedAccount(-1);
                return {
                  v: Promise.resolve()
                };
              };

              i = 0;

            case 3:
              if (!(i < supportedParachains.length)) {
                _context3.next = 10;
                break;
              }

              _ret2 = _loop2(i);

              if (!(typeof _ret2 === "object")) {
                _context3.next = 7;
                break;
              }

              return _context3.abrupt("return", _ret2.v);

            case 7:
              i++;
              _context3.next = 3;
              break;

            case 10:
            case "end":
              return _context3.stop();
          }
        }
      }, _callee2);
    }));

    return function deactivate() {
      return _ref5.apply(this, arguments);
    };
  }();

  var _useState3 = React.useState({
    extensionStatus: 'initializing',
    activate: activate,
    deactivate: deactivate
  }),
      extension = _useState3[0],
      setExtension = _useState3[1];

  React.useEffect(function () {
    var _loop3 = function _loop3(i) {
      var _supportedParachains$ = supportedParachains[i],
          rpcUrl = _supportedParachains$.rpcUrl,
          chainId = _supportedParachains$.chainId,
          rpc = _supportedParachains$.rpc,
          types = _supportedParachains$.types;
      var prefix = ParachainNetworks[chainId].prefix; // just so we can activate ASAP (where ss58Format is needed)
      // setParachainProviders(s => {
      //   s[chainId] = {
      //     parachainApi: undefined,
      //     apiStatus: 'initializing',
      //     accounts: [],
      //     prefix,
      //     chainId,
      //   };
      //   return s;
      // });

      var wsProvider = new rpcProvider.WsProvider(rpcUrl);
      var parachainApi = new api.ApiPromise({
        provider: wsProvider,
        rpc: rpc,
        types: types
      });
      parachainApi.isReady.then(function (parachainApi) {
        setParachainProviders(function (s) {
          if (!(chainId in parachainProviders)) {
            s[chainId] = {
              chainId: chainId,
              parachainApi: parachainApi,
              apiStatus: 'connected',
              accounts: [],
              prefix: prefix
            };
          } else {
            s[chainId].apiStatus = 'connected';
            s[chainId].parachainApi = parachainApi;
          }

          return s;
        });
      })["catch"](function (e) {
        console.error(e);
        setParachainProviders(function (s) {
          s[chainId] = {
            chainId: chainId,
            parachainApi: undefined,
            apiStatus: 'failed',
            accounts: [],
            prefix: prefix
          };
          return s;
        });
      });
    };

    for (var i = 0; i < supportedParachains.length; i++) {
      _loop3(i);
    }
  }, []);

  var _useState4 = React.useState(-1),
      selectedAccount = _useState4[0],
      setSelectedAccount = _useState4[1];

  return React__default.createElement(DotsamaContext.Provider, {
    value: _extends({
      relaychainProviders: relaychainProviders,
      parachainProviders: parachainProviders,
      setSelectedAccount: setSelectedAccount,
      selectedAccount: selectedAccount
    }, extension)
  }, children);
};

var getSigner = /*#__PURE__*/function () {
  var _ref = /*#__PURE__*/_asyncToGenerator( /*#__PURE__*/_regeneratorRuntime().mark(function _callee(applicationName, address) {
    var extensionPackage, web3FromAddress, web3Enable, injector;
    return _regeneratorRuntime().wrap(function _callee$(_context) {
      while (1) {
        switch (_context.prev = _context.next) {
          case 0:
            _context.next = 2;
            return new Promise(function (resolve) { resolve(_interopNamespace(require('@polkadot/extension-dapp'))); });

          case 2:
            extensionPackage = _context.sent;
            web3FromAddress = extensionPackage.web3FromAddress, web3Enable = extensionPackage.web3Enable;
            _context.next = 6;
            return web3Enable(applicationName);

          case 6:
            _context.next = 8;
            return web3FromAddress(address);

          case 8:
            injector = _context.sent;
            return _context.abrupt("return", injector.signer);

          case 10:
          case "end":
            return _context.stop();
        }
      }
    }, _callee);
  }));

  return function getSigner(_x, _x2) {
    return _ref.apply(this, arguments);
  };
}();

var useDotSamaContext = function useDotSamaContext() {
  return React__default.useContext(DotsamaContext);
};
var useParachainApi = function useParachainApi(parachainId) {
  var _React$useContext = React__default.useContext(DotsamaContext),
      parachainProviders = _React$useContext.parachainProviders;

  return parachainProviders[parachainId];
};
var useRelayChainApi = function useRelayChainApi(relaychainId) {
  var _React$useContext2 = React__default.useContext(DotsamaContext),
      relaychainProviders = _React$useContext2.relaychainProviders;

  return relaychainProviders[relaychainId];
};
var useSelectedAccount = function useSelectedAccount(parachainId) {
  var _React$useContext3 = React__default.useContext(DotsamaContext),
      selectedAccount = _React$useContext3.selectedAccount,
      parachainProviders = _React$useContext3.parachainProviders;

  var accounts = parachainProviders[parachainId].accounts;
  return selectedAccount !== -1 ? accounts[selectedAccount] : undefined;
};

exports.DotSamaContextProvider = DotSamaContextProvider;
exports.DotsamaContext = DotsamaContext;
exports.ExecutorProvider = ExecutorProvider;
exports.ParachainNetworks = ParachainNetworks;
exports.RelayChainNetworks = RelayChainNetworks;
exports.getParachainNetwork = getParachainNetwork;
exports.getRelaychainNetwork = getRelaychainNetwork;
exports.getSigner = getSigner;
exports.useDotSamaContext = useDotSamaContext;
exports.useExecutor = useExecutor;
exports.useExtrinsicCalls = useExtrinsicCalls;
exports.useExtrinsics = useExtrinsics;
exports.useParachainApi = useParachainApi;
exports.usePendingExtrinsic = usePendingExtrinsic;
exports.useRelayChainApi = useRelayChainApi;
exports.useSelectedAccount = useSelectedAccount;
//# sourceMappingURL=substrate-react.cjs.development.js.map
