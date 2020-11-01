(() => {
  var __create = Object.create;
  var __defProp = Object.defineProperty;
  var __getProtoOf = Object.getPrototypeOf;
  var __hasOwnProp = Object.prototype.hasOwnProperty;
  var __getOwnPropNames = Object.getOwnPropertyNames;
  var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
  var __markAsModule = (target) => __defProp(target, "__esModule", {value: true});
  var __commonJS = (callback, module) => () => {
    if (!module) {
      module = {exports: {}};
      callback(module.exports, module);
    }
    return module.exports;
  };
  var __exportStar = (target, module, desc) => {
    __markAsModule(target);
    if (typeof module === "object" || typeof module === "function") {
      for (let key of __getOwnPropNames(module))
        if (!__hasOwnProp.call(target, key) && key !== "default")
          __defProp(target, key, {get: () => module[key], enumerable: !(desc = __getOwnPropDesc(module, key)) || desc.enumerable});
    }
    return target;
  };
  var __toModule = (module) => {
    if (module && module.__esModule)
      return module;
    return __exportStar(__defProp(__create(__getProtoOf(module)), "default", {value: module, enumerable: true}), module);
  };

  // node_modules/object-assign/index.js
  var require_object_assign = __commonJS((exports, module) => {
    /*
    object-assign
    (c) Sindre Sorhus
    @license MIT
    */
    "use strict";
    var getOwnPropertySymbols = Object.getOwnPropertySymbols;
    var hasOwnProperty3 = Object.prototype.hasOwnProperty;
    var propIsEnumerable = Object.prototype.propertyIsEnumerable;
    function toObject(val) {
      if (val === null || val === void 0) {
        throw new TypeError("Object.assign cannot be called with null or undefined");
      }
      return Object(val);
    }
    function shouldUseNative() {
      try {
        if (!Object.assign) {
          return false;
        }
        var test1 = new String("abc");
        test1[5] = "de";
        if (Object.getOwnPropertyNames(test1)[0] === "5") {
          return false;
        }
        var test2 = {};
        for (var i = 0; i < 10; i++) {
          test2["_" + String.fromCharCode(i)] = i;
        }
        var order2 = Object.getOwnPropertyNames(test2).map(function(n) {
          return test2[n];
        });
        if (order2.join("") !== "0123456789") {
          return false;
        }
        var test3 = {};
        "abcdefghijklmnopqrst".split("").forEach(function(letter) {
          test3[letter] = letter;
        });
        if (Object.keys(Object.assign({}, test3)).join("") !== "abcdefghijklmnopqrst") {
          return false;
        }
        return true;
      } catch (err) {
        return false;
      }
    }
    module.exports = shouldUseNative() ? Object.assign : function(target, source) {
      var from;
      var to = toObject(target);
      var symbols;
      for (var s = 1; s < arguments.length; s++) {
        from = Object(arguments[s]);
        for (var key in from) {
          if (hasOwnProperty3.call(from, key)) {
            to[key] = from[key];
          }
        }
        if (getOwnPropertySymbols) {
          symbols = getOwnPropertySymbols(from);
          for (var i = 0; i < symbols.length; i++) {
            if (propIsEnumerable.call(from, symbols[i])) {
              to[symbols[i]] = from[symbols[i]];
            }
          }
        }
      }
      return to;
    };
  });

  // node_modules/react/cjs/react.production.min.js
  var require_react_production_min = __commonJS((exports) => {
    /** @license React v17.0.1
     * react.production.min.js
     *
     * Copyright (c) Facebook, Inc. and its affiliates.
     *
     * This source code is licensed under the MIT license found in the
     * LICENSE file in the root directory of this source tree.
     */
    "use strict";
    var l = require_object_assign();
    var n = 60103;
    var p = 60106;
    exports.Fragment = 60107;
    exports.StrictMode = 60108;
    exports.Profiler = 60114;
    var q = 60109;
    var r = 60110;
    var t2 = 60112;
    exports.Suspense = 60113;
    var u = 60115;
    var v = 60116;
    if (typeof Symbol === "function" && Symbol.for) {
      var w = Symbol.for;
      n = w("react.element");
      p = w("react.portal");
      exports.Fragment = w("react.fragment");
      exports.StrictMode = w("react.strict_mode");
      exports.Profiler = w("react.profiler");
      q = w("react.provider");
      r = w("react.context");
      t2 = w("react.forward_ref");
      exports.Suspense = w("react.suspense");
      u = w("react.memo");
      v = w("react.lazy");
    }
    var x = typeof Symbol === "function" && Symbol.iterator;
    function y(a) {
      if (a === null || typeof a !== "object")
        return null;
      a = x && a[x] || a["@@iterator"];
      return typeof a === "function" ? a : null;
    }
    function z(a) {
      for (var b = "https://reactjs.org/docs/error-decoder.html?invariant=" + a, c = 1; c < arguments.length; c++)
        b += "&args[]=" + encodeURIComponent(arguments[c]);
      return "Minified React error #" + a + "; visit " + b + " for the full message or use the non-minified dev environment for full errors and additional helpful warnings.";
    }
    var A = {isMounted: function() {
      return false;
    }, enqueueForceUpdate: function() {
    }, enqueueReplaceState: function() {
    }, enqueueSetState: function() {
    }};
    var B = {};
    function C(a, b, c) {
      this.props = a;
      this.context = b;
      this.refs = B;
      this.updater = c || A;
    }
    C.prototype.isReactComponent = {};
    C.prototype.setState = function(a, b) {
      if (typeof a !== "object" && typeof a !== "function" && a != null)
        throw Error(z(85));
      this.updater.enqueueSetState(this, a, b, "setState");
    };
    C.prototype.forceUpdate = function(a) {
      this.updater.enqueueForceUpdate(this, a, "forceUpdate");
    };
    function D() {
    }
    D.prototype = C.prototype;
    function E(a, b, c) {
      this.props = a;
      this.context = b;
      this.refs = B;
      this.updater = c || A;
    }
    var F = E.prototype = new D();
    F.constructor = E;
    l(F, C.prototype);
    F.isPureReactComponent = true;
    var G = {current: null};
    var H = Object.prototype.hasOwnProperty;
    var I = {key: true, ref: true, __self: true, __source: true};
    function J(a, b, c) {
      var e, d = {}, k = null, h = null;
      if (b != null)
        for (e in b.ref !== void 0 && (h = b.ref), b.key !== void 0 && (k = "" + b.key), b)
          H.call(b, e) && !I.hasOwnProperty(e) && (d[e] = b[e]);
      var g = arguments.length - 2;
      if (g === 1)
        d.children = c;
      else if (1 < g) {
        for (var f = Array(g), m = 0; m < g; m++)
          f[m] = arguments[m + 2];
        d.children = f;
      }
      if (a && a.defaultProps)
        for (e in g = a.defaultProps, g)
          d[e] === void 0 && (d[e] = g[e]);
      return {$$typeof: n, type: a, key: k, ref: h, props: d, _owner: G.current};
    }
    function K(a, b) {
      return {$$typeof: n, type: a.type, key: b, ref: a.ref, props: a.props, _owner: a._owner};
    }
    function L(a) {
      return typeof a === "object" && a !== null && a.$$typeof === n;
    }
    function escape(a) {
      var b = {"=": "=0", ":": "=2"};
      return "$" + a.replace(/[=:]/g, function(a2) {
        return b[a2];
      });
    }
    var M = /\/+/g;
    function N(a, b) {
      return typeof a === "object" && a !== null && a.key != null ? escape("" + a.key) : b.toString(36);
    }
    function O(a, b, c, e, d) {
      var k = typeof a;
      if (k === "undefined" || k === "boolean")
        a = null;
      var h = false;
      if (a === null)
        h = true;
      else
        switch (k) {
          case "string":
          case "number":
            h = true;
            break;
          case "object":
            switch (a.$$typeof) {
              case n:
              case p:
                h = true;
            }
        }
      if (h)
        return h = a, d = d(h), a = e === "" ? "." + N(h, 0) : e, Array.isArray(d) ? (c = "", a != null && (c = a.replace(M, "$&/") + "/"), O(d, b, c, "", function(a2) {
          return a2;
        })) : d != null && (L(d) && (d = K(d, c + (!d.key || h && h.key === d.key ? "" : ("" + d.key).replace(M, "$&/") + "/") + a)), b.push(d)), 1;
      h = 0;
      e = e === "" ? "." : e + ":";
      if (Array.isArray(a))
        for (var g = 0; g < a.length; g++) {
          k = a[g];
          var f = e + N(k, g);
          h += O(k, b, c, f, d);
        }
      else if (f = y(a), typeof f === "function")
        for (a = f.call(a), g = 0; !(k = a.next()).done; )
          k = k.value, f = e + N(k, g++), h += O(k, b, c, f, d);
      else if (k === "object")
        throw b = "" + a, Error(z(31, b === "[object Object]" ? "object with keys {" + Object.keys(a).join(", ") + "}" : b));
      return h;
    }
    function P(a, b, c) {
      if (a == null)
        return a;
      var e = [], d = 0;
      O(a, e, "", "", function(a2) {
        return b.call(c, a2, d++);
      });
      return e;
    }
    function Q(a) {
      if (a._status === -1) {
        var b = a._result;
        b = b();
        a._status = 0;
        a._result = b;
        b.then(function(b2) {
          a._status === 0 && (b2 = b2.default, a._status = 1, a._result = b2);
        }, function(b2) {
          a._status === 0 && (a._status = 2, a._result = b2);
        });
      }
      if (a._status === 1)
        return a._result;
      throw a._result;
    }
    var R = {current: null};
    function S() {
      var a = R.current;
      if (a === null)
        throw Error(z(321));
      return a;
    }
    var T = {ReactCurrentDispatcher: R, ReactCurrentBatchConfig: {transition: 0}, ReactCurrentOwner: G, IsSomeRendererActing: {current: false}, assign: l};
    exports.Children = {map: P, forEach: function(a, b, c) {
      P(a, function() {
        b.apply(this, arguments);
      }, c);
    }, count: function(a) {
      var b = 0;
      P(a, function() {
        b++;
      });
      return b;
    }, toArray: function(a) {
      return P(a, function(a2) {
        return a2;
      }) || [];
    }, only: function(a) {
      if (!L(a))
        throw Error(z(143));
      return a;
    }};
    exports.Component = C;
    exports.PureComponent = E;
    exports.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED = T;
    exports.cloneElement = function(a, b, c) {
      if (a === null || a === void 0)
        throw Error(z(267, a));
      var e = l({}, a.props), d = a.key, k = a.ref, h = a._owner;
      if (b != null) {
        b.ref !== void 0 && (k = b.ref, h = G.current);
        b.key !== void 0 && (d = "" + b.key);
        if (a.type && a.type.defaultProps)
          var g = a.type.defaultProps;
        for (f in b)
          H.call(b, f) && !I.hasOwnProperty(f) && (e[f] = b[f] === void 0 && g !== void 0 ? g[f] : b[f]);
      }
      var f = arguments.length - 2;
      if (f === 1)
        e.children = c;
      else if (1 < f) {
        g = Array(f);
        for (var m = 0; m < f; m++)
          g[m] = arguments[m + 2];
        e.children = g;
      }
      return {
        $$typeof: n,
        type: a.type,
        key: d,
        ref: k,
        props: e,
        _owner: h
      };
    };
    exports.createContext = function(a, b) {
      b === void 0 && (b = null);
      a = {$$typeof: r, _calculateChangedBits: b, _currentValue: a, _currentValue2: a, _threadCount: 0, Provider: null, Consumer: null};
      a.Provider = {$$typeof: q, _context: a};
      return a.Consumer = a;
    };
    exports.createElement = J;
    exports.createFactory = function(a) {
      var b = J.bind(null, a);
      b.type = a;
      return b;
    };
    exports.createRef = function() {
      return {current: null};
    };
    exports.forwardRef = function(a) {
      return {$$typeof: t2, render: a};
    };
    exports.isValidElement = L;
    exports.lazy = function(a) {
      return {$$typeof: v, _payload: {_status: -1, _result: a}, _init: Q};
    };
    exports.memo = function(a, b) {
      return {$$typeof: u, type: a, compare: b === void 0 ? null : b};
    };
    exports.useCallback = function(a, b) {
      return S().useCallback(a, b);
    };
    exports.useContext = function(a, b) {
      return S().useContext(a, b);
    };
    exports.useDebugValue = function() {
    };
    exports.useEffect = function(a, b) {
      return S().useEffect(a, b);
    };
    exports.useImperativeHandle = function(a, b, c) {
      return S().useImperativeHandle(a, b, c);
    };
    exports.useLayoutEffect = function(a, b) {
      return S().useLayoutEffect(a, b);
    };
    exports.useMemo = function(a, b) {
      return S().useMemo(a, b);
    };
    exports.useReducer = function(a, b, c) {
      return S().useReducer(a, b, c);
    };
    exports.useRef = function(a) {
      return S().useRef(a);
    };
    exports.useState = function(a) {
      return S().useState(a);
    };
    exports.version = "17.0.1";
  });

  // node_modules/react/index.js
  var require_react = __commonJS((exports, module) => {
    "use strict";
    if (true) {
      module.exports = require_react_production_min();
    } else {
      module.exports = null;
    }
  });

  // node_modules/scheduler/cjs/scheduler.production.min.js
  var require_scheduler_production_min = __commonJS((exports) => {
    /** @license React v0.20.1
     * scheduler.production.min.js
     *
     * Copyright (c) Facebook, Inc. and its affiliates.
     *
     * This source code is licensed under the MIT license found in the
     * LICENSE file in the root directory of this source tree.
     */
    "use strict";
    var f;
    var g;
    var h;
    var k;
    if (typeof performance === "object" && typeof performance.now === "function") {
      var l = performance;
      exports.unstable_now = function() {
        return l.now();
      };
    } else {
      var p = Date, q = p.now();
      exports.unstable_now = function() {
        return p.now() - q;
      };
    }
    if (typeof window === "undefined" || typeof MessageChannel !== "function") {
      var t2 = null, u = null, w = function() {
        if (t2 !== null)
          try {
            var a = exports.unstable_now();
            t2(true, a);
            t2 = null;
          } catch (b) {
            throw setTimeout(w, 0), b;
          }
      };
      f = function(a) {
        t2 !== null ? setTimeout(f, 0, a) : (t2 = a, setTimeout(w, 0));
      };
      g = function(a, b) {
        u = setTimeout(a, b);
      };
      h = function() {
        clearTimeout(u);
      };
      exports.unstable_shouldYield = function() {
        return false;
      };
      k = exports.unstable_forceFrameRate = function() {
      };
    } else {
      var x = window.setTimeout, y = window.clearTimeout;
      if (typeof console !== "undefined") {
        var z = window.cancelAnimationFrame;
        typeof window.requestAnimationFrame !== "function" && console.error("This browser doesn't support requestAnimationFrame. Make sure that you load a polyfill in older browsers. https://reactjs.org/link/react-polyfills");
        typeof z !== "function" && console.error("This browser doesn't support cancelAnimationFrame. Make sure that you load a polyfill in older browsers. https://reactjs.org/link/react-polyfills");
      }
      var A = false, B = null, C = -1, D = 5, E = 0;
      exports.unstable_shouldYield = function() {
        return exports.unstable_now() >= E;
      };
      k = function() {
      };
      exports.unstable_forceFrameRate = function(a) {
        0 > a || 125 < a ? console.error("forceFrameRate takes a positive int between 0 and 125, forcing frame rates higher than 125 fps is not supported") : D = 0 < a ? Math.floor(1e3 / a) : 5;
      };
      var F = new MessageChannel(), G = F.port2;
      F.port1.onmessage = function() {
        if (B !== null) {
          var a = exports.unstable_now();
          E = a + D;
          try {
            B(true, a) ? G.postMessage(null) : (A = false, B = null);
          } catch (b) {
            throw G.postMessage(null), b;
          }
        } else
          A = false;
      };
      f = function(a) {
        B = a;
        A || (A = true, G.postMessage(null));
      };
      g = function(a, b) {
        C = x(function() {
          a(exports.unstable_now());
        }, b);
      };
      h = function() {
        y(C);
        C = -1;
      };
    }
    function H(a, b) {
      var c = a.length;
      a.push(b);
      a:
        for (; ; ) {
          var d = c - 1 >>> 1, e = a[d];
          if (e !== void 0 && 0 < I(e, b))
            a[d] = b, a[c] = e, c = d;
          else
            break a;
        }
    }
    function J(a) {
      a = a[0];
      return a === void 0 ? null : a;
    }
    function K(a) {
      var b = a[0];
      if (b !== void 0) {
        var c = a.pop();
        if (c !== b) {
          a[0] = c;
          a:
            for (var d = 0, e = a.length; d < e; ) {
              var m = 2 * (d + 1) - 1, n = a[m], v = m + 1, r = a[v];
              if (n !== void 0 && 0 > I(n, c))
                r !== void 0 && 0 > I(r, n) ? (a[d] = r, a[v] = c, d = v) : (a[d] = n, a[m] = c, d = m);
              else if (r !== void 0 && 0 > I(r, c))
                a[d] = r, a[v] = c, d = v;
              else
                break a;
            }
        }
        return b;
      }
      return null;
    }
    function I(a, b) {
      var c = a.sortIndex - b.sortIndex;
      return c !== 0 ? c : a.id - b.id;
    }
    var L = [];
    var M = [];
    var N = 1;
    var O = null;
    var P = 3;
    var Q = false;
    var R = false;
    var S = false;
    function T(a) {
      for (var b = J(M); b !== null; ) {
        if (b.callback === null)
          K(M);
        else if (b.startTime <= a)
          K(M), b.sortIndex = b.expirationTime, H(L, b);
        else
          break;
        b = J(M);
      }
    }
    function U(a) {
      S = false;
      T(a);
      if (!R)
        if (J(L) !== null)
          R = true, f(V);
        else {
          var b = J(M);
          b !== null && g(U, b.startTime - a);
        }
    }
    function V(a, b) {
      R = false;
      S && (S = false, h());
      Q = true;
      var c = P;
      try {
        T(b);
        for (O = J(L); O !== null && (!(O.expirationTime > b) || a && !exports.unstable_shouldYield()); ) {
          var d = O.callback;
          if (typeof d === "function") {
            O.callback = null;
            P = O.priorityLevel;
            var e = d(O.expirationTime <= b);
            b = exports.unstable_now();
            typeof e === "function" ? O.callback = e : O === J(L) && K(L);
            T(b);
          } else
            K(L);
          O = J(L);
        }
        if (O !== null)
          var m = true;
        else {
          var n = J(M);
          n !== null && g(U, n.startTime - b);
          m = false;
        }
        return m;
      } finally {
        O = null, P = c, Q = false;
      }
    }
    var W = k;
    exports.unstable_IdlePriority = 5;
    exports.unstable_ImmediatePriority = 1;
    exports.unstable_LowPriority = 4;
    exports.unstable_NormalPriority = 3;
    exports.unstable_Profiling = null;
    exports.unstable_UserBlockingPriority = 2;
    exports.unstable_cancelCallback = function(a) {
      a.callback = null;
    };
    exports.unstable_continueExecution = function() {
      R || Q || (R = true, f(V));
    };
    exports.unstable_getCurrentPriorityLevel = function() {
      return P;
    };
    exports.unstable_getFirstCallbackNode = function() {
      return J(L);
    };
    exports.unstable_next = function(a) {
      switch (P) {
        case 1:
        case 2:
        case 3:
          var b = 3;
          break;
        default:
          b = P;
      }
      var c = P;
      P = b;
      try {
        return a();
      } finally {
        P = c;
      }
    };
    exports.unstable_pauseExecution = function() {
    };
    exports.unstable_requestPaint = W;
    exports.unstable_runWithPriority = function(a, b) {
      switch (a) {
        case 1:
        case 2:
        case 3:
        case 4:
        case 5:
          break;
        default:
          a = 3;
      }
      var c = P;
      P = a;
      try {
        return b();
      } finally {
        P = c;
      }
    };
    exports.unstable_scheduleCallback = function(a, b, c) {
      var d = exports.unstable_now();
      typeof c === "object" && c !== null ? (c = c.delay, c = typeof c === "number" && 0 < c ? d + c : d) : c = d;
      switch (a) {
        case 1:
          var e = -1;
          break;
        case 2:
          e = 250;
          break;
        case 5:
          e = 1073741823;
          break;
        case 4:
          e = 1e4;
          break;
        default:
          e = 5e3;
      }
      e = c + e;
      a = {id: N++, callback: b, priorityLevel: a, startTime: c, expirationTime: e, sortIndex: -1};
      c > d ? (a.sortIndex = c, H(M, a), J(L) === null && a === J(M) && (S ? h() : S = true, g(U, c - d))) : (a.sortIndex = e, H(L, a), R || Q || (R = true, f(V)));
      return a;
    };
    exports.unstable_wrapCallback = function(a) {
      var b = P;
      return function() {
        var c = P;
        P = b;
        try {
          return a.apply(this, arguments);
        } finally {
          P = c;
        }
      };
    };
  });

  // node_modules/scheduler/index.js
  var require_scheduler = __commonJS((exports, module) => {
    "use strict";
    if (true) {
      module.exports = require_scheduler_production_min();
    } else {
      module.exports = null;
    }
  });

  // node_modules/react-dom/cjs/react-dom.production.min.js
  var require_react_dom_production_min = __commonJS((exports) => {
    /** @license React v17.0.1
     * react-dom.production.min.js
     *
     * Copyright (c) Facebook, Inc. and its affiliates.
     *
     * This source code is licensed under the MIT license found in the
     * LICENSE file in the root directory of this source tree.
     */
    "use strict";
    var aa = require_react();
    var m = require_object_assign();
    var r = require_scheduler();
    function y(a) {
      for (var b = "https://reactjs.org/docs/error-decoder.html?invariant=" + a, c = 1; c < arguments.length; c++)
        b += "&args[]=" + encodeURIComponent(arguments[c]);
      return "Minified React error #" + a + "; visit " + b + " for the full message or use the non-minified dev environment for full errors and additional helpful warnings.";
    }
    if (!aa)
      throw Error(y(227));
    var ba = new Set();
    var ca = {};
    function da(a, b) {
      ea(a, b);
      ea(a + "Capture", b);
    }
    function ea(a, b) {
      ca[a] = b;
      for (a = 0; a < b.length; a++)
        ba.add(b[a]);
    }
    var fa = !(typeof window === "undefined" || typeof window.document === "undefined" || typeof window.document.createElement === "undefined");
    var ha = /^[:A-Z_a-z\u00C0-\u00D6\u00D8-\u00F6\u00F8-\u02FF\u0370-\u037D\u037F-\u1FFF\u200C-\u200D\u2070-\u218F\u2C00-\u2FEF\u3001-\uD7FF\uF900-\uFDCF\uFDF0-\uFFFD][:A-Z_a-z\u00C0-\u00D6\u00D8-\u00F6\u00F8-\u02FF\u0370-\u037D\u037F-\u1FFF\u200C-\u200D\u2070-\u218F\u2C00-\u2FEF\u3001-\uD7FF\uF900-\uFDCF\uFDF0-\uFFFD\-.0-9\u00B7\u0300-\u036F\u203F-\u2040]*$/;
    var ia = Object.prototype.hasOwnProperty;
    var ja = {};
    var ka = {};
    function la(a) {
      if (ia.call(ka, a))
        return true;
      if (ia.call(ja, a))
        return false;
      if (ha.test(a))
        return ka[a] = true;
      ja[a] = true;
      return false;
    }
    function ma(a, b, c, d) {
      if (c !== null && c.type === 0)
        return false;
      switch (typeof b) {
        case "function":
        case "symbol":
          return true;
        case "boolean":
          if (d)
            return false;
          if (c !== null)
            return !c.acceptsBooleans;
          a = a.toLowerCase().slice(0, 5);
          return a !== "data-" && a !== "aria-";
        default:
          return false;
      }
    }
    function na(a, b, c, d) {
      if (b === null || typeof b === "undefined" || ma(a, b, c, d))
        return true;
      if (d)
        return false;
      if (c !== null)
        switch (c.type) {
          case 3:
            return !b;
          case 4:
            return b === false;
          case 5:
            return isNaN(b);
          case 6:
            return isNaN(b) || 1 > b;
        }
      return false;
    }
    function B(a, b, c, d, e, f, g) {
      this.acceptsBooleans = b === 2 || b === 3 || b === 4;
      this.attributeName = d;
      this.attributeNamespace = e;
      this.mustUseProperty = c;
      this.propertyName = a;
      this.type = b;
      this.sanitizeURL = f;
      this.removeEmptyString = g;
    }
    var D = {};
    "children dangerouslySetInnerHTML defaultValue defaultChecked innerHTML suppressContentEditableWarning suppressHydrationWarning style".split(" ").forEach(function(a) {
      D[a] = new B(a, 0, false, a, null, false, false);
    });
    [["acceptCharset", "accept-charset"], ["className", "class"], ["htmlFor", "for"], ["httpEquiv", "http-equiv"]].forEach(function(a) {
      var b = a[0];
      D[b] = new B(b, 1, false, a[1], null, false, false);
    });
    ["contentEditable", "draggable", "spellCheck", "value"].forEach(function(a) {
      D[a] = new B(a, 2, false, a.toLowerCase(), null, false, false);
    });
    ["autoReverse", "externalResourcesRequired", "focusable", "preserveAlpha"].forEach(function(a) {
      D[a] = new B(a, 2, false, a, null, false, false);
    });
    "allowFullScreen async autoFocus autoPlay controls default defer disabled disablePictureInPicture disableRemotePlayback formNoValidate hidden loop noModule noValidate open playsInline readOnly required reversed scoped seamless itemScope".split(" ").forEach(function(a) {
      D[a] = new B(a, 3, false, a.toLowerCase(), null, false, false);
    });
    ["checked", "multiple", "muted", "selected"].forEach(function(a) {
      D[a] = new B(a, 3, true, a, null, false, false);
    });
    ["capture", "download"].forEach(function(a) {
      D[a] = new B(a, 4, false, a, null, false, false);
    });
    ["cols", "rows", "size", "span"].forEach(function(a) {
      D[a] = new B(a, 6, false, a, null, false, false);
    });
    ["rowSpan", "start"].forEach(function(a) {
      D[a] = new B(a, 5, false, a.toLowerCase(), null, false, false);
    });
    var oa = /[\-:]([a-z])/g;
    function pa(a) {
      return a[1].toUpperCase();
    }
    "accent-height alignment-baseline arabic-form baseline-shift cap-height clip-path clip-rule color-interpolation color-interpolation-filters color-profile color-rendering dominant-baseline enable-background fill-opacity fill-rule flood-color flood-opacity font-family font-size font-size-adjust font-stretch font-style font-variant font-weight glyph-name glyph-orientation-horizontal glyph-orientation-vertical horiz-adv-x horiz-origin-x image-rendering letter-spacing lighting-color marker-end marker-mid marker-start overline-position overline-thickness paint-order panose-1 pointer-events rendering-intent shape-rendering stop-color stop-opacity strikethrough-position strikethrough-thickness stroke-dasharray stroke-dashoffset stroke-linecap stroke-linejoin stroke-miterlimit stroke-opacity stroke-width text-anchor text-decoration text-rendering underline-position underline-thickness unicode-bidi unicode-range units-per-em v-alphabetic v-hanging v-ideographic v-mathematical vector-effect vert-adv-y vert-origin-x vert-origin-y word-spacing writing-mode xmlns:xlink x-height".split(" ").forEach(function(a) {
      var b = a.replace(oa, pa);
      D[b] = new B(b, 1, false, a, null, false, false);
    });
    "xlink:actuate xlink:arcrole xlink:role xlink:show xlink:title xlink:type".split(" ").forEach(function(a) {
      var b = a.replace(oa, pa);
      D[b] = new B(b, 1, false, a, "http://www.w3.org/1999/xlink", false, false);
    });
    ["xml:base", "xml:lang", "xml:space"].forEach(function(a) {
      var b = a.replace(oa, pa);
      D[b] = new B(b, 1, false, a, "http://www.w3.org/XML/1998/namespace", false, false);
    });
    ["tabIndex", "crossOrigin"].forEach(function(a) {
      D[a] = new B(a, 1, false, a.toLowerCase(), null, false, false);
    });
    D.xlinkHref = new B("xlinkHref", 1, false, "xlink:href", "http://www.w3.org/1999/xlink", true, false);
    ["src", "href", "action", "formAction"].forEach(function(a) {
      D[a] = new B(a, 1, false, a.toLowerCase(), null, true, true);
    });
    function qa(a, b, c, d) {
      var e = D.hasOwnProperty(b) ? D[b] : null;
      var f = e !== null ? e.type === 0 : d ? false : !(2 < b.length) || b[0] !== "o" && b[0] !== "O" || b[1] !== "n" && b[1] !== "N" ? false : true;
      f || (na(b, c, e, d) && (c = null), d || e === null ? la(b) && (c === null ? a.removeAttribute(b) : a.setAttribute(b, "" + c)) : e.mustUseProperty ? a[e.propertyName] = c === null ? e.type === 3 ? false : "" : c : (b = e.attributeName, d = e.attributeNamespace, c === null ? a.removeAttribute(b) : (e = e.type, c = e === 3 || e === 4 && c === true ? "" : "" + c, d ? a.setAttributeNS(d, b, c) : a.setAttribute(b, c))));
    }
    var ra = aa.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED;
    var sa = 60103;
    var ta = 60106;
    var ua = 60107;
    var wa = 60108;
    var xa = 60114;
    var ya = 60109;
    var za = 60110;
    var Aa = 60112;
    var Ba = 60113;
    var Ca = 60120;
    var Da = 60115;
    var Ea = 60116;
    var Fa = 60121;
    var Ga = 60128;
    var Ha = 60129;
    var Ia = 60130;
    var Ja = 60131;
    if (typeof Symbol === "function" && Symbol.for) {
      var E = Symbol.for;
      sa = E("react.element");
      ta = E("react.portal");
      ua = E("react.fragment");
      wa = E("react.strict_mode");
      xa = E("react.profiler");
      ya = E("react.provider");
      za = E("react.context");
      Aa = E("react.forward_ref");
      Ba = E("react.suspense");
      Ca = E("react.suspense_list");
      Da = E("react.memo");
      Ea = E("react.lazy");
      Fa = E("react.block");
      E("react.scope");
      Ga = E("react.opaque.id");
      Ha = E("react.debug_trace_mode");
      Ia = E("react.offscreen");
      Ja = E("react.legacy_hidden");
    }
    var Ka = typeof Symbol === "function" && Symbol.iterator;
    function La(a) {
      if (a === null || typeof a !== "object")
        return null;
      a = Ka && a[Ka] || a["@@iterator"];
      return typeof a === "function" ? a : null;
    }
    var Ma;
    function Na(a) {
      if (Ma === void 0)
        try {
          throw Error();
        } catch (c) {
          var b = c.stack.trim().match(/\n( *(at )?)/);
          Ma = b && b[1] || "";
        }
      return "\n" + Ma + a;
    }
    var Oa = false;
    function Pa(a, b) {
      if (!a || Oa)
        return "";
      Oa = true;
      var c = Error.prepareStackTrace;
      Error.prepareStackTrace = void 0;
      try {
        if (b)
          if (b = function() {
            throw Error();
          }, Object.defineProperty(b.prototype, "props", {set: function() {
            throw Error();
          }}), typeof Reflect === "object" && Reflect.construct) {
            try {
              Reflect.construct(b, []);
            } catch (k) {
              var d = k;
            }
            Reflect.construct(a, [], b);
          } else {
            try {
              b.call();
            } catch (k) {
              d = k;
            }
            a.call(b.prototype);
          }
        else {
          try {
            throw Error();
          } catch (k) {
            d = k;
          }
          a();
        }
      } catch (k) {
        if (k && d && typeof k.stack === "string") {
          for (var e = k.stack.split("\n"), f = d.stack.split("\n"), g = e.length - 1, h = f.length - 1; 1 <= g && 0 <= h && e[g] !== f[h]; )
            h--;
          for (; 1 <= g && 0 <= h; g--, h--)
            if (e[g] !== f[h]) {
              if (g !== 1 || h !== 1) {
                do
                  if (g--, h--, 0 > h || e[g] !== f[h])
                    return "\n" + e[g].replace(" at new ", " at ");
                while (1 <= g && 0 <= h);
              }
              break;
            }
        }
      } finally {
        Oa = false, Error.prepareStackTrace = c;
      }
      return (a = a ? a.displayName || a.name : "") ? Na(a) : "";
    }
    function Qa(a) {
      switch (a.tag) {
        case 5:
          return Na(a.type);
        case 16:
          return Na("Lazy");
        case 13:
          return Na("Suspense");
        case 19:
          return Na("SuspenseList");
        case 0:
        case 2:
        case 15:
          return a = Pa(a.type, false), a;
        case 11:
          return a = Pa(a.type.render, false), a;
        case 22:
          return a = Pa(a.type._render, false), a;
        case 1:
          return a = Pa(a.type, true), a;
        default:
          return "";
      }
    }
    function Ra(a) {
      if (a == null)
        return null;
      if (typeof a === "function")
        return a.displayName || a.name || null;
      if (typeof a === "string")
        return a;
      switch (a) {
        case ua:
          return "Fragment";
        case ta:
          return "Portal";
        case xa:
          return "Profiler";
        case wa:
          return "StrictMode";
        case Ba:
          return "Suspense";
        case Ca:
          return "SuspenseList";
      }
      if (typeof a === "object")
        switch (a.$$typeof) {
          case za:
            return (a.displayName || "Context") + ".Consumer";
          case ya:
            return (a._context.displayName || "Context") + ".Provider";
          case Aa:
            var b = a.render;
            b = b.displayName || b.name || "";
            return a.displayName || (b !== "" ? "ForwardRef(" + b + ")" : "ForwardRef");
          case Da:
            return Ra(a.type);
          case Fa:
            return Ra(a._render);
          case Ea:
            b = a._payload;
            a = a._init;
            try {
              return Ra(a(b));
            } catch (c) {
            }
        }
      return null;
    }
    function Sa(a) {
      switch (typeof a) {
        case "boolean":
        case "number":
        case "object":
        case "string":
        case "undefined":
          return a;
        default:
          return "";
      }
    }
    function Ta(a) {
      var b = a.type;
      return (a = a.nodeName) && a.toLowerCase() === "input" && (b === "checkbox" || b === "radio");
    }
    function Ua(a) {
      var b = Ta(a) ? "checked" : "value", c = Object.getOwnPropertyDescriptor(a.constructor.prototype, b), d = "" + a[b];
      if (!a.hasOwnProperty(b) && typeof c !== "undefined" && typeof c.get === "function" && typeof c.set === "function") {
        var e = c.get, f = c.set;
        Object.defineProperty(a, b, {configurable: true, get: function() {
          return e.call(this);
        }, set: function(a2) {
          d = "" + a2;
          f.call(this, a2);
        }});
        Object.defineProperty(a, b, {enumerable: c.enumerable});
        return {getValue: function() {
          return d;
        }, setValue: function(a2) {
          d = "" + a2;
        }, stopTracking: function() {
          a._valueTracker = null;
          delete a[b];
        }};
      }
    }
    function Va(a) {
      a._valueTracker || (a._valueTracker = Ua(a));
    }
    function Wa(a) {
      if (!a)
        return false;
      var b = a._valueTracker;
      if (!b)
        return true;
      var c = b.getValue();
      var d = "";
      a && (d = Ta(a) ? a.checked ? "true" : "false" : a.value);
      a = d;
      return a !== c ? (b.setValue(a), true) : false;
    }
    function Xa(a) {
      a = a || (typeof document !== "undefined" ? document : void 0);
      if (typeof a === "undefined")
        return null;
      try {
        return a.activeElement || a.body;
      } catch (b) {
        return a.body;
      }
    }
    function Ya(a, b) {
      var c = b.checked;
      return m({}, b, {defaultChecked: void 0, defaultValue: void 0, value: void 0, checked: c != null ? c : a._wrapperState.initialChecked});
    }
    function Za(a, b) {
      var c = b.defaultValue == null ? "" : b.defaultValue, d = b.checked != null ? b.checked : b.defaultChecked;
      c = Sa(b.value != null ? b.value : c);
      a._wrapperState = {initialChecked: d, initialValue: c, controlled: b.type === "checkbox" || b.type === "radio" ? b.checked != null : b.value != null};
    }
    function $a(a, b) {
      b = b.checked;
      b != null && qa(a, "checked", b, false);
    }
    function ab(a, b) {
      $a(a, b);
      var c = Sa(b.value), d = b.type;
      if (c != null)
        if (d === "number") {
          if (c === 0 && a.value === "" || a.value != c)
            a.value = "" + c;
        } else
          a.value !== "" + c && (a.value = "" + c);
      else if (d === "submit" || d === "reset") {
        a.removeAttribute("value");
        return;
      }
      b.hasOwnProperty("value") ? bb(a, b.type, c) : b.hasOwnProperty("defaultValue") && bb(a, b.type, Sa(b.defaultValue));
      b.checked == null && b.defaultChecked != null && (a.defaultChecked = !!b.defaultChecked);
    }
    function cb(a, b, c) {
      if (b.hasOwnProperty("value") || b.hasOwnProperty("defaultValue")) {
        var d = b.type;
        if (!(d !== "submit" && d !== "reset" || b.value !== void 0 && b.value !== null))
          return;
        b = "" + a._wrapperState.initialValue;
        c || b === a.value || (a.value = b);
        a.defaultValue = b;
      }
      c = a.name;
      c !== "" && (a.name = "");
      a.defaultChecked = !!a._wrapperState.initialChecked;
      c !== "" && (a.name = c);
    }
    function bb(a, b, c) {
      if (b !== "number" || Xa(a.ownerDocument) !== a)
        c == null ? a.defaultValue = "" + a._wrapperState.initialValue : a.defaultValue !== "" + c && (a.defaultValue = "" + c);
    }
    function db(a) {
      var b = "";
      aa.Children.forEach(a, function(a2) {
        a2 != null && (b += a2);
      });
      return b;
    }
    function eb(a, b) {
      a = m({children: void 0}, b);
      if (b = db(b.children))
        a.children = b;
      return a;
    }
    function fb(a, b, c, d) {
      a = a.options;
      if (b) {
        b = {};
        for (var e = 0; e < c.length; e++)
          b["$" + c[e]] = true;
        for (c = 0; c < a.length; c++)
          e = b.hasOwnProperty("$" + a[c].value), a[c].selected !== e && (a[c].selected = e), e && d && (a[c].defaultSelected = true);
      } else {
        c = "" + Sa(c);
        b = null;
        for (e = 0; e < a.length; e++) {
          if (a[e].value === c) {
            a[e].selected = true;
            d && (a[e].defaultSelected = true);
            return;
          }
          b !== null || a[e].disabled || (b = a[e]);
        }
        b !== null && (b.selected = true);
      }
    }
    function gb(a, b) {
      if (b.dangerouslySetInnerHTML != null)
        throw Error(y(91));
      return m({}, b, {value: void 0, defaultValue: void 0, children: "" + a._wrapperState.initialValue});
    }
    function hb(a, b) {
      var c = b.value;
      if (c == null) {
        c = b.children;
        b = b.defaultValue;
        if (c != null) {
          if (b != null)
            throw Error(y(92));
          if (Array.isArray(c)) {
            if (!(1 >= c.length))
              throw Error(y(93));
            c = c[0];
          }
          b = c;
        }
        b == null && (b = "");
        c = b;
      }
      a._wrapperState = {initialValue: Sa(c)};
    }
    function ib(a, b) {
      var c = Sa(b.value), d = Sa(b.defaultValue);
      c != null && (c = "" + c, c !== a.value && (a.value = c), b.defaultValue == null && a.defaultValue !== c && (a.defaultValue = c));
      d != null && (a.defaultValue = "" + d);
    }
    function jb(a) {
      var b = a.textContent;
      b === a._wrapperState.initialValue && b !== "" && b !== null && (a.value = b);
    }
    var kb = {html: "http://www.w3.org/1999/xhtml", mathml: "http://www.w3.org/1998/Math/MathML", svg: "http://www.w3.org/2000/svg"};
    function lb(a) {
      switch (a) {
        case "svg":
          return "http://www.w3.org/2000/svg";
        case "math":
          return "http://www.w3.org/1998/Math/MathML";
        default:
          return "http://www.w3.org/1999/xhtml";
      }
    }
    function mb(a, b) {
      return a == null || a === "http://www.w3.org/1999/xhtml" ? lb(b) : a === "http://www.w3.org/2000/svg" && b === "foreignObject" ? "http://www.w3.org/1999/xhtml" : a;
    }
    var nb;
    var ob = function(a) {
      return typeof MSApp !== "undefined" && MSApp.execUnsafeLocalFunction ? function(b, c, d, e) {
        MSApp.execUnsafeLocalFunction(function() {
          return a(b, c, d, e);
        });
      } : a;
    }(function(a, b) {
      if (a.namespaceURI !== kb.svg || "innerHTML" in a)
        a.innerHTML = b;
      else {
        nb = nb || document.createElement("div");
        nb.innerHTML = "<svg>" + b.valueOf().toString() + "</svg>";
        for (b = nb.firstChild; a.firstChild; )
          a.removeChild(a.firstChild);
        for (; b.firstChild; )
          a.appendChild(b.firstChild);
      }
    });
    function pb(a, b) {
      if (b) {
        var c = a.firstChild;
        if (c && c === a.lastChild && c.nodeType === 3) {
          c.nodeValue = b;
          return;
        }
      }
      a.textContent = b;
    }
    var qb = {
      animationIterationCount: true,
      borderImageOutset: true,
      borderImageSlice: true,
      borderImageWidth: true,
      boxFlex: true,
      boxFlexGroup: true,
      boxOrdinalGroup: true,
      columnCount: true,
      columns: true,
      flex: true,
      flexGrow: true,
      flexPositive: true,
      flexShrink: true,
      flexNegative: true,
      flexOrder: true,
      gridArea: true,
      gridRow: true,
      gridRowEnd: true,
      gridRowSpan: true,
      gridRowStart: true,
      gridColumn: true,
      gridColumnEnd: true,
      gridColumnSpan: true,
      gridColumnStart: true,
      fontWeight: true,
      lineClamp: true,
      lineHeight: true,
      opacity: true,
      order: true,
      orphans: true,
      tabSize: true,
      widows: true,
      zIndex: true,
      zoom: true,
      fillOpacity: true,
      floodOpacity: true,
      stopOpacity: true,
      strokeDasharray: true,
      strokeDashoffset: true,
      strokeMiterlimit: true,
      strokeOpacity: true,
      strokeWidth: true
    };
    var rb = ["Webkit", "ms", "Moz", "O"];
    Object.keys(qb).forEach(function(a) {
      rb.forEach(function(b) {
        b = b + a.charAt(0).toUpperCase() + a.substring(1);
        qb[b] = qb[a];
      });
    });
    function sb(a, b, c) {
      return b == null || typeof b === "boolean" || b === "" ? "" : c || typeof b !== "number" || b === 0 || qb.hasOwnProperty(a) && qb[a] ? ("" + b).trim() : b + "px";
    }
    function tb(a, b) {
      a = a.style;
      for (var c in b)
        if (b.hasOwnProperty(c)) {
          var d = c.indexOf("--") === 0, e = sb(c, b[c], d);
          c === "float" && (c = "cssFloat");
          d ? a.setProperty(c, e) : a[c] = e;
        }
    }
    var ub = m({menuitem: true}, {area: true, base: true, br: true, col: true, embed: true, hr: true, img: true, input: true, keygen: true, link: true, meta: true, param: true, source: true, track: true, wbr: true});
    function vb(a, b) {
      if (b) {
        if (ub[a] && (b.children != null || b.dangerouslySetInnerHTML != null))
          throw Error(y(137, a));
        if (b.dangerouslySetInnerHTML != null) {
          if (b.children != null)
            throw Error(y(60));
          if (!(typeof b.dangerouslySetInnerHTML === "object" && "__html" in b.dangerouslySetInnerHTML))
            throw Error(y(61));
        }
        if (b.style != null && typeof b.style !== "object")
          throw Error(y(62));
      }
    }
    function wb(a, b) {
      if (a.indexOf("-") === -1)
        return typeof b.is === "string";
      switch (a) {
        case "annotation-xml":
        case "color-profile":
        case "font-face":
        case "font-face-src":
        case "font-face-uri":
        case "font-face-format":
        case "font-face-name":
        case "missing-glyph":
          return false;
        default:
          return true;
      }
    }
    function xb(a) {
      a = a.target || a.srcElement || window;
      a.correspondingUseElement && (a = a.correspondingUseElement);
      return a.nodeType === 3 ? a.parentNode : a;
    }
    var yb = null;
    var zb = null;
    var Ab = null;
    function Bb(a) {
      if (a = Cb(a)) {
        if (typeof yb !== "function")
          throw Error(y(280));
        var b = a.stateNode;
        b && (b = Db(b), yb(a.stateNode, a.type, b));
      }
    }
    function Eb(a) {
      zb ? Ab ? Ab.push(a) : Ab = [a] : zb = a;
    }
    function Fb() {
      if (zb) {
        var a = zb, b = Ab;
        Ab = zb = null;
        Bb(a);
        if (b)
          for (a = 0; a < b.length; a++)
            Bb(b[a]);
      }
    }
    function Gb(a, b) {
      return a(b);
    }
    function Hb(a, b, c, d, e) {
      return a(b, c, d, e);
    }
    function Ib() {
    }
    var Jb = Gb;
    var Kb = false;
    var Lb = false;
    function Mb() {
      if (zb !== null || Ab !== null)
        Ib(), Fb();
    }
    function Nb(a, b, c) {
      if (Lb)
        return a(b, c);
      Lb = true;
      try {
        return Jb(a, b, c);
      } finally {
        Lb = false, Mb();
      }
    }
    function Ob(a, b) {
      var c = a.stateNode;
      if (c === null)
        return null;
      var d = Db(c);
      if (d === null)
        return null;
      c = d[b];
      a:
        switch (b) {
          case "onClick":
          case "onClickCapture":
          case "onDoubleClick":
          case "onDoubleClickCapture":
          case "onMouseDown":
          case "onMouseDownCapture":
          case "onMouseMove":
          case "onMouseMoveCapture":
          case "onMouseUp":
          case "onMouseUpCapture":
          case "onMouseEnter":
            (d = !d.disabled) || (a = a.type, d = !(a === "button" || a === "input" || a === "select" || a === "textarea"));
            a = !d;
            break a;
          default:
            a = false;
        }
      if (a)
        return null;
      if (c && typeof c !== "function")
        throw Error(y(231, b, typeof c));
      return c;
    }
    var Pb = false;
    if (fa)
      try {
        var Qb = {};
        Object.defineProperty(Qb, "passive", {get: function() {
          Pb = true;
        }});
        window.addEventListener("test", Qb, Qb);
        window.removeEventListener("test", Qb, Qb);
      } catch (a) {
        Pb = false;
      }
    function Rb(a, b, c, d, e, f, g, h, k) {
      var l = Array.prototype.slice.call(arguments, 3);
      try {
        b.apply(c, l);
      } catch (n) {
        this.onError(n);
      }
    }
    var Sb = false;
    var Tb = null;
    var Ub = false;
    var Vb = null;
    var Wb = {onError: function(a) {
      Sb = true;
      Tb = a;
    }};
    function Xb(a, b, c, d, e, f, g, h, k) {
      Sb = false;
      Tb = null;
      Rb.apply(Wb, arguments);
    }
    function Yb(a, b, c, d, e, f, g, h, k) {
      Xb.apply(this, arguments);
      if (Sb) {
        if (Sb) {
          var l = Tb;
          Sb = false;
          Tb = null;
        } else
          throw Error(y(198));
        Ub || (Ub = true, Vb = l);
      }
    }
    function Zb(a) {
      var b = a, c = a;
      if (a.alternate)
        for (; b.return; )
          b = b.return;
      else {
        a = b;
        do
          b = a, (b.flags & 1026) !== 0 && (c = b.return), a = b.return;
        while (a);
      }
      return b.tag === 3 ? c : null;
    }
    function $b(a) {
      if (a.tag === 13) {
        var b = a.memoizedState;
        b === null && (a = a.alternate, a !== null && (b = a.memoizedState));
        if (b !== null)
          return b.dehydrated;
      }
      return null;
    }
    function ac(a) {
      if (Zb(a) !== a)
        throw Error(y(188));
    }
    function bc(a) {
      var b = a.alternate;
      if (!b) {
        b = Zb(a);
        if (b === null)
          throw Error(y(188));
        return b !== a ? null : a;
      }
      for (var c = a, d = b; ; ) {
        var e = c.return;
        if (e === null)
          break;
        var f = e.alternate;
        if (f === null) {
          d = e.return;
          if (d !== null) {
            c = d;
            continue;
          }
          break;
        }
        if (e.child === f.child) {
          for (f = e.child; f; ) {
            if (f === c)
              return ac(e), a;
            if (f === d)
              return ac(e), b;
            f = f.sibling;
          }
          throw Error(y(188));
        }
        if (c.return !== d.return)
          c = e, d = f;
        else {
          for (var g = false, h = e.child; h; ) {
            if (h === c) {
              g = true;
              c = e;
              d = f;
              break;
            }
            if (h === d) {
              g = true;
              d = e;
              c = f;
              break;
            }
            h = h.sibling;
          }
          if (!g) {
            for (h = f.child; h; ) {
              if (h === c) {
                g = true;
                c = f;
                d = e;
                break;
              }
              if (h === d) {
                g = true;
                d = f;
                c = e;
                break;
              }
              h = h.sibling;
            }
            if (!g)
              throw Error(y(189));
          }
        }
        if (c.alternate !== d)
          throw Error(y(190));
      }
      if (c.tag !== 3)
        throw Error(y(188));
      return c.stateNode.current === c ? a : b;
    }
    function cc(a) {
      a = bc(a);
      if (!a)
        return null;
      for (var b = a; ; ) {
        if (b.tag === 5 || b.tag === 6)
          return b;
        if (b.child)
          b.child.return = b, b = b.child;
        else {
          if (b === a)
            break;
          for (; !b.sibling; ) {
            if (!b.return || b.return === a)
              return null;
            b = b.return;
          }
          b.sibling.return = b.return;
          b = b.sibling;
        }
      }
      return null;
    }
    function dc(a, b) {
      for (var c = a.alternate; b !== null; ) {
        if (b === a || b === c)
          return true;
        b = b.return;
      }
      return false;
    }
    var ec;
    var fc;
    var gc;
    var hc;
    var ic = false;
    var jc = [];
    var kc = null;
    var lc = null;
    var mc = null;
    var nc = new Map();
    var oc = new Map();
    var pc = [];
    var qc = "mousedown mouseup touchcancel touchend touchstart auxclick dblclick pointercancel pointerdown pointerup dragend dragstart drop compositionend compositionstart keydown keypress keyup input textInput copy cut paste click change contextmenu reset submit".split(" ");
    function rc(a, b, c, d, e) {
      return {blockedOn: a, domEventName: b, eventSystemFlags: c | 16, nativeEvent: e, targetContainers: [d]};
    }
    function sc(a, b) {
      switch (a) {
        case "focusin":
        case "focusout":
          kc = null;
          break;
        case "dragenter":
        case "dragleave":
          lc = null;
          break;
        case "mouseover":
        case "mouseout":
          mc = null;
          break;
        case "pointerover":
        case "pointerout":
          nc.delete(b.pointerId);
          break;
        case "gotpointercapture":
        case "lostpointercapture":
          oc.delete(b.pointerId);
      }
    }
    function tc(a, b, c, d, e, f) {
      if (a === null || a.nativeEvent !== f)
        return a = rc(b, c, d, e, f), b !== null && (b = Cb(b), b !== null && fc(b)), a;
      a.eventSystemFlags |= d;
      b = a.targetContainers;
      e !== null && b.indexOf(e) === -1 && b.push(e);
      return a;
    }
    function uc(a, b, c, d, e) {
      switch (b) {
        case "focusin":
          return kc = tc(kc, a, b, c, d, e), true;
        case "dragenter":
          return lc = tc(lc, a, b, c, d, e), true;
        case "mouseover":
          return mc = tc(mc, a, b, c, d, e), true;
        case "pointerover":
          var f = e.pointerId;
          nc.set(f, tc(nc.get(f) || null, a, b, c, d, e));
          return true;
        case "gotpointercapture":
          return f = e.pointerId, oc.set(f, tc(oc.get(f) || null, a, b, c, d, e)), true;
      }
      return false;
    }
    function vc(a) {
      var b = wc(a.target);
      if (b !== null) {
        var c = Zb(b);
        if (c !== null) {
          if (b = c.tag, b === 13) {
            if (b = $b(c), b !== null) {
              a.blockedOn = b;
              hc(a.lanePriority, function() {
                r.unstable_runWithPriority(a.priority, function() {
                  gc(c);
                });
              });
              return;
            }
          } else if (b === 3 && c.stateNode.hydrate) {
            a.blockedOn = c.tag === 3 ? c.stateNode.containerInfo : null;
            return;
          }
        }
      }
      a.blockedOn = null;
    }
    function xc(a) {
      if (a.blockedOn !== null)
        return false;
      for (var b = a.targetContainers; 0 < b.length; ) {
        var c = yc(a.domEventName, a.eventSystemFlags, b[0], a.nativeEvent);
        if (c !== null)
          return b = Cb(c), b !== null && fc(b), a.blockedOn = c, false;
        b.shift();
      }
      return true;
    }
    function zc(a, b, c) {
      xc(a) && c.delete(b);
    }
    function Ac() {
      for (ic = false; 0 < jc.length; ) {
        var a = jc[0];
        if (a.blockedOn !== null) {
          a = Cb(a.blockedOn);
          a !== null && ec(a);
          break;
        }
        for (var b = a.targetContainers; 0 < b.length; ) {
          var c = yc(a.domEventName, a.eventSystemFlags, b[0], a.nativeEvent);
          if (c !== null) {
            a.blockedOn = c;
            break;
          }
          b.shift();
        }
        a.blockedOn === null && jc.shift();
      }
      kc !== null && xc(kc) && (kc = null);
      lc !== null && xc(lc) && (lc = null);
      mc !== null && xc(mc) && (mc = null);
      nc.forEach(zc);
      oc.forEach(zc);
    }
    function Bc(a, b) {
      a.blockedOn === b && (a.blockedOn = null, ic || (ic = true, r.unstable_scheduleCallback(r.unstable_NormalPriority, Ac)));
    }
    function Cc(a) {
      function b(b2) {
        return Bc(b2, a);
      }
      if (0 < jc.length) {
        Bc(jc[0], a);
        for (var c = 1; c < jc.length; c++) {
          var d = jc[c];
          d.blockedOn === a && (d.blockedOn = null);
        }
      }
      kc !== null && Bc(kc, a);
      lc !== null && Bc(lc, a);
      mc !== null && Bc(mc, a);
      nc.forEach(b);
      oc.forEach(b);
      for (c = 0; c < pc.length; c++)
        d = pc[c], d.blockedOn === a && (d.blockedOn = null);
      for (; 0 < pc.length && (c = pc[0], c.blockedOn === null); )
        vc(c), c.blockedOn === null && pc.shift();
    }
    function Dc(a, b) {
      var c = {};
      c[a.toLowerCase()] = b.toLowerCase();
      c["Webkit" + a] = "webkit" + b;
      c["Moz" + a] = "moz" + b;
      return c;
    }
    var Ec = {animationend: Dc("Animation", "AnimationEnd"), animationiteration: Dc("Animation", "AnimationIteration"), animationstart: Dc("Animation", "AnimationStart"), transitionend: Dc("Transition", "TransitionEnd")};
    var Fc = {};
    var Gc = {};
    fa && (Gc = document.createElement("div").style, "AnimationEvent" in window || (delete Ec.animationend.animation, delete Ec.animationiteration.animation, delete Ec.animationstart.animation), "TransitionEvent" in window || delete Ec.transitionend.transition);
    function Hc(a) {
      if (Fc[a])
        return Fc[a];
      if (!Ec[a])
        return a;
      var b = Ec[a], c;
      for (c in b)
        if (b.hasOwnProperty(c) && c in Gc)
          return Fc[a] = b[c];
      return a;
    }
    var Ic = Hc("animationend");
    var Jc = Hc("animationiteration");
    var Kc = Hc("animationstart");
    var Lc = Hc("transitionend");
    var Mc = new Map();
    var Nc = new Map();
    var Oc = [
      "abort",
      "abort",
      Ic,
      "animationEnd",
      Jc,
      "animationIteration",
      Kc,
      "animationStart",
      "canplay",
      "canPlay",
      "canplaythrough",
      "canPlayThrough",
      "durationchange",
      "durationChange",
      "emptied",
      "emptied",
      "encrypted",
      "encrypted",
      "ended",
      "ended",
      "error",
      "error",
      "gotpointercapture",
      "gotPointerCapture",
      "load",
      "load",
      "loadeddata",
      "loadedData",
      "loadedmetadata",
      "loadedMetadata",
      "loadstart",
      "loadStart",
      "lostpointercapture",
      "lostPointerCapture",
      "playing",
      "playing",
      "progress",
      "progress",
      "seeking",
      "seeking",
      "stalled",
      "stalled",
      "suspend",
      "suspend",
      "timeupdate",
      "timeUpdate",
      Lc,
      "transitionEnd",
      "waiting",
      "waiting"
    ];
    function Pc(a, b) {
      for (var c = 0; c < a.length; c += 2) {
        var d = a[c], e = a[c + 1];
        e = "on" + (e[0].toUpperCase() + e.slice(1));
        Nc.set(d, b);
        Mc.set(d, e);
        da(e, [d]);
      }
    }
    var Qc = r.unstable_now;
    Qc();
    var F = 8;
    function Rc(a) {
      if ((1 & a) !== 0)
        return F = 15, 1;
      if ((2 & a) !== 0)
        return F = 14, 2;
      if ((4 & a) !== 0)
        return F = 13, 4;
      var b = 24 & a;
      if (b !== 0)
        return F = 12, b;
      if ((a & 32) !== 0)
        return F = 11, 32;
      b = 192 & a;
      if (b !== 0)
        return F = 10, b;
      if ((a & 256) !== 0)
        return F = 9, 256;
      b = 3584 & a;
      if (b !== 0)
        return F = 8, b;
      if ((a & 4096) !== 0)
        return F = 7, 4096;
      b = 4186112 & a;
      if (b !== 0)
        return F = 6, b;
      b = 62914560 & a;
      if (b !== 0)
        return F = 5, b;
      if (a & 67108864)
        return F = 4, 67108864;
      if ((a & 134217728) !== 0)
        return F = 3, 134217728;
      b = 805306368 & a;
      if (b !== 0)
        return F = 2, b;
      if ((1073741824 & a) !== 0)
        return F = 1, 1073741824;
      F = 8;
      return a;
    }
    function Sc(a) {
      switch (a) {
        case 99:
          return 15;
        case 98:
          return 10;
        case 97:
        case 96:
          return 8;
        case 95:
          return 2;
        default:
          return 0;
      }
    }
    function Tc(a) {
      switch (a) {
        case 15:
        case 14:
          return 99;
        case 13:
        case 12:
        case 11:
        case 10:
          return 98;
        case 9:
        case 8:
        case 7:
        case 6:
        case 4:
        case 5:
          return 97;
        case 3:
        case 2:
        case 1:
          return 95;
        case 0:
          return 90;
        default:
          throw Error(y(358, a));
      }
    }
    function Uc(a, b) {
      var c = a.pendingLanes;
      if (c === 0)
        return F = 0;
      var d = 0, e = 0, f = a.expiredLanes, g = a.suspendedLanes, h = a.pingedLanes;
      if (f !== 0)
        d = f, e = F = 15;
      else if (f = c & 134217727, f !== 0) {
        var k = f & ~g;
        k !== 0 ? (d = Rc(k), e = F) : (h &= f, h !== 0 && (d = Rc(h), e = F));
      } else
        f = c & ~g, f !== 0 ? (d = Rc(f), e = F) : h !== 0 && (d = Rc(h), e = F);
      if (d === 0)
        return 0;
      d = 31 - Vc(d);
      d = c & ((0 > d ? 0 : 1 << d) << 1) - 1;
      if (b !== 0 && b !== d && (b & g) === 0) {
        Rc(b);
        if (e <= F)
          return b;
        F = e;
      }
      b = a.entangledLanes;
      if (b !== 0)
        for (a = a.entanglements, b &= d; 0 < b; )
          c = 31 - Vc(b), e = 1 << c, d |= a[c], b &= ~e;
      return d;
    }
    function Wc(a) {
      a = a.pendingLanes & -1073741825;
      return a !== 0 ? a : a & 1073741824 ? 1073741824 : 0;
    }
    function Xc(a, b) {
      switch (a) {
        case 15:
          return 1;
        case 14:
          return 2;
        case 12:
          return a = Yc(24 & ~b), a === 0 ? Xc(10, b) : a;
        case 10:
          return a = Yc(192 & ~b), a === 0 ? Xc(8, b) : a;
        case 8:
          return a = Yc(3584 & ~b), a === 0 && (a = Yc(4186112 & ~b), a === 0 && (a = 512)), a;
        case 2:
          return b = Yc(805306368 & ~b), b === 0 && (b = 268435456), b;
      }
      throw Error(y(358, a));
    }
    function Yc(a) {
      return a & -a;
    }
    function Zc(a) {
      for (var b = [], c = 0; 31 > c; c++)
        b.push(a);
      return b;
    }
    function $c(a, b, c) {
      a.pendingLanes |= b;
      var d = b - 1;
      a.suspendedLanes &= d;
      a.pingedLanes &= d;
      a = a.eventTimes;
      b = 31 - Vc(b);
      a[b] = c;
    }
    var Vc = Math.clz32 ? Math.clz32 : ad;
    var bd = Math.log;
    var cd = Math.LN2;
    function ad(a) {
      return a === 0 ? 32 : 31 - (bd(a) / cd | 0) | 0;
    }
    var dd = r.unstable_UserBlockingPriority;
    var ed = r.unstable_runWithPriority;
    var fd = true;
    function gd(a, b, c, d) {
      Kb || Ib();
      var e = hd, f = Kb;
      Kb = true;
      try {
        Hb(e, a, b, c, d);
      } finally {
        (Kb = f) || Mb();
      }
    }
    function id(a, b, c, d) {
      ed(dd, hd.bind(null, a, b, c, d));
    }
    function hd(a, b, c, d) {
      if (fd) {
        var e;
        if ((e = (b & 4) === 0) && 0 < jc.length && -1 < qc.indexOf(a))
          a = rc(null, a, b, c, d), jc.push(a);
        else {
          var f = yc(a, b, c, d);
          if (f === null)
            e && sc(a, d);
          else {
            if (e) {
              if (-1 < qc.indexOf(a)) {
                a = rc(f, a, b, c, d);
                jc.push(a);
                return;
              }
              if (uc(f, a, b, c, d))
                return;
              sc(a, d);
            }
            jd(a, b, d, null, c);
          }
        }
      }
    }
    function yc(a, b, c, d) {
      var e = xb(d);
      e = wc(e);
      if (e !== null) {
        var f = Zb(e);
        if (f === null)
          e = null;
        else {
          var g = f.tag;
          if (g === 13) {
            e = $b(f);
            if (e !== null)
              return e;
            e = null;
          } else if (g === 3) {
            if (f.stateNode.hydrate)
              return f.tag === 3 ? f.stateNode.containerInfo : null;
            e = null;
          } else
            f !== e && (e = null);
        }
      }
      jd(a, b, d, e, c);
      return null;
    }
    var kd = null;
    var ld = null;
    var md = null;
    function nd() {
      if (md)
        return md;
      var a, b = ld, c = b.length, d, e = "value" in kd ? kd.value : kd.textContent, f = e.length;
      for (a = 0; a < c && b[a] === e[a]; a++)
        ;
      var g = c - a;
      for (d = 1; d <= g && b[c - d] === e[f - d]; d++)
        ;
      return md = e.slice(a, 1 < d ? 1 - d : void 0);
    }
    function od(a) {
      var b = a.keyCode;
      "charCode" in a ? (a = a.charCode, a === 0 && b === 13 && (a = 13)) : a = b;
      a === 10 && (a = 13);
      return 32 <= a || a === 13 ? a : 0;
    }
    function pd() {
      return true;
    }
    function qd() {
      return false;
    }
    function rd(a) {
      function b(b2, d, e, f, g) {
        this._reactName = b2;
        this._targetInst = e;
        this.type = d;
        this.nativeEvent = f;
        this.target = g;
        this.currentTarget = null;
        for (var c in a)
          a.hasOwnProperty(c) && (b2 = a[c], this[c] = b2 ? b2(f) : f[c]);
        this.isDefaultPrevented = (f.defaultPrevented != null ? f.defaultPrevented : f.returnValue === false) ? pd : qd;
        this.isPropagationStopped = qd;
        return this;
      }
      m(b.prototype, {preventDefault: function() {
        this.defaultPrevented = true;
        var a2 = this.nativeEvent;
        a2 && (a2.preventDefault ? a2.preventDefault() : typeof a2.returnValue !== "unknown" && (a2.returnValue = false), this.isDefaultPrevented = pd);
      }, stopPropagation: function() {
        var a2 = this.nativeEvent;
        a2 && (a2.stopPropagation ? a2.stopPropagation() : typeof a2.cancelBubble !== "unknown" && (a2.cancelBubble = true), this.isPropagationStopped = pd);
      }, persist: function() {
      }, isPersistent: pd});
      return b;
    }
    var sd = {eventPhase: 0, bubbles: 0, cancelable: 0, timeStamp: function(a) {
      return a.timeStamp || Date.now();
    }, defaultPrevented: 0, isTrusted: 0};
    var td = rd(sd);
    var ud = m({}, sd, {view: 0, detail: 0});
    var vd = rd(ud);
    var wd;
    var xd;
    var yd;
    var Ad = m({}, ud, {screenX: 0, screenY: 0, clientX: 0, clientY: 0, pageX: 0, pageY: 0, ctrlKey: 0, shiftKey: 0, altKey: 0, metaKey: 0, getModifierState: zd, button: 0, buttons: 0, relatedTarget: function(a) {
      return a.relatedTarget === void 0 ? a.fromElement === a.srcElement ? a.toElement : a.fromElement : a.relatedTarget;
    }, movementX: function(a) {
      if ("movementX" in a)
        return a.movementX;
      a !== yd && (yd && a.type === "mousemove" ? (wd = a.screenX - yd.screenX, xd = a.screenY - yd.screenY) : xd = wd = 0, yd = a);
      return wd;
    }, movementY: function(a) {
      return "movementY" in a ? a.movementY : xd;
    }});
    var Bd = rd(Ad);
    var Cd = m({}, Ad, {dataTransfer: 0});
    var Dd = rd(Cd);
    var Ed = m({}, ud, {relatedTarget: 0});
    var Fd = rd(Ed);
    var Gd = m({}, sd, {animationName: 0, elapsedTime: 0, pseudoElement: 0});
    var Hd = rd(Gd);
    var Id = m({}, sd, {clipboardData: function(a) {
      return "clipboardData" in a ? a.clipboardData : window.clipboardData;
    }});
    var Jd = rd(Id);
    var Kd = m({}, sd, {data: 0});
    var Ld = rd(Kd);
    var Md = {
      Esc: "Escape",
      Spacebar: " ",
      Left: "ArrowLeft",
      Up: "ArrowUp",
      Right: "ArrowRight",
      Down: "ArrowDown",
      Del: "Delete",
      Win: "OS",
      Menu: "ContextMenu",
      Apps: "ContextMenu",
      Scroll: "ScrollLock",
      MozPrintableKey: "Unidentified"
    };
    var Nd = {
      8: "Backspace",
      9: "Tab",
      12: "Clear",
      13: "Enter",
      16: "Shift",
      17: "Control",
      18: "Alt",
      19: "Pause",
      20: "CapsLock",
      27: "Escape",
      32: " ",
      33: "PageUp",
      34: "PageDown",
      35: "End",
      36: "Home",
      37: "ArrowLeft",
      38: "ArrowUp",
      39: "ArrowRight",
      40: "ArrowDown",
      45: "Insert",
      46: "Delete",
      112: "F1",
      113: "F2",
      114: "F3",
      115: "F4",
      116: "F5",
      117: "F6",
      118: "F7",
      119: "F8",
      120: "F9",
      121: "F10",
      122: "F11",
      123: "F12",
      144: "NumLock",
      145: "ScrollLock",
      224: "Meta"
    };
    var Od = {Alt: "altKey", Control: "ctrlKey", Meta: "metaKey", Shift: "shiftKey"};
    function Pd(a) {
      var b = this.nativeEvent;
      return b.getModifierState ? b.getModifierState(a) : (a = Od[a]) ? !!b[a] : false;
    }
    function zd() {
      return Pd;
    }
    var Qd = m({}, ud, {key: function(a) {
      if (a.key) {
        var b = Md[a.key] || a.key;
        if (b !== "Unidentified")
          return b;
      }
      return a.type === "keypress" ? (a = od(a), a === 13 ? "Enter" : String.fromCharCode(a)) : a.type === "keydown" || a.type === "keyup" ? Nd[a.keyCode] || "Unidentified" : "";
    }, code: 0, location: 0, ctrlKey: 0, shiftKey: 0, altKey: 0, metaKey: 0, repeat: 0, locale: 0, getModifierState: zd, charCode: function(a) {
      return a.type === "keypress" ? od(a) : 0;
    }, keyCode: function(a) {
      return a.type === "keydown" || a.type === "keyup" ? a.keyCode : 0;
    }, which: function(a) {
      return a.type === "keypress" ? od(a) : a.type === "keydown" || a.type === "keyup" ? a.keyCode : 0;
    }});
    var Rd = rd(Qd);
    var Sd = m({}, Ad, {pointerId: 0, width: 0, height: 0, pressure: 0, tangentialPressure: 0, tiltX: 0, tiltY: 0, twist: 0, pointerType: 0, isPrimary: 0});
    var Td = rd(Sd);
    var Ud = m({}, ud, {touches: 0, targetTouches: 0, changedTouches: 0, altKey: 0, metaKey: 0, ctrlKey: 0, shiftKey: 0, getModifierState: zd});
    var Vd = rd(Ud);
    var Wd = m({}, sd, {propertyName: 0, elapsedTime: 0, pseudoElement: 0});
    var Xd = rd(Wd);
    var Yd = m({}, Ad, {
      deltaX: function(a) {
        return "deltaX" in a ? a.deltaX : "wheelDeltaX" in a ? -a.wheelDeltaX : 0;
      },
      deltaY: function(a) {
        return "deltaY" in a ? a.deltaY : "wheelDeltaY" in a ? -a.wheelDeltaY : "wheelDelta" in a ? -a.wheelDelta : 0;
      },
      deltaZ: 0,
      deltaMode: 0
    });
    var Zd = rd(Yd);
    var $d = [9, 13, 27, 32];
    var ae = fa && "CompositionEvent" in window;
    var be = null;
    fa && "documentMode" in document && (be = document.documentMode);
    var ce = fa && "TextEvent" in window && !be;
    var de = fa && (!ae || be && 8 < be && 11 >= be);
    var ee = String.fromCharCode(32);
    var fe = false;
    function ge(a, b) {
      switch (a) {
        case "keyup":
          return $d.indexOf(b.keyCode) !== -1;
        case "keydown":
          return b.keyCode !== 229;
        case "keypress":
        case "mousedown":
        case "focusout":
          return true;
        default:
          return false;
      }
    }
    function he(a) {
      a = a.detail;
      return typeof a === "object" && "data" in a ? a.data : null;
    }
    var ie = false;
    function je(a, b) {
      switch (a) {
        case "compositionend":
          return he(b);
        case "keypress":
          if (b.which !== 32)
            return null;
          fe = true;
          return ee;
        case "textInput":
          return a = b.data, a === ee && fe ? null : a;
        default:
          return null;
      }
    }
    function ke(a, b) {
      if (ie)
        return a === "compositionend" || !ae && ge(a, b) ? (a = nd(), md = ld = kd = null, ie = false, a) : null;
      switch (a) {
        case "paste":
          return null;
        case "keypress":
          if (!(b.ctrlKey || b.altKey || b.metaKey) || b.ctrlKey && b.altKey) {
            if (b.char && 1 < b.char.length)
              return b.char;
            if (b.which)
              return String.fromCharCode(b.which);
          }
          return null;
        case "compositionend":
          return de && b.locale !== "ko" ? null : b.data;
        default:
          return null;
      }
    }
    var le = {color: true, date: true, datetime: true, "datetime-local": true, email: true, month: true, number: true, password: true, range: true, search: true, tel: true, text: true, time: true, url: true, week: true};
    function me(a) {
      var b = a && a.nodeName && a.nodeName.toLowerCase();
      return b === "input" ? !!le[a.type] : b === "textarea" ? true : false;
    }
    function ne(a, b, c, d) {
      Eb(d);
      b = oe(b, "onChange");
      0 < b.length && (c = new td("onChange", "change", null, c, d), a.push({event: c, listeners: b}));
    }
    var pe = null;
    var qe = null;
    function re(a) {
      se(a, 0);
    }
    function te(a) {
      var b = ue(a);
      if (Wa(b))
        return a;
    }
    function ve(a, b) {
      if (a === "change")
        return b;
    }
    var we = false;
    if (fa) {
      var xe;
      if (fa) {
        var ye = "oninput" in document;
        if (!ye) {
          var ze = document.createElement("div");
          ze.setAttribute("oninput", "return;");
          ye = typeof ze.oninput === "function";
        }
        xe = ye;
      } else
        xe = false;
      we = xe && (!document.documentMode || 9 < document.documentMode);
    }
    function Ae() {
      pe && (pe.detachEvent("onpropertychange", Be), qe = pe = null);
    }
    function Be(a) {
      if (a.propertyName === "value" && te(qe)) {
        var b = [];
        ne(b, qe, a, xb(a));
        a = re;
        if (Kb)
          a(b);
        else {
          Kb = true;
          try {
            Gb(a, b);
          } finally {
            Kb = false, Mb();
          }
        }
      }
    }
    function Ce(a, b, c) {
      a === "focusin" ? (Ae(), pe = b, qe = c, pe.attachEvent("onpropertychange", Be)) : a === "focusout" && Ae();
    }
    function De(a) {
      if (a === "selectionchange" || a === "keyup" || a === "keydown")
        return te(qe);
    }
    function Ee(a, b) {
      if (a === "click")
        return te(b);
    }
    function Fe(a, b) {
      if (a === "input" || a === "change")
        return te(b);
    }
    function Ge(a, b) {
      return a === b && (a !== 0 || 1 / a === 1 / b) || a !== a && b !== b;
    }
    var He = typeof Object.is === "function" ? Object.is : Ge;
    var Ie = Object.prototype.hasOwnProperty;
    function Je(a, b) {
      if (He(a, b))
        return true;
      if (typeof a !== "object" || a === null || typeof b !== "object" || b === null)
        return false;
      var c = Object.keys(a), d = Object.keys(b);
      if (c.length !== d.length)
        return false;
      for (d = 0; d < c.length; d++)
        if (!Ie.call(b, c[d]) || !He(a[c[d]], b[c[d]]))
          return false;
      return true;
    }
    function Ke(a) {
      for (; a && a.firstChild; )
        a = a.firstChild;
      return a;
    }
    function Le(a, b) {
      var c = Ke(a);
      a = 0;
      for (var d; c; ) {
        if (c.nodeType === 3) {
          d = a + c.textContent.length;
          if (a <= b && d >= b)
            return {node: c, offset: b - a};
          a = d;
        }
        a: {
          for (; c; ) {
            if (c.nextSibling) {
              c = c.nextSibling;
              break a;
            }
            c = c.parentNode;
          }
          c = void 0;
        }
        c = Ke(c);
      }
    }
    function Me(a, b) {
      return a && b ? a === b ? true : a && a.nodeType === 3 ? false : b && b.nodeType === 3 ? Me(a, b.parentNode) : "contains" in a ? a.contains(b) : a.compareDocumentPosition ? !!(a.compareDocumentPosition(b) & 16) : false : false;
    }
    function Ne() {
      for (var a = window, b = Xa(); b instanceof a.HTMLIFrameElement; ) {
        try {
          var c = typeof b.contentWindow.location.href === "string";
        } catch (d) {
          c = false;
        }
        if (c)
          a = b.contentWindow;
        else
          break;
        b = Xa(a.document);
      }
      return b;
    }
    function Oe(a) {
      var b = a && a.nodeName && a.nodeName.toLowerCase();
      return b && (b === "input" && (a.type === "text" || a.type === "search" || a.type === "tel" || a.type === "url" || a.type === "password") || b === "textarea" || a.contentEditable === "true");
    }
    var Pe = fa && "documentMode" in document && 11 >= document.documentMode;
    var Qe = null;
    var Re = null;
    var Se = null;
    var Te = false;
    function Ue(a, b, c) {
      var d = c.window === c ? c.document : c.nodeType === 9 ? c : c.ownerDocument;
      Te || Qe == null || Qe !== Xa(d) || (d = Qe, "selectionStart" in d && Oe(d) ? d = {start: d.selectionStart, end: d.selectionEnd} : (d = (d.ownerDocument && d.ownerDocument.defaultView || window).getSelection(), d = {anchorNode: d.anchorNode, anchorOffset: d.anchorOffset, focusNode: d.focusNode, focusOffset: d.focusOffset}), Se && Je(Se, d) || (Se = d, d = oe(Re, "onSelect"), 0 < d.length && (b = new td("onSelect", "select", null, b, c), a.push({event: b, listeners: d}), b.target = Qe)));
    }
    Pc("cancel cancel click click close close contextmenu contextMenu copy copy cut cut auxclick auxClick dblclick doubleClick dragend dragEnd dragstart dragStart drop drop focusin focus focusout blur input input invalid invalid keydown keyDown keypress keyPress keyup keyUp mousedown mouseDown mouseup mouseUp paste paste pause pause play play pointercancel pointerCancel pointerdown pointerDown pointerup pointerUp ratechange rateChange reset reset seeked seeked submit submit touchcancel touchCancel touchend touchEnd touchstart touchStart volumechange volumeChange".split(" "), 0);
    Pc("drag drag dragenter dragEnter dragexit dragExit dragleave dragLeave dragover dragOver mousemove mouseMove mouseout mouseOut mouseover mouseOver pointermove pointerMove pointerout pointerOut pointerover pointerOver scroll scroll toggle toggle touchmove touchMove wheel wheel".split(" "), 1);
    Pc(Oc, 2);
    for (var Ve = "change selectionchange textInput compositionstart compositionend compositionupdate".split(" "), We = 0; We < Ve.length; We++)
      Nc.set(Ve[We], 0);
    ea("onMouseEnter", ["mouseout", "mouseover"]);
    ea("onMouseLeave", ["mouseout", "mouseover"]);
    ea("onPointerEnter", ["pointerout", "pointerover"]);
    ea("onPointerLeave", ["pointerout", "pointerover"]);
    da("onChange", "change click focusin focusout input keydown keyup selectionchange".split(" "));
    da("onSelect", "focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange".split(" "));
    da("onBeforeInput", ["compositionend", "keypress", "textInput", "paste"]);
    da("onCompositionEnd", "compositionend focusout keydown keypress keyup mousedown".split(" "));
    da("onCompositionStart", "compositionstart focusout keydown keypress keyup mousedown".split(" "));
    da("onCompositionUpdate", "compositionupdate focusout keydown keypress keyup mousedown".split(" "));
    var Xe = "abort canplay canplaythrough durationchange emptied encrypted ended error loadeddata loadedmetadata loadstart pause play playing progress ratechange seeked seeking stalled suspend timeupdate volumechange waiting".split(" ");
    var Ye = new Set("cancel close invalid load scroll toggle".split(" ").concat(Xe));
    function Ze(a, b, c) {
      var d = a.type || "unknown-event";
      a.currentTarget = c;
      Yb(d, b, void 0, a);
      a.currentTarget = null;
    }
    function se(a, b) {
      b = (b & 4) !== 0;
      for (var c = 0; c < a.length; c++) {
        var d = a[c], e = d.event;
        d = d.listeners;
        a: {
          var f = void 0;
          if (b)
            for (var g = d.length - 1; 0 <= g; g--) {
              var h = d[g], k = h.instance, l = h.currentTarget;
              h = h.listener;
              if (k !== f && e.isPropagationStopped())
                break a;
              Ze(e, h, l);
              f = k;
            }
          else
            for (g = 0; g < d.length; g++) {
              h = d[g];
              k = h.instance;
              l = h.currentTarget;
              h = h.listener;
              if (k !== f && e.isPropagationStopped())
                break a;
              Ze(e, h, l);
              f = k;
            }
        }
      }
      if (Ub)
        throw a = Vb, Ub = false, Vb = null, a;
    }
    function G(a, b) {
      var c = $e(b), d = a + "__bubble";
      c.has(d) || (af(b, a, 2, false), c.add(d));
    }
    var bf = "_reactListening" + Math.random().toString(36).slice(2);
    function cf(a) {
      a[bf] || (a[bf] = true, ba.forEach(function(b) {
        Ye.has(b) || df(b, false, a, null);
        df(b, true, a, null);
      }));
    }
    function df(a, b, c, d) {
      var e = 4 < arguments.length && arguments[4] !== void 0 ? arguments[4] : 0, f = c;
      a === "selectionchange" && c.nodeType !== 9 && (f = c.ownerDocument);
      if (d !== null && !b && Ye.has(a)) {
        if (a !== "scroll")
          return;
        e |= 2;
        f = d;
      }
      var g = $e(f), h = a + "__" + (b ? "capture" : "bubble");
      g.has(h) || (b && (e |= 4), af(f, a, e, b), g.add(h));
    }
    function af(a, b, c, d) {
      var e = Nc.get(b);
      switch (e === void 0 ? 2 : e) {
        case 0:
          e = gd;
          break;
        case 1:
          e = id;
          break;
        default:
          e = hd;
      }
      c = e.bind(null, b, c, a);
      e = void 0;
      !Pb || b !== "touchstart" && b !== "touchmove" && b !== "wheel" || (e = true);
      d ? e !== void 0 ? a.addEventListener(b, c, {capture: true, passive: e}) : a.addEventListener(b, c, true) : e !== void 0 ? a.addEventListener(b, c, {passive: e}) : a.addEventListener(b, c, false);
    }
    function jd(a, b, c, d, e) {
      var f = d;
      if ((b & 1) === 0 && (b & 2) === 0 && d !== null)
        a:
          for (; ; ) {
            if (d === null)
              return;
            var g = d.tag;
            if (g === 3 || g === 4) {
              var h = d.stateNode.containerInfo;
              if (h === e || h.nodeType === 8 && h.parentNode === e)
                break;
              if (g === 4)
                for (g = d.return; g !== null; ) {
                  var k = g.tag;
                  if (k === 3 || k === 4) {
                    if (k = g.stateNode.containerInfo, k === e || k.nodeType === 8 && k.parentNode === e)
                      return;
                  }
                  g = g.return;
                }
              for (; h !== null; ) {
                g = wc(h);
                if (g === null)
                  return;
                k = g.tag;
                if (k === 5 || k === 6) {
                  d = f = g;
                  continue a;
                }
                h = h.parentNode;
              }
            }
            d = d.return;
          }
      Nb(function() {
        var d2 = f, e2 = xb(c), g2 = [];
        a: {
          var h2 = Mc.get(a);
          if (h2 !== void 0) {
            var k2 = td, x = a;
            switch (a) {
              case "keypress":
                if (od(c) === 0)
                  break a;
              case "keydown":
              case "keyup":
                k2 = Rd;
                break;
              case "focusin":
                x = "focus";
                k2 = Fd;
                break;
              case "focusout":
                x = "blur";
                k2 = Fd;
                break;
              case "beforeblur":
              case "afterblur":
                k2 = Fd;
                break;
              case "click":
                if (c.button === 2)
                  break a;
              case "auxclick":
              case "dblclick":
              case "mousedown":
              case "mousemove":
              case "mouseup":
              case "mouseout":
              case "mouseover":
              case "contextmenu":
                k2 = Bd;
                break;
              case "drag":
              case "dragend":
              case "dragenter":
              case "dragexit":
              case "dragleave":
              case "dragover":
              case "dragstart":
              case "drop":
                k2 = Dd;
                break;
              case "touchcancel":
              case "touchend":
              case "touchmove":
              case "touchstart":
                k2 = Vd;
                break;
              case Ic:
              case Jc:
              case Kc:
                k2 = Hd;
                break;
              case Lc:
                k2 = Xd;
                break;
              case "scroll":
                k2 = vd;
                break;
              case "wheel":
                k2 = Zd;
                break;
              case "copy":
              case "cut":
              case "paste":
                k2 = Jd;
                break;
              case "gotpointercapture":
              case "lostpointercapture":
              case "pointercancel":
              case "pointerdown":
              case "pointermove":
              case "pointerout":
              case "pointerover":
              case "pointerup":
                k2 = Td;
            }
            var w = (b & 4) !== 0, z = !w && a === "scroll", u = w ? h2 !== null ? h2 + "Capture" : null : h2;
            w = [];
            for (var t2 = d2, q; t2 !== null; ) {
              q = t2;
              var v = q.stateNode;
              q.tag === 5 && v !== null && (q = v, u !== null && (v = Ob(t2, u), v != null && w.push(ef(t2, v, q))));
              if (z)
                break;
              t2 = t2.return;
            }
            0 < w.length && (h2 = new k2(h2, x, null, c, e2), g2.push({event: h2, listeners: w}));
          }
        }
        if ((b & 7) === 0) {
          a: {
            h2 = a === "mouseover" || a === "pointerover";
            k2 = a === "mouseout" || a === "pointerout";
            if (h2 && (b & 16) === 0 && (x = c.relatedTarget || c.fromElement) && (wc(x) || x[ff]))
              break a;
            if (k2 || h2) {
              h2 = e2.window === e2 ? e2 : (h2 = e2.ownerDocument) ? h2.defaultView || h2.parentWindow : window;
              if (k2) {
                if (x = c.relatedTarget || c.toElement, k2 = d2, x = x ? wc(x) : null, x !== null && (z = Zb(x), x !== z || x.tag !== 5 && x.tag !== 6))
                  x = null;
              } else
                k2 = null, x = d2;
              if (k2 !== x) {
                w = Bd;
                v = "onMouseLeave";
                u = "onMouseEnter";
                t2 = "mouse";
                if (a === "pointerout" || a === "pointerover")
                  w = Td, v = "onPointerLeave", u = "onPointerEnter", t2 = "pointer";
                z = k2 == null ? h2 : ue(k2);
                q = x == null ? h2 : ue(x);
                h2 = new w(v, t2 + "leave", k2, c, e2);
                h2.target = z;
                h2.relatedTarget = q;
                v = null;
                wc(e2) === d2 && (w = new w(u, t2 + "enter", x, c, e2), w.target = q, w.relatedTarget = z, v = w);
                z = v;
                if (k2 && x)
                  b: {
                    w = k2;
                    u = x;
                    t2 = 0;
                    for (q = w; q; q = gf(q))
                      t2++;
                    q = 0;
                    for (v = u; v; v = gf(v))
                      q++;
                    for (; 0 < t2 - q; )
                      w = gf(w), t2--;
                    for (; 0 < q - t2; )
                      u = gf(u), q--;
                    for (; t2--; ) {
                      if (w === u || u !== null && w === u.alternate)
                        break b;
                      w = gf(w);
                      u = gf(u);
                    }
                    w = null;
                  }
                else
                  w = null;
                k2 !== null && hf(g2, h2, k2, w, false);
                x !== null && z !== null && hf(g2, z, x, w, true);
              }
            }
          }
          a: {
            h2 = d2 ? ue(d2) : window;
            k2 = h2.nodeName && h2.nodeName.toLowerCase();
            if (k2 === "select" || k2 === "input" && h2.type === "file")
              var J = ve;
            else if (me(h2))
              if (we)
                J = Fe;
              else {
                J = De;
                var K = Ce;
              }
            else
              (k2 = h2.nodeName) && k2.toLowerCase() === "input" && (h2.type === "checkbox" || h2.type === "radio") && (J = Ee);
            if (J && (J = J(a, d2))) {
              ne(g2, J, c, e2);
              break a;
            }
            K && K(a, h2, d2);
            a === "focusout" && (K = h2._wrapperState) && K.controlled && h2.type === "number" && bb(h2, "number", h2.value);
          }
          K = d2 ? ue(d2) : window;
          switch (a) {
            case "focusin":
              if (me(K) || K.contentEditable === "true")
                Qe = K, Re = d2, Se = null;
              break;
            case "focusout":
              Se = Re = Qe = null;
              break;
            case "mousedown":
              Te = true;
              break;
            case "contextmenu":
            case "mouseup":
            case "dragend":
              Te = false;
              Ue(g2, c, e2);
              break;
            case "selectionchange":
              if (Pe)
                break;
            case "keydown":
            case "keyup":
              Ue(g2, c, e2);
          }
          var Q;
          if (ae)
            b: {
              switch (a) {
                case "compositionstart":
                  var L = "onCompositionStart";
                  break b;
                case "compositionend":
                  L = "onCompositionEnd";
                  break b;
                case "compositionupdate":
                  L = "onCompositionUpdate";
                  break b;
              }
              L = void 0;
            }
          else
            ie ? ge(a, c) && (L = "onCompositionEnd") : a === "keydown" && c.keyCode === 229 && (L = "onCompositionStart");
          L && (de && c.locale !== "ko" && (ie || L !== "onCompositionStart" ? L === "onCompositionEnd" && ie && (Q = nd()) : (kd = e2, ld = "value" in kd ? kd.value : kd.textContent, ie = true)), K = oe(d2, L), 0 < K.length && (L = new Ld(L, a, null, c, e2), g2.push({event: L, listeners: K}), Q ? L.data = Q : (Q = he(c), Q !== null && (L.data = Q))));
          if (Q = ce ? je(a, c) : ke(a, c))
            d2 = oe(d2, "onBeforeInput"), 0 < d2.length && (e2 = new Ld("onBeforeInput", "beforeinput", null, c, e2), g2.push({event: e2, listeners: d2}), e2.data = Q);
        }
        se(g2, b);
      });
    }
    function ef(a, b, c) {
      return {instance: a, listener: b, currentTarget: c};
    }
    function oe(a, b) {
      for (var c = b + "Capture", d = []; a !== null; ) {
        var e = a, f = e.stateNode;
        e.tag === 5 && f !== null && (e = f, f = Ob(a, c), f != null && d.unshift(ef(a, f, e)), f = Ob(a, b), f != null && d.push(ef(a, f, e)));
        a = a.return;
      }
      return d;
    }
    function gf(a) {
      if (a === null)
        return null;
      do
        a = a.return;
      while (a && a.tag !== 5);
      return a ? a : null;
    }
    function hf(a, b, c, d, e) {
      for (var f = b._reactName, g = []; c !== null && c !== d; ) {
        var h = c, k = h.alternate, l = h.stateNode;
        if (k !== null && k === d)
          break;
        h.tag === 5 && l !== null && (h = l, e ? (k = Ob(c, f), k != null && g.unshift(ef(c, k, h))) : e || (k = Ob(c, f), k != null && g.push(ef(c, k, h))));
        c = c.return;
      }
      g.length !== 0 && a.push({event: b, listeners: g});
    }
    function jf() {
    }
    var kf = null;
    var lf = null;
    function mf(a, b) {
      switch (a) {
        case "button":
        case "input":
        case "select":
        case "textarea":
          return !!b.autoFocus;
      }
      return false;
    }
    function nf(a, b) {
      return a === "textarea" || a === "option" || a === "noscript" || typeof b.children === "string" || typeof b.children === "number" || typeof b.dangerouslySetInnerHTML === "object" && b.dangerouslySetInnerHTML !== null && b.dangerouslySetInnerHTML.__html != null;
    }
    var of = typeof setTimeout === "function" ? setTimeout : void 0;
    var pf = typeof clearTimeout === "function" ? clearTimeout : void 0;
    function qf(a) {
      a.nodeType === 1 ? a.textContent = "" : a.nodeType === 9 && (a = a.body, a != null && (a.textContent = ""));
    }
    function rf(a) {
      for (; a != null; a = a.nextSibling) {
        var b = a.nodeType;
        if (b === 1 || b === 3)
          break;
      }
      return a;
    }
    function sf(a) {
      a = a.previousSibling;
      for (var b = 0; a; ) {
        if (a.nodeType === 8) {
          var c = a.data;
          if (c === "$" || c === "$!" || c === "$?") {
            if (b === 0)
              return a;
            b--;
          } else
            c === "/$" && b++;
        }
        a = a.previousSibling;
      }
      return null;
    }
    var tf = 0;
    function uf(a) {
      return {$$typeof: Ga, toString: a, valueOf: a};
    }
    var vf = Math.random().toString(36).slice(2);
    var wf = "__reactFiber$" + vf;
    var xf = "__reactProps$" + vf;
    var ff = "__reactContainer$" + vf;
    var yf = "__reactEvents$" + vf;
    function wc(a) {
      var b = a[wf];
      if (b)
        return b;
      for (var c = a.parentNode; c; ) {
        if (b = c[ff] || c[wf]) {
          c = b.alternate;
          if (b.child !== null || c !== null && c.child !== null)
            for (a = sf(a); a !== null; ) {
              if (c = a[wf])
                return c;
              a = sf(a);
            }
          return b;
        }
        a = c;
        c = a.parentNode;
      }
      return null;
    }
    function Cb(a) {
      a = a[wf] || a[ff];
      return !a || a.tag !== 5 && a.tag !== 6 && a.tag !== 13 && a.tag !== 3 ? null : a;
    }
    function ue(a) {
      if (a.tag === 5 || a.tag === 6)
        return a.stateNode;
      throw Error(y(33));
    }
    function Db(a) {
      return a[xf] || null;
    }
    function $e(a) {
      var b = a[yf];
      b === void 0 && (b = a[yf] = new Set());
      return b;
    }
    var zf = [];
    var Af = -1;
    function Bf(a) {
      return {current: a};
    }
    function H(a) {
      0 > Af || (a.current = zf[Af], zf[Af] = null, Af--);
    }
    function I(a, b) {
      Af++;
      zf[Af] = a.current;
      a.current = b;
    }
    var Cf = {};
    var M = Bf(Cf);
    var N = Bf(false);
    var Df = Cf;
    function Ef(a, b) {
      var c = a.type.contextTypes;
      if (!c)
        return Cf;
      var d = a.stateNode;
      if (d && d.__reactInternalMemoizedUnmaskedChildContext === b)
        return d.__reactInternalMemoizedMaskedChildContext;
      var e = {}, f;
      for (f in c)
        e[f] = b[f];
      d && (a = a.stateNode, a.__reactInternalMemoizedUnmaskedChildContext = b, a.__reactInternalMemoizedMaskedChildContext = e);
      return e;
    }
    function Ff(a) {
      a = a.childContextTypes;
      return a !== null && a !== void 0;
    }
    function Gf() {
      H(N);
      H(M);
    }
    function Hf(a, b, c) {
      if (M.current !== Cf)
        throw Error(y(168));
      I(M, b);
      I(N, c);
    }
    function If(a, b, c) {
      var d = a.stateNode;
      a = b.childContextTypes;
      if (typeof d.getChildContext !== "function")
        return c;
      d = d.getChildContext();
      for (var e in d)
        if (!(e in a))
          throw Error(y(108, Ra(b) || "Unknown", e));
      return m({}, c, d);
    }
    function Jf(a) {
      a = (a = a.stateNode) && a.__reactInternalMemoizedMergedChildContext || Cf;
      Df = M.current;
      I(M, a);
      I(N, N.current);
      return true;
    }
    function Kf(a, b, c) {
      var d = a.stateNode;
      if (!d)
        throw Error(y(169));
      c ? (a = If(a, b, Df), d.__reactInternalMemoizedMergedChildContext = a, H(N), H(M), I(M, a)) : H(N);
      I(N, c);
    }
    var Lf = null;
    var Mf = null;
    var Nf = r.unstable_runWithPriority;
    var Of = r.unstable_scheduleCallback;
    var Pf = r.unstable_cancelCallback;
    var Qf = r.unstable_shouldYield;
    var Rf = r.unstable_requestPaint;
    var Sf = r.unstable_now;
    var Tf = r.unstable_getCurrentPriorityLevel;
    var Uf = r.unstable_ImmediatePriority;
    var Vf = r.unstable_UserBlockingPriority;
    var Wf = r.unstable_NormalPriority;
    var Xf = r.unstable_LowPriority;
    var Yf = r.unstable_IdlePriority;
    var Zf = {};
    var $f = Rf !== void 0 ? Rf : function() {
    };
    var ag = null;
    var bg = null;
    var cg = false;
    var dg = Sf();
    var O = 1e4 > dg ? Sf : function() {
      return Sf() - dg;
    };
    function eg() {
      switch (Tf()) {
        case Uf:
          return 99;
        case Vf:
          return 98;
        case Wf:
          return 97;
        case Xf:
          return 96;
        case Yf:
          return 95;
        default:
          throw Error(y(332));
      }
    }
    function fg(a) {
      switch (a) {
        case 99:
          return Uf;
        case 98:
          return Vf;
        case 97:
          return Wf;
        case 96:
          return Xf;
        case 95:
          return Yf;
        default:
          throw Error(y(332));
      }
    }
    function gg(a, b) {
      a = fg(a);
      return Nf(a, b);
    }
    function hg(a, b, c) {
      a = fg(a);
      return Of(a, b, c);
    }
    function ig() {
      if (bg !== null) {
        var a = bg;
        bg = null;
        Pf(a);
      }
      jg();
    }
    function jg() {
      if (!cg && ag !== null) {
        cg = true;
        var a = 0;
        try {
          var b = ag;
          gg(99, function() {
            for (; a < b.length; a++) {
              var c = b[a];
              do
                c = c(true);
              while (c !== null);
            }
          });
          ag = null;
        } catch (c) {
          throw ag !== null && (ag = ag.slice(a + 1)), Of(Uf, ig), c;
        } finally {
          cg = false;
        }
      }
    }
    var kg = ra.ReactCurrentBatchConfig;
    function lg(a, b) {
      if (a && a.defaultProps) {
        b = m({}, b);
        a = a.defaultProps;
        for (var c in a)
          b[c] === void 0 && (b[c] = a[c]);
        return b;
      }
      return b;
    }
    var mg = Bf(null);
    var ng = null;
    var og = null;
    var pg = null;
    function qg() {
      pg = og = ng = null;
    }
    function rg(a) {
      var b = mg.current;
      H(mg);
      a.type._context._currentValue = b;
    }
    function sg(a, b) {
      for (; a !== null; ) {
        var c = a.alternate;
        if ((a.childLanes & b) === b)
          if (c === null || (c.childLanes & b) === b)
            break;
          else
            c.childLanes |= b;
        else
          a.childLanes |= b, c !== null && (c.childLanes |= b);
        a = a.return;
      }
    }
    function tg(a, b) {
      ng = a;
      pg = og = null;
      a = a.dependencies;
      a !== null && a.firstContext !== null && ((a.lanes & b) !== 0 && (ug = true), a.firstContext = null);
    }
    function vg(a, b) {
      if (pg !== a && b !== false && b !== 0) {
        if (typeof b !== "number" || b === 1073741823)
          pg = a, b = 1073741823;
        b = {context: a, observedBits: b, next: null};
        if (og === null) {
          if (ng === null)
            throw Error(y(308));
          og = b;
          ng.dependencies = {lanes: 0, firstContext: b, responders: null};
        } else
          og = og.next = b;
      }
      return a._currentValue;
    }
    var wg = false;
    function xg(a) {
      a.updateQueue = {baseState: a.memoizedState, firstBaseUpdate: null, lastBaseUpdate: null, shared: {pending: null}, effects: null};
    }
    function yg(a, b) {
      a = a.updateQueue;
      b.updateQueue === a && (b.updateQueue = {baseState: a.baseState, firstBaseUpdate: a.firstBaseUpdate, lastBaseUpdate: a.lastBaseUpdate, shared: a.shared, effects: a.effects});
    }
    function zg(a, b) {
      return {eventTime: a, lane: b, tag: 0, payload: null, callback: null, next: null};
    }
    function Ag(a, b) {
      a = a.updateQueue;
      if (a !== null) {
        a = a.shared;
        var c = a.pending;
        c === null ? b.next = b : (b.next = c.next, c.next = b);
        a.pending = b;
      }
    }
    function Bg(a, b) {
      var c = a.updateQueue, d = a.alternate;
      if (d !== null && (d = d.updateQueue, c === d)) {
        var e = null, f = null;
        c = c.firstBaseUpdate;
        if (c !== null) {
          do {
            var g = {eventTime: c.eventTime, lane: c.lane, tag: c.tag, payload: c.payload, callback: c.callback, next: null};
            f === null ? e = f = g : f = f.next = g;
            c = c.next;
          } while (c !== null);
          f === null ? e = f = b : f = f.next = b;
        } else
          e = f = b;
        c = {baseState: d.baseState, firstBaseUpdate: e, lastBaseUpdate: f, shared: d.shared, effects: d.effects};
        a.updateQueue = c;
        return;
      }
      a = c.lastBaseUpdate;
      a === null ? c.firstBaseUpdate = b : a.next = b;
      c.lastBaseUpdate = b;
    }
    function Cg(a, b, c, d) {
      var e = a.updateQueue;
      wg = false;
      var f = e.firstBaseUpdate, g = e.lastBaseUpdate, h = e.shared.pending;
      if (h !== null) {
        e.shared.pending = null;
        var k = h, l = k.next;
        k.next = null;
        g === null ? f = l : g.next = l;
        g = k;
        var n = a.alternate;
        if (n !== null) {
          n = n.updateQueue;
          var A = n.lastBaseUpdate;
          A !== g && (A === null ? n.firstBaseUpdate = l : A.next = l, n.lastBaseUpdate = k);
        }
      }
      if (f !== null) {
        A = e.baseState;
        g = 0;
        n = l = k = null;
        do {
          h = f.lane;
          var p = f.eventTime;
          if ((d & h) === h) {
            n !== null && (n = n.next = {
              eventTime: p,
              lane: 0,
              tag: f.tag,
              payload: f.payload,
              callback: f.callback,
              next: null
            });
            a: {
              var C = a, x = f;
              h = b;
              p = c;
              switch (x.tag) {
                case 1:
                  C = x.payload;
                  if (typeof C === "function") {
                    A = C.call(p, A, h);
                    break a;
                  }
                  A = C;
                  break a;
                case 3:
                  C.flags = C.flags & -4097 | 64;
                case 0:
                  C = x.payload;
                  h = typeof C === "function" ? C.call(p, A, h) : C;
                  if (h === null || h === void 0)
                    break a;
                  A = m({}, A, h);
                  break a;
                case 2:
                  wg = true;
              }
            }
            f.callback !== null && (a.flags |= 32, h = e.effects, h === null ? e.effects = [f] : h.push(f));
          } else
            p = {eventTime: p, lane: h, tag: f.tag, payload: f.payload, callback: f.callback, next: null}, n === null ? (l = n = p, k = A) : n = n.next = p, g |= h;
          f = f.next;
          if (f === null)
            if (h = e.shared.pending, h === null)
              break;
            else
              f = h.next, h.next = null, e.lastBaseUpdate = h, e.shared.pending = null;
        } while (1);
        n === null && (k = A);
        e.baseState = k;
        e.firstBaseUpdate = l;
        e.lastBaseUpdate = n;
        Dg |= g;
        a.lanes = g;
        a.memoizedState = A;
      }
    }
    function Eg(a, b, c) {
      a = b.effects;
      b.effects = null;
      if (a !== null)
        for (b = 0; b < a.length; b++) {
          var d = a[b], e = d.callback;
          if (e !== null) {
            d.callback = null;
            d = c;
            if (typeof e !== "function")
              throw Error(y(191, e));
            e.call(d);
          }
        }
    }
    var Fg = new aa.Component().refs;
    function Gg(a, b, c, d) {
      b = a.memoizedState;
      c = c(d, b);
      c = c === null || c === void 0 ? b : m({}, b, c);
      a.memoizedState = c;
      a.lanes === 0 && (a.updateQueue.baseState = c);
    }
    var Kg = {isMounted: function(a) {
      return (a = a._reactInternals) ? Zb(a) === a : false;
    }, enqueueSetState: function(a, b, c) {
      a = a._reactInternals;
      var d = Hg(), e = Ig(a), f = zg(d, e);
      f.payload = b;
      c !== void 0 && c !== null && (f.callback = c);
      Ag(a, f);
      Jg(a, e, d);
    }, enqueueReplaceState: function(a, b, c) {
      a = a._reactInternals;
      var d = Hg(), e = Ig(a), f = zg(d, e);
      f.tag = 1;
      f.payload = b;
      c !== void 0 && c !== null && (f.callback = c);
      Ag(a, f);
      Jg(a, e, d);
    }, enqueueForceUpdate: function(a, b) {
      a = a._reactInternals;
      var c = Hg(), d = Ig(a), e = zg(c, d);
      e.tag = 2;
      b !== void 0 && b !== null && (e.callback = b);
      Ag(a, e);
      Jg(a, d, c);
    }};
    function Lg(a, b, c, d, e, f, g) {
      a = a.stateNode;
      return typeof a.shouldComponentUpdate === "function" ? a.shouldComponentUpdate(d, f, g) : b.prototype && b.prototype.isPureReactComponent ? !Je(c, d) || !Je(e, f) : true;
    }
    function Mg(a, b, c) {
      var d = false, e = Cf;
      var f = b.contextType;
      typeof f === "object" && f !== null ? f = vg(f) : (e = Ff(b) ? Df : M.current, d = b.contextTypes, f = (d = d !== null && d !== void 0) ? Ef(a, e) : Cf);
      b = new b(c, f);
      a.memoizedState = b.state !== null && b.state !== void 0 ? b.state : null;
      b.updater = Kg;
      a.stateNode = b;
      b._reactInternals = a;
      d && (a = a.stateNode, a.__reactInternalMemoizedUnmaskedChildContext = e, a.__reactInternalMemoizedMaskedChildContext = f);
      return b;
    }
    function Ng(a, b, c, d) {
      a = b.state;
      typeof b.componentWillReceiveProps === "function" && b.componentWillReceiveProps(c, d);
      typeof b.UNSAFE_componentWillReceiveProps === "function" && b.UNSAFE_componentWillReceiveProps(c, d);
      b.state !== a && Kg.enqueueReplaceState(b, b.state, null);
    }
    function Og(a, b, c, d) {
      var e = a.stateNode;
      e.props = c;
      e.state = a.memoizedState;
      e.refs = Fg;
      xg(a);
      var f = b.contextType;
      typeof f === "object" && f !== null ? e.context = vg(f) : (f = Ff(b) ? Df : M.current, e.context = Ef(a, f));
      Cg(a, c, e, d);
      e.state = a.memoizedState;
      f = b.getDerivedStateFromProps;
      typeof f === "function" && (Gg(a, b, f, c), e.state = a.memoizedState);
      typeof b.getDerivedStateFromProps === "function" || typeof e.getSnapshotBeforeUpdate === "function" || typeof e.UNSAFE_componentWillMount !== "function" && typeof e.componentWillMount !== "function" || (b = e.state, typeof e.componentWillMount === "function" && e.componentWillMount(), typeof e.UNSAFE_componentWillMount === "function" && e.UNSAFE_componentWillMount(), b !== e.state && Kg.enqueueReplaceState(e, e.state, null), Cg(a, c, e, d), e.state = a.memoizedState);
      typeof e.componentDidMount === "function" && (a.flags |= 4);
    }
    var Pg = Array.isArray;
    function Qg(a, b, c) {
      a = c.ref;
      if (a !== null && typeof a !== "function" && typeof a !== "object") {
        if (c._owner) {
          c = c._owner;
          if (c) {
            if (c.tag !== 1)
              throw Error(y(309));
            var d = c.stateNode;
          }
          if (!d)
            throw Error(y(147, a));
          var e = "" + a;
          if (b !== null && b.ref !== null && typeof b.ref === "function" && b.ref._stringRef === e)
            return b.ref;
          b = function(a2) {
            var b2 = d.refs;
            b2 === Fg && (b2 = d.refs = {});
            a2 === null ? delete b2[e] : b2[e] = a2;
          };
          b._stringRef = e;
          return b;
        }
        if (typeof a !== "string")
          throw Error(y(284));
        if (!c._owner)
          throw Error(y(290, a));
      }
      return a;
    }
    function Rg(a, b) {
      if (a.type !== "textarea")
        throw Error(y(31, Object.prototype.toString.call(b) === "[object Object]" ? "object with keys {" + Object.keys(b).join(", ") + "}" : b));
    }
    function Sg(a) {
      function b(b2, c2) {
        if (a) {
          var d2 = b2.lastEffect;
          d2 !== null ? (d2.nextEffect = c2, b2.lastEffect = c2) : b2.firstEffect = b2.lastEffect = c2;
          c2.nextEffect = null;
          c2.flags = 8;
        }
      }
      function c(c2, d2) {
        if (!a)
          return null;
        for (; d2 !== null; )
          b(c2, d2), d2 = d2.sibling;
        return null;
      }
      function d(a2, b2) {
        for (a2 = new Map(); b2 !== null; )
          b2.key !== null ? a2.set(b2.key, b2) : a2.set(b2.index, b2), b2 = b2.sibling;
        return a2;
      }
      function e(a2, b2) {
        a2 = Tg(a2, b2);
        a2.index = 0;
        a2.sibling = null;
        return a2;
      }
      function f(b2, c2, d2) {
        b2.index = d2;
        if (!a)
          return c2;
        d2 = b2.alternate;
        if (d2 !== null)
          return d2 = d2.index, d2 < c2 ? (b2.flags = 2, c2) : d2;
        b2.flags = 2;
        return c2;
      }
      function g(b2) {
        a && b2.alternate === null && (b2.flags = 2);
        return b2;
      }
      function h(a2, b2, c2, d2) {
        if (b2 === null || b2.tag !== 6)
          return b2 = Ug(c2, a2.mode, d2), b2.return = a2, b2;
        b2 = e(b2, c2);
        b2.return = a2;
        return b2;
      }
      function k(a2, b2, c2, d2) {
        if (b2 !== null && b2.elementType === c2.type)
          return d2 = e(b2, c2.props), d2.ref = Qg(a2, b2, c2), d2.return = a2, d2;
        d2 = Vg(c2.type, c2.key, c2.props, null, a2.mode, d2);
        d2.ref = Qg(a2, b2, c2);
        d2.return = a2;
        return d2;
      }
      function l(a2, b2, c2, d2) {
        if (b2 === null || b2.tag !== 4 || b2.stateNode.containerInfo !== c2.containerInfo || b2.stateNode.implementation !== c2.implementation)
          return b2 = Wg(c2, a2.mode, d2), b2.return = a2, b2;
        b2 = e(b2, c2.children || []);
        b2.return = a2;
        return b2;
      }
      function n(a2, b2, c2, d2, f2) {
        if (b2 === null || b2.tag !== 7)
          return b2 = Xg(c2, a2.mode, d2, f2), b2.return = a2, b2;
        b2 = e(b2, c2);
        b2.return = a2;
        return b2;
      }
      function A(a2, b2, c2) {
        if (typeof b2 === "string" || typeof b2 === "number")
          return b2 = Ug("" + b2, a2.mode, c2), b2.return = a2, b2;
        if (typeof b2 === "object" && b2 !== null) {
          switch (b2.$$typeof) {
            case sa:
              return c2 = Vg(b2.type, b2.key, b2.props, null, a2.mode, c2), c2.ref = Qg(a2, null, b2), c2.return = a2, c2;
            case ta:
              return b2 = Wg(b2, a2.mode, c2), b2.return = a2, b2;
          }
          if (Pg(b2) || La(b2))
            return b2 = Xg(b2, a2.mode, c2, null), b2.return = a2, b2;
          Rg(a2, b2);
        }
        return null;
      }
      function p(a2, b2, c2, d2) {
        var e2 = b2 !== null ? b2.key : null;
        if (typeof c2 === "string" || typeof c2 === "number")
          return e2 !== null ? null : h(a2, b2, "" + c2, d2);
        if (typeof c2 === "object" && c2 !== null) {
          switch (c2.$$typeof) {
            case sa:
              return c2.key === e2 ? c2.type === ua ? n(a2, b2, c2.props.children, d2, e2) : k(a2, b2, c2, d2) : null;
            case ta:
              return c2.key === e2 ? l(a2, b2, c2, d2) : null;
          }
          if (Pg(c2) || La(c2))
            return e2 !== null ? null : n(a2, b2, c2, d2, null);
          Rg(a2, c2);
        }
        return null;
      }
      function C(a2, b2, c2, d2, e2) {
        if (typeof d2 === "string" || typeof d2 === "number")
          return a2 = a2.get(c2) || null, h(b2, a2, "" + d2, e2);
        if (typeof d2 === "object" && d2 !== null) {
          switch (d2.$$typeof) {
            case sa:
              return a2 = a2.get(d2.key === null ? c2 : d2.key) || null, d2.type === ua ? n(b2, a2, d2.props.children, e2, d2.key) : k(b2, a2, d2, e2);
            case ta:
              return a2 = a2.get(d2.key === null ? c2 : d2.key) || null, l(b2, a2, d2, e2);
          }
          if (Pg(d2) || La(d2))
            return a2 = a2.get(c2) || null, n(b2, a2, d2, e2, null);
          Rg(b2, d2);
        }
        return null;
      }
      function x(e2, g2, h2, k2) {
        for (var l2 = null, t2 = null, u = g2, z = g2 = 0, q = null; u !== null && z < h2.length; z++) {
          u.index > z ? (q = u, u = null) : q = u.sibling;
          var n2 = p(e2, u, h2[z], k2);
          if (n2 === null) {
            u === null && (u = q);
            break;
          }
          a && u && n2.alternate === null && b(e2, u);
          g2 = f(n2, g2, z);
          t2 === null ? l2 = n2 : t2.sibling = n2;
          t2 = n2;
          u = q;
        }
        if (z === h2.length)
          return c(e2, u), l2;
        if (u === null) {
          for (; z < h2.length; z++)
            u = A(e2, h2[z], k2), u !== null && (g2 = f(u, g2, z), t2 === null ? l2 = u : t2.sibling = u, t2 = u);
          return l2;
        }
        for (u = d(e2, u); z < h2.length; z++)
          q = C(u, e2, z, h2[z], k2), q !== null && (a && q.alternate !== null && u.delete(q.key === null ? z : q.key), g2 = f(q, g2, z), t2 === null ? l2 = q : t2.sibling = q, t2 = q);
        a && u.forEach(function(a2) {
          return b(e2, a2);
        });
        return l2;
      }
      function w(e2, g2, h2, k2) {
        var l2 = La(h2);
        if (typeof l2 !== "function")
          throw Error(y(150));
        h2 = l2.call(h2);
        if (h2 == null)
          throw Error(y(151));
        for (var t2 = l2 = null, u = g2, z = g2 = 0, q = null, n2 = h2.next(); u !== null && !n2.done; z++, n2 = h2.next()) {
          u.index > z ? (q = u, u = null) : q = u.sibling;
          var w2 = p(e2, u, n2.value, k2);
          if (w2 === null) {
            u === null && (u = q);
            break;
          }
          a && u && w2.alternate === null && b(e2, u);
          g2 = f(w2, g2, z);
          t2 === null ? l2 = w2 : t2.sibling = w2;
          t2 = w2;
          u = q;
        }
        if (n2.done)
          return c(e2, u), l2;
        if (u === null) {
          for (; !n2.done; z++, n2 = h2.next())
            n2 = A(e2, n2.value, k2), n2 !== null && (g2 = f(n2, g2, z), t2 === null ? l2 = n2 : t2.sibling = n2, t2 = n2);
          return l2;
        }
        for (u = d(e2, u); !n2.done; z++, n2 = h2.next())
          n2 = C(u, e2, z, n2.value, k2), n2 !== null && (a && n2.alternate !== null && u.delete(n2.key === null ? z : n2.key), g2 = f(n2, g2, z), t2 === null ? l2 = n2 : t2.sibling = n2, t2 = n2);
        a && u.forEach(function(a2) {
          return b(e2, a2);
        });
        return l2;
      }
      return function(a2, d2, f2, h2) {
        var k2 = typeof f2 === "object" && f2 !== null && f2.type === ua && f2.key === null;
        k2 && (f2 = f2.props.children);
        var l2 = typeof f2 === "object" && f2 !== null;
        if (l2)
          switch (f2.$$typeof) {
            case sa:
              a: {
                l2 = f2.key;
                for (k2 = d2; k2 !== null; ) {
                  if (k2.key === l2) {
                    switch (k2.tag) {
                      case 7:
                        if (f2.type === ua) {
                          c(a2, k2.sibling);
                          d2 = e(k2, f2.props.children);
                          d2.return = a2;
                          a2 = d2;
                          break a;
                        }
                        break;
                      default:
                        if (k2.elementType === f2.type) {
                          c(a2, k2.sibling);
                          d2 = e(k2, f2.props);
                          d2.ref = Qg(a2, k2, f2);
                          d2.return = a2;
                          a2 = d2;
                          break a;
                        }
                    }
                    c(a2, k2);
                    break;
                  } else
                    b(a2, k2);
                  k2 = k2.sibling;
                }
                f2.type === ua ? (d2 = Xg(f2.props.children, a2.mode, h2, f2.key), d2.return = a2, a2 = d2) : (h2 = Vg(f2.type, f2.key, f2.props, null, a2.mode, h2), h2.ref = Qg(a2, d2, f2), h2.return = a2, a2 = h2);
              }
              return g(a2);
            case ta:
              a: {
                for (k2 = f2.key; d2 !== null; ) {
                  if (d2.key === k2)
                    if (d2.tag === 4 && d2.stateNode.containerInfo === f2.containerInfo && d2.stateNode.implementation === f2.implementation) {
                      c(a2, d2.sibling);
                      d2 = e(d2, f2.children || []);
                      d2.return = a2;
                      a2 = d2;
                      break a;
                    } else {
                      c(a2, d2);
                      break;
                    }
                  else
                    b(a2, d2);
                  d2 = d2.sibling;
                }
                d2 = Wg(f2, a2.mode, h2);
                d2.return = a2;
                a2 = d2;
              }
              return g(a2);
          }
        if (typeof f2 === "string" || typeof f2 === "number")
          return f2 = "" + f2, d2 !== null && d2.tag === 6 ? (c(a2, d2.sibling), d2 = e(d2, f2), d2.return = a2, a2 = d2) : (c(a2, d2), d2 = Ug(f2, a2.mode, h2), d2.return = a2, a2 = d2), g(a2);
        if (Pg(f2))
          return x(a2, d2, f2, h2);
        if (La(f2))
          return w(a2, d2, f2, h2);
        l2 && Rg(a2, f2);
        if (typeof f2 === "undefined" && !k2)
          switch (a2.tag) {
            case 1:
            case 22:
            case 0:
            case 11:
            case 15:
              throw Error(y(152, Ra(a2.type) || "Component"));
          }
        return c(a2, d2);
      };
    }
    var Yg = Sg(true);
    var Zg = Sg(false);
    var $g = {};
    var ah = Bf($g);
    var bh = Bf($g);
    var ch = Bf($g);
    function dh(a) {
      if (a === $g)
        throw Error(y(174));
      return a;
    }
    function eh(a, b) {
      I(ch, b);
      I(bh, a);
      I(ah, $g);
      a = b.nodeType;
      switch (a) {
        case 9:
        case 11:
          b = (b = b.documentElement) ? b.namespaceURI : mb(null, "");
          break;
        default:
          a = a === 8 ? b.parentNode : b, b = a.namespaceURI || null, a = a.tagName, b = mb(b, a);
      }
      H(ah);
      I(ah, b);
    }
    function fh() {
      H(ah);
      H(bh);
      H(ch);
    }
    function gh(a) {
      dh(ch.current);
      var b = dh(ah.current);
      var c = mb(b, a.type);
      b !== c && (I(bh, a), I(ah, c));
    }
    function hh(a) {
      bh.current === a && (H(ah), H(bh));
    }
    var P = Bf(0);
    function ih(a) {
      for (var b = a; b !== null; ) {
        if (b.tag === 13) {
          var c = b.memoizedState;
          if (c !== null && (c = c.dehydrated, c === null || c.data === "$?" || c.data === "$!"))
            return b;
        } else if (b.tag === 19 && b.memoizedProps.revealOrder !== void 0) {
          if ((b.flags & 64) !== 0)
            return b;
        } else if (b.child !== null) {
          b.child.return = b;
          b = b.child;
          continue;
        }
        if (b === a)
          break;
        for (; b.sibling === null; ) {
          if (b.return === null || b.return === a)
            return null;
          b = b.return;
        }
        b.sibling.return = b.return;
        b = b.sibling;
      }
      return null;
    }
    var jh = null;
    var kh = null;
    var lh = false;
    function mh(a, b) {
      var c = nh(5, null, null, 0);
      c.elementType = "DELETED";
      c.type = "DELETED";
      c.stateNode = b;
      c.return = a;
      c.flags = 8;
      a.lastEffect !== null ? (a.lastEffect.nextEffect = c, a.lastEffect = c) : a.firstEffect = a.lastEffect = c;
    }
    function oh(a, b) {
      switch (a.tag) {
        case 5:
          var c = a.type;
          b = b.nodeType !== 1 || c.toLowerCase() !== b.nodeName.toLowerCase() ? null : b;
          return b !== null ? (a.stateNode = b, true) : false;
        case 6:
          return b = a.pendingProps === "" || b.nodeType !== 3 ? null : b, b !== null ? (a.stateNode = b, true) : false;
        case 13:
          return false;
        default:
          return false;
      }
    }
    function ph(a) {
      if (lh) {
        var b = kh;
        if (b) {
          var c = b;
          if (!oh(a, b)) {
            b = rf(c.nextSibling);
            if (!b || !oh(a, b)) {
              a.flags = a.flags & -1025 | 2;
              lh = false;
              jh = a;
              return;
            }
            mh(jh, c);
          }
          jh = a;
          kh = rf(b.firstChild);
        } else
          a.flags = a.flags & -1025 | 2, lh = false, jh = a;
      }
    }
    function qh(a) {
      for (a = a.return; a !== null && a.tag !== 5 && a.tag !== 3 && a.tag !== 13; )
        a = a.return;
      jh = a;
    }
    function rh(a) {
      if (a !== jh)
        return false;
      if (!lh)
        return qh(a), lh = true, false;
      var b = a.type;
      if (a.tag !== 5 || b !== "head" && b !== "body" && !nf(b, a.memoizedProps))
        for (b = kh; b; )
          mh(a, b), b = rf(b.nextSibling);
      qh(a);
      if (a.tag === 13) {
        a = a.memoizedState;
        a = a !== null ? a.dehydrated : null;
        if (!a)
          throw Error(y(317));
        a: {
          a = a.nextSibling;
          for (b = 0; a; ) {
            if (a.nodeType === 8) {
              var c = a.data;
              if (c === "/$") {
                if (b === 0) {
                  kh = rf(a.nextSibling);
                  break a;
                }
                b--;
              } else
                c !== "$" && c !== "$!" && c !== "$?" || b++;
            }
            a = a.nextSibling;
          }
          kh = null;
        }
      } else
        kh = jh ? rf(a.stateNode.nextSibling) : null;
      return true;
    }
    function sh() {
      kh = jh = null;
      lh = false;
    }
    var th = [];
    function uh() {
      for (var a = 0; a < th.length; a++)
        th[a]._workInProgressVersionPrimary = null;
      th.length = 0;
    }
    var vh = ra.ReactCurrentDispatcher;
    var wh = ra.ReactCurrentBatchConfig;
    var xh = 0;
    var R = null;
    var S = null;
    var T = null;
    var yh = false;
    var zh = false;
    function Ah() {
      throw Error(y(321));
    }
    function Bh(a, b) {
      if (b === null)
        return false;
      for (var c = 0; c < b.length && c < a.length; c++)
        if (!He(a[c], b[c]))
          return false;
      return true;
    }
    function Ch(a, b, c, d, e, f) {
      xh = f;
      R = b;
      b.memoizedState = null;
      b.updateQueue = null;
      b.lanes = 0;
      vh.current = a === null || a.memoizedState === null ? Dh : Eh;
      a = c(d, e);
      if (zh) {
        f = 0;
        do {
          zh = false;
          if (!(25 > f))
            throw Error(y(301));
          f += 1;
          T = S = null;
          b.updateQueue = null;
          vh.current = Fh;
          a = c(d, e);
        } while (zh);
      }
      vh.current = Gh;
      b = S !== null && S.next !== null;
      xh = 0;
      T = S = R = null;
      yh = false;
      if (b)
        throw Error(y(300));
      return a;
    }
    function Hh() {
      var a = {memoizedState: null, baseState: null, baseQueue: null, queue: null, next: null};
      T === null ? R.memoizedState = T = a : T = T.next = a;
      return T;
    }
    function Ih() {
      if (S === null) {
        var a = R.alternate;
        a = a !== null ? a.memoizedState : null;
      } else
        a = S.next;
      var b = T === null ? R.memoizedState : T.next;
      if (b !== null)
        T = b, S = a;
      else {
        if (a === null)
          throw Error(y(310));
        S = a;
        a = {memoizedState: S.memoizedState, baseState: S.baseState, baseQueue: S.baseQueue, queue: S.queue, next: null};
        T === null ? R.memoizedState = T = a : T = T.next = a;
      }
      return T;
    }
    function Jh(a, b) {
      return typeof b === "function" ? b(a) : b;
    }
    function Kh(a) {
      var b = Ih(), c = b.queue;
      if (c === null)
        throw Error(y(311));
      c.lastRenderedReducer = a;
      var d = S, e = d.baseQueue, f = c.pending;
      if (f !== null) {
        if (e !== null) {
          var g = e.next;
          e.next = f.next;
          f.next = g;
        }
        d.baseQueue = e = f;
        c.pending = null;
      }
      if (e !== null) {
        e = e.next;
        d = d.baseState;
        var h = g = f = null, k = e;
        do {
          var l = k.lane;
          if ((xh & l) === l)
            h !== null && (h = h.next = {lane: 0, action: k.action, eagerReducer: k.eagerReducer, eagerState: k.eagerState, next: null}), d = k.eagerReducer === a ? k.eagerState : a(d, k.action);
          else {
            var n = {
              lane: l,
              action: k.action,
              eagerReducer: k.eagerReducer,
              eagerState: k.eagerState,
              next: null
            };
            h === null ? (g = h = n, f = d) : h = h.next = n;
            R.lanes |= l;
            Dg |= l;
          }
          k = k.next;
        } while (k !== null && k !== e);
        h === null ? f = d : h.next = g;
        He(d, b.memoizedState) || (ug = true);
        b.memoizedState = d;
        b.baseState = f;
        b.baseQueue = h;
        c.lastRenderedState = d;
      }
      return [b.memoizedState, c.dispatch];
    }
    function Lh(a) {
      var b = Ih(), c = b.queue;
      if (c === null)
        throw Error(y(311));
      c.lastRenderedReducer = a;
      var d = c.dispatch, e = c.pending, f = b.memoizedState;
      if (e !== null) {
        c.pending = null;
        var g = e = e.next;
        do
          f = a(f, g.action), g = g.next;
        while (g !== e);
        He(f, b.memoizedState) || (ug = true);
        b.memoizedState = f;
        b.baseQueue === null && (b.baseState = f);
        c.lastRenderedState = f;
      }
      return [f, d];
    }
    function Mh(a, b, c) {
      var d = b._getVersion;
      d = d(b._source);
      var e = b._workInProgressVersionPrimary;
      if (e !== null)
        a = e === d;
      else if (a = a.mutableReadLanes, a = (xh & a) === a)
        b._workInProgressVersionPrimary = d, th.push(b);
      if (a)
        return c(b._source);
      th.push(b);
      throw Error(y(350));
    }
    function Nh(a, b, c, d) {
      var e = U;
      if (e === null)
        throw Error(y(349));
      var f = b._getVersion, g = f(b._source), h = vh.current, k = h.useState(function() {
        return Mh(e, b, c);
      }), l = k[1], n = k[0];
      k = T;
      var A = a.memoizedState, p = A.refs, C = p.getSnapshot, x = A.source;
      A = A.subscribe;
      var w = R;
      a.memoizedState = {refs: p, source: b, subscribe: d};
      h.useEffect(function() {
        p.getSnapshot = c;
        p.setSnapshot = l;
        var a2 = f(b._source);
        if (!He(g, a2)) {
          a2 = c(b._source);
          He(n, a2) || (l(a2), a2 = Ig(w), e.mutableReadLanes |= a2 & e.pendingLanes);
          a2 = e.mutableReadLanes;
          e.entangledLanes |= a2;
          for (var d2 = e.entanglements, h2 = a2; 0 < h2; ) {
            var k2 = 31 - Vc(h2), v = 1 << k2;
            d2[k2] |= a2;
            h2 &= ~v;
          }
        }
      }, [c, b, d]);
      h.useEffect(function() {
        return d(b._source, function() {
          var a2 = p.getSnapshot, c2 = p.setSnapshot;
          try {
            c2(a2(b._source));
            var d2 = Ig(w);
            e.mutableReadLanes |= d2 & e.pendingLanes;
          } catch (q) {
            c2(function() {
              throw q;
            });
          }
        });
      }, [b, d]);
      He(C, c) && He(x, b) && He(A, d) || (a = {pending: null, dispatch: null, lastRenderedReducer: Jh, lastRenderedState: n}, a.dispatch = l = Oh.bind(null, R, a), k.queue = a, k.baseQueue = null, n = Mh(e, b, c), k.memoizedState = k.baseState = n);
      return n;
    }
    function Ph(a, b, c) {
      var d = Ih();
      return Nh(d, a, b, c);
    }
    function Qh(a) {
      var b = Hh();
      typeof a === "function" && (a = a());
      b.memoizedState = b.baseState = a;
      a = b.queue = {pending: null, dispatch: null, lastRenderedReducer: Jh, lastRenderedState: a};
      a = a.dispatch = Oh.bind(null, R, a);
      return [b.memoizedState, a];
    }
    function Rh(a, b, c, d) {
      a = {tag: a, create: b, destroy: c, deps: d, next: null};
      b = R.updateQueue;
      b === null ? (b = {lastEffect: null}, R.updateQueue = b, b.lastEffect = a.next = a) : (c = b.lastEffect, c === null ? b.lastEffect = a.next = a : (d = c.next, c.next = a, a.next = d, b.lastEffect = a));
      return a;
    }
    function Sh(a) {
      var b = Hh();
      a = {current: a};
      return b.memoizedState = a;
    }
    function Th() {
      return Ih().memoizedState;
    }
    function Uh(a, b, c, d) {
      var e = Hh();
      R.flags |= a;
      e.memoizedState = Rh(1 | b, c, void 0, d === void 0 ? null : d);
    }
    function Vh(a, b, c, d) {
      var e = Ih();
      d = d === void 0 ? null : d;
      var f = void 0;
      if (S !== null) {
        var g = S.memoizedState;
        f = g.destroy;
        if (d !== null && Bh(d, g.deps)) {
          Rh(b, c, f, d);
          return;
        }
      }
      R.flags |= a;
      e.memoizedState = Rh(1 | b, c, f, d);
    }
    function Wh(a, b) {
      return Uh(516, 4, a, b);
    }
    function Xh(a, b) {
      return Vh(516, 4, a, b);
    }
    function Yh(a, b) {
      return Vh(4, 2, a, b);
    }
    function Zh(a, b) {
      if (typeof b === "function")
        return a = a(), b(a), function() {
          b(null);
        };
      if (b !== null && b !== void 0)
        return a = a(), b.current = a, function() {
          b.current = null;
        };
    }
    function $h(a, b, c) {
      c = c !== null && c !== void 0 ? c.concat([a]) : null;
      return Vh(4, 2, Zh.bind(null, b, a), c);
    }
    function ai() {
    }
    function bi(a, b) {
      var c = Ih();
      b = b === void 0 ? null : b;
      var d = c.memoizedState;
      if (d !== null && b !== null && Bh(b, d[1]))
        return d[0];
      c.memoizedState = [a, b];
      return a;
    }
    function ci(a, b) {
      var c = Ih();
      b = b === void 0 ? null : b;
      var d = c.memoizedState;
      if (d !== null && b !== null && Bh(b, d[1]))
        return d[0];
      a = a();
      c.memoizedState = [a, b];
      return a;
    }
    function di(a, b) {
      var c = eg();
      gg(98 > c ? 98 : c, function() {
        a(true);
      });
      gg(97 < c ? 97 : c, function() {
        var c2 = wh.transition;
        wh.transition = 1;
        try {
          a(false), b();
        } finally {
          wh.transition = c2;
        }
      });
    }
    function Oh(a, b, c) {
      var d = Hg(), e = Ig(a), f = {lane: e, action: c, eagerReducer: null, eagerState: null, next: null}, g = b.pending;
      g === null ? f.next = f : (f.next = g.next, g.next = f);
      b.pending = f;
      g = a.alternate;
      if (a === R || g !== null && g === R)
        zh = yh = true;
      else {
        if (a.lanes === 0 && (g === null || g.lanes === 0) && (g = b.lastRenderedReducer, g !== null))
          try {
            var h = b.lastRenderedState, k = g(h, c);
            f.eagerReducer = g;
            f.eagerState = k;
            if (He(k, h))
              return;
          } catch (l) {
          } finally {
          }
        Jg(a, e, d);
      }
    }
    var Gh = {readContext: vg, useCallback: Ah, useContext: Ah, useEffect: Ah, useImperativeHandle: Ah, useLayoutEffect: Ah, useMemo: Ah, useReducer: Ah, useRef: Ah, useState: Ah, useDebugValue: Ah, useDeferredValue: Ah, useTransition: Ah, useMutableSource: Ah, useOpaqueIdentifier: Ah, unstable_isNewReconciler: false};
    var Dh = {readContext: vg, useCallback: function(a, b) {
      Hh().memoizedState = [a, b === void 0 ? null : b];
      return a;
    }, useContext: vg, useEffect: Wh, useImperativeHandle: function(a, b, c) {
      c = c !== null && c !== void 0 ? c.concat([a]) : null;
      return Uh(4, 2, Zh.bind(null, b, a), c);
    }, useLayoutEffect: function(a, b) {
      return Uh(4, 2, a, b);
    }, useMemo: function(a, b) {
      var c = Hh();
      b = b === void 0 ? null : b;
      a = a();
      c.memoizedState = [a, b];
      return a;
    }, useReducer: function(a, b, c) {
      var d = Hh();
      b = c !== void 0 ? c(b) : b;
      d.memoizedState = d.baseState = b;
      a = d.queue = {pending: null, dispatch: null, lastRenderedReducer: a, lastRenderedState: b};
      a = a.dispatch = Oh.bind(null, R, a);
      return [d.memoizedState, a];
    }, useRef: Sh, useState: Qh, useDebugValue: ai, useDeferredValue: function(a) {
      var b = Qh(a), c = b[0], d = b[1];
      Wh(function() {
        var b2 = wh.transition;
        wh.transition = 1;
        try {
          d(a);
        } finally {
          wh.transition = b2;
        }
      }, [a]);
      return c;
    }, useTransition: function() {
      var a = Qh(false), b = a[0];
      a = di.bind(null, a[1]);
      Sh(a);
      return [a, b];
    }, useMutableSource: function(a, b, c) {
      var d = Hh();
      d.memoizedState = {refs: {getSnapshot: b, setSnapshot: null}, source: a, subscribe: c};
      return Nh(d, a, b, c);
    }, useOpaqueIdentifier: function() {
      if (lh) {
        var a = false, b = uf(function() {
          a || (a = true, c("r:" + (tf++).toString(36)));
          throw Error(y(355));
        }), c = Qh(b)[1];
        (R.mode & 2) === 0 && (R.flags |= 516, Rh(5, function() {
          c("r:" + (tf++).toString(36));
        }, void 0, null));
        return b;
      }
      b = "r:" + (tf++).toString(36);
      Qh(b);
      return b;
    }, unstable_isNewReconciler: false};
    var Eh = {readContext: vg, useCallback: bi, useContext: vg, useEffect: Xh, useImperativeHandle: $h, useLayoutEffect: Yh, useMemo: ci, useReducer: Kh, useRef: Th, useState: function() {
      return Kh(Jh);
    }, useDebugValue: ai, useDeferredValue: function(a) {
      var b = Kh(Jh), c = b[0], d = b[1];
      Xh(function() {
        var b2 = wh.transition;
        wh.transition = 1;
        try {
          d(a);
        } finally {
          wh.transition = b2;
        }
      }, [a]);
      return c;
    }, useTransition: function() {
      var a = Kh(Jh)[0];
      return [
        Th().current,
        a
      ];
    }, useMutableSource: Ph, useOpaqueIdentifier: function() {
      return Kh(Jh)[0];
    }, unstable_isNewReconciler: false};
    var Fh = {readContext: vg, useCallback: bi, useContext: vg, useEffect: Xh, useImperativeHandle: $h, useLayoutEffect: Yh, useMemo: ci, useReducer: Lh, useRef: Th, useState: function() {
      return Lh(Jh);
    }, useDebugValue: ai, useDeferredValue: function(a) {
      var b = Lh(Jh), c = b[0], d = b[1];
      Xh(function() {
        var b2 = wh.transition;
        wh.transition = 1;
        try {
          d(a);
        } finally {
          wh.transition = b2;
        }
      }, [a]);
      return c;
    }, useTransition: function() {
      var a = Lh(Jh)[0];
      return [
        Th().current,
        a
      ];
    }, useMutableSource: Ph, useOpaqueIdentifier: function() {
      return Lh(Jh)[0];
    }, unstable_isNewReconciler: false};
    var ei = ra.ReactCurrentOwner;
    var ug = false;
    function fi(a, b, c, d) {
      b.child = a === null ? Zg(b, null, c, d) : Yg(b, a.child, c, d);
    }
    function gi(a, b, c, d, e) {
      c = c.render;
      var f = b.ref;
      tg(b, e);
      d = Ch(a, b, c, d, f, e);
      if (a !== null && !ug)
        return b.updateQueue = a.updateQueue, b.flags &= -517, a.lanes &= ~e, hi(a, b, e);
      b.flags |= 1;
      fi(a, b, d, e);
      return b.child;
    }
    function ii(a, b, c, d, e, f) {
      if (a === null) {
        var g = c.type;
        if (typeof g === "function" && !ji(g) && g.defaultProps === void 0 && c.compare === null && c.defaultProps === void 0)
          return b.tag = 15, b.type = g, ki(a, b, g, d, e, f);
        a = Vg(c.type, null, d, b, b.mode, f);
        a.ref = b.ref;
        a.return = b;
        return b.child = a;
      }
      g = a.child;
      if ((e & f) === 0 && (e = g.memoizedProps, c = c.compare, c = c !== null ? c : Je, c(e, d) && a.ref === b.ref))
        return hi(a, b, f);
      b.flags |= 1;
      a = Tg(g, d);
      a.ref = b.ref;
      a.return = b;
      return b.child = a;
    }
    function ki(a, b, c, d, e, f) {
      if (a !== null && Je(a.memoizedProps, d) && a.ref === b.ref)
        if (ug = false, (f & e) !== 0)
          (a.flags & 16384) !== 0 && (ug = true);
        else
          return b.lanes = a.lanes, hi(a, b, f);
      return li(a, b, c, d, f);
    }
    function mi(a, b, c) {
      var d = b.pendingProps, e = d.children, f = a !== null ? a.memoizedState : null;
      if (d.mode === "hidden" || d.mode === "unstable-defer-without-hiding")
        if ((b.mode & 4) === 0)
          b.memoizedState = {baseLanes: 0}, ni(b, c);
        else if ((c & 1073741824) !== 0)
          b.memoizedState = {baseLanes: 0}, ni(b, f !== null ? f.baseLanes : c);
        else
          return a = f !== null ? f.baseLanes | c : c, b.lanes = b.childLanes = 1073741824, b.memoizedState = {baseLanes: a}, ni(b, a), null;
      else
        f !== null ? (d = f.baseLanes | c, b.memoizedState = null) : d = c, ni(b, d);
      fi(a, b, e, c);
      return b.child;
    }
    function oi(a, b) {
      var c = b.ref;
      if (a === null && c !== null || a !== null && a.ref !== c)
        b.flags |= 128;
    }
    function li(a, b, c, d, e) {
      var f = Ff(c) ? Df : M.current;
      f = Ef(b, f);
      tg(b, e);
      c = Ch(a, b, c, d, f, e);
      if (a !== null && !ug)
        return b.updateQueue = a.updateQueue, b.flags &= -517, a.lanes &= ~e, hi(a, b, e);
      b.flags |= 1;
      fi(a, b, c, e);
      return b.child;
    }
    function pi(a, b, c, d, e) {
      if (Ff(c)) {
        var f = true;
        Jf(b);
      } else
        f = false;
      tg(b, e);
      if (b.stateNode === null)
        a !== null && (a.alternate = null, b.alternate = null, b.flags |= 2), Mg(b, c, d), Og(b, c, d, e), d = true;
      else if (a === null) {
        var g = b.stateNode, h = b.memoizedProps;
        g.props = h;
        var k = g.context, l = c.contextType;
        typeof l === "object" && l !== null ? l = vg(l) : (l = Ff(c) ? Df : M.current, l = Ef(b, l));
        var n = c.getDerivedStateFromProps, A = typeof n === "function" || typeof g.getSnapshotBeforeUpdate === "function";
        A || typeof g.UNSAFE_componentWillReceiveProps !== "function" && typeof g.componentWillReceiveProps !== "function" || (h !== d || k !== l) && Ng(b, g, d, l);
        wg = false;
        var p = b.memoizedState;
        g.state = p;
        Cg(b, d, g, e);
        k = b.memoizedState;
        h !== d || p !== k || N.current || wg ? (typeof n === "function" && (Gg(b, c, n, d), k = b.memoizedState), (h = wg || Lg(b, c, h, d, p, k, l)) ? (A || typeof g.UNSAFE_componentWillMount !== "function" && typeof g.componentWillMount !== "function" || (typeof g.componentWillMount === "function" && g.componentWillMount(), typeof g.UNSAFE_componentWillMount === "function" && g.UNSAFE_componentWillMount()), typeof g.componentDidMount === "function" && (b.flags |= 4)) : (typeof g.componentDidMount === "function" && (b.flags |= 4), b.memoizedProps = d, b.memoizedState = k), g.props = d, g.state = k, g.context = l, d = h) : (typeof g.componentDidMount === "function" && (b.flags |= 4), d = false);
      } else {
        g = b.stateNode;
        yg(a, b);
        h = b.memoizedProps;
        l = b.type === b.elementType ? h : lg(b.type, h);
        g.props = l;
        A = b.pendingProps;
        p = g.context;
        k = c.contextType;
        typeof k === "object" && k !== null ? k = vg(k) : (k = Ff(c) ? Df : M.current, k = Ef(b, k));
        var C = c.getDerivedStateFromProps;
        (n = typeof C === "function" || typeof g.getSnapshotBeforeUpdate === "function") || typeof g.UNSAFE_componentWillReceiveProps !== "function" && typeof g.componentWillReceiveProps !== "function" || (h !== A || p !== k) && Ng(b, g, d, k);
        wg = false;
        p = b.memoizedState;
        g.state = p;
        Cg(b, d, g, e);
        var x = b.memoizedState;
        h !== A || p !== x || N.current || wg ? (typeof C === "function" && (Gg(b, c, C, d), x = b.memoizedState), (l = wg || Lg(b, c, l, d, p, x, k)) ? (n || typeof g.UNSAFE_componentWillUpdate !== "function" && typeof g.componentWillUpdate !== "function" || (typeof g.componentWillUpdate === "function" && g.componentWillUpdate(d, x, k), typeof g.UNSAFE_componentWillUpdate === "function" && g.UNSAFE_componentWillUpdate(d, x, k)), typeof g.componentDidUpdate === "function" && (b.flags |= 4), typeof g.getSnapshotBeforeUpdate === "function" && (b.flags |= 256)) : (typeof g.componentDidUpdate !== "function" || h === a.memoizedProps && p === a.memoizedState || (b.flags |= 4), typeof g.getSnapshotBeforeUpdate !== "function" || h === a.memoizedProps && p === a.memoizedState || (b.flags |= 256), b.memoizedProps = d, b.memoizedState = x), g.props = d, g.state = x, g.context = k, d = l) : (typeof g.componentDidUpdate !== "function" || h === a.memoizedProps && p === a.memoizedState || (b.flags |= 4), typeof g.getSnapshotBeforeUpdate !== "function" || h === a.memoizedProps && p === a.memoizedState || (b.flags |= 256), d = false);
      }
      return qi(a, b, c, d, f, e);
    }
    function qi(a, b, c, d, e, f) {
      oi(a, b);
      var g = (b.flags & 64) !== 0;
      if (!d && !g)
        return e && Kf(b, c, false), hi(a, b, f);
      d = b.stateNode;
      ei.current = b;
      var h = g && typeof c.getDerivedStateFromError !== "function" ? null : d.render();
      b.flags |= 1;
      a !== null && g ? (b.child = Yg(b, a.child, null, f), b.child = Yg(b, null, h, f)) : fi(a, b, h, f);
      b.memoizedState = d.state;
      e && Kf(b, c, true);
      return b.child;
    }
    function ri(a) {
      var b = a.stateNode;
      b.pendingContext ? Hf(a, b.pendingContext, b.pendingContext !== b.context) : b.context && Hf(a, b.context, false);
      eh(a, b.containerInfo);
    }
    var si = {dehydrated: null, retryLane: 0};
    function ti(a, b, c) {
      var d = b.pendingProps, e = P.current, f = false, g;
      (g = (b.flags & 64) !== 0) || (g = a !== null && a.memoizedState === null ? false : (e & 2) !== 0);
      g ? (f = true, b.flags &= -65) : a !== null && a.memoizedState === null || d.fallback === void 0 || d.unstable_avoidThisFallback === true || (e |= 1);
      I(P, e & 1);
      if (a === null) {
        d.fallback !== void 0 && ph(b);
        a = d.children;
        e = d.fallback;
        if (f)
          return a = ui(b, a, e, c), b.child.memoizedState = {baseLanes: c}, b.memoizedState = si, a;
        if (typeof d.unstable_expectedLoadTime === "number")
          return a = ui(b, a, e, c), b.child.memoizedState = {baseLanes: c}, b.memoizedState = si, b.lanes = 33554432, a;
        c = vi({mode: "visible", children: a}, b.mode, c, null);
        c.return = b;
        return b.child = c;
      }
      if (a.memoizedState !== null) {
        if (f)
          return d = wi(a, b, d.children, d.fallback, c), f = b.child, e = a.child.memoizedState, f.memoizedState = e === null ? {baseLanes: c} : {baseLanes: e.baseLanes | c}, f.childLanes = a.childLanes & ~c, b.memoizedState = si, d;
        c = xi(a, b, d.children, c);
        b.memoizedState = null;
        return c;
      }
      if (f)
        return d = wi(a, b, d.children, d.fallback, c), f = b.child, e = a.child.memoizedState, f.memoizedState = e === null ? {baseLanes: c} : {baseLanes: e.baseLanes | c}, f.childLanes = a.childLanes & ~c, b.memoizedState = si, d;
      c = xi(a, b, d.children, c);
      b.memoizedState = null;
      return c;
    }
    function ui(a, b, c, d) {
      var e = a.mode, f = a.child;
      b = {mode: "hidden", children: b};
      (e & 2) === 0 && f !== null ? (f.childLanes = 0, f.pendingProps = b) : f = vi(b, e, 0, null);
      c = Xg(c, e, d, null);
      f.return = a;
      c.return = a;
      f.sibling = c;
      a.child = f;
      return c;
    }
    function xi(a, b, c, d) {
      var e = a.child;
      a = e.sibling;
      c = Tg(e, {mode: "visible", children: c});
      (b.mode & 2) === 0 && (c.lanes = d);
      c.return = b;
      c.sibling = null;
      a !== null && (a.nextEffect = null, a.flags = 8, b.firstEffect = b.lastEffect = a);
      return b.child = c;
    }
    function wi(a, b, c, d, e) {
      var f = b.mode, g = a.child;
      a = g.sibling;
      var h = {mode: "hidden", children: c};
      (f & 2) === 0 && b.child !== g ? (c = b.child, c.childLanes = 0, c.pendingProps = h, g = c.lastEffect, g !== null ? (b.firstEffect = c.firstEffect, b.lastEffect = g, g.nextEffect = null) : b.firstEffect = b.lastEffect = null) : c = Tg(g, h);
      a !== null ? d = Tg(a, d) : (d = Xg(d, f, e, null), d.flags |= 2);
      d.return = b;
      c.return = b;
      c.sibling = d;
      b.child = c;
      return d;
    }
    function yi(a, b) {
      a.lanes |= b;
      var c = a.alternate;
      c !== null && (c.lanes |= b);
      sg(a.return, b);
    }
    function zi(a, b, c, d, e, f) {
      var g = a.memoizedState;
      g === null ? a.memoizedState = {isBackwards: b, rendering: null, renderingStartTime: 0, last: d, tail: c, tailMode: e, lastEffect: f} : (g.isBackwards = b, g.rendering = null, g.renderingStartTime = 0, g.last = d, g.tail = c, g.tailMode = e, g.lastEffect = f);
    }
    function Ai(a, b, c) {
      var d = b.pendingProps, e = d.revealOrder, f = d.tail;
      fi(a, b, d.children, c);
      d = P.current;
      if ((d & 2) !== 0)
        d = d & 1 | 2, b.flags |= 64;
      else {
        if (a !== null && (a.flags & 64) !== 0)
          a:
            for (a = b.child; a !== null; ) {
              if (a.tag === 13)
                a.memoizedState !== null && yi(a, c);
              else if (a.tag === 19)
                yi(a, c);
              else if (a.child !== null) {
                a.child.return = a;
                a = a.child;
                continue;
              }
              if (a === b)
                break a;
              for (; a.sibling === null; ) {
                if (a.return === null || a.return === b)
                  break a;
                a = a.return;
              }
              a.sibling.return = a.return;
              a = a.sibling;
            }
        d &= 1;
      }
      I(P, d);
      if ((b.mode & 2) === 0)
        b.memoizedState = null;
      else
        switch (e) {
          case "forwards":
            c = b.child;
            for (e = null; c !== null; )
              a = c.alternate, a !== null && ih(a) === null && (e = c), c = c.sibling;
            c = e;
            c === null ? (e = b.child, b.child = null) : (e = c.sibling, c.sibling = null);
            zi(b, false, e, c, f, b.lastEffect);
            break;
          case "backwards":
            c = null;
            e = b.child;
            for (b.child = null; e !== null; ) {
              a = e.alternate;
              if (a !== null && ih(a) === null) {
                b.child = e;
                break;
              }
              a = e.sibling;
              e.sibling = c;
              c = e;
              e = a;
            }
            zi(b, true, c, null, f, b.lastEffect);
            break;
          case "together":
            zi(b, false, null, null, void 0, b.lastEffect);
            break;
          default:
            b.memoizedState = null;
        }
      return b.child;
    }
    function hi(a, b, c) {
      a !== null && (b.dependencies = a.dependencies);
      Dg |= b.lanes;
      if ((c & b.childLanes) !== 0) {
        if (a !== null && b.child !== a.child)
          throw Error(y(153));
        if (b.child !== null) {
          a = b.child;
          c = Tg(a, a.pendingProps);
          b.child = c;
          for (c.return = b; a.sibling !== null; )
            a = a.sibling, c = c.sibling = Tg(a, a.pendingProps), c.return = b;
          c.sibling = null;
        }
        return b.child;
      }
      return null;
    }
    var Bi;
    var Ci;
    var Di;
    var Ei;
    Bi = function(a, b) {
      for (var c = b.child; c !== null; ) {
        if (c.tag === 5 || c.tag === 6)
          a.appendChild(c.stateNode);
        else if (c.tag !== 4 && c.child !== null) {
          c.child.return = c;
          c = c.child;
          continue;
        }
        if (c === b)
          break;
        for (; c.sibling === null; ) {
          if (c.return === null || c.return === b)
            return;
          c = c.return;
        }
        c.sibling.return = c.return;
        c = c.sibling;
      }
    };
    Ci = function() {
    };
    Di = function(a, b, c, d) {
      var e = a.memoizedProps;
      if (e !== d) {
        a = b.stateNode;
        dh(ah.current);
        var f = null;
        switch (c) {
          case "input":
            e = Ya(a, e);
            d = Ya(a, d);
            f = [];
            break;
          case "option":
            e = eb(a, e);
            d = eb(a, d);
            f = [];
            break;
          case "select":
            e = m({}, e, {value: void 0});
            d = m({}, d, {value: void 0});
            f = [];
            break;
          case "textarea":
            e = gb(a, e);
            d = gb(a, d);
            f = [];
            break;
          default:
            typeof e.onClick !== "function" && typeof d.onClick === "function" && (a.onclick = jf);
        }
        vb(c, d);
        var g;
        c = null;
        for (l in e)
          if (!d.hasOwnProperty(l) && e.hasOwnProperty(l) && e[l] != null)
            if (l === "style") {
              var h = e[l];
              for (g in h)
                h.hasOwnProperty(g) && (c || (c = {}), c[g] = "");
            } else
              l !== "dangerouslySetInnerHTML" && l !== "children" && l !== "suppressContentEditableWarning" && l !== "suppressHydrationWarning" && l !== "autoFocus" && (ca.hasOwnProperty(l) ? f || (f = []) : (f = f || []).push(l, null));
        for (l in d) {
          var k = d[l];
          h = e != null ? e[l] : void 0;
          if (d.hasOwnProperty(l) && k !== h && (k != null || h != null))
            if (l === "style")
              if (h) {
                for (g in h)
                  !h.hasOwnProperty(g) || k && k.hasOwnProperty(g) || (c || (c = {}), c[g] = "");
                for (g in k)
                  k.hasOwnProperty(g) && h[g] !== k[g] && (c || (c = {}), c[g] = k[g]);
              } else
                c || (f || (f = []), f.push(l, c)), c = k;
            else
              l === "dangerouslySetInnerHTML" ? (k = k ? k.__html : void 0, h = h ? h.__html : void 0, k != null && h !== k && (f = f || []).push(l, k)) : l === "children" ? typeof k !== "string" && typeof k !== "number" || (f = f || []).push(l, "" + k) : l !== "suppressContentEditableWarning" && l !== "suppressHydrationWarning" && (ca.hasOwnProperty(l) ? (k != null && l === "onScroll" && G("scroll", a), f || h === k || (f = [])) : typeof k === "object" && k !== null && k.$$typeof === Ga ? k.toString() : (f = f || []).push(l, k));
        }
        c && (f = f || []).push("style", c);
        var l = f;
        if (b.updateQueue = l)
          b.flags |= 4;
      }
    };
    Ei = function(a, b, c, d) {
      c !== d && (b.flags |= 4);
    };
    function Fi(a, b) {
      if (!lh)
        switch (a.tailMode) {
          case "hidden":
            b = a.tail;
            for (var c = null; b !== null; )
              b.alternate !== null && (c = b), b = b.sibling;
            c === null ? a.tail = null : c.sibling = null;
            break;
          case "collapsed":
            c = a.tail;
            for (var d = null; c !== null; )
              c.alternate !== null && (d = c), c = c.sibling;
            d === null ? b || a.tail === null ? a.tail = null : a.tail.sibling = null : d.sibling = null;
        }
    }
    function Gi(a, b, c) {
      var d = b.pendingProps;
      switch (b.tag) {
        case 2:
        case 16:
        case 15:
        case 0:
        case 11:
        case 7:
        case 8:
        case 12:
        case 9:
        case 14:
          return null;
        case 1:
          return Ff(b.type) && Gf(), null;
        case 3:
          fh();
          H(N);
          H(M);
          uh();
          d = b.stateNode;
          d.pendingContext && (d.context = d.pendingContext, d.pendingContext = null);
          if (a === null || a.child === null)
            rh(b) ? b.flags |= 4 : d.hydrate || (b.flags |= 256);
          Ci(b);
          return null;
        case 5:
          hh(b);
          var e = dh(ch.current);
          c = b.type;
          if (a !== null && b.stateNode != null)
            Di(a, b, c, d, e), a.ref !== b.ref && (b.flags |= 128);
          else {
            if (!d) {
              if (b.stateNode === null)
                throw Error(y(166));
              return null;
            }
            a = dh(ah.current);
            if (rh(b)) {
              d = b.stateNode;
              c = b.type;
              var f = b.memoizedProps;
              d[wf] = b;
              d[xf] = f;
              switch (c) {
                case "dialog":
                  G("cancel", d);
                  G("close", d);
                  break;
                case "iframe":
                case "object":
                case "embed":
                  G("load", d);
                  break;
                case "video":
                case "audio":
                  for (a = 0; a < Xe.length; a++)
                    G(Xe[a], d);
                  break;
                case "source":
                  G("error", d);
                  break;
                case "img":
                case "image":
                case "link":
                  G("error", d);
                  G("load", d);
                  break;
                case "details":
                  G("toggle", d);
                  break;
                case "input":
                  Za(d, f);
                  G("invalid", d);
                  break;
                case "select":
                  d._wrapperState = {wasMultiple: !!f.multiple};
                  G("invalid", d);
                  break;
                case "textarea":
                  hb(d, f), G("invalid", d);
              }
              vb(c, f);
              a = null;
              for (var g in f)
                f.hasOwnProperty(g) && (e = f[g], g === "children" ? typeof e === "string" ? d.textContent !== e && (a = ["children", e]) : typeof e === "number" && d.textContent !== "" + e && (a = ["children", "" + e]) : ca.hasOwnProperty(g) && e != null && g === "onScroll" && G("scroll", d));
              switch (c) {
                case "input":
                  Va(d);
                  cb(d, f, true);
                  break;
                case "textarea":
                  Va(d);
                  jb(d);
                  break;
                case "select":
                case "option":
                  break;
                default:
                  typeof f.onClick === "function" && (d.onclick = jf);
              }
              d = a;
              b.updateQueue = d;
              d !== null && (b.flags |= 4);
            } else {
              g = e.nodeType === 9 ? e : e.ownerDocument;
              a === kb.html && (a = lb(c));
              a === kb.html ? c === "script" ? (a = g.createElement("div"), a.innerHTML = "<script></script>", a = a.removeChild(a.firstChild)) : typeof d.is === "string" ? a = g.createElement(c, {is: d.is}) : (a = g.createElement(c), c === "select" && (g = a, d.multiple ? g.multiple = true : d.size && (g.size = d.size))) : a = g.createElementNS(a, c);
              a[wf] = b;
              a[xf] = d;
              Bi(a, b, false, false);
              b.stateNode = a;
              g = wb(c, d);
              switch (c) {
                case "dialog":
                  G("cancel", a);
                  G("close", a);
                  e = d;
                  break;
                case "iframe":
                case "object":
                case "embed":
                  G("load", a);
                  e = d;
                  break;
                case "video":
                case "audio":
                  for (e = 0; e < Xe.length; e++)
                    G(Xe[e], a);
                  e = d;
                  break;
                case "source":
                  G("error", a);
                  e = d;
                  break;
                case "img":
                case "image":
                case "link":
                  G("error", a);
                  G("load", a);
                  e = d;
                  break;
                case "details":
                  G("toggle", a);
                  e = d;
                  break;
                case "input":
                  Za(a, d);
                  e = Ya(a, d);
                  G("invalid", a);
                  break;
                case "option":
                  e = eb(a, d);
                  break;
                case "select":
                  a._wrapperState = {wasMultiple: !!d.multiple};
                  e = m({}, d, {value: void 0});
                  G("invalid", a);
                  break;
                case "textarea":
                  hb(a, d);
                  e = gb(a, d);
                  G("invalid", a);
                  break;
                default:
                  e = d;
              }
              vb(c, e);
              var h = e;
              for (f in h)
                if (h.hasOwnProperty(f)) {
                  var k = h[f];
                  f === "style" ? tb(a, k) : f === "dangerouslySetInnerHTML" ? (k = k ? k.__html : void 0, k != null && ob(a, k)) : f === "children" ? typeof k === "string" ? (c !== "textarea" || k !== "") && pb(a, k) : typeof k === "number" && pb(a, "" + k) : f !== "suppressContentEditableWarning" && f !== "suppressHydrationWarning" && f !== "autoFocus" && (ca.hasOwnProperty(f) ? k != null && f === "onScroll" && G("scroll", a) : k != null && qa(a, f, k, g));
                }
              switch (c) {
                case "input":
                  Va(a);
                  cb(a, d, false);
                  break;
                case "textarea":
                  Va(a);
                  jb(a);
                  break;
                case "option":
                  d.value != null && a.setAttribute("value", "" + Sa(d.value));
                  break;
                case "select":
                  a.multiple = !!d.multiple;
                  f = d.value;
                  f != null ? fb(a, !!d.multiple, f, false) : d.defaultValue != null && fb(a, !!d.multiple, d.defaultValue, true);
                  break;
                default:
                  typeof e.onClick === "function" && (a.onclick = jf);
              }
              mf(c, d) && (b.flags |= 4);
            }
            b.ref !== null && (b.flags |= 128);
          }
          return null;
        case 6:
          if (a && b.stateNode != null)
            Ei(a, b, a.memoizedProps, d);
          else {
            if (typeof d !== "string" && b.stateNode === null)
              throw Error(y(166));
            c = dh(ch.current);
            dh(ah.current);
            rh(b) ? (d = b.stateNode, c = b.memoizedProps, d[wf] = b, d.nodeValue !== c && (b.flags |= 4)) : (d = (c.nodeType === 9 ? c : c.ownerDocument).createTextNode(d), d[wf] = b, b.stateNode = d);
          }
          return null;
        case 13:
          H(P);
          d = b.memoizedState;
          if ((b.flags & 64) !== 0)
            return b.lanes = c, b;
          d = d !== null;
          c = false;
          a === null ? b.memoizedProps.fallback !== void 0 && rh(b) : c = a.memoizedState !== null;
          if (d && !c && (b.mode & 2) !== 0)
            if (a === null && b.memoizedProps.unstable_avoidThisFallback !== true || (P.current & 1) !== 0)
              V === 0 && (V = 3);
            else {
              if (V === 0 || V === 3)
                V = 4;
              U === null || (Dg & 134217727) === 0 && (Hi & 134217727) === 0 || Ii(U, W);
            }
          if (d || c)
            b.flags |= 4;
          return null;
        case 4:
          return fh(), Ci(b), a === null && cf(b.stateNode.containerInfo), null;
        case 10:
          return rg(b), null;
        case 17:
          return Ff(b.type) && Gf(), null;
        case 19:
          H(P);
          d = b.memoizedState;
          if (d === null)
            return null;
          f = (b.flags & 64) !== 0;
          g = d.rendering;
          if (g === null)
            if (f)
              Fi(d, false);
            else {
              if (V !== 0 || a !== null && (a.flags & 64) !== 0)
                for (a = b.child; a !== null; ) {
                  g = ih(a);
                  if (g !== null) {
                    b.flags |= 64;
                    Fi(d, false);
                    f = g.updateQueue;
                    f !== null && (b.updateQueue = f, b.flags |= 4);
                    d.lastEffect === null && (b.firstEffect = null);
                    b.lastEffect = d.lastEffect;
                    d = c;
                    for (c = b.child; c !== null; )
                      f = c, a = d, f.flags &= 2, f.nextEffect = null, f.firstEffect = null, f.lastEffect = null, g = f.alternate, g === null ? (f.childLanes = 0, f.lanes = a, f.child = null, f.memoizedProps = null, f.memoizedState = null, f.updateQueue = null, f.dependencies = null, f.stateNode = null) : (f.childLanes = g.childLanes, f.lanes = g.lanes, f.child = g.child, f.memoizedProps = g.memoizedProps, f.memoizedState = g.memoizedState, f.updateQueue = g.updateQueue, f.type = g.type, a = g.dependencies, f.dependencies = a === null ? null : {lanes: a.lanes, firstContext: a.firstContext}), c = c.sibling;
                    I(P, P.current & 1 | 2);
                    return b.child;
                  }
                  a = a.sibling;
                }
              d.tail !== null && O() > Ji && (b.flags |= 64, f = true, Fi(d, false), b.lanes = 33554432);
            }
          else {
            if (!f)
              if (a = ih(g), a !== null) {
                if (b.flags |= 64, f = true, c = a.updateQueue, c !== null && (b.updateQueue = c, b.flags |= 4), Fi(d, true), d.tail === null && d.tailMode === "hidden" && !g.alternate && !lh)
                  return b = b.lastEffect = d.lastEffect, b !== null && (b.nextEffect = null), null;
              } else
                2 * O() - d.renderingStartTime > Ji && c !== 1073741824 && (b.flags |= 64, f = true, Fi(d, false), b.lanes = 33554432);
            d.isBackwards ? (g.sibling = b.child, b.child = g) : (c = d.last, c !== null ? c.sibling = g : b.child = g, d.last = g);
          }
          return d.tail !== null ? (c = d.tail, d.rendering = c, d.tail = c.sibling, d.lastEffect = b.lastEffect, d.renderingStartTime = O(), c.sibling = null, b = P.current, I(P, f ? b & 1 | 2 : b & 1), c) : null;
        case 23:
        case 24:
          return Ki(), a !== null && a.memoizedState !== null !== (b.memoizedState !== null) && d.mode !== "unstable-defer-without-hiding" && (b.flags |= 4), null;
      }
      throw Error(y(156, b.tag));
    }
    function Li(a) {
      switch (a.tag) {
        case 1:
          Ff(a.type) && Gf();
          var b = a.flags;
          return b & 4096 ? (a.flags = b & -4097 | 64, a) : null;
        case 3:
          fh();
          H(N);
          H(M);
          uh();
          b = a.flags;
          if ((b & 64) !== 0)
            throw Error(y(285));
          a.flags = b & -4097 | 64;
          return a;
        case 5:
          return hh(a), null;
        case 13:
          return H(P), b = a.flags, b & 4096 ? (a.flags = b & -4097 | 64, a) : null;
        case 19:
          return H(P), null;
        case 4:
          return fh(), null;
        case 10:
          return rg(a), null;
        case 23:
        case 24:
          return Ki(), null;
        default:
          return null;
      }
    }
    function Mi(a, b) {
      try {
        var c = "", d = b;
        do
          c += Qa(d), d = d.return;
        while (d);
        var e = c;
      } catch (f) {
        e = "\nError generating stack: " + f.message + "\n" + f.stack;
      }
      return {value: a, source: b, stack: e};
    }
    function Ni(a, b) {
      try {
        console.error(b.value);
      } catch (c) {
        setTimeout(function() {
          throw c;
        });
      }
    }
    var Oi = typeof WeakMap === "function" ? WeakMap : Map;
    function Pi(a, b, c) {
      c = zg(-1, c);
      c.tag = 3;
      c.payload = {element: null};
      var d = b.value;
      c.callback = function() {
        Qi || (Qi = true, Ri = d);
        Ni(a, b);
      };
      return c;
    }
    function Si(a, b, c) {
      c = zg(-1, c);
      c.tag = 3;
      var d = a.type.getDerivedStateFromError;
      if (typeof d === "function") {
        var e = b.value;
        c.payload = function() {
          Ni(a, b);
          return d(e);
        };
      }
      var f = a.stateNode;
      f !== null && typeof f.componentDidCatch === "function" && (c.callback = function() {
        typeof d !== "function" && (Ti === null ? Ti = new Set([this]) : Ti.add(this), Ni(a, b));
        var c2 = b.stack;
        this.componentDidCatch(b.value, {componentStack: c2 !== null ? c2 : ""});
      });
      return c;
    }
    var Ui = typeof WeakSet === "function" ? WeakSet : Set;
    function Vi(a) {
      var b = a.ref;
      if (b !== null)
        if (typeof b === "function")
          try {
            b(null);
          } catch (c) {
            Wi(a, c);
          }
        else
          b.current = null;
    }
    function Xi(a, b) {
      switch (b.tag) {
        case 0:
        case 11:
        case 15:
        case 22:
          return;
        case 1:
          if (b.flags & 256 && a !== null) {
            var c = a.memoizedProps, d = a.memoizedState;
            a = b.stateNode;
            b = a.getSnapshotBeforeUpdate(b.elementType === b.type ? c : lg(b.type, c), d);
            a.__reactInternalSnapshotBeforeUpdate = b;
          }
          return;
        case 3:
          b.flags & 256 && qf(b.stateNode.containerInfo);
          return;
        case 5:
        case 6:
        case 4:
        case 17:
          return;
      }
      throw Error(y(163));
    }
    function Yi(a, b, c) {
      switch (c.tag) {
        case 0:
        case 11:
        case 15:
        case 22:
          b = c.updateQueue;
          b = b !== null ? b.lastEffect : null;
          if (b !== null) {
            a = b = b.next;
            do {
              if ((a.tag & 3) === 3) {
                var d = a.create;
                a.destroy = d();
              }
              a = a.next;
            } while (a !== b);
          }
          b = c.updateQueue;
          b = b !== null ? b.lastEffect : null;
          if (b !== null) {
            a = b = b.next;
            do {
              var e = a;
              d = e.next;
              e = e.tag;
              (e & 4) !== 0 && (e & 1) !== 0 && (Zi(c, a), $i(c, a));
              a = d;
            } while (a !== b);
          }
          return;
        case 1:
          a = c.stateNode;
          c.flags & 4 && (b === null ? a.componentDidMount() : (d = c.elementType === c.type ? b.memoizedProps : lg(c.type, b.memoizedProps), a.componentDidUpdate(d, b.memoizedState, a.__reactInternalSnapshotBeforeUpdate)));
          b = c.updateQueue;
          b !== null && Eg(c, b, a);
          return;
        case 3:
          b = c.updateQueue;
          if (b !== null) {
            a = null;
            if (c.child !== null)
              switch (c.child.tag) {
                case 5:
                  a = c.child.stateNode;
                  break;
                case 1:
                  a = c.child.stateNode;
              }
            Eg(c, b, a);
          }
          return;
        case 5:
          a = c.stateNode;
          b === null && c.flags & 4 && mf(c.type, c.memoizedProps) && a.focus();
          return;
        case 6:
          return;
        case 4:
          return;
        case 12:
          return;
        case 13:
          c.memoizedState === null && (c = c.alternate, c !== null && (c = c.memoizedState, c !== null && (c = c.dehydrated, c !== null && Cc(c))));
          return;
        case 19:
        case 17:
        case 20:
        case 21:
        case 23:
        case 24:
          return;
      }
      throw Error(y(163));
    }
    function aj(a, b) {
      for (var c = a; ; ) {
        if (c.tag === 5) {
          var d = c.stateNode;
          if (b)
            d = d.style, typeof d.setProperty === "function" ? d.setProperty("display", "none", "important") : d.display = "none";
          else {
            d = c.stateNode;
            var e = c.memoizedProps.style;
            e = e !== void 0 && e !== null && e.hasOwnProperty("display") ? e.display : null;
            d.style.display = sb("display", e);
          }
        } else if (c.tag === 6)
          c.stateNode.nodeValue = b ? "" : c.memoizedProps;
        else if ((c.tag !== 23 && c.tag !== 24 || c.memoizedState === null || c === a) && c.child !== null) {
          c.child.return = c;
          c = c.child;
          continue;
        }
        if (c === a)
          break;
        for (; c.sibling === null; ) {
          if (c.return === null || c.return === a)
            return;
          c = c.return;
        }
        c.sibling.return = c.return;
        c = c.sibling;
      }
    }
    function bj(a, b) {
      if (Mf && typeof Mf.onCommitFiberUnmount === "function")
        try {
          Mf.onCommitFiberUnmount(Lf, b);
        } catch (f) {
        }
      switch (b.tag) {
        case 0:
        case 11:
        case 14:
        case 15:
        case 22:
          a = b.updateQueue;
          if (a !== null && (a = a.lastEffect, a !== null)) {
            var c = a = a.next;
            do {
              var d = c, e = d.destroy;
              d = d.tag;
              if (e !== void 0)
                if ((d & 4) !== 0)
                  Zi(b, c);
                else {
                  d = b;
                  try {
                    e();
                  } catch (f) {
                    Wi(d, f);
                  }
                }
              c = c.next;
            } while (c !== a);
          }
          break;
        case 1:
          Vi(b);
          a = b.stateNode;
          if (typeof a.componentWillUnmount === "function")
            try {
              a.props = b.memoizedProps, a.state = b.memoizedState, a.componentWillUnmount();
            } catch (f) {
              Wi(b, f);
            }
          break;
        case 5:
          Vi(b);
          break;
        case 4:
          cj(a, b);
      }
    }
    function dj(a) {
      a.alternate = null;
      a.child = null;
      a.dependencies = null;
      a.firstEffect = null;
      a.lastEffect = null;
      a.memoizedProps = null;
      a.memoizedState = null;
      a.pendingProps = null;
      a.return = null;
      a.updateQueue = null;
    }
    function ej(a) {
      return a.tag === 5 || a.tag === 3 || a.tag === 4;
    }
    function fj(a) {
      a: {
        for (var b = a.return; b !== null; ) {
          if (ej(b))
            break a;
          b = b.return;
        }
        throw Error(y(160));
      }
      var c = b;
      b = c.stateNode;
      switch (c.tag) {
        case 5:
          var d = false;
          break;
        case 3:
          b = b.containerInfo;
          d = true;
          break;
        case 4:
          b = b.containerInfo;
          d = true;
          break;
        default:
          throw Error(y(161));
      }
      c.flags & 16 && (pb(b, ""), c.flags &= -17);
      a:
        b:
          for (c = a; ; ) {
            for (; c.sibling === null; ) {
              if (c.return === null || ej(c.return)) {
                c = null;
                break a;
              }
              c = c.return;
            }
            c.sibling.return = c.return;
            for (c = c.sibling; c.tag !== 5 && c.tag !== 6 && c.tag !== 18; ) {
              if (c.flags & 2)
                continue b;
              if (c.child === null || c.tag === 4)
                continue b;
              else
                c.child.return = c, c = c.child;
            }
            if (!(c.flags & 2)) {
              c = c.stateNode;
              break a;
            }
          }
      d ? gj(a, c, b) : hj(a, c, b);
    }
    function gj(a, b, c) {
      var d = a.tag, e = d === 5 || d === 6;
      if (e)
        a = e ? a.stateNode : a.stateNode.instance, b ? c.nodeType === 8 ? c.parentNode.insertBefore(a, b) : c.insertBefore(a, b) : (c.nodeType === 8 ? (b = c.parentNode, b.insertBefore(a, c)) : (b = c, b.appendChild(a)), c = c._reactRootContainer, c !== null && c !== void 0 || b.onclick !== null || (b.onclick = jf));
      else if (d !== 4 && (a = a.child, a !== null))
        for (gj(a, b, c), a = a.sibling; a !== null; )
          gj(a, b, c), a = a.sibling;
    }
    function hj(a, b, c) {
      var d = a.tag, e = d === 5 || d === 6;
      if (e)
        a = e ? a.stateNode : a.stateNode.instance, b ? c.insertBefore(a, b) : c.appendChild(a);
      else if (d !== 4 && (a = a.child, a !== null))
        for (hj(a, b, c), a = a.sibling; a !== null; )
          hj(a, b, c), a = a.sibling;
    }
    function cj(a, b) {
      for (var c = b, d = false, e, f; ; ) {
        if (!d) {
          d = c.return;
          a:
            for (; ; ) {
              if (d === null)
                throw Error(y(160));
              e = d.stateNode;
              switch (d.tag) {
                case 5:
                  f = false;
                  break a;
                case 3:
                  e = e.containerInfo;
                  f = true;
                  break a;
                case 4:
                  e = e.containerInfo;
                  f = true;
                  break a;
              }
              d = d.return;
            }
          d = true;
        }
        if (c.tag === 5 || c.tag === 6) {
          a:
            for (var g = a, h = c, k = h; ; )
              if (bj(g, k), k.child !== null && k.tag !== 4)
                k.child.return = k, k = k.child;
              else {
                if (k === h)
                  break a;
                for (; k.sibling === null; ) {
                  if (k.return === null || k.return === h)
                    break a;
                  k = k.return;
                }
                k.sibling.return = k.return;
                k = k.sibling;
              }
          f ? (g = e, h = c.stateNode, g.nodeType === 8 ? g.parentNode.removeChild(h) : g.removeChild(h)) : e.removeChild(c.stateNode);
        } else if (c.tag === 4) {
          if (c.child !== null) {
            e = c.stateNode.containerInfo;
            f = true;
            c.child.return = c;
            c = c.child;
            continue;
          }
        } else if (bj(a, c), c.child !== null) {
          c.child.return = c;
          c = c.child;
          continue;
        }
        if (c === b)
          break;
        for (; c.sibling === null; ) {
          if (c.return === null || c.return === b)
            return;
          c = c.return;
          c.tag === 4 && (d = false);
        }
        c.sibling.return = c.return;
        c = c.sibling;
      }
    }
    function ij(a, b) {
      switch (b.tag) {
        case 0:
        case 11:
        case 14:
        case 15:
        case 22:
          var c = b.updateQueue;
          c = c !== null ? c.lastEffect : null;
          if (c !== null) {
            var d = c = c.next;
            do
              (d.tag & 3) === 3 && (a = d.destroy, d.destroy = void 0, a !== void 0 && a()), d = d.next;
            while (d !== c);
          }
          return;
        case 1:
          return;
        case 5:
          c = b.stateNode;
          if (c != null) {
            d = b.memoizedProps;
            var e = a !== null ? a.memoizedProps : d;
            a = b.type;
            var f = b.updateQueue;
            b.updateQueue = null;
            if (f !== null) {
              c[xf] = d;
              a === "input" && d.type === "radio" && d.name != null && $a(c, d);
              wb(a, e);
              b = wb(a, d);
              for (e = 0; e < f.length; e += 2) {
                var g = f[e], h = f[e + 1];
                g === "style" ? tb(c, h) : g === "dangerouslySetInnerHTML" ? ob(c, h) : g === "children" ? pb(c, h) : qa(c, g, h, b);
              }
              switch (a) {
                case "input":
                  ab(c, d);
                  break;
                case "textarea":
                  ib(c, d);
                  break;
                case "select":
                  a = c._wrapperState.wasMultiple, c._wrapperState.wasMultiple = !!d.multiple, f = d.value, f != null ? fb(c, !!d.multiple, f, false) : a !== !!d.multiple && (d.defaultValue != null ? fb(c, !!d.multiple, d.defaultValue, true) : fb(c, !!d.multiple, d.multiple ? [] : "", false));
              }
            }
          }
          return;
        case 6:
          if (b.stateNode === null)
            throw Error(y(162));
          b.stateNode.nodeValue = b.memoizedProps;
          return;
        case 3:
          c = b.stateNode;
          c.hydrate && (c.hydrate = false, Cc(c.containerInfo));
          return;
        case 12:
          return;
        case 13:
          b.memoizedState !== null && (jj = O(), aj(b.child, true));
          kj(b);
          return;
        case 19:
          kj(b);
          return;
        case 17:
          return;
        case 23:
        case 24:
          aj(b, b.memoizedState !== null);
          return;
      }
      throw Error(y(163));
    }
    function kj(a) {
      var b = a.updateQueue;
      if (b !== null) {
        a.updateQueue = null;
        var c = a.stateNode;
        c === null && (c = a.stateNode = new Ui());
        b.forEach(function(b2) {
          var d = lj.bind(null, a, b2);
          c.has(b2) || (c.add(b2), b2.then(d, d));
        });
      }
    }
    function mj(a, b) {
      return a !== null && (a = a.memoizedState, a === null || a.dehydrated !== null) ? (b = b.memoizedState, b !== null && b.dehydrated === null) : false;
    }
    var nj = Math.ceil;
    var oj = ra.ReactCurrentDispatcher;
    var pj = ra.ReactCurrentOwner;
    var X = 0;
    var U = null;
    var Y = null;
    var W = 0;
    var qj = 0;
    var rj = Bf(0);
    var V = 0;
    var sj = null;
    var tj = 0;
    var Dg = 0;
    var Hi = 0;
    var uj = 0;
    var vj = null;
    var jj = 0;
    var Ji = Infinity;
    function wj() {
      Ji = O() + 500;
    }
    var Z = null;
    var Qi = false;
    var Ri = null;
    var Ti = null;
    var xj = false;
    var yj = null;
    var zj = 90;
    var Aj = [];
    var Bj = [];
    var Cj = null;
    var Dj = 0;
    var Ej = null;
    var Fj = -1;
    var Gj = 0;
    var Hj = 0;
    var Ij = null;
    var Jj = false;
    function Hg() {
      return (X & 48) !== 0 ? O() : Fj !== -1 ? Fj : Fj = O();
    }
    function Ig(a) {
      a = a.mode;
      if ((a & 2) === 0)
        return 1;
      if ((a & 4) === 0)
        return eg() === 99 ? 1 : 2;
      Gj === 0 && (Gj = tj);
      if (kg.transition !== 0) {
        Hj !== 0 && (Hj = vj !== null ? vj.pendingLanes : 0);
        a = Gj;
        var b = 4186112 & ~Hj;
        b &= -b;
        b === 0 && (a = 4186112 & ~a, b = a & -a, b === 0 && (b = 8192));
        return b;
      }
      a = eg();
      (X & 4) !== 0 && a === 98 ? a = Xc(12, Gj) : (a = Sc(a), a = Xc(a, Gj));
      return a;
    }
    function Jg(a, b, c) {
      if (50 < Dj)
        throw Dj = 0, Ej = null, Error(y(185));
      a = Kj(a, b);
      if (a === null)
        return null;
      $c(a, b, c);
      a === U && (Hi |= b, V === 4 && Ii(a, W));
      var d = eg();
      b === 1 ? (X & 8) !== 0 && (X & 48) === 0 ? Lj(a) : (Mj(a, c), X === 0 && (wj(), ig())) : ((X & 4) === 0 || d !== 98 && d !== 99 || (Cj === null ? Cj = new Set([a]) : Cj.add(a)), Mj(a, c));
      vj = a;
    }
    function Kj(a, b) {
      a.lanes |= b;
      var c = a.alternate;
      c !== null && (c.lanes |= b);
      c = a;
      for (a = a.return; a !== null; )
        a.childLanes |= b, c = a.alternate, c !== null && (c.childLanes |= b), c = a, a = a.return;
      return c.tag === 3 ? c.stateNode : null;
    }
    function Mj(a, b) {
      for (var c = a.callbackNode, d = a.suspendedLanes, e = a.pingedLanes, f = a.expirationTimes, g = a.pendingLanes; 0 < g; ) {
        var h = 31 - Vc(g), k = 1 << h, l = f[h];
        if (l === -1) {
          if ((k & d) === 0 || (k & e) !== 0) {
            l = b;
            Rc(k);
            var n = F;
            f[h] = 10 <= n ? l + 250 : 6 <= n ? l + 5e3 : -1;
          }
        } else
          l <= b && (a.expiredLanes |= k);
        g &= ~k;
      }
      d = Uc(a, a === U ? W : 0);
      b = F;
      if (d === 0)
        c !== null && (c !== Zf && Pf(c), a.callbackNode = null, a.callbackPriority = 0);
      else {
        if (c !== null) {
          if (a.callbackPriority === b)
            return;
          c !== Zf && Pf(c);
        }
        b === 15 ? (c = Lj.bind(null, a), ag === null ? (ag = [c], bg = Of(Uf, jg)) : ag.push(c), c = Zf) : b === 14 ? c = hg(99, Lj.bind(null, a)) : (c = Tc(b), c = hg(c, Nj.bind(null, a)));
        a.callbackPriority = b;
        a.callbackNode = c;
      }
    }
    function Nj(a) {
      Fj = -1;
      Hj = Gj = 0;
      if ((X & 48) !== 0)
        throw Error(y(327));
      var b = a.callbackNode;
      if (Oj() && a.callbackNode !== b)
        return null;
      var c = Uc(a, a === U ? W : 0);
      if (c === 0)
        return null;
      var d = c;
      var e = X;
      X |= 16;
      var f = Pj();
      if (U !== a || W !== d)
        wj(), Qj(a, d);
      do
        try {
          Rj();
          break;
        } catch (h) {
          Sj(a, h);
        }
      while (1);
      qg();
      oj.current = f;
      X = e;
      Y !== null ? d = 0 : (U = null, W = 0, d = V);
      if ((tj & Hi) !== 0)
        Qj(a, 0);
      else if (d !== 0) {
        d === 2 && (X |= 64, a.hydrate && (a.hydrate = false, qf(a.containerInfo)), c = Wc(a), c !== 0 && (d = Tj(a, c)));
        if (d === 1)
          throw b = sj, Qj(a, 0), Ii(a, c), Mj(a, O()), b;
        a.finishedWork = a.current.alternate;
        a.finishedLanes = c;
        switch (d) {
          case 0:
          case 1:
            throw Error(y(345));
          case 2:
            Uj(a);
            break;
          case 3:
            Ii(a, c);
            if ((c & 62914560) === c && (d = jj + 500 - O(), 10 < d)) {
              if (Uc(a, 0) !== 0)
                break;
              e = a.suspendedLanes;
              if ((e & c) !== c) {
                Hg();
                a.pingedLanes |= a.suspendedLanes & e;
                break;
              }
              a.timeoutHandle = of(Uj.bind(null, a), d);
              break;
            }
            Uj(a);
            break;
          case 4:
            Ii(a, c);
            if ((c & 4186112) === c)
              break;
            d = a.eventTimes;
            for (e = -1; 0 < c; ) {
              var g = 31 - Vc(c);
              f = 1 << g;
              g = d[g];
              g > e && (e = g);
              c &= ~f;
            }
            c = e;
            c = O() - c;
            c = (120 > c ? 120 : 480 > c ? 480 : 1080 > c ? 1080 : 1920 > c ? 1920 : 3e3 > c ? 3e3 : 4320 > c ? 4320 : 1960 * nj(c / 1960)) - c;
            if (10 < c) {
              a.timeoutHandle = of(Uj.bind(null, a), c);
              break;
            }
            Uj(a);
            break;
          case 5:
            Uj(a);
            break;
          default:
            throw Error(y(329));
        }
      }
      Mj(a, O());
      return a.callbackNode === b ? Nj.bind(null, a) : null;
    }
    function Ii(a, b) {
      b &= ~uj;
      b &= ~Hi;
      a.suspendedLanes |= b;
      a.pingedLanes &= ~b;
      for (a = a.expirationTimes; 0 < b; ) {
        var c = 31 - Vc(b), d = 1 << c;
        a[c] = -1;
        b &= ~d;
      }
    }
    function Lj(a) {
      if ((X & 48) !== 0)
        throw Error(y(327));
      Oj();
      if (a === U && (a.expiredLanes & W) !== 0) {
        var b = W;
        var c = Tj(a, b);
        (tj & Hi) !== 0 && (b = Uc(a, b), c = Tj(a, b));
      } else
        b = Uc(a, 0), c = Tj(a, b);
      a.tag !== 0 && c === 2 && (X |= 64, a.hydrate && (a.hydrate = false, qf(a.containerInfo)), b = Wc(a), b !== 0 && (c = Tj(a, b)));
      if (c === 1)
        throw c = sj, Qj(a, 0), Ii(a, b), Mj(a, O()), c;
      a.finishedWork = a.current.alternate;
      a.finishedLanes = b;
      Uj(a);
      Mj(a, O());
      return null;
    }
    function Vj() {
      if (Cj !== null) {
        var a = Cj;
        Cj = null;
        a.forEach(function(a2) {
          a2.expiredLanes |= 24 & a2.pendingLanes;
          Mj(a2, O());
        });
      }
      ig();
    }
    function Wj(a, b) {
      var c = X;
      X |= 1;
      try {
        return a(b);
      } finally {
        X = c, X === 0 && (wj(), ig());
      }
    }
    function Xj(a, b) {
      var c = X;
      X &= -2;
      X |= 8;
      try {
        return a(b);
      } finally {
        X = c, X === 0 && (wj(), ig());
      }
    }
    function ni(a, b) {
      I(rj, qj);
      qj |= b;
      tj |= b;
    }
    function Ki() {
      qj = rj.current;
      H(rj);
    }
    function Qj(a, b) {
      a.finishedWork = null;
      a.finishedLanes = 0;
      var c = a.timeoutHandle;
      c !== -1 && (a.timeoutHandle = -1, pf(c));
      if (Y !== null)
        for (c = Y.return; c !== null; ) {
          var d = c;
          switch (d.tag) {
            case 1:
              d = d.type.childContextTypes;
              d !== null && d !== void 0 && Gf();
              break;
            case 3:
              fh();
              H(N);
              H(M);
              uh();
              break;
            case 5:
              hh(d);
              break;
            case 4:
              fh();
              break;
            case 13:
              H(P);
              break;
            case 19:
              H(P);
              break;
            case 10:
              rg(d);
              break;
            case 23:
            case 24:
              Ki();
          }
          c = c.return;
        }
      U = a;
      Y = Tg(a.current, null);
      W = qj = tj = b;
      V = 0;
      sj = null;
      uj = Hi = Dg = 0;
    }
    function Sj(a, b) {
      do {
        var c = Y;
        try {
          qg();
          vh.current = Gh;
          if (yh) {
            for (var d = R.memoizedState; d !== null; ) {
              var e = d.queue;
              e !== null && (e.pending = null);
              d = d.next;
            }
            yh = false;
          }
          xh = 0;
          T = S = R = null;
          zh = false;
          pj.current = null;
          if (c === null || c.return === null) {
            V = 1;
            sj = b;
            Y = null;
            break;
          }
          a: {
            var f = a, g = c.return, h = c, k = b;
            b = W;
            h.flags |= 2048;
            h.firstEffect = h.lastEffect = null;
            if (k !== null && typeof k === "object" && typeof k.then === "function") {
              var l = k;
              if ((h.mode & 2) === 0) {
                var n = h.alternate;
                n ? (h.updateQueue = n.updateQueue, h.memoizedState = n.memoizedState, h.lanes = n.lanes) : (h.updateQueue = null, h.memoizedState = null);
              }
              var A = (P.current & 1) !== 0, p = g;
              do {
                var C;
                if (C = p.tag === 13) {
                  var x = p.memoizedState;
                  if (x !== null)
                    C = x.dehydrated !== null ? true : false;
                  else {
                    var w = p.memoizedProps;
                    C = w.fallback === void 0 ? false : w.unstable_avoidThisFallback !== true ? true : A ? false : true;
                  }
                }
                if (C) {
                  var z = p.updateQueue;
                  if (z === null) {
                    var u = new Set();
                    u.add(l);
                    p.updateQueue = u;
                  } else
                    z.add(l);
                  if ((p.mode & 2) === 0) {
                    p.flags |= 64;
                    h.flags |= 16384;
                    h.flags &= -2981;
                    if (h.tag === 1)
                      if (h.alternate === null)
                        h.tag = 17;
                      else {
                        var t2 = zg(-1, 1);
                        t2.tag = 2;
                        Ag(h, t2);
                      }
                    h.lanes |= 1;
                    break a;
                  }
                  k = void 0;
                  h = b;
                  var q = f.pingCache;
                  q === null ? (q = f.pingCache = new Oi(), k = new Set(), q.set(l, k)) : (k = q.get(l), k === void 0 && (k = new Set(), q.set(l, k)));
                  if (!k.has(h)) {
                    k.add(h);
                    var v = Yj.bind(null, f, l, h);
                    l.then(v, v);
                  }
                  p.flags |= 4096;
                  p.lanes = b;
                  break a;
                }
                p = p.return;
              } while (p !== null);
              k = Error((Ra(h.type) || "A React component") + " suspended while rendering, but no fallback UI was specified.\n\nAdd a <Suspense fallback=...> component higher in the tree to provide a loading indicator or placeholder to display.");
            }
            V !== 5 && (V = 2);
            k = Mi(k, h);
            p = g;
            do {
              switch (p.tag) {
                case 3:
                  f = k;
                  p.flags |= 4096;
                  b &= -b;
                  p.lanes |= b;
                  var J = Pi(p, f, b);
                  Bg(p, J);
                  break a;
                case 1:
                  f = k;
                  var K = p.type, Q = p.stateNode;
                  if ((p.flags & 64) === 0 && (typeof K.getDerivedStateFromError === "function" || Q !== null && typeof Q.componentDidCatch === "function" && (Ti === null || !Ti.has(Q)))) {
                    p.flags |= 4096;
                    b &= -b;
                    p.lanes |= b;
                    var L = Si(p, f, b);
                    Bg(p, L);
                    break a;
                  }
              }
              p = p.return;
            } while (p !== null);
          }
          Zj(c);
        } catch (va) {
          b = va;
          Y === c && c !== null && (Y = c = c.return);
          continue;
        }
        break;
      } while (1);
    }
    function Pj() {
      var a = oj.current;
      oj.current = Gh;
      return a === null ? Gh : a;
    }
    function Tj(a, b) {
      var c = X;
      X |= 16;
      var d = Pj();
      U === a && W === b || Qj(a, b);
      do
        try {
          ak();
          break;
        } catch (e) {
          Sj(a, e);
        }
      while (1);
      qg();
      X = c;
      oj.current = d;
      if (Y !== null)
        throw Error(y(261));
      U = null;
      W = 0;
      return V;
    }
    function ak() {
      for (; Y !== null; )
        bk(Y);
    }
    function Rj() {
      for (; Y !== null && !Qf(); )
        bk(Y);
    }
    function bk(a) {
      var b = ck(a.alternate, a, qj);
      a.memoizedProps = a.pendingProps;
      b === null ? Zj(a) : Y = b;
      pj.current = null;
    }
    function Zj(a) {
      var b = a;
      do {
        var c = b.alternate;
        a = b.return;
        if ((b.flags & 2048) === 0) {
          c = Gi(c, b, qj);
          if (c !== null) {
            Y = c;
            return;
          }
          c = b;
          if (c.tag !== 24 && c.tag !== 23 || c.memoizedState === null || (qj & 1073741824) !== 0 || (c.mode & 4) === 0) {
            for (var d = 0, e = c.child; e !== null; )
              d |= e.lanes | e.childLanes, e = e.sibling;
            c.childLanes = d;
          }
          a !== null && (a.flags & 2048) === 0 && (a.firstEffect === null && (a.firstEffect = b.firstEffect), b.lastEffect !== null && (a.lastEffect !== null && (a.lastEffect.nextEffect = b.firstEffect), a.lastEffect = b.lastEffect), 1 < b.flags && (a.lastEffect !== null ? a.lastEffect.nextEffect = b : a.firstEffect = b, a.lastEffect = b));
        } else {
          c = Li(b);
          if (c !== null) {
            c.flags &= 2047;
            Y = c;
            return;
          }
          a !== null && (a.firstEffect = a.lastEffect = null, a.flags |= 2048);
        }
        b = b.sibling;
        if (b !== null) {
          Y = b;
          return;
        }
        Y = b = a;
      } while (b !== null);
      V === 0 && (V = 5);
    }
    function Uj(a) {
      var b = eg();
      gg(99, dk.bind(null, a, b));
      return null;
    }
    function dk(a, b) {
      do
        Oj();
      while (yj !== null);
      if ((X & 48) !== 0)
        throw Error(y(327));
      var c = a.finishedWork;
      if (c === null)
        return null;
      a.finishedWork = null;
      a.finishedLanes = 0;
      if (c === a.current)
        throw Error(y(177));
      a.callbackNode = null;
      var d = c.lanes | c.childLanes, e = d, f = a.pendingLanes & ~e;
      a.pendingLanes = e;
      a.suspendedLanes = 0;
      a.pingedLanes = 0;
      a.expiredLanes &= e;
      a.mutableReadLanes &= e;
      a.entangledLanes &= e;
      e = a.entanglements;
      for (var g = a.eventTimes, h = a.expirationTimes; 0 < f; ) {
        var k = 31 - Vc(f), l = 1 << k;
        e[k] = 0;
        g[k] = -1;
        h[k] = -1;
        f &= ~l;
      }
      Cj !== null && (d & 24) === 0 && Cj.has(a) && Cj.delete(a);
      a === U && (Y = U = null, W = 0);
      1 < c.flags ? c.lastEffect !== null ? (c.lastEffect.nextEffect = c, d = c.firstEffect) : d = c : d = c.firstEffect;
      if (d !== null) {
        e = X;
        X |= 32;
        pj.current = null;
        kf = fd;
        g = Ne();
        if (Oe(g)) {
          if ("selectionStart" in g)
            h = {start: g.selectionStart, end: g.selectionEnd};
          else
            a:
              if (h = (h = g.ownerDocument) && h.defaultView || window, (l = h.getSelection && h.getSelection()) && l.rangeCount !== 0) {
                h = l.anchorNode;
                f = l.anchorOffset;
                k = l.focusNode;
                l = l.focusOffset;
                try {
                  h.nodeType, k.nodeType;
                } catch (va) {
                  h = null;
                  break a;
                }
                var n = 0, A = -1, p = -1, C = 0, x = 0, w = g, z = null;
                b:
                  for (; ; ) {
                    for (var u; ; ) {
                      w !== h || f !== 0 && w.nodeType !== 3 || (A = n + f);
                      w !== k || l !== 0 && w.nodeType !== 3 || (p = n + l);
                      w.nodeType === 3 && (n += w.nodeValue.length);
                      if ((u = w.firstChild) === null)
                        break;
                      z = w;
                      w = u;
                    }
                    for (; ; ) {
                      if (w === g)
                        break b;
                      z === h && ++C === f && (A = n);
                      z === k && ++x === l && (p = n);
                      if ((u = w.nextSibling) !== null)
                        break;
                      w = z;
                      z = w.parentNode;
                    }
                    w = u;
                  }
                h = A === -1 || p === -1 ? null : {start: A, end: p};
              } else
                h = null;
          h = h || {start: 0, end: 0};
        } else
          h = null;
        lf = {focusedElem: g, selectionRange: h};
        fd = false;
        Ij = null;
        Jj = false;
        Z = d;
        do
          try {
            ek();
          } catch (va) {
            if (Z === null)
              throw Error(y(330));
            Wi(Z, va);
            Z = Z.nextEffect;
          }
        while (Z !== null);
        Ij = null;
        Z = d;
        do
          try {
            for (g = a; Z !== null; ) {
              var t2 = Z.flags;
              t2 & 16 && pb(Z.stateNode, "");
              if (t2 & 128) {
                var q = Z.alternate;
                if (q !== null) {
                  var v = q.ref;
                  v !== null && (typeof v === "function" ? v(null) : v.current = null);
                }
              }
              switch (t2 & 1038) {
                case 2:
                  fj(Z);
                  Z.flags &= -3;
                  break;
                case 6:
                  fj(Z);
                  Z.flags &= -3;
                  ij(Z.alternate, Z);
                  break;
                case 1024:
                  Z.flags &= -1025;
                  break;
                case 1028:
                  Z.flags &= -1025;
                  ij(Z.alternate, Z);
                  break;
                case 4:
                  ij(Z.alternate, Z);
                  break;
                case 8:
                  h = Z;
                  cj(g, h);
                  var J = h.alternate;
                  dj(h);
                  J !== null && dj(J);
              }
              Z = Z.nextEffect;
            }
          } catch (va) {
            if (Z === null)
              throw Error(y(330));
            Wi(Z, va);
            Z = Z.nextEffect;
          }
        while (Z !== null);
        v = lf;
        q = Ne();
        t2 = v.focusedElem;
        g = v.selectionRange;
        if (q !== t2 && t2 && t2.ownerDocument && Me(t2.ownerDocument.documentElement, t2)) {
          g !== null && Oe(t2) && (q = g.start, v = g.end, v === void 0 && (v = q), "selectionStart" in t2 ? (t2.selectionStart = q, t2.selectionEnd = Math.min(v, t2.value.length)) : (v = (q = t2.ownerDocument || document) && q.defaultView || window, v.getSelection && (v = v.getSelection(), h = t2.textContent.length, J = Math.min(g.start, h), g = g.end === void 0 ? J : Math.min(g.end, h), !v.extend && J > g && (h = g, g = J, J = h), h = Le(t2, J), f = Le(t2, g), h && f && (v.rangeCount !== 1 || v.anchorNode !== h.node || v.anchorOffset !== h.offset || v.focusNode !== f.node || v.focusOffset !== f.offset) && (q = q.createRange(), q.setStart(h.node, h.offset), v.removeAllRanges(), J > g ? (v.addRange(q), v.extend(f.node, f.offset)) : (q.setEnd(f.node, f.offset), v.addRange(q))))));
          q = [];
          for (v = t2; v = v.parentNode; )
            v.nodeType === 1 && q.push({element: v, left: v.scrollLeft, top: v.scrollTop});
          typeof t2.focus === "function" && t2.focus();
          for (t2 = 0; t2 < q.length; t2++)
            v = q[t2], v.element.scrollLeft = v.left, v.element.scrollTop = v.top;
        }
        fd = !!kf;
        lf = kf = null;
        a.current = c;
        Z = d;
        do
          try {
            for (t2 = a; Z !== null; ) {
              var K = Z.flags;
              K & 36 && Yi(t2, Z.alternate, Z);
              if (K & 128) {
                q = void 0;
                var Q = Z.ref;
                if (Q !== null) {
                  var L = Z.stateNode;
                  switch (Z.tag) {
                    case 5:
                      q = L;
                      break;
                    default:
                      q = L;
                  }
                  typeof Q === "function" ? Q(q) : Q.current = q;
                }
              }
              Z = Z.nextEffect;
            }
          } catch (va) {
            if (Z === null)
              throw Error(y(330));
            Wi(Z, va);
            Z = Z.nextEffect;
          }
        while (Z !== null);
        Z = null;
        $f();
        X = e;
      } else
        a.current = c;
      if (xj)
        xj = false, yj = a, zj = b;
      else
        for (Z = d; Z !== null; )
          b = Z.nextEffect, Z.nextEffect = null, Z.flags & 8 && (K = Z, K.sibling = null, K.stateNode = null), Z = b;
      d = a.pendingLanes;
      d === 0 && (Ti = null);
      d === 1 ? a === Ej ? Dj++ : (Dj = 0, Ej = a) : Dj = 0;
      c = c.stateNode;
      if (Mf && typeof Mf.onCommitFiberRoot === "function")
        try {
          Mf.onCommitFiberRoot(Lf, c, void 0, (c.current.flags & 64) === 64);
        } catch (va) {
        }
      Mj(a, O());
      if (Qi)
        throw Qi = false, a = Ri, Ri = null, a;
      if ((X & 8) !== 0)
        return null;
      ig();
      return null;
    }
    function ek() {
      for (; Z !== null; ) {
        var a = Z.alternate;
        Jj || Ij === null || ((Z.flags & 8) !== 0 ? dc(Z, Ij) && (Jj = true) : Z.tag === 13 && mj(a, Z) && dc(Z, Ij) && (Jj = true));
        var b = Z.flags;
        (b & 256) !== 0 && Xi(a, Z);
        (b & 512) === 0 || xj || (xj = true, hg(97, function() {
          Oj();
          return null;
        }));
        Z = Z.nextEffect;
      }
    }
    function Oj() {
      if (zj !== 90) {
        var a = 97 < zj ? 97 : zj;
        zj = 90;
        return gg(a, fk);
      }
      return false;
    }
    function $i(a, b) {
      Aj.push(b, a);
      xj || (xj = true, hg(97, function() {
        Oj();
        return null;
      }));
    }
    function Zi(a, b) {
      Bj.push(b, a);
      xj || (xj = true, hg(97, function() {
        Oj();
        return null;
      }));
    }
    function fk() {
      if (yj === null)
        return false;
      var a = yj;
      yj = null;
      if ((X & 48) !== 0)
        throw Error(y(331));
      var b = X;
      X |= 32;
      var c = Bj;
      Bj = [];
      for (var d = 0; d < c.length; d += 2) {
        var e = c[d], f = c[d + 1], g = e.destroy;
        e.destroy = void 0;
        if (typeof g === "function")
          try {
            g();
          } catch (k) {
            if (f === null)
              throw Error(y(330));
            Wi(f, k);
          }
      }
      c = Aj;
      Aj = [];
      for (d = 0; d < c.length; d += 2) {
        e = c[d];
        f = c[d + 1];
        try {
          var h = e.create;
          e.destroy = h();
        } catch (k) {
          if (f === null)
            throw Error(y(330));
          Wi(f, k);
        }
      }
      for (h = a.current.firstEffect; h !== null; )
        a = h.nextEffect, h.nextEffect = null, h.flags & 8 && (h.sibling = null, h.stateNode = null), h = a;
      X = b;
      ig();
      return true;
    }
    function gk(a, b, c) {
      b = Mi(c, b);
      b = Pi(a, b, 1);
      Ag(a, b);
      b = Hg();
      a = Kj(a, 1);
      a !== null && ($c(a, 1, b), Mj(a, b));
    }
    function Wi(a, b) {
      if (a.tag === 3)
        gk(a, a, b);
      else
        for (var c = a.return; c !== null; ) {
          if (c.tag === 3) {
            gk(c, a, b);
            break;
          } else if (c.tag === 1) {
            var d = c.stateNode;
            if (typeof c.type.getDerivedStateFromError === "function" || typeof d.componentDidCatch === "function" && (Ti === null || !Ti.has(d))) {
              a = Mi(b, a);
              var e = Si(c, a, 1);
              Ag(c, e);
              e = Hg();
              c = Kj(c, 1);
              if (c !== null)
                $c(c, 1, e), Mj(c, e);
              else if (typeof d.componentDidCatch === "function" && (Ti === null || !Ti.has(d)))
                try {
                  d.componentDidCatch(b, a);
                } catch (f) {
                }
              break;
            }
          }
          c = c.return;
        }
    }
    function Yj(a, b, c) {
      var d = a.pingCache;
      d !== null && d.delete(b);
      b = Hg();
      a.pingedLanes |= a.suspendedLanes & c;
      U === a && (W & c) === c && (V === 4 || V === 3 && (W & 62914560) === W && 500 > O() - jj ? Qj(a, 0) : uj |= c);
      Mj(a, b);
    }
    function lj(a, b) {
      var c = a.stateNode;
      c !== null && c.delete(b);
      b = 0;
      b === 0 && (b = a.mode, (b & 2) === 0 ? b = 1 : (b & 4) === 0 ? b = eg() === 99 ? 1 : 2 : (Gj === 0 && (Gj = tj), b = Yc(62914560 & ~Gj), b === 0 && (b = 4194304)));
      c = Hg();
      a = Kj(a, b);
      a !== null && ($c(a, b, c), Mj(a, c));
    }
    var ck;
    ck = function(a, b, c) {
      var d = b.lanes;
      if (a !== null)
        if (a.memoizedProps !== b.pendingProps || N.current)
          ug = true;
        else if ((c & d) !== 0)
          ug = (a.flags & 16384) !== 0 ? true : false;
        else {
          ug = false;
          switch (b.tag) {
            case 3:
              ri(b);
              sh();
              break;
            case 5:
              gh(b);
              break;
            case 1:
              Ff(b.type) && Jf(b);
              break;
            case 4:
              eh(b, b.stateNode.containerInfo);
              break;
            case 10:
              d = b.memoizedProps.value;
              var e = b.type._context;
              I(mg, e._currentValue);
              e._currentValue = d;
              break;
            case 13:
              if (b.memoizedState !== null) {
                if ((c & b.child.childLanes) !== 0)
                  return ti(a, b, c);
                I(P, P.current & 1);
                b = hi(a, b, c);
                return b !== null ? b.sibling : null;
              }
              I(P, P.current & 1);
              break;
            case 19:
              d = (c & b.childLanes) !== 0;
              if ((a.flags & 64) !== 0) {
                if (d)
                  return Ai(a, b, c);
                b.flags |= 64;
              }
              e = b.memoizedState;
              e !== null && (e.rendering = null, e.tail = null, e.lastEffect = null);
              I(P, P.current);
              if (d)
                break;
              else
                return null;
            case 23:
            case 24:
              return b.lanes = 0, mi(a, b, c);
          }
          return hi(a, b, c);
        }
      else
        ug = false;
      b.lanes = 0;
      switch (b.tag) {
        case 2:
          d = b.type;
          a !== null && (a.alternate = null, b.alternate = null, b.flags |= 2);
          a = b.pendingProps;
          e = Ef(b, M.current);
          tg(b, c);
          e = Ch(null, b, d, a, e, c);
          b.flags |= 1;
          if (typeof e === "object" && e !== null && typeof e.render === "function" && e.$$typeof === void 0) {
            b.tag = 1;
            b.memoizedState = null;
            b.updateQueue = null;
            if (Ff(d)) {
              var f = true;
              Jf(b);
            } else
              f = false;
            b.memoizedState = e.state !== null && e.state !== void 0 ? e.state : null;
            xg(b);
            var g = d.getDerivedStateFromProps;
            typeof g === "function" && Gg(b, d, g, a);
            e.updater = Kg;
            b.stateNode = e;
            e._reactInternals = b;
            Og(b, d, a, c);
            b = qi(null, b, d, true, f, c);
          } else
            b.tag = 0, fi(null, b, e, c), b = b.child;
          return b;
        case 16:
          e = b.elementType;
          a: {
            a !== null && (a.alternate = null, b.alternate = null, b.flags |= 2);
            a = b.pendingProps;
            f = e._init;
            e = f(e._payload);
            b.type = e;
            f = b.tag = hk(e);
            a = lg(e, a);
            switch (f) {
              case 0:
                b = li(null, b, e, a, c);
                break a;
              case 1:
                b = pi(null, b, e, a, c);
                break a;
              case 11:
                b = gi(null, b, e, a, c);
                break a;
              case 14:
                b = ii(null, b, e, lg(e.type, a), d, c);
                break a;
            }
            throw Error(y(306, e, ""));
          }
          return b;
        case 0:
          return d = b.type, e = b.pendingProps, e = b.elementType === d ? e : lg(d, e), li(a, b, d, e, c);
        case 1:
          return d = b.type, e = b.pendingProps, e = b.elementType === d ? e : lg(d, e), pi(a, b, d, e, c);
        case 3:
          ri(b);
          d = b.updateQueue;
          if (a === null || d === null)
            throw Error(y(282));
          d = b.pendingProps;
          e = b.memoizedState;
          e = e !== null ? e.element : null;
          yg(a, b);
          Cg(b, d, null, c);
          d = b.memoizedState.element;
          if (d === e)
            sh(), b = hi(a, b, c);
          else {
            e = b.stateNode;
            if (f = e.hydrate)
              kh = rf(b.stateNode.containerInfo.firstChild), jh = b, f = lh = true;
            if (f) {
              a = e.mutableSourceEagerHydrationData;
              if (a != null)
                for (e = 0; e < a.length; e += 2)
                  f = a[e], f._workInProgressVersionPrimary = a[e + 1], th.push(f);
              c = Zg(b, null, d, c);
              for (b.child = c; c; )
                c.flags = c.flags & -3 | 1024, c = c.sibling;
            } else
              fi(a, b, d, c), sh();
            b = b.child;
          }
          return b;
        case 5:
          return gh(b), a === null && ph(b), d = b.type, e = b.pendingProps, f = a !== null ? a.memoizedProps : null, g = e.children, nf(d, e) ? g = null : f !== null && nf(d, f) && (b.flags |= 16), oi(a, b), fi(a, b, g, c), b.child;
        case 6:
          return a === null && ph(b), null;
        case 13:
          return ti(a, b, c);
        case 4:
          return eh(b, b.stateNode.containerInfo), d = b.pendingProps, a === null ? b.child = Yg(b, null, d, c) : fi(a, b, d, c), b.child;
        case 11:
          return d = b.type, e = b.pendingProps, e = b.elementType === d ? e : lg(d, e), gi(a, b, d, e, c);
        case 7:
          return fi(a, b, b.pendingProps, c), b.child;
        case 8:
          return fi(a, b, b.pendingProps.children, c), b.child;
        case 12:
          return fi(a, b, b.pendingProps.children, c), b.child;
        case 10:
          a: {
            d = b.type._context;
            e = b.pendingProps;
            g = b.memoizedProps;
            f = e.value;
            var h = b.type._context;
            I(mg, h._currentValue);
            h._currentValue = f;
            if (g !== null)
              if (h = g.value, f = He(h, f) ? 0 : (typeof d._calculateChangedBits === "function" ? d._calculateChangedBits(h, f) : 1073741823) | 0, f === 0) {
                if (g.children === e.children && !N.current) {
                  b = hi(a, b, c);
                  break a;
                }
              } else
                for (h = b.child, h !== null && (h.return = b); h !== null; ) {
                  var k = h.dependencies;
                  if (k !== null) {
                    g = h.child;
                    for (var l = k.firstContext; l !== null; ) {
                      if (l.context === d && (l.observedBits & f) !== 0) {
                        h.tag === 1 && (l = zg(-1, c & -c), l.tag = 2, Ag(h, l));
                        h.lanes |= c;
                        l = h.alternate;
                        l !== null && (l.lanes |= c);
                        sg(h.return, c);
                        k.lanes |= c;
                        break;
                      }
                      l = l.next;
                    }
                  } else
                    g = h.tag === 10 ? h.type === b.type ? null : h.child : h.child;
                  if (g !== null)
                    g.return = h;
                  else
                    for (g = h; g !== null; ) {
                      if (g === b) {
                        g = null;
                        break;
                      }
                      h = g.sibling;
                      if (h !== null) {
                        h.return = g.return;
                        g = h;
                        break;
                      }
                      g = g.return;
                    }
                  h = g;
                }
            fi(a, b, e.children, c);
            b = b.child;
          }
          return b;
        case 9:
          return e = b.type, f = b.pendingProps, d = f.children, tg(b, c), e = vg(e, f.unstable_observedBits), d = d(e), b.flags |= 1, fi(a, b, d, c), b.child;
        case 14:
          return e = b.type, f = lg(e, b.pendingProps), f = lg(e.type, f), ii(a, b, e, f, d, c);
        case 15:
          return ki(a, b, b.type, b.pendingProps, d, c);
        case 17:
          return d = b.type, e = b.pendingProps, e = b.elementType === d ? e : lg(d, e), a !== null && (a.alternate = null, b.alternate = null, b.flags |= 2), b.tag = 1, Ff(d) ? (a = true, Jf(b)) : a = false, tg(b, c), Mg(b, d, e), Og(b, d, e, c), qi(null, b, d, true, a, c);
        case 19:
          return Ai(a, b, c);
        case 23:
          return mi(a, b, c);
        case 24:
          return mi(a, b, c);
      }
      throw Error(y(156, b.tag));
    };
    function ik(a, b, c, d) {
      this.tag = a;
      this.key = c;
      this.sibling = this.child = this.return = this.stateNode = this.type = this.elementType = null;
      this.index = 0;
      this.ref = null;
      this.pendingProps = b;
      this.dependencies = this.memoizedState = this.updateQueue = this.memoizedProps = null;
      this.mode = d;
      this.flags = 0;
      this.lastEffect = this.firstEffect = this.nextEffect = null;
      this.childLanes = this.lanes = 0;
      this.alternate = null;
    }
    function nh(a, b, c, d) {
      return new ik(a, b, c, d);
    }
    function ji(a) {
      a = a.prototype;
      return !(!a || !a.isReactComponent);
    }
    function hk(a) {
      if (typeof a === "function")
        return ji(a) ? 1 : 0;
      if (a !== void 0 && a !== null) {
        a = a.$$typeof;
        if (a === Aa)
          return 11;
        if (a === Da)
          return 14;
      }
      return 2;
    }
    function Tg(a, b) {
      var c = a.alternate;
      c === null ? (c = nh(a.tag, b, a.key, a.mode), c.elementType = a.elementType, c.type = a.type, c.stateNode = a.stateNode, c.alternate = a, a.alternate = c) : (c.pendingProps = b, c.type = a.type, c.flags = 0, c.nextEffect = null, c.firstEffect = null, c.lastEffect = null);
      c.childLanes = a.childLanes;
      c.lanes = a.lanes;
      c.child = a.child;
      c.memoizedProps = a.memoizedProps;
      c.memoizedState = a.memoizedState;
      c.updateQueue = a.updateQueue;
      b = a.dependencies;
      c.dependencies = b === null ? null : {lanes: b.lanes, firstContext: b.firstContext};
      c.sibling = a.sibling;
      c.index = a.index;
      c.ref = a.ref;
      return c;
    }
    function Vg(a, b, c, d, e, f) {
      var g = 2;
      d = a;
      if (typeof a === "function")
        ji(a) && (g = 1);
      else if (typeof a === "string")
        g = 5;
      else
        a:
          switch (a) {
            case ua:
              return Xg(c.children, e, f, b);
            case Ha:
              g = 8;
              e |= 16;
              break;
            case wa:
              g = 8;
              e |= 1;
              break;
            case xa:
              return a = nh(12, c, b, e | 8), a.elementType = xa, a.type = xa, a.lanes = f, a;
            case Ba:
              return a = nh(13, c, b, e), a.type = Ba, a.elementType = Ba, a.lanes = f, a;
            case Ca:
              return a = nh(19, c, b, e), a.elementType = Ca, a.lanes = f, a;
            case Ia:
              return vi(c, e, f, b);
            case Ja:
              return a = nh(24, c, b, e), a.elementType = Ja, a.lanes = f, a;
            default:
              if (typeof a === "object" && a !== null)
                switch (a.$$typeof) {
                  case ya:
                    g = 10;
                    break a;
                  case za:
                    g = 9;
                    break a;
                  case Aa:
                    g = 11;
                    break a;
                  case Da:
                    g = 14;
                    break a;
                  case Ea:
                    g = 16;
                    d = null;
                    break a;
                  case Fa:
                    g = 22;
                    break a;
                }
              throw Error(y(130, a == null ? a : typeof a, ""));
          }
      b = nh(g, c, b, e);
      b.elementType = a;
      b.type = d;
      b.lanes = f;
      return b;
    }
    function Xg(a, b, c, d) {
      a = nh(7, a, d, b);
      a.lanes = c;
      return a;
    }
    function vi(a, b, c, d) {
      a = nh(23, a, d, b);
      a.elementType = Ia;
      a.lanes = c;
      return a;
    }
    function Ug(a, b, c) {
      a = nh(6, a, null, b);
      a.lanes = c;
      return a;
    }
    function Wg(a, b, c) {
      b = nh(4, a.children !== null ? a.children : [], a.key, b);
      b.lanes = c;
      b.stateNode = {containerInfo: a.containerInfo, pendingChildren: null, implementation: a.implementation};
      return b;
    }
    function jk(a, b, c) {
      this.tag = b;
      this.containerInfo = a;
      this.finishedWork = this.pingCache = this.current = this.pendingChildren = null;
      this.timeoutHandle = -1;
      this.pendingContext = this.context = null;
      this.hydrate = c;
      this.callbackNode = null;
      this.callbackPriority = 0;
      this.eventTimes = Zc(0);
      this.expirationTimes = Zc(-1);
      this.entangledLanes = this.finishedLanes = this.mutableReadLanes = this.expiredLanes = this.pingedLanes = this.suspendedLanes = this.pendingLanes = 0;
      this.entanglements = Zc(0);
      this.mutableSourceEagerHydrationData = null;
    }
    function kk(a, b, c) {
      var d = 3 < arguments.length && arguments[3] !== void 0 ? arguments[3] : null;
      return {$$typeof: ta, key: d == null ? null : "" + d, children: a, containerInfo: b, implementation: c};
    }
    function lk(a, b, c, d) {
      var e = b.current, f = Hg(), g = Ig(e);
      a:
        if (c) {
          c = c._reactInternals;
          b: {
            if (Zb(c) !== c || c.tag !== 1)
              throw Error(y(170));
            var h = c;
            do {
              switch (h.tag) {
                case 3:
                  h = h.stateNode.context;
                  break b;
                case 1:
                  if (Ff(h.type)) {
                    h = h.stateNode.__reactInternalMemoizedMergedChildContext;
                    break b;
                  }
              }
              h = h.return;
            } while (h !== null);
            throw Error(y(171));
          }
          if (c.tag === 1) {
            var k = c.type;
            if (Ff(k)) {
              c = If(c, k, h);
              break a;
            }
          }
          c = h;
        } else
          c = Cf;
      b.context === null ? b.context = c : b.pendingContext = c;
      b = zg(f, g);
      b.payload = {element: a};
      d = d === void 0 ? null : d;
      d !== null && (b.callback = d);
      Ag(e, b);
      Jg(e, g, f);
      return g;
    }
    function mk(a) {
      a = a.current;
      if (!a.child)
        return null;
      switch (a.child.tag) {
        case 5:
          return a.child.stateNode;
        default:
          return a.child.stateNode;
      }
    }
    function nk(a, b) {
      a = a.memoizedState;
      if (a !== null && a.dehydrated !== null) {
        var c = a.retryLane;
        a.retryLane = c !== 0 && c < b ? c : b;
      }
    }
    function ok(a, b) {
      nk(a, b);
      (a = a.alternate) && nk(a, b);
    }
    function pk() {
      return null;
    }
    function qk(a, b, c) {
      var d = c != null && c.hydrationOptions != null && c.hydrationOptions.mutableSources || null;
      c = new jk(a, b, c != null && c.hydrate === true);
      b = nh(3, null, null, b === 2 ? 7 : b === 1 ? 3 : 0);
      c.current = b;
      b.stateNode = c;
      xg(b);
      a[ff] = c.current;
      cf(a.nodeType === 8 ? a.parentNode : a);
      if (d)
        for (a = 0; a < d.length; a++) {
          b = d[a];
          var e = b._getVersion;
          e = e(b._source);
          c.mutableSourceEagerHydrationData == null ? c.mutableSourceEagerHydrationData = [b, e] : c.mutableSourceEagerHydrationData.push(b, e);
        }
      this._internalRoot = c;
    }
    qk.prototype.render = function(a) {
      lk(a, this._internalRoot, null, null);
    };
    qk.prototype.unmount = function() {
      var a = this._internalRoot, b = a.containerInfo;
      lk(null, a, null, function() {
        b[ff] = null;
      });
    };
    function rk(a) {
      return !(!a || a.nodeType !== 1 && a.nodeType !== 9 && a.nodeType !== 11 && (a.nodeType !== 8 || a.nodeValue !== " react-mount-point-unstable "));
    }
    function sk(a, b) {
      b || (b = a ? a.nodeType === 9 ? a.documentElement : a.firstChild : null, b = !(!b || b.nodeType !== 1 || !b.hasAttribute("data-reactroot")));
      if (!b)
        for (var c; c = a.lastChild; )
          a.removeChild(c);
      return new qk(a, 0, b ? {hydrate: true} : void 0);
    }
    function tk(a, b, c, d, e) {
      var f = c._reactRootContainer;
      if (f) {
        var g = f._internalRoot;
        if (typeof e === "function") {
          var h = e;
          e = function() {
            var a2 = mk(g);
            h.call(a2);
          };
        }
        lk(b, g, a, e);
      } else {
        f = c._reactRootContainer = sk(c, d);
        g = f._internalRoot;
        if (typeof e === "function") {
          var k = e;
          e = function() {
            var a2 = mk(g);
            k.call(a2);
          };
        }
        Xj(function() {
          lk(b, g, a, e);
        });
      }
      return mk(g);
    }
    ec = function(a) {
      if (a.tag === 13) {
        var b = Hg();
        Jg(a, 4, b);
        ok(a, 4);
      }
    };
    fc = function(a) {
      if (a.tag === 13) {
        var b = Hg();
        Jg(a, 67108864, b);
        ok(a, 67108864);
      }
    };
    gc = function(a) {
      if (a.tag === 13) {
        var b = Hg(), c = Ig(a);
        Jg(a, c, b);
        ok(a, c);
      }
    };
    hc = function(a, b) {
      return b();
    };
    yb = function(a, b, c) {
      switch (b) {
        case "input":
          ab(a, c);
          b = c.name;
          if (c.type === "radio" && b != null) {
            for (c = a; c.parentNode; )
              c = c.parentNode;
            c = c.querySelectorAll("input[name=" + JSON.stringify("" + b) + '][type="radio"]');
            for (b = 0; b < c.length; b++) {
              var d = c[b];
              if (d !== a && d.form === a.form) {
                var e = Db(d);
                if (!e)
                  throw Error(y(90));
                Wa(d);
                ab(d, e);
              }
            }
          }
          break;
        case "textarea":
          ib(a, c);
          break;
        case "select":
          b = c.value, b != null && fb(a, !!c.multiple, b, false);
      }
    };
    Gb = Wj;
    Hb = function(a, b, c, d, e) {
      var f = X;
      X |= 4;
      try {
        return gg(98, a.bind(null, b, c, d, e));
      } finally {
        X = f, X === 0 && (wj(), ig());
      }
    };
    Ib = function() {
      (X & 49) === 0 && (Vj(), Oj());
    };
    Jb = function(a, b) {
      var c = X;
      X |= 2;
      try {
        return a(b);
      } finally {
        X = c, X === 0 && (wj(), ig());
      }
    };
    function uk(a, b) {
      var c = 2 < arguments.length && arguments[2] !== void 0 ? arguments[2] : null;
      if (!rk(b))
        throw Error(y(200));
      return kk(a, b, null, c);
    }
    var vk = {Events: [Cb, ue, Db, Eb, Fb, Oj, {current: false}]};
    var wk = {findFiberByHostInstance: wc, bundleType: 0, version: "17.0.1", rendererPackageName: "react-dom"};
    var xk = {bundleType: wk.bundleType, version: wk.version, rendererPackageName: wk.rendererPackageName, rendererConfig: wk.rendererConfig, overrideHookState: null, overrideHookStateDeletePath: null, overrideHookStateRenamePath: null, overrideProps: null, overridePropsDeletePath: null, overridePropsRenamePath: null, setSuspenseHandler: null, scheduleUpdate: null, currentDispatcherRef: ra.ReactCurrentDispatcher, findHostInstanceByFiber: function(a) {
      a = cc(a);
      return a === null ? null : a.stateNode;
    }, findFiberByHostInstance: wk.findFiberByHostInstance || pk, findHostInstancesForRefresh: null, scheduleRefresh: null, scheduleRoot: null, setRefreshHandler: null, getCurrentFiber: null};
    if (typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ !== "undefined") {
      var yk = __REACT_DEVTOOLS_GLOBAL_HOOK__;
      if (!yk.isDisabled && yk.supportsFiber)
        try {
          Lf = yk.inject(xk), Mf = yk;
        } catch (a) {
        }
    }
    exports.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED = vk;
    exports.createPortal = uk;
    exports.findDOMNode = function(a) {
      if (a == null)
        return null;
      if (a.nodeType === 1)
        return a;
      var b = a._reactInternals;
      if (b === void 0) {
        if (typeof a.render === "function")
          throw Error(y(188));
        throw Error(y(268, Object.keys(a)));
      }
      a = cc(b);
      a = a === null ? null : a.stateNode;
      return a;
    };
    exports.flushSync = function(a, b) {
      var c = X;
      if ((c & 48) !== 0)
        return a(b);
      X |= 1;
      try {
        if (a)
          return gg(99, a.bind(null, b));
      } finally {
        X = c, ig();
      }
    };
    exports.hydrate = function(a, b, c) {
      if (!rk(b))
        throw Error(y(200));
      return tk(null, a, b, true, c);
    };
    exports.render = function(a, b, c) {
      if (!rk(b))
        throw Error(y(200));
      return tk(null, a, b, false, c);
    };
    exports.unmountComponentAtNode = function(a) {
      if (!rk(a))
        throw Error(y(40));
      return a._reactRootContainer ? (Xj(function() {
        tk(null, null, a, false, function() {
          a._reactRootContainer = null;
          a[ff] = null;
        });
      }), true) : false;
    };
    exports.unstable_batchedUpdates = Wj;
    exports.unstable_createPortal = function(a, b) {
      return uk(a, b, 2 < arguments.length && arguments[2] !== void 0 ? arguments[2] : null);
    };
    exports.unstable_renderSubtreeIntoContainer = function(a, b, c, d) {
      if (!rk(c))
        throw Error(y(200));
      if (a == null || a._reactInternals === void 0)
        throw Error(y(38));
      return tk(a, b, c, false, d);
    };
    exports.version = "17.0.1";
  });

  // node_modules/react-dom/index.js
  var require_react_dom = __commonJS((exports, module) => {
    "use strict";
    function checkDCE() {
      if (typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ === "undefined" || typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE !== "function") {
        return;
      }
      if (false) {
        throw new Error("^_^");
      }
      try {
        __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(checkDCE);
      } catch (err) {
        console.error(err);
      }
    }
    if (true) {
      checkDCE();
      module.exports = require_react_dom_production_min();
    } else {
      module.exports = null;
    }
  });

  // node_modules/@babel/runtime/helpers/defineProperty.js
  var require_defineProperty = __commonJS((exports, module) => {
    function _defineProperty2(obj, key, value) {
      if (key in obj) {
        Object.defineProperty(obj, key, {
          value,
          enumerable: true,
          configurable: true,
          writable: true
        });
      } else {
        obj[key] = value;
      }
      return obj;
    }
    module.exports = _defineProperty2;
  });

  // node_modules/@babel/runtime/helpers/inheritsLoose.js
  var require_inheritsLoose = __commonJS((exports, module) => {
    function _inheritsLoose3(subClass, superClass) {
      subClass.prototype = Object.create(superClass.prototype);
      subClass.prototype.constructor = subClass;
      subClass.__proto__ = superClass;
    }
    module.exports = _inheritsLoose3;
  });

  // node_modules/axios/lib/helpers/bind.js
  var require_bind = __commonJS((exports, module) => {
    "use strict";
    module.exports = function bind(fn, thisArg) {
      return function wrap() {
        var args = new Array(arguments.length);
        for (var i = 0; i < args.length; i++) {
          args[i] = arguments[i];
        }
        return fn.apply(thisArg, args);
      };
    };
  });

  // node_modules/axios/lib/utils.js
  var require_utils = __commonJS((exports, module) => {
    "use strict";
    var bind = require_bind();
    var toString2 = Object.prototype.toString;
    function isArray(val) {
      return toString2.call(val) === "[object Array]";
    }
    function isUndefined(val) {
      return typeof val === "undefined";
    }
    function isBuffer(val) {
      return val !== null && !isUndefined(val) && val.constructor !== null && !isUndefined(val.constructor) && typeof val.constructor.isBuffer === "function" && val.constructor.isBuffer(val);
    }
    function isArrayBuffer(val) {
      return toString2.call(val) === "[object ArrayBuffer]";
    }
    function isFormData(val) {
      return typeof FormData !== "undefined" && val instanceof FormData;
    }
    function isArrayBufferView(val) {
      var result;
      if (typeof ArrayBuffer !== "undefined" && ArrayBuffer.isView) {
        result = ArrayBuffer.isView(val);
      } else {
        result = val && val.buffer && val.buffer instanceof ArrayBuffer;
      }
      return result;
    }
    function isString(val) {
      return typeof val === "string";
    }
    function isNumber(val) {
      return typeof val === "number";
    }
    function isObject2(val) {
      return val !== null && typeof val === "object";
    }
    function isPlainObject2(val) {
      if (toString2.call(val) !== "[object Object]") {
        return false;
      }
      var prototype = Object.getPrototypeOf(val);
      return prototype === null || prototype === Object.prototype;
    }
    function isDate(val) {
      return toString2.call(val) === "[object Date]";
    }
    function isFile(val) {
      return toString2.call(val) === "[object File]";
    }
    function isBlob(val) {
      return toString2.call(val) === "[object Blob]";
    }
    function isFunction2(val) {
      return toString2.call(val) === "[object Function]";
    }
    function isStream(val) {
      return isObject2(val) && isFunction2(val.pipe);
    }
    function isURLSearchParams(val) {
      return typeof URLSearchParams !== "undefined" && val instanceof URLSearchParams;
    }
    function trim(str) {
      return str.replace(/^\s*/, "").replace(/\s*$/, "");
    }
    function isStandardBrowserEnv() {
      if (typeof navigator !== "undefined" && (navigator.product === "ReactNative" || navigator.product === "NativeScript" || navigator.product === "NS")) {
        return false;
      }
      return typeof window !== "undefined" && typeof document !== "undefined";
    }
    function forEach(obj, fn) {
      if (obj === null || typeof obj === "undefined") {
        return;
      }
      if (typeof obj !== "object") {
        obj = [obj];
      }
      if (isArray(obj)) {
        for (var i = 0, l = obj.length; i < l; i++) {
          fn.call(null, obj[i], i, obj);
        }
      } else {
        for (var key in obj) {
          if (Object.prototype.hasOwnProperty.call(obj, key)) {
            fn.call(null, obj[key], key, obj);
          }
        }
      }
    }
    function merge2() {
      var result = {};
      function assignValue(val, key) {
        if (isPlainObject2(result[key]) && isPlainObject2(val)) {
          result[key] = merge2(result[key], val);
        } else if (isPlainObject2(val)) {
          result[key] = merge2({}, val);
        } else if (isArray(val)) {
          result[key] = val.slice();
        } else {
          result[key] = val;
        }
      }
      for (var i = 0, l = arguments.length; i < l; i++) {
        forEach(arguments[i], assignValue);
      }
      return result;
    }
    function extend2(a, b, thisArg) {
      forEach(b, function assignValue(val, key) {
        if (thisArg && typeof val === "function") {
          a[key] = bind(val, thisArg);
        } else {
          a[key] = val;
        }
      });
      return a;
    }
    function stripBOM(content) {
      if (content.charCodeAt(0) === 65279) {
        content = content.slice(1);
      }
      return content;
    }
    module.exports = {
      isArray,
      isArrayBuffer,
      isBuffer,
      isFormData,
      isArrayBufferView,
      isString,
      isNumber,
      isObject: isObject2,
      isPlainObject: isPlainObject2,
      isUndefined,
      isDate,
      isFile,
      isBlob,
      isFunction: isFunction2,
      isStream,
      isURLSearchParams,
      isStandardBrowserEnv,
      forEach,
      merge: merge2,
      extend: extend2,
      trim,
      stripBOM
    };
  });

  // node_modules/axios/lib/helpers/buildURL.js
  var require_buildURL = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    function encode(val) {
      return encodeURIComponent(val).replace(/%3A/gi, ":").replace(/%24/g, "$").replace(/%2C/gi, ",").replace(/%20/g, "+").replace(/%5B/gi, "[").replace(/%5D/gi, "]");
    }
    module.exports = function buildURL(url, params, paramsSerializer) {
      if (!params) {
        return url;
      }
      var serializedParams;
      if (paramsSerializer) {
        serializedParams = paramsSerializer(params);
      } else if (utils8.isURLSearchParams(params)) {
        serializedParams = params.toString();
      } else {
        var parts = [];
        utils8.forEach(params, function serialize5(val, key) {
          if (val === null || typeof val === "undefined") {
            return;
          }
          if (utils8.isArray(val)) {
            key = key + "[]";
          } else {
            val = [val];
          }
          utils8.forEach(val, function parseValue(v) {
            if (utils8.isDate(v)) {
              v = v.toISOString();
            } else if (utils8.isObject(v)) {
              v = JSON.stringify(v);
            }
            parts.push(encode(key) + "=" + encode(v));
          });
        });
        serializedParams = parts.join("&");
      }
      if (serializedParams) {
        var hashmarkIndex = url.indexOf("#");
        if (hashmarkIndex !== -1) {
          url = url.slice(0, hashmarkIndex);
        }
        url += (url.indexOf("?") === -1 ? "?" : "&") + serializedParams;
      }
      return url;
    };
  });

  // node_modules/axios/lib/core/InterceptorManager.js
  var require_InterceptorManager = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    function InterceptorManager() {
      this.handlers = [];
    }
    InterceptorManager.prototype.use = function use(fulfilled, rejected) {
      this.handlers.push({
        fulfilled,
        rejected
      });
      return this.handlers.length - 1;
    };
    InterceptorManager.prototype.eject = function eject(id) {
      if (this.handlers[id]) {
        this.handlers[id] = null;
      }
    };
    InterceptorManager.prototype.forEach = function forEach(fn) {
      utils8.forEach(this.handlers, function forEachHandler(h) {
        if (h !== null) {
          fn(h);
        }
      });
    };
    module.exports = InterceptorManager;
  });

  // node_modules/axios/lib/core/transformData.js
  var require_transformData = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    module.exports = function transformData(data, headers, fns) {
      utils8.forEach(fns, function transform(fn) {
        data = fn(data, headers);
      });
      return data;
    };
  });

  // node_modules/axios/lib/cancel/isCancel.js
  var require_isCancel = __commonJS((exports, module) => {
    "use strict";
    module.exports = function isCancel(value) {
      return !!(value && value.__CANCEL__);
    };
  });

  // node_modules/axios/lib/helpers/normalizeHeaderName.js
  var require_normalizeHeaderName = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    module.exports = function normalizeHeaderName(headers, normalizedName) {
      utils8.forEach(headers, function processHeader(value, name) {
        if (name !== normalizedName && name.toUpperCase() === normalizedName.toUpperCase()) {
          headers[normalizedName] = value;
          delete headers[name];
        }
      });
    };
  });

  // node_modules/axios/lib/core/enhanceError.js
  var require_enhanceError = __commonJS((exports, module) => {
    "use strict";
    module.exports = function enhanceError(error, config, code, request, response) {
      error.config = config;
      if (code) {
        error.code = code;
      }
      error.request = request;
      error.response = response;
      error.isAxiosError = true;
      error.toJSON = function toJSON2() {
        return {
          message: this.message,
          name: this.name,
          description: this.description,
          number: this.number,
          fileName: this.fileName,
          lineNumber: this.lineNumber,
          columnNumber: this.columnNumber,
          stack: this.stack,
          config: this.config,
          code: this.code
        };
      };
      return error;
    };
  });

  // node_modules/axios/lib/core/createError.js
  var require_createError = __commonJS((exports, module) => {
    "use strict";
    var enhanceError = require_enhanceError();
    module.exports = function createError(message, config, code, request, response) {
      var error = new Error(message);
      return enhanceError(error, config, code, request, response);
    };
  });

  // node_modules/axios/lib/core/settle.js
  var require_settle = __commonJS((exports, module) => {
    "use strict";
    var createError = require_createError();
    module.exports = function settle(resolve, reject, response) {
      var validateStatus = response.config.validateStatus;
      if (!response.status || !validateStatus || validateStatus(response.status)) {
        resolve(response);
      } else {
        reject(createError("Request failed with status code " + response.status, response.config, null, response.request, response));
      }
    };
  });

  // node_modules/axios/lib/helpers/cookies.js
  var require_cookies = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    module.exports = utils8.isStandardBrowserEnv() ? function standardBrowserEnv() {
      return {
        write: function write(name, value, expires, path, domain, secure) {
          var cookie = [];
          cookie.push(name + "=" + encodeURIComponent(value));
          if (utils8.isNumber(expires)) {
            cookie.push("expires=" + new Date(expires).toGMTString());
          }
          if (utils8.isString(path)) {
            cookie.push("path=" + path);
          }
          if (utils8.isString(domain)) {
            cookie.push("domain=" + domain);
          }
          if (secure === true) {
            cookie.push("secure");
          }
          document.cookie = cookie.join("; ");
        },
        read: function read(name) {
          var match = document.cookie.match(new RegExp("(^|;\\s*)(" + name + ")=([^;]*)"));
          return match ? decodeURIComponent(match[3]) : null;
        },
        remove: function remove2(name) {
          this.write(name, "", Date.now() - 864e5);
        }
      };
    }() : function nonStandardBrowserEnv() {
      return {
        write: function write() {
        },
        read: function read() {
          return null;
        },
        remove: function remove2() {
        }
      };
    }();
  });

  // node_modules/axios/lib/helpers/isAbsoluteURL.js
  var require_isAbsoluteURL = __commonJS((exports, module) => {
    "use strict";
    module.exports = function isAbsoluteURL(url) {
      return /^([a-z][a-z\d\+\-\.]*:)?\/\//i.test(url);
    };
  });

  // node_modules/axios/lib/helpers/combineURLs.js
  var require_combineURLs = __commonJS((exports, module) => {
    "use strict";
    module.exports = function combineURLs(baseURL, relativeURL) {
      return relativeURL ? baseURL.replace(/\/+$/, "") + "/" + relativeURL.replace(/^\/+/, "") : baseURL;
    };
  });

  // node_modules/axios/lib/core/buildFullPath.js
  var require_buildFullPath = __commonJS((exports, module) => {
    "use strict";
    var isAbsoluteURL = require_isAbsoluteURL();
    var combineURLs = require_combineURLs();
    module.exports = function buildFullPath(baseURL, requestedURL) {
      if (baseURL && !isAbsoluteURL(requestedURL)) {
        return combineURLs(baseURL, requestedURL);
      }
      return requestedURL;
    };
  });

  // node_modules/axios/lib/helpers/parseHeaders.js
  var require_parseHeaders = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    var ignoreDuplicateOf = [
      "age",
      "authorization",
      "content-length",
      "content-type",
      "etag",
      "expires",
      "from",
      "host",
      "if-modified-since",
      "if-unmodified-since",
      "last-modified",
      "location",
      "max-forwards",
      "proxy-authorization",
      "referer",
      "retry-after",
      "user-agent"
    ];
    module.exports = function parseHeaders(headers) {
      var parsed = {};
      var key;
      var val;
      var i;
      if (!headers) {
        return parsed;
      }
      utils8.forEach(headers.split("\n"), function parser(line) {
        i = line.indexOf(":");
        key = utils8.trim(line.substr(0, i)).toLowerCase();
        val = utils8.trim(line.substr(i + 1));
        if (key) {
          if (parsed[key] && ignoreDuplicateOf.indexOf(key) >= 0) {
            return;
          }
          if (key === "set-cookie") {
            parsed[key] = (parsed[key] ? parsed[key] : []).concat([val]);
          } else {
            parsed[key] = parsed[key] ? parsed[key] + ", " + val : val;
          }
        }
      });
      return parsed;
    };
  });

  // node_modules/axios/lib/helpers/isURLSameOrigin.js
  var require_isURLSameOrigin = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    module.exports = utils8.isStandardBrowserEnv() ? function standardBrowserEnv() {
      var msie = /(msie|trident)/i.test(navigator.userAgent);
      var urlParsingNode = document.createElement("a");
      var originURL;
      function resolveURL(url) {
        var href = url;
        if (msie) {
          urlParsingNode.setAttribute("href", href);
          href = urlParsingNode.href;
        }
        urlParsingNode.setAttribute("href", href);
        return {
          href: urlParsingNode.href,
          protocol: urlParsingNode.protocol ? urlParsingNode.protocol.replace(/:$/, "") : "",
          host: urlParsingNode.host,
          search: urlParsingNode.search ? urlParsingNode.search.replace(/^\?/, "") : "",
          hash: urlParsingNode.hash ? urlParsingNode.hash.replace(/^#/, "") : "",
          hostname: urlParsingNode.hostname,
          port: urlParsingNode.port,
          pathname: urlParsingNode.pathname.charAt(0) === "/" ? urlParsingNode.pathname : "/" + urlParsingNode.pathname
        };
      }
      originURL = resolveURL(window.location.href);
      return function isURLSameOrigin(requestURL) {
        var parsed = utils8.isString(requestURL) ? resolveURL(requestURL) : requestURL;
        return parsed.protocol === originURL.protocol && parsed.host === originURL.host;
      };
    }() : function nonStandardBrowserEnv() {
      return function isURLSameOrigin() {
        return true;
      };
    }();
  });

  // node_modules/axios/lib/adapters/xhr.js
  var require_xhr = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    var settle = require_settle();
    var cookies = require_cookies();
    var buildURL = require_buildURL();
    var buildFullPath = require_buildFullPath();
    var parseHeaders = require_parseHeaders();
    var isURLSameOrigin = require_isURLSameOrigin();
    var createError = require_createError();
    module.exports = function xhrAdapter(config) {
      return new Promise(function dispatchXhrRequest(resolve, reject) {
        var requestData = config.data;
        var requestHeaders = config.headers;
        if (utils8.isFormData(requestData)) {
          delete requestHeaders["Content-Type"];
        }
        var request = new XMLHttpRequest();
        if (config.auth) {
          var username = config.auth.username || "";
          var password = config.auth.password ? unescape(encodeURIComponent(config.auth.password)) : "";
          requestHeaders.Authorization = "Basic " + btoa(username + ":" + password);
        }
        var fullPath = buildFullPath(config.baseURL, config.url);
        request.open(config.method.toUpperCase(), buildURL(fullPath, config.params, config.paramsSerializer), true);
        request.timeout = config.timeout;
        request.onreadystatechange = function handleLoad() {
          if (!request || request.readyState !== 4) {
            return;
          }
          if (request.status === 0 && !(request.responseURL && request.responseURL.indexOf("file:") === 0)) {
            return;
          }
          var responseHeaders = "getAllResponseHeaders" in request ? parseHeaders(request.getAllResponseHeaders()) : null;
          var responseData = !config.responseType || config.responseType === "text" ? request.responseText : request.response;
          var response = {
            data: responseData,
            status: request.status,
            statusText: request.statusText,
            headers: responseHeaders,
            config,
            request
          };
          settle(resolve, reject, response);
          request = null;
        };
        request.onabort = function handleAbort() {
          if (!request) {
            return;
          }
          reject(createError("Request aborted", config, "ECONNABORTED", request));
          request = null;
        };
        request.onerror = function handleError() {
          reject(createError("Network Error", config, null, request));
          request = null;
        };
        request.ontimeout = function handleTimeout() {
          var timeoutErrorMessage = "timeout of " + config.timeout + "ms exceeded";
          if (config.timeoutErrorMessage) {
            timeoutErrorMessage = config.timeoutErrorMessage;
          }
          reject(createError(timeoutErrorMessage, config, "ECONNABORTED", request));
          request = null;
        };
        if (utils8.isStandardBrowserEnv()) {
          var xsrfValue = (config.withCredentials || isURLSameOrigin(fullPath)) && config.xsrfCookieName ? cookies.read(config.xsrfCookieName) : void 0;
          if (xsrfValue) {
            requestHeaders[config.xsrfHeaderName] = xsrfValue;
          }
        }
        if ("setRequestHeader" in request) {
          utils8.forEach(requestHeaders, function setRequestHeader(val, key) {
            if (typeof requestData === "undefined" && key.toLowerCase() === "content-type") {
              delete requestHeaders[key];
            } else {
              request.setRequestHeader(key, val);
            }
          });
        }
        if (!utils8.isUndefined(config.withCredentials)) {
          request.withCredentials = !!config.withCredentials;
        }
        if (config.responseType) {
          try {
            request.responseType = config.responseType;
          } catch (e) {
            if (config.responseType !== "json") {
              throw e;
            }
          }
        }
        if (typeof config.onDownloadProgress === "function") {
          request.addEventListener("progress", config.onDownloadProgress);
        }
        if (typeof config.onUploadProgress === "function" && request.upload) {
          request.upload.addEventListener("progress", config.onUploadProgress);
        }
        if (config.cancelToken) {
          config.cancelToken.promise.then(function onCanceled(cancel) {
            if (!request) {
              return;
            }
            request.abort();
            reject(cancel);
            request = null;
          });
        }
        if (!requestData) {
          requestData = null;
        }
        request.send(requestData);
      });
    };
  });

  // node_modules/axios/lib/defaults.js
  var require_defaults = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    var normalizeHeaderName = require_normalizeHeaderName();
    var DEFAULT_CONTENT_TYPE = {
      "Content-Type": "application/x-www-form-urlencoded"
    };
    function setContentTypeIfUnset(headers, value) {
      if (!utils8.isUndefined(headers) && utils8.isUndefined(headers["Content-Type"])) {
        headers["Content-Type"] = value;
      }
    }
    function getDefaultAdapter() {
      var adapter;
      if (typeof XMLHttpRequest !== "undefined") {
        adapter = require_xhr();
      } else if (typeof process !== "undefined" && Object.prototype.toString.call(process) === "[object process]") {
        adapter = require_xhr();
      }
      return adapter;
    }
    var defaults = {
      adapter: getDefaultAdapter(),
      transformRequest: [function transformRequest(data, headers) {
        normalizeHeaderName(headers, "Accept");
        normalizeHeaderName(headers, "Content-Type");
        if (utils8.isFormData(data) || utils8.isArrayBuffer(data) || utils8.isBuffer(data) || utils8.isStream(data) || utils8.isFile(data) || utils8.isBlob(data)) {
          return data;
        }
        if (utils8.isArrayBufferView(data)) {
          return data.buffer;
        }
        if (utils8.isURLSearchParams(data)) {
          setContentTypeIfUnset(headers, "application/x-www-form-urlencoded;charset=utf-8");
          return data.toString();
        }
        if (utils8.isObject(data)) {
          setContentTypeIfUnset(headers, "application/json;charset=utf-8");
          return JSON.stringify(data);
        }
        return data;
      }],
      transformResponse: [function transformResponse(data) {
        if (typeof data === "string") {
          try {
            data = JSON.parse(data);
          } catch (e) {
          }
        }
        return data;
      }],
      timeout: 0,
      xsrfCookieName: "XSRF-TOKEN",
      xsrfHeaderName: "X-XSRF-TOKEN",
      maxContentLength: -1,
      maxBodyLength: -1,
      validateStatus: function validateStatus(status) {
        return status >= 200 && status < 300;
      }
    };
    defaults.headers = {
      common: {
        Accept: "application/json, text/plain, */*"
      }
    };
    utils8.forEach(["delete", "get", "head"], function forEachMethodNoData(method) {
      defaults.headers[method] = {};
    });
    utils8.forEach(["post", "put", "patch"], function forEachMethodWithData(method) {
      defaults.headers[method] = utils8.merge(DEFAULT_CONTENT_TYPE);
    });
    module.exports = defaults;
  });

  // node_modules/axios/lib/core/dispatchRequest.js
  var require_dispatchRequest = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    var transformData = require_transformData();
    var isCancel = require_isCancel();
    var defaults = require_defaults();
    function throwIfCancellationRequested(config) {
      if (config.cancelToken) {
        config.cancelToken.throwIfRequested();
      }
    }
    module.exports = function dispatchRequest(config) {
      throwIfCancellationRequested(config);
      config.headers = config.headers || {};
      config.data = transformData(config.data, config.headers, config.transformRequest);
      config.headers = utils8.merge(config.headers.common || {}, config.headers[config.method] || {}, config.headers);
      utils8.forEach(["delete", "get", "head", "post", "put", "patch", "common"], function cleanHeaderConfig(method) {
        delete config.headers[method];
      });
      var adapter = config.adapter || defaults.adapter;
      return adapter(config).then(function onAdapterResolution(response) {
        throwIfCancellationRequested(config);
        response.data = transformData(response.data, response.headers, config.transformResponse);
        return response;
      }, function onAdapterRejection(reason) {
        if (!isCancel(reason)) {
          throwIfCancellationRequested(config);
          if (reason && reason.response) {
            reason.response.data = transformData(reason.response.data, reason.response.headers, config.transformResponse);
          }
        }
        return Promise.reject(reason);
      });
    };
  });

  // node_modules/axios/lib/core/mergeConfig.js
  var require_mergeConfig = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    module.exports = function mergeConfig(config1, config2) {
      config2 = config2 || {};
      var config = {};
      var valueFromConfig2Keys = ["url", "method", "data"];
      var mergeDeepPropertiesKeys = ["headers", "auth", "proxy", "params"];
      var defaultToConfig2Keys = [
        "baseURL",
        "transformRequest",
        "transformResponse",
        "paramsSerializer",
        "timeout",
        "timeoutMessage",
        "withCredentials",
        "adapter",
        "responseType",
        "xsrfCookieName",
        "xsrfHeaderName",
        "onUploadProgress",
        "onDownloadProgress",
        "decompress",
        "maxContentLength",
        "maxBodyLength",
        "maxRedirects",
        "transport",
        "httpAgent",
        "httpsAgent",
        "cancelToken",
        "socketPath",
        "responseEncoding"
      ];
      var directMergeKeys = ["validateStatus"];
      function getMergedValue(target, source) {
        if (utils8.isPlainObject(target) && utils8.isPlainObject(source)) {
          return utils8.merge(target, source);
        } else if (utils8.isPlainObject(source)) {
          return utils8.merge({}, source);
        } else if (utils8.isArray(source)) {
          return source.slice();
        }
        return source;
      }
      function mergeDeepProperties(prop) {
        if (!utils8.isUndefined(config2[prop])) {
          config[prop] = getMergedValue(config1[prop], config2[prop]);
        } else if (!utils8.isUndefined(config1[prop])) {
          config[prop] = getMergedValue(void 0, config1[prop]);
        }
      }
      utils8.forEach(valueFromConfig2Keys, function valueFromConfig2(prop) {
        if (!utils8.isUndefined(config2[prop])) {
          config[prop] = getMergedValue(void 0, config2[prop]);
        }
      });
      utils8.forEach(mergeDeepPropertiesKeys, mergeDeepProperties);
      utils8.forEach(defaultToConfig2Keys, function defaultToConfig2(prop) {
        if (!utils8.isUndefined(config2[prop])) {
          config[prop] = getMergedValue(void 0, config2[prop]);
        } else if (!utils8.isUndefined(config1[prop])) {
          config[prop] = getMergedValue(void 0, config1[prop]);
        }
      });
      utils8.forEach(directMergeKeys, function merge2(prop) {
        if (prop in config2) {
          config[prop] = getMergedValue(config1[prop], config2[prop]);
        } else if (prop in config1) {
          config[prop] = getMergedValue(void 0, config1[prop]);
        }
      });
      var axiosKeys = valueFromConfig2Keys.concat(mergeDeepPropertiesKeys).concat(defaultToConfig2Keys).concat(directMergeKeys);
      var otherKeys = Object.keys(config1).concat(Object.keys(config2)).filter(function filterAxiosKeys(key) {
        return axiosKeys.indexOf(key) === -1;
      });
      utils8.forEach(otherKeys, mergeDeepProperties);
      return config;
    };
  });

  // node_modules/axios/lib/core/Axios.js
  var require_Axios = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    var buildURL = require_buildURL();
    var InterceptorManager = require_InterceptorManager();
    var dispatchRequest = require_dispatchRequest();
    var mergeConfig = require_mergeConfig();
    function Axios(instanceConfig) {
      this.defaults = instanceConfig;
      this.interceptors = {
        request: new InterceptorManager(),
        response: new InterceptorManager()
      };
    }
    Axios.prototype.request = function request(config) {
      if (typeof config === "string") {
        config = arguments[1] || {};
        config.url = arguments[0];
      } else {
        config = config || {};
      }
      config = mergeConfig(this.defaults, config);
      if (config.method) {
        config.method = config.method.toLowerCase();
      } else if (this.defaults.method) {
        config.method = this.defaults.method.toLowerCase();
      } else {
        config.method = "get";
      }
      var chain2 = [dispatchRequest, void 0];
      var promise = Promise.resolve(config);
      this.interceptors.request.forEach(function unshiftRequestInterceptors(interceptor) {
        chain2.unshift(interceptor.fulfilled, interceptor.rejected);
      });
      this.interceptors.response.forEach(function pushResponseInterceptors(interceptor) {
        chain2.push(interceptor.fulfilled, interceptor.rejected);
      });
      while (chain2.length) {
        promise = promise.then(chain2.shift(), chain2.shift());
      }
      return promise;
    };
    Axios.prototype.getUri = function getUri(config) {
      config = mergeConfig(this.defaults, config);
      return buildURL(config.url, config.params, config.paramsSerializer).replace(/^\?/, "");
    };
    utils8.forEach(["delete", "get", "head", "options"], function forEachMethodNoData(method) {
      Axios.prototype[method] = function(url, config) {
        return this.request(mergeConfig(config || {}, {
          method,
          url,
          data: (config || {}).data
        }));
      };
    });
    utils8.forEach(["post", "put", "patch"], function forEachMethodWithData(method) {
      Axios.prototype[method] = function(url, data, config) {
        return this.request(mergeConfig(config || {}, {
          method,
          url,
          data
        }));
      };
    });
    module.exports = Axios;
  });

  // node_modules/axios/lib/cancel/Cancel.js
  var require_Cancel = __commonJS((exports, module) => {
    "use strict";
    function Cancel(message) {
      this.message = message;
    }
    Cancel.prototype.toString = function toString2() {
      return "Cancel" + (this.message ? ": " + this.message : "");
    };
    Cancel.prototype.__CANCEL__ = true;
    module.exports = Cancel;
  });

  // node_modules/axios/lib/cancel/CancelToken.js
  var require_CancelToken = __commonJS((exports, module) => {
    "use strict";
    var Cancel = require_Cancel();
    function CancelToken(executor) {
      if (typeof executor !== "function") {
        throw new TypeError("executor must be a function.");
      }
      var resolvePromise;
      this.promise = new Promise(function promiseExecutor(resolve) {
        resolvePromise = resolve;
      });
      var token = this;
      executor(function cancel(message) {
        if (token.reason) {
          return;
        }
        token.reason = new Cancel(message);
        resolvePromise(token.reason);
      });
    }
    CancelToken.prototype.throwIfRequested = function throwIfRequested() {
      if (this.reason) {
        throw this.reason;
      }
    };
    CancelToken.source = function source() {
      var cancel;
      var token = new CancelToken(function executor(c) {
        cancel = c;
      });
      return {
        token,
        cancel
      };
    };
    module.exports = CancelToken;
  });

  // node_modules/axios/lib/helpers/spread.js
  var require_spread = __commonJS((exports, module) => {
    "use strict";
    module.exports = function spread(callback) {
      return function wrap(arr) {
        return callback.apply(null, arr);
      };
    };
  });

  // node_modules/axios/lib/axios.js
  var require_axios = __commonJS((exports, module) => {
    "use strict";
    var utils8 = require_utils();
    var bind = require_bind();
    var Axios = require_Axios();
    var mergeConfig = require_mergeConfig();
    var defaults = require_defaults();
    function createInstance(defaultConfig) {
      var context = new Axios(defaultConfig);
      var instance = bind(Axios.prototype.request, context);
      utils8.extend(instance, Axios.prototype, context);
      utils8.extend(instance, context);
      return instance;
    }
    var axios3 = createInstance(defaults);
    axios3.Axios = Axios;
    axios3.create = function create(instanceConfig) {
      return createInstance(mergeConfig(axios3.defaults, instanceConfig));
    };
    axios3.Cancel = require_Cancel();
    axios3.CancelToken = require_CancelToken();
    axios3.isCancel = require_isCancel();
    axios3.all = function all(promises) {
      return Promise.all(promises);
    };
    axios3.spread = require_spread();
    module.exports = axios3;
    module.exports.default = axios3;
  });

  // node_modules/axios/index.js
  var require_axios2 = __commonJS((exports, module) => {
    module.exports = require_axios();
  });

  // src/client.tsx
  const React4 = __toModule(require_react());
  const ReactDOM = __toModule(require_react_dom());

  // src/client/utils/createContext.ts
  const React = __toModule(require_react());
  const createContext2 = (label) => {
    const marker = {};
    const AppContext2 = React.createContext(marker);
    const useContext3 = () => {
      const context = React.useContext(AppContext2);
      if (context === marker) {
        throw Error(`value was read out of context => "${label}"`);
      }
      return context;
    };
    return {
      AppContext: AppContext2,
      useContext: useContext3
    };
  };

  // src/client/Context.ts
  const {AppContext, useContext: useContext2} = createContext2("appState");
  const Provider = AppContext.Provider;
  const useAppStateContext = () => {
    return useContext2();
  };

  // src/client/App.tsx
  const React3 = __toModule(require_react());

  // node_modules/mobx/dist/mobx.esm.js
  function die(error) {
    for (var _len = arguments.length, args = new Array(_len > 1 ? _len - 1 : 0), _key = 1; _key < _len; _key++) {
      args[_key - 1] = arguments[_key];
    }
    if (false) {
      var e = typeof error === "string" ? error : errors[error];
      if (typeof e === "function")
        e = e.apply(null, args);
      throw new Error("[MobX] " + e);
    }
    throw new Error(typeof error === "number" ? "[MobX] minified error nr: " + error + (args.length ? " " + args.join(",") : "") + ". Find the full error at: https://github.com/mobxjs/mobx/blob/mobx6/src/errors.ts" : "[MobX] " + error);
  }
  function getGlobal() {
    return typeof global !== "undefined" ? global : window;
  }
  var assign = Object.assign;
  var getDescriptor = Object.getOwnPropertyDescriptor;
  var defineProperty = Object.defineProperty;
  var objectPrototype = Object.prototype;
  var EMPTY_ARRAY = [];
  Object.freeze(EMPTY_ARRAY);
  var EMPTY_OBJECT = {};
  Object.freeze(EMPTY_OBJECT);
  var hasProxy = typeof Proxy !== "undefined";
  var plainObjectString = /* @__PURE__ */ Object.toString();
  function assertProxies() {
    if (!hasProxy) {
      die("Proxy not available");
    }
  }
  function getNextId() {
    return ++globalState.mobxGuid;
  }
  function once(func) {
    var invoked = false;
    return function() {
      if (invoked)
        return;
      invoked = true;
      return func.apply(this, arguments);
    };
  }
  var noop = function noop2() {
  };
  function isFunction(fn) {
    return typeof fn === "function";
  }
  function isStringish(value) {
    var t2 = typeof value;
    switch (t2) {
      case "string":
      case "symbol":
      case "number":
        return true;
    }
    return false;
  }
  function isObject(value) {
    return value !== null && typeof value === "object";
  }
  function isPlainObject(value) {
    var _proto$constructor;
    if (!isObject(value))
      return false;
    var proto = Object.getPrototypeOf(value);
    if (proto == null)
      return true;
    return ((_proto$constructor = proto.constructor) === null || _proto$constructor === void 0 ? void 0 : _proto$constructor.toString()) === plainObjectString;
  }
  function isGenerator(obj) {
    var constructor = obj === null || obj === void 0 ? void 0 : obj.constructor;
    if (!constructor)
      return false;
    if (constructor.name === "GeneratorFunction" || constructor.displayName === "GeneratorFunction")
      return true;
    return false;
  }
  function addHiddenProp(object3, propName, value) {
    defineProperty(object3, propName, {
      enumerable: false,
      writable: true,
      configurable: true,
      value
    });
  }
  function addHiddenFinalProp(object3, propName, value) {
    defineProperty(object3, propName, {
      enumerable: false,
      writable: false,
      configurable: true,
      value
    });
  }
  function createInstanceofPredicate(name, clazz) {
    var propName = "isMobX" + name;
    clazz.prototype[propName] = true;
    return function(x) {
      return isObject(x) && x[propName] === true;
    };
  }
  function isES6Map(thing) {
    return thing instanceof Map;
  }
  function isES6Set(thing) {
    return thing instanceof Set;
  }
  var hasGetOwnPropertySymbols = typeof Object.getOwnPropertySymbols !== "undefined";
  function getPlainObjectKeys(object3) {
    var keys = Object.keys(object3);
    if (!hasGetOwnPropertySymbols)
      return keys;
    var symbols = Object.getOwnPropertySymbols(object3);
    if (!symbols.length)
      return keys;
    return [].concat(keys, symbols.filter(function(s) {
      return objectPrototype.propertyIsEnumerable.call(object3, s);
    }));
  }
  var ownKeys = typeof Reflect !== "undefined" && Reflect.ownKeys ? Reflect.ownKeys : hasGetOwnPropertySymbols ? function(obj) {
    return Object.getOwnPropertyNames(obj).concat(Object.getOwnPropertySymbols(obj));
  } : Object.getOwnPropertyNames;
  function stringifyKey(key) {
    if (typeof key === "string")
      return key;
    if (typeof key === "symbol")
      return key.toString();
    return new String(key).toString();
  }
  function toPrimitive(value) {
    return value === null ? null : typeof value === "object" ? "" + value : value;
  }
  function hasProp(target, prop) {
    return objectPrototype.hasOwnProperty.call(target, prop);
  }
  var getOwnPropertyDescriptors = Object.getOwnPropertyDescriptors || function getOwnPropertyDescriptors2(target) {
    var res = {};
    ownKeys(target).forEach(function(key) {
      res[key] = getDescriptor(target, key);
    });
    return res;
  };
  var mobxDecoratorsSymbol = /* @__PURE__ */ Symbol("mobx-decorators");
  var mobxAppliedDecoratorsSymbol = /* @__PURE__ */ Symbol("mobx-applied-decorators");
  function createDecorator(type2) {
    return assign(function(target, property) {
      if (property === void 0) {
        createDecoratorAndAnnotation(type2, target);
      } else {
        storeDecorator(target, property, type2);
      }
    }, {
      annotationType_: type2
    });
  }
  function createDecoratorAndAnnotation(type2, arg_) {
    return assign(function(target, property) {
      storeDecorator(target, property, type2, arg_);
    }, {
      annotationType_: type2,
      arg_
    });
  }
  function storeDecorator(target, property, type2, arg_) {
    var desc = getDescriptor(target, mobxDecoratorsSymbol);
    var map2;
    if (desc) {
      map2 = desc.value;
    } else {
      map2 = {};
      addHiddenProp(target, mobxDecoratorsSymbol, map2);
    }
    map2[property] = {
      annotationType_: type2,
      arg_
    };
  }
  function applyDecorators(target) {
    if (target[mobxAppliedDecoratorsSymbol])
      return true;
    var current = target;
    var annotations = [];
    while (current && current !== objectPrototype) {
      var desc = getDescriptor(current, mobxDecoratorsSymbol);
      if (desc) {
        if (!annotations.length) {
          for (var key in desc.value) {
            if (!hasProp(target, key) && !hasProp(current, key)) {
              return true;
            }
          }
        }
        annotations.unshift(desc.value);
      }
      current = Object.getPrototypeOf(current);
    }
    annotations.forEach(function(a) {
      makeObservable(target, a);
    });
    addHiddenProp(target, mobxAppliedDecoratorsSymbol, true);
    return annotations.length > 0;
  }
  var $mobx = /* @__PURE__ */ Symbol("mobx administration");
  var Atom = /* @__PURE__ */ function() {
    function Atom2(name_) {
      if (name_ === void 0) {
        name_ = "Atom@" + getNextId();
      }
      this.name_ = void 0;
      this.isPendingUnobservation_ = false;
      this.isBeingObserved_ = false;
      this.observers_ = new Set();
      this.diffValue_ = 0;
      this.lastAccessedBy_ = 0;
      this.lowestObserverState_ = IDerivationState_.NOT_TRACKING_;
      this.onBOL = void 0;
      this.onBUOL = void 0;
      this.name_ = name_;
    }
    var _proto = Atom2.prototype;
    _proto.onBO = function onBO() {
      if (this.onBOL) {
        this.onBOL.forEach(function(listener) {
          return listener();
        });
      }
    };
    _proto.onBUO = function onBUO() {
      if (this.onBUOL) {
        this.onBUOL.forEach(function(listener) {
          return listener();
        });
      }
    };
    _proto.reportObserved = function reportObserved$1() {
      return reportObserved(this);
    };
    _proto.reportChanged = function reportChanged() {
      startBatch();
      propagateChanged(this);
      endBatch();
    };
    _proto.toString = function toString2() {
      return this.name_;
    };
    return Atom2;
  }();
  var isAtom = /* @__PURE__ */ createInstanceofPredicate("Atom", Atom);
  function createAtom(name, onBecomeObservedHandler, onBecomeUnobservedHandler) {
    if (onBecomeObservedHandler === void 0) {
      onBecomeObservedHandler = noop;
    }
    if (onBecomeUnobservedHandler === void 0) {
      onBecomeUnobservedHandler = noop;
    }
    var atom = new Atom(name);
    if (onBecomeObservedHandler !== noop) {
      onBecomeObserved(atom, onBecomeObservedHandler);
    }
    if (onBecomeUnobservedHandler !== noop) {
      onBecomeUnobserved(atom, onBecomeUnobservedHandler);
    }
    return atom;
  }
  function identityComparer(a, b) {
    return a === b;
  }
  function structuralComparer(a, b) {
    return deepEqual(a, b);
  }
  function shallowComparer(a, b) {
    return deepEqual(a, b, 1);
  }
  function defaultComparer(a, b) {
    return Object.is(a, b);
  }
  var comparer = {
    identity: identityComparer,
    structural: structuralComparer,
    default: defaultComparer,
    shallow: shallowComparer
  };
  function deepEnhancer(v, _, name) {
    if (isObservable(v))
      return v;
    if (Array.isArray(v))
      return observable.array(v, {
        name
      });
    if (isPlainObject(v))
      return observable.object(v, void 0, {
        name
      });
    if (isES6Map(v))
      return observable.map(v, {
        name
      });
    if (isES6Set(v))
      return observable.set(v, {
        name
      });
    return v;
  }
  function shallowEnhancer(v, _, name) {
    if (v === void 0 || v === null)
      return v;
    if (isObservableObject(v) || isObservableArray(v) || isObservableMap(v) || isObservableSet(v))
      return v;
    if (Array.isArray(v))
      return observable.array(v, {
        name,
        deep: false
      });
    if (isPlainObject(v))
      return observable.object(v, void 0, {
        name,
        deep: false
      });
    if (isES6Map(v))
      return observable.map(v, {
        name,
        deep: false
      });
    if (isES6Set(v))
      return observable.set(v, {
        name,
        deep: false
      });
    if (false)
      die("The shallow modifier / decorator can only used in combination with arrays, objects, maps and sets");
  }
  function referenceEnhancer(newValue) {
    return newValue;
  }
  function refStructEnhancer(v, oldValue) {
    if (false)
      die("observable.struct should not be used with observable values");
    if (deepEqual(v, oldValue))
      return oldValue;
    return v;
  }
  var _annotationToEnhancer;
  var OBSERVABLE = "observable";
  var OBSERVABLE_REF = "observable.ref";
  var OBSERVABLE_SHALLOW = "observable.shallow";
  var OBSERVABLE_STRUCT = "observable.struct";
  var defaultCreateObservableOptions = {
    deep: true,
    name: void 0,
    defaultDecorator: void 0,
    proxy: true
  };
  Object.freeze(defaultCreateObservableOptions);
  function asCreateObservableOptions(thing) {
    return thing || defaultCreateObservableOptions;
  }
  function getEnhancerFromOption(options) {
    return options.deep === true ? deepEnhancer : options.deep === false ? referenceEnhancer : getEnhancerFromAnnotation(options.defaultDecorator);
  }
  var annotationToEnhancer = (_annotationToEnhancer = {}, _annotationToEnhancer[OBSERVABLE] = deepEnhancer, _annotationToEnhancer[OBSERVABLE_REF] = referenceEnhancer, _annotationToEnhancer[OBSERVABLE_SHALLOW] = shallowEnhancer, _annotationToEnhancer[OBSERVABLE_STRUCT] = refStructEnhancer, _annotationToEnhancer);
  function getEnhancerFromAnnotation(annotation) {
    var _annotationToEnhancer2;
    return !annotation ? deepEnhancer : (_annotationToEnhancer2 = annotationToEnhancer[annotation.annotationType_]) !== null && _annotationToEnhancer2 !== void 0 ? _annotationToEnhancer2 : die(12);
  }
  function createObservable(v, arg2, arg3) {
    if (isStringish(arg2)) {
      storeDecorator(v, arg2, OBSERVABLE);
      return;
    }
    if (isObservable(v))
      return v;
    var res = isPlainObject(v) ? observable.object(v, arg2, arg3) : Array.isArray(v) ? observable.array(v, arg2) : isES6Map(v) ? observable.map(v, arg2) : isES6Set(v) ? observable.set(v, arg2) : v;
    if (res !== v)
      return res;
    return observable.box(v);
  }
  createObservable.annotationType_ = OBSERVABLE;
  var observableFactories = {
    box: function box(value, options) {
      var o = asCreateObservableOptions(options);
      return new ObservableValue(value, getEnhancerFromOption(o), o.name, true, o.equals);
    },
    array: function array(initialValues, options) {
      var o = asCreateObservableOptions(options);
      return (globalState.useProxies === false || o.proxy === false ? createLegacyArray : createObservableArray)(initialValues, getEnhancerFromOption(o), o.name);
    },
    map: function map(initialValues, options) {
      var o = asCreateObservableOptions(options);
      return new ObservableMap(initialValues, getEnhancerFromOption(o), o.name);
    },
    set: function set(initialValues, options) {
      var o = asCreateObservableOptions(options);
      return new ObservableSet(initialValues, getEnhancerFromOption(o), o.name);
    },
    object: function object(props, decorators, options) {
      var o = asCreateObservableOptions(options);
      var base = {};
      asObservableObject(base, options === null || options === void 0 ? void 0 : options.name, getEnhancerFromOption(o));
      return extendObservable(globalState.useProxies === false || o.proxy === false ? base : createDynamicObservableObject(base), props, decorators, options);
    },
    ref: /* @__PURE__ */ createDecorator(OBSERVABLE_REF),
    shallow: /* @__PURE__ */ createDecorator(OBSERVABLE_SHALLOW),
    deep: /* @__PURE__ */ createDecorator(OBSERVABLE),
    struct: /* @__PURE__ */ createDecorator(OBSERVABLE_STRUCT)
  };
  var observable = /* @__PURE__ */ assign(createObservable, observableFactories);
  var COMPUTED = "computed";
  var COMPUTED_STRUCT = "computed.struct";
  var computed = function computed2(arg1, arg2, arg3) {
    if (isStringish(arg2)) {
      return storeDecorator(arg1, arg2, COMPUTED);
    }
    if (isPlainObject(arg1)) {
      return createDecoratorAndAnnotation(COMPUTED, arg1);
    }
    if (false) {
      if (!isFunction(arg1))
        die("First argument to `computed` should be an expression.");
      if (isFunction(arg2))
        die("A setter as second argument is no longer supported, use `{set: fn }` option instead");
    }
    var opts = isPlainObject(arg2) ? arg2 : {};
    opts.get = arg1;
    opts.name = opts.name || arg1.name || "";
    return new ComputedValue(opts);
  };
  computed.annotationType_ = COMPUTED;
  computed.struct = /* @__PURE__ */ assign(function(target, property) {
    storeDecorator(target, property, COMPUTED_STRUCT);
  }, {
    annotationType_: COMPUTED_STRUCT
  });
  var _getDescriptor$config;
  var _getDescriptor;
  var currentActionId = 0;
  var nextActionId = 1;
  var isFunctionNameConfigurable = (_getDescriptor$config = (_getDescriptor = /* @__PURE__ */ getDescriptor(function() {
  }, "name")) === null || _getDescriptor === void 0 ? void 0 : _getDescriptor.configurable) !== null && _getDescriptor$config !== void 0 ? _getDescriptor$config : false;
  var tmpNameDescriptor = {
    value: "action",
    configurable: true,
    writable: false,
    enumerable: false
  };
  function createAction(actionName, fn, autoAction2, ref) {
    if (autoAction2 === void 0) {
      autoAction2 = false;
    }
    if (false) {
      if (!isFunction(fn))
        die("`action` can only be invoked on functions");
      if (typeof actionName !== "string" || !actionName)
        die("actions should have valid names, got: '" + actionName + "'");
    }
    function res() {
      return executeAction(actionName, autoAction2, fn, ref || this, arguments);
    }
    res.isMobxAction = true;
    if (isFunctionNameConfigurable) {
      tmpNameDescriptor.value = actionName;
      Object.defineProperty(res, "name", tmpNameDescriptor);
    }
    return res;
  }
  function executeAction(actionName, canRunAsDerivation, fn, scope, args) {
    var runInfo = _startAction(actionName, canRunAsDerivation, scope, args);
    try {
      return fn.apply(scope, args);
    } catch (err) {
      runInfo.error_ = err;
      throw err;
    } finally {
      _endAction(runInfo);
    }
  }
  function _startAction(actionName, canRunAsDerivation, scope, args) {
    var notifySpy_ = false;
    var startTime_ = 0;
    if (false) {
      startTime_ = Date.now();
      var flattenedArgs = args ? Array.from(args) : EMPTY_ARRAY;
      spyReportStart({
        type: ACTION,
        name: actionName,
        object: scope,
        arguments: flattenedArgs
      });
    }
    var prevDerivation_ = globalState.trackingDerivation;
    var runAsAction = !canRunAsDerivation || !prevDerivation_;
    startBatch();
    var prevAllowStateChanges_ = globalState.allowStateChanges;
    if (runAsAction) {
      untrackedStart();
      prevAllowStateChanges_ = allowStateChangesStart(true);
    }
    var prevAllowStateReads_ = allowStateReadsStart(true);
    var runInfo = {
      runAsAction_: runAsAction,
      prevDerivation_,
      prevAllowStateChanges_,
      prevAllowStateReads_,
      notifySpy_,
      startTime_,
      actionId_: nextActionId++,
      parentActionId_: currentActionId
    };
    currentActionId = runInfo.actionId_;
    return runInfo;
  }
  function _endAction(runInfo) {
    if (currentActionId !== runInfo.actionId_) {
      die(30);
    }
    currentActionId = runInfo.parentActionId_;
    if (runInfo.error_ !== void 0) {
      globalState.suppressReactionErrors = true;
    }
    allowStateChangesEnd(runInfo.prevAllowStateChanges_);
    allowStateReadsEnd(runInfo.prevAllowStateReads_);
    endBatch();
    if (runInfo.runAsAction_)
      untrackedEnd(runInfo.prevDerivation_);
    if (false) {
      spyReportEnd({
        time: Date.now() - runInfo.startTime_
      });
    }
    globalState.suppressReactionErrors = false;
  }
  function allowStateChanges(allowStateChanges2, func) {
    var prev = allowStateChangesStart(allowStateChanges2);
    try {
      return func();
    } finally {
      allowStateChangesEnd(prev);
    }
  }
  function allowStateChangesStart(allowStateChanges2) {
    var prev = globalState.allowStateChanges;
    globalState.allowStateChanges = allowStateChanges2;
    return prev;
  }
  function allowStateChangesEnd(prev) {
    globalState.allowStateChanges = prev;
  }
  function _defineProperties(target, props) {
    for (var i = 0; i < props.length; i++) {
      var descriptor = props[i];
      descriptor.enumerable = descriptor.enumerable || false;
      descriptor.configurable = true;
      if ("value" in descriptor)
        descriptor.writable = true;
      Object.defineProperty(target, descriptor.key, descriptor);
    }
  }
  function _createClass(Constructor, protoProps, staticProps) {
    if (protoProps)
      _defineProperties(Constructor.prototype, protoProps);
    if (staticProps)
      _defineProperties(Constructor, staticProps);
    return Constructor;
  }
  function _extends() {
    _extends = Object.assign || function(target) {
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
  function _inheritsLoose(subClass, superClass) {
    subClass.prototype = Object.create(superClass.prototype);
    subClass.prototype.constructor = subClass;
    subClass.__proto__ = superClass;
  }
  function _assertThisInitialized(self) {
    if (self === void 0) {
      throw new ReferenceError("this hasn't been initialised - super() hasn't been called");
    }
    return self;
  }
  function _unsupportedIterableToArray(o, minLen) {
    if (!o)
      return;
    if (typeof o === "string")
      return _arrayLikeToArray(o, minLen);
    var n = Object.prototype.toString.call(o).slice(8, -1);
    if (n === "Object" && o.constructor)
      n = o.constructor.name;
    if (n === "Map" || n === "Set")
      return Array.from(o);
    if (n === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n))
      return _arrayLikeToArray(o, minLen);
  }
  function _arrayLikeToArray(arr, len) {
    if (len == null || len > arr.length)
      len = arr.length;
    for (var i = 0, arr2 = new Array(len); i < len; i++)
      arr2[i] = arr[i];
    return arr2;
  }
  function _createForOfIteratorHelperLoose(o, allowArrayLike) {
    var it;
    if (typeof Symbol === "undefined" || o[Symbol.iterator] == null) {
      if (Array.isArray(o) || (it = _unsupportedIterableToArray(o)) || allowArrayLike && o && typeof o.length === "number") {
        if (it)
          o = it;
        var i = 0;
        return function() {
          if (i >= o.length)
            return {
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
    it = o[Symbol.iterator]();
    return it.next.bind(it);
  }
  var _Symbol$toPrimitive;
  _Symbol$toPrimitive = Symbol.toPrimitive;
  var ObservableValue = /* @__PURE__ */ function(_Atom) {
    _inheritsLoose(ObservableValue2, _Atom);
    function ObservableValue2(value, enhancer, name_, notifySpy, equals) {
      var _this;
      if (name_ === void 0) {
        name_ = "ObservableValue@" + getNextId();
      }
      if (notifySpy === void 0) {
        notifySpy = true;
      }
      if (equals === void 0) {
        equals = comparer["default"];
      }
      _this = _Atom.call(this, name_) || this;
      _this.enhancer = void 0;
      _this.name_ = void 0;
      _this.equals = void 0;
      _this.hasUnreportedChange_ = false;
      _this.interceptors_ = void 0;
      _this.changeListeners_ = void 0;
      _this.value_ = void 0;
      _this.dehancer = void 0;
      _this.enhancer = enhancer;
      _this.name_ = name_;
      _this.equals = equals;
      _this.value_ = enhancer(value, void 0, name_);
      if (false) {
        spyReport({
          type: CREATE,
          object: _assertThisInitialized(_this),
          observableKind: "value",
          debugObjectName: _this.name_,
          newValue: "" + _this.value_
        });
      }
      return _this;
    }
    var _proto = ObservableValue2.prototype;
    _proto.dehanceValue = function dehanceValue(value) {
      if (this.dehancer !== void 0)
        return this.dehancer(value);
      return value;
    };
    _proto.set = function set4(newValue) {
      var oldValue = this.value_;
      newValue = this.prepareNewValue_(newValue);
      if (newValue !== globalState.UNCHANGED) {
        var notifySpy = isSpyEnabled();
        if (false) {
          spyReportStart({
            type: UPDATE,
            object: this,
            observableKind: "value",
            debugObjectName: this.name_,
            newValue,
            oldValue
          });
        }
        this.setNewValue_(newValue);
        if (false)
          spyReportEnd();
      }
    };
    _proto.prepareNewValue_ = function prepareNewValue_(newValue) {
      checkIfStateModificationsAreAllowed(this);
      if (hasInterceptors(this)) {
        var change = interceptChange(this, {
          object: this,
          type: UPDATE,
          newValue
        });
        if (!change)
          return globalState.UNCHANGED;
        newValue = change.newValue;
      }
      newValue = this.enhancer(newValue, this.value_, this.name_);
      return this.equals(this.value_, newValue) ? globalState.UNCHANGED : newValue;
    };
    _proto.setNewValue_ = function setNewValue_(newValue) {
      var oldValue = this.value_;
      this.value_ = newValue;
      this.reportChanged();
      if (hasListeners(this)) {
        notifyListeners(this, {
          type: UPDATE,
          object: this,
          newValue,
          oldValue
        });
      }
    };
    _proto.get = function get3() {
      this.reportObserved();
      return this.dehanceValue(this.value_);
    };
    _proto.intercept_ = function intercept_(handler) {
      return registerInterceptor(this, handler);
    };
    _proto.observe_ = function observe_(listener, fireImmediately) {
      if (fireImmediately)
        listener({
          observableKind: "value",
          debugObjectName: this.name_,
          object: this,
          type: UPDATE,
          newValue: this.value_,
          oldValue: void 0
        });
      return registerListener(this, listener);
    };
    _proto.raw = function raw() {
      return this.value_;
    };
    _proto.toJSON = function toJSON2() {
      return this.get();
    };
    _proto.toString = function toString2() {
      return this.name_ + "[" + this.value_ + "]";
    };
    _proto.valueOf = function valueOf() {
      return toPrimitive(this.get());
    };
    _proto[_Symbol$toPrimitive] = function() {
      return this.valueOf();
    };
    return ObservableValue2;
  }(Atom);
  var _Symbol$toPrimitive$1;
  _Symbol$toPrimitive$1 = Symbol.toPrimitive;
  var ComputedValue = /* @__PURE__ */ function() {
    function ComputedValue2(options) {
      this.dependenciesState_ = IDerivationState_.NOT_TRACKING_;
      this.observing_ = [];
      this.newObserving_ = null;
      this.isBeingObserved_ = false;
      this.isPendingUnobservation_ = false;
      this.observers_ = new Set();
      this.diffValue_ = 0;
      this.runId_ = 0;
      this.lastAccessedBy_ = 0;
      this.lowestObserverState_ = IDerivationState_.UP_TO_DATE_;
      this.unboundDepsCount_ = 0;
      this.mapid_ = "#" + getNextId();
      this.value_ = new CaughtException(null);
      this.name_ = void 0;
      this.triggeredBy_ = void 0;
      this.isComputing_ = false;
      this.isRunningSetter_ = false;
      this.derivation = void 0;
      this.setter_ = void 0;
      this.isTracing_ = TraceMode.NONE;
      this.scope_ = void 0;
      this.equals_ = void 0;
      this.requiresReaction_ = void 0;
      this.keepAlive_ = void 0;
      this.onBOL = void 0;
      this.onBUOL = void 0;
      if (!options.get)
        die(31);
      this.derivation = options.get;
      this.name_ = options.name || "ComputedValue@" + getNextId();
      if (options.set)
        this.setter_ = createAction(this.name_ + "-setter", options.set);
      this.equals_ = options.equals || (options.compareStructural || options.struct ? comparer.structural : comparer["default"]);
      this.scope_ = options.context;
      this.requiresReaction_ = !!options.requiresReaction;
      this.keepAlive_ = !!options.keepAlive;
    }
    var _proto = ComputedValue2.prototype;
    _proto.onBecomeStale_ = function onBecomeStale_() {
      propagateMaybeChanged(this);
    };
    _proto.onBO = function onBO() {
      if (this.onBOL) {
        this.onBOL.forEach(function(listener) {
          return listener();
        });
      }
    };
    _proto.onBUO = function onBUO() {
      if (this.onBUOL) {
        this.onBUOL.forEach(function(listener) {
          return listener();
        });
      }
    };
    _proto.get = function get3() {
      if (this.isComputing_)
        die(32, this.name_, this.derivation);
      if (globalState.inBatch === 0 && this.observers_.size === 0 && !this.keepAlive_) {
        if (shouldCompute(this)) {
          this.warnAboutUntrackedRead_();
          startBatch();
          this.value_ = this.computeValue_(false);
          endBatch();
        }
      } else {
        reportObserved(this);
        if (shouldCompute(this)) {
          var prevTrackingContext = globalState.trackingContext;
          if (this.keepAlive_ && !prevTrackingContext)
            globalState.trackingContext = this;
          if (this.trackAndCompute())
            propagateChangeConfirmed(this);
          globalState.trackingContext = prevTrackingContext;
        }
      }
      var result = this.value_;
      if (isCaughtException(result))
        throw result.cause;
      return result;
    };
    _proto.set = function set4(value) {
      if (this.setter_) {
        if (this.isRunningSetter_)
          die(33, this.name_);
        this.isRunningSetter_ = true;
        try {
          this.setter_.call(this.scope_, value);
        } finally {
          this.isRunningSetter_ = false;
        }
      } else
        die(34, this.name_);
    };
    _proto.trackAndCompute = function trackAndCompute() {
      var oldValue = this.value_;
      var wasSuspended = this.dependenciesState_ === IDerivationState_.NOT_TRACKING_;
      var newValue = this.computeValue_(true);
      if (false) {
        spyReport({
          observableKind: "computed",
          debugObjectName: this.name_,
          object: this.scope_,
          type: "update",
          oldValue: this.value_,
          newValue
        });
      }
      var changed = wasSuspended || isCaughtException(oldValue) || isCaughtException(newValue) || !this.equals_(oldValue, newValue);
      if (changed) {
        this.value_ = newValue;
      }
      return changed;
    };
    _proto.computeValue_ = function computeValue_(track) {
      this.isComputing_ = true;
      var prev = allowStateChangesStart(false);
      var res;
      if (track) {
        res = trackDerivedFunction(this, this.derivation, this.scope_);
      } else {
        if (globalState.disableErrorBoundaries === true) {
          res = this.derivation.call(this.scope_);
        } else {
          try {
            res = this.derivation.call(this.scope_);
          } catch (e) {
            res = new CaughtException(e);
          }
        }
      }
      allowStateChangesEnd(prev);
      this.isComputing_ = false;
      return res;
    };
    _proto.suspend_ = function suspend_() {
      if (!this.keepAlive_) {
        clearObserving(this);
        this.value_ = void 0;
      }
    };
    _proto.observe_ = function observe_(listener, fireImmediately) {
      var _this = this;
      var firstTime = true;
      var prevValue = void 0;
      return autorun(function() {
        var newValue = _this.get();
        if (!firstTime || fireImmediately) {
          var prevU = untrackedStart();
          listener({
            observableKind: "computed",
            debugObjectName: _this.name_,
            type: UPDATE,
            object: _this,
            newValue,
            oldValue: prevValue
          });
          untrackedEnd(prevU);
        }
        firstTime = false;
        prevValue = newValue;
      });
    };
    _proto.warnAboutUntrackedRead_ = function warnAboutUntrackedRead_() {
      if (true)
        return;
      if (this.requiresReaction_ === true) {
        die("[mobx] Computed value " + this.name_ + " is read outside a reactive context");
      }
      if (this.isTracing_ !== TraceMode.NONE) {
        console.log("[mobx.trace] '" + this.name_ + "' is being read outside a reactive context. Doing a full recompute");
      }
      if (globalState.computedRequiresReaction) {
        console.warn("[mobx] Computed value " + this.name_ + " is being read outside a reactive context. Doing a full recompute");
      }
    };
    _proto.toString = function toString2() {
      return this.name_ + "[" + this.derivation.toString() + "]";
    };
    _proto.valueOf = function valueOf() {
      return toPrimitive(this.get());
    };
    _proto[_Symbol$toPrimitive$1] = function() {
      return this.valueOf();
    };
    return ComputedValue2;
  }();
  var isComputedValue = /* @__PURE__ */ createInstanceofPredicate("ComputedValue", ComputedValue);
  var IDerivationState_;
  (function(IDerivationState_2) {
    IDerivationState_2[IDerivationState_2["NOT_TRACKING_"] = -1] = "NOT_TRACKING_";
    IDerivationState_2[IDerivationState_2["UP_TO_DATE_"] = 0] = "UP_TO_DATE_";
    IDerivationState_2[IDerivationState_2["POSSIBLY_STALE_"] = 1] = "POSSIBLY_STALE_";
    IDerivationState_2[IDerivationState_2["STALE_"] = 2] = "STALE_";
  })(IDerivationState_ || (IDerivationState_ = {}));
  var TraceMode;
  (function(TraceMode2) {
    TraceMode2[TraceMode2["NONE"] = 0] = "NONE";
    TraceMode2[TraceMode2["LOG"] = 1] = "LOG";
    TraceMode2[TraceMode2["BREAK"] = 2] = "BREAK";
  })(TraceMode || (TraceMode = {}));
  var CaughtException = function CaughtException2(cause) {
    this.cause = void 0;
    this.cause = cause;
  };
  function isCaughtException(e) {
    return e instanceof CaughtException;
  }
  function shouldCompute(derivation) {
    switch (derivation.dependenciesState_) {
      case IDerivationState_.UP_TO_DATE_:
        return false;
      case IDerivationState_.NOT_TRACKING_:
      case IDerivationState_.STALE_:
        return true;
      case IDerivationState_.POSSIBLY_STALE_: {
        var prevAllowStateReads = allowStateReadsStart(true);
        var prevUntracked = untrackedStart();
        var obs = derivation.observing_, l = obs.length;
        for (var i = 0; i < l; i++) {
          var obj = obs[i];
          if (isComputedValue(obj)) {
            if (globalState.disableErrorBoundaries) {
              obj.get();
            } else {
              try {
                obj.get();
              } catch (e) {
                untrackedEnd(prevUntracked);
                allowStateReadsEnd(prevAllowStateReads);
                return true;
              }
            }
            if (derivation.dependenciesState_ === IDerivationState_.STALE_) {
              untrackedEnd(prevUntracked);
              allowStateReadsEnd(prevAllowStateReads);
              return true;
            }
          }
        }
        changeDependenciesStateTo0(derivation);
        untrackedEnd(prevUntracked);
        allowStateReadsEnd(prevAllowStateReads);
        return false;
      }
    }
  }
  function checkIfStateModificationsAreAllowed(atom) {
    if (true) {
      return;
    }
    var hasObservers = atom.observers_.size > 0;
    if (!globalState.allowStateChanges && (hasObservers || globalState.enforceActions === "always"))
      console.warn("[MobX] " + (globalState.enforceActions ? "Since strict-mode is enabled, changing (observed) observable values without using an action is not allowed. Tried to modify: " : "Side effects like changing state are not allowed at this point. Are you trying to modify state from, for example, a computed value or the render function of a React component? You can wrap side effects in 'runInAction' (or decorate functions with 'action') if needed. Tried to modify: ") + atom.name_);
  }
  function checkIfStateReadsAreAllowed(observable2) {
    if (false) {
      console.warn("[mobx] Observable " + observable2.name_ + " being read outside a reactive context");
    }
  }
  function trackDerivedFunction(derivation, f, context) {
    var prevAllowStateReads = allowStateReadsStart(true);
    changeDependenciesStateTo0(derivation);
    derivation.newObserving_ = new Array(derivation.observing_.length + 100);
    derivation.unboundDepsCount_ = 0;
    derivation.runId_ = ++globalState.runId;
    var prevTracking = globalState.trackingDerivation;
    globalState.trackingDerivation = derivation;
    globalState.inBatch++;
    var result;
    if (globalState.disableErrorBoundaries === true) {
      result = f.call(context);
    } else {
      try {
        result = f.call(context);
      } catch (e) {
        result = new CaughtException(e);
      }
    }
    globalState.inBatch--;
    globalState.trackingDerivation = prevTracking;
    bindDependencies(derivation);
    warnAboutDerivationWithoutDependencies(derivation);
    allowStateReadsEnd(prevAllowStateReads);
    return result;
  }
  function warnAboutDerivationWithoutDependencies(derivation) {
    if (true)
      return;
    if (derivation.observing_.length !== 0)
      return;
    if (globalState.reactionRequiresObservable || derivation.requiresObservable_) {
      console.warn("[mobx] Derivation " + derivation.name_ + " is created/updated without reading any observable value");
    }
  }
  function bindDependencies(derivation) {
    var prevObserving = derivation.observing_;
    var observing = derivation.observing_ = derivation.newObserving_;
    var lowestNewObservingDerivationState = IDerivationState_.UP_TO_DATE_;
    var i0 = 0, l = derivation.unboundDepsCount_;
    for (var i = 0; i < l; i++) {
      var dep = observing[i];
      if (dep.diffValue_ === 0) {
        dep.diffValue_ = 1;
        if (i0 !== i)
          observing[i0] = dep;
        i0++;
      }
      if (dep.dependenciesState_ > lowestNewObservingDerivationState) {
        lowestNewObservingDerivationState = dep.dependenciesState_;
      }
    }
    observing.length = i0;
    derivation.newObserving_ = null;
    l = prevObserving.length;
    while (l--) {
      var _dep = prevObserving[l];
      if (_dep.diffValue_ === 0) {
        removeObserver(_dep, derivation);
      }
      _dep.diffValue_ = 0;
    }
    while (i0--) {
      var _dep2 = observing[i0];
      if (_dep2.diffValue_ === 1) {
        _dep2.diffValue_ = 0;
        addObserver(_dep2, derivation);
      }
    }
    if (lowestNewObservingDerivationState !== IDerivationState_.UP_TO_DATE_) {
      derivation.dependenciesState_ = lowestNewObservingDerivationState;
      derivation.onBecomeStale_();
    }
  }
  function clearObserving(derivation) {
    var obs = derivation.observing_;
    derivation.observing_ = [];
    var i = obs.length;
    while (i--) {
      removeObserver(obs[i], derivation);
    }
    derivation.dependenciesState_ = IDerivationState_.NOT_TRACKING_;
  }
  function untracked(action2) {
    var prev = untrackedStart();
    try {
      return action2();
    } finally {
      untrackedEnd(prev);
    }
  }
  function untrackedStart() {
    var prev = globalState.trackingDerivation;
    globalState.trackingDerivation = null;
    return prev;
  }
  function untrackedEnd(prev) {
    globalState.trackingDerivation = prev;
  }
  function allowStateReadsStart(allowStateReads) {
    var prev = globalState.allowStateReads;
    globalState.allowStateReads = allowStateReads;
    return prev;
  }
  function allowStateReadsEnd(prev) {
    globalState.allowStateReads = prev;
  }
  function changeDependenciesStateTo0(derivation) {
    if (derivation.dependenciesState_ === IDerivationState_.UP_TO_DATE_)
      return;
    derivation.dependenciesState_ = IDerivationState_.UP_TO_DATE_;
    var obs = derivation.observing_;
    var i = obs.length;
    while (i--) {
      obs[i].lowestObserverState_ = IDerivationState_.UP_TO_DATE_;
    }
  }
  var MobXGlobals = function MobXGlobals2() {
    this.version = 6;
    this.UNCHANGED = {};
    this.trackingDerivation = null;
    this.trackingContext = null;
    this.runId = 0;
    this.mobxGuid = 0;
    this.inBatch = 0;
    this.pendingUnobservations = [];
    this.pendingReactions = [];
    this.isRunningReactions = false;
    this.allowStateChanges = false;
    this.allowStateReads = true;
    this.enforceActions = true;
    this.spyListeners = [];
    this.globalReactionErrorHandlers = [];
    this.computedRequiresReaction = false;
    this.reactionRequiresObservable = false;
    this.observableRequiresReaction = false;
    this.disableErrorBoundaries = false;
    this.suppressReactionErrors = false;
    this.useProxies = true;
    this.verifyProxies = false;
  };
  var canMergeGlobalState = true;
  var isolateCalled = false;
  var globalState = /* @__PURE__ */ function() {
    var global2 = /* @__PURE__ */ getGlobal();
    if (global2.__mobxInstanceCount > 0 && !global2.__mobxGlobals)
      canMergeGlobalState = false;
    if (global2.__mobxGlobals && global2.__mobxGlobals.version !== new MobXGlobals().version)
      canMergeGlobalState = false;
    if (!canMergeGlobalState) {
      setTimeout(function() {
        if (!isolateCalled) {
          die(35);
        }
      }, 1);
      return new MobXGlobals();
    } else if (global2.__mobxGlobals) {
      global2.__mobxInstanceCount += 1;
      if (!global2.__mobxGlobals.UNCHANGED)
        global2.__mobxGlobals.UNCHANGED = {};
      return global2.__mobxGlobals;
    } else {
      global2.__mobxInstanceCount = 1;
      return global2.__mobxGlobals = /* @__PURE__ */ new MobXGlobals();
    }
  }();
  function isolateGlobalState() {
    if (globalState.pendingReactions.length || globalState.inBatch || globalState.isRunningReactions)
      die(36);
    isolateCalled = true;
    if (canMergeGlobalState) {
      var global2 = getGlobal();
      if (--global2.__mobxInstanceCount === 0)
        global2.__mobxGlobals = void 0;
      globalState = new MobXGlobals();
    }
  }
  function addObserver(observable2, node) {
    observable2.observers_.add(node);
    if (observable2.lowestObserverState_ > node.dependenciesState_)
      observable2.lowestObserverState_ = node.dependenciesState_;
  }
  function removeObserver(observable2, node) {
    observable2.observers_["delete"](node);
    if (observable2.observers_.size === 0) {
      queueForUnobservation(observable2);
    }
  }
  function queueForUnobservation(observable2) {
    if (observable2.isPendingUnobservation_ === false) {
      observable2.isPendingUnobservation_ = true;
      globalState.pendingUnobservations.push(observable2);
    }
  }
  function startBatch() {
    globalState.inBatch++;
  }
  function endBatch() {
    if (--globalState.inBatch === 0) {
      runReactions();
      var list = globalState.pendingUnobservations;
      for (var i = 0; i < list.length; i++) {
        var observable2 = list[i];
        observable2.isPendingUnobservation_ = false;
        if (observable2.observers_.size === 0) {
          if (observable2.isBeingObserved_) {
            observable2.isBeingObserved_ = false;
            observable2.onBUO();
          }
          if (observable2 instanceof ComputedValue) {
            observable2.suspend_();
          }
        }
      }
      globalState.pendingUnobservations = [];
    }
  }
  function reportObserved(observable2) {
    checkIfStateReadsAreAllowed(observable2);
    var derivation = globalState.trackingDerivation;
    if (derivation !== null) {
      if (derivation.runId_ !== observable2.lastAccessedBy_) {
        observable2.lastAccessedBy_ = derivation.runId_;
        derivation.newObserving_[derivation.unboundDepsCount_++] = observable2;
        if (!observable2.isBeingObserved_ && globalState.trackingContext) {
          observable2.isBeingObserved_ = true;
          observable2.onBO();
        }
      }
      return true;
    } else if (observable2.observers_.size === 0 && globalState.inBatch > 0) {
      queueForUnobservation(observable2);
    }
    return false;
  }
  function propagateChanged(observable2) {
    if (observable2.lowestObserverState_ === IDerivationState_.STALE_)
      return;
    observable2.lowestObserverState_ = IDerivationState_.STALE_;
    observable2.observers_.forEach(function(d) {
      if (d.dependenciesState_ === IDerivationState_.UP_TO_DATE_) {
        if (false) {
          logTraceInfo(d, observable2);
        }
        d.onBecomeStale_();
      }
      d.dependenciesState_ = IDerivationState_.STALE_;
    });
  }
  function propagateChangeConfirmed(observable2) {
    if (observable2.lowestObserverState_ === IDerivationState_.STALE_)
      return;
    observable2.lowestObserverState_ = IDerivationState_.STALE_;
    observable2.observers_.forEach(function(d) {
      if (d.dependenciesState_ === IDerivationState_.POSSIBLY_STALE_)
        d.dependenciesState_ = IDerivationState_.STALE_;
      else if (d.dependenciesState_ === IDerivationState_.UP_TO_DATE_)
        observable2.lowestObserverState_ = IDerivationState_.UP_TO_DATE_;
    });
  }
  function propagateMaybeChanged(observable2) {
    if (observable2.lowestObserverState_ !== IDerivationState_.UP_TO_DATE_)
      return;
    observable2.lowestObserverState_ = IDerivationState_.POSSIBLY_STALE_;
    observable2.observers_.forEach(function(d) {
      if (d.dependenciesState_ === IDerivationState_.UP_TO_DATE_) {
        d.dependenciesState_ = IDerivationState_.POSSIBLY_STALE_;
        if (false) {
          logTraceInfo(d, observable2);
        }
        d.onBecomeStale_();
      }
    });
  }
  var Reaction = /* @__PURE__ */ function() {
    function Reaction2(name_, onInvalidate_, errorHandler_, requiresObservable_) {
      if (name_ === void 0) {
        name_ = "Reaction@" + getNextId();
      }
      if (requiresObservable_ === void 0) {
        requiresObservable_ = false;
      }
      this.name_ = void 0;
      this.onInvalidate_ = void 0;
      this.errorHandler_ = void 0;
      this.requiresObservable_ = void 0;
      this.observing_ = [];
      this.newObserving_ = [];
      this.dependenciesState_ = IDerivationState_.NOT_TRACKING_;
      this.diffValue_ = 0;
      this.runId_ = 0;
      this.unboundDepsCount_ = 0;
      this.mapid_ = "#" + getNextId();
      this.isDisposed_ = false;
      this.isScheduled_ = false;
      this.isTrackPending_ = false;
      this.isRunning_ = false;
      this.isTracing_ = TraceMode.NONE;
      this.name_ = name_;
      this.onInvalidate_ = onInvalidate_;
      this.errorHandler_ = errorHandler_;
      this.requiresObservable_ = requiresObservable_;
    }
    var _proto = Reaction2.prototype;
    _proto.onBecomeStale_ = function onBecomeStale_() {
      this.schedule_();
    };
    _proto.schedule_ = function schedule_() {
      if (!this.isScheduled_) {
        this.isScheduled_ = true;
        globalState.pendingReactions.push(this);
        runReactions();
      }
    };
    _proto.isScheduled = function isScheduled() {
      return this.isScheduled_;
    };
    _proto.runReaction_ = function runReaction_() {
      if (!this.isDisposed_) {
        startBatch();
        this.isScheduled_ = false;
        if (shouldCompute(this)) {
          this.isTrackPending_ = true;
          try {
            this.onInvalidate_();
            if (false) {
              spyReport({
                name: this.name_,
                type: "scheduled-reaction"
              });
            }
          } catch (e) {
            this.reportExceptionInDerivation_(e);
          }
        }
        endBatch();
      }
    };
    _proto.track = function track(fn) {
      if (this.isDisposed_) {
        return;
      }
      startBatch();
      var notify = isSpyEnabled();
      var startTime;
      if (false) {
        startTime = Date.now();
        spyReportStart({
          name: this.name_,
          type: "reaction"
        });
      }
      this.isRunning_ = true;
      var prevReaction = globalState.trackingContext;
      globalState.trackingContext = this;
      var result = trackDerivedFunction(this, fn, void 0);
      globalState.trackingContext = prevReaction;
      this.isRunning_ = false;
      this.isTrackPending_ = false;
      if (this.isDisposed_) {
        clearObserving(this);
      }
      if (isCaughtException(result))
        this.reportExceptionInDerivation_(result.cause);
      if (false) {
        spyReportEnd({
          time: Date.now() - startTime
        });
      }
      endBatch();
    };
    _proto.reportExceptionInDerivation_ = function reportExceptionInDerivation_(error) {
      var _this = this;
      if (this.errorHandler_) {
        this.errorHandler_(error, this);
        return;
      }
      if (globalState.disableErrorBoundaries)
        throw error;
      var message = "[mobx] uncaught error in '" + this + "'";
      if (!globalState.suppressReactionErrors) {
        console.error(message, error);
      } else if (false)
        console.warn("[mobx] (error in reaction '" + this.name_ + "' suppressed, fix error of causing action below)");
      if (false) {
        spyReport({
          type: "error",
          name: this.name_,
          message,
          error: "" + error
        });
      }
      globalState.globalReactionErrorHandlers.forEach(function(f) {
        return f(error, _this);
      });
    };
    _proto.dispose = function dispose() {
      if (!this.isDisposed_) {
        this.isDisposed_ = true;
        if (!this.isRunning_) {
          startBatch();
          clearObserving(this);
          endBatch();
        }
      }
    };
    _proto.getDisposer_ = function getDisposer_() {
      var r = this.dispose.bind(this);
      r[$mobx] = this;
      return r;
    };
    _proto.toString = function toString2() {
      return "Reaction[" + this.name_ + "]";
    };
    _proto.trace = function trace$1(enterBreakPoint) {
      if (enterBreakPoint === void 0) {
        enterBreakPoint = false;
      }
      trace(this, enterBreakPoint);
    };
    return Reaction2;
  }();
  var MAX_REACTION_ITERATIONS = 100;
  var reactionScheduler = function reactionScheduler2(f) {
    return f();
  };
  function runReactions() {
    if (globalState.inBatch > 0 || globalState.isRunningReactions)
      return;
    reactionScheduler(runReactionsHelper);
  }
  function runReactionsHelper() {
    globalState.isRunningReactions = true;
    var allReactions = globalState.pendingReactions;
    var iterations = 0;
    while (allReactions.length > 0) {
      if (++iterations === MAX_REACTION_ITERATIONS) {
        console.error("[mobx] cycle in reaction: " + allReactions[0]);
        allReactions.splice(0);
      }
      var remainingReactions = allReactions.splice(0);
      for (var i = 0, l = remainingReactions.length; i < l; i++) {
        remainingReactions[i].runReaction_();
      }
    }
    globalState.isRunningReactions = false;
  }
  var isReaction = /* @__PURE__ */ createInstanceofPredicate("Reaction", Reaction);
  function setReactionScheduler(fn) {
    var baseScheduler = reactionScheduler;
    reactionScheduler = function reactionScheduler3(f) {
      return fn(function() {
        return baseScheduler(f);
      });
    };
  }
  function isSpyEnabled() {
    return false;
  }
  function spyReport(event) {
    if (true)
      return;
    if (!globalState.spyListeners.length)
      return;
    var listeners = globalState.spyListeners;
    for (var i = 0, l = listeners.length; i < l; i++) {
      listeners[i](event);
    }
  }
  function spyReportStart(event) {
    if (true)
      return;
    var change = _extends({}, event, {
      spyReportStart: true
    });
    spyReport(change);
  }
  var END_EVENT = {
    type: "report-end",
    spyReportEnd: true
  };
  function spyReportEnd(change) {
    if (true)
      return;
    if (change)
      spyReport(_extends({}, change, {
        type: "report-end",
        spyReportEnd: true
      }));
    else
      spyReport(END_EVENT);
  }
  function spy(listener) {
    if (true) {
      console.warn("[mobx.spy] Is a no-op in production builds");
      return function() {
      };
    } else {
      globalState.spyListeners.push(listener);
      return once(function() {
        globalState.spyListeners = globalState.spyListeners.filter(function(l) {
          return l !== listener;
        });
      });
    }
  }
  var ACTION = "action";
  var ACTION_BOUND = "action.bound";
  var AUTOACTION = "autoAction";
  var AUTOACTION_BOUND = "autoAction.bound";
  var ACTION_UNNAMED = "<unnamed action>";
  function createActionFactory(autoAction2, annotation) {
    var res = function action2(arg1, arg2) {
      if (isFunction(arg1))
        return createAction(arg1.name || ACTION_UNNAMED, arg1, autoAction2);
      if (isFunction(arg2))
        return createAction(arg1, arg2, autoAction2);
      if (isStringish(arg2)) {
        return storeDecorator(arg1, arg2, annotation);
      }
      if (isStringish(arg1)) {
        return createDecoratorAndAnnotation(annotation, arg1);
      }
      if (false)
        die("Invalid arguments for `action`");
    };
    res.annotationType_ = annotation;
    return res;
  }
  var action = /* @__PURE__ */ createActionFactory(false, ACTION);
  var autoAction = /* @__PURE__ */ createActionFactory(true, AUTOACTION);
  action.bound = /* @__PURE__ */ createDecorator(ACTION_BOUND);
  autoAction.bound = /* @__PURE__ */ createDecorator(AUTOACTION_BOUND);
  function isAction(thing) {
    return isFunction(thing) && thing.isMobxAction === true;
  }
  function autorun(view, opts) {
    if (opts === void 0) {
      opts = EMPTY_OBJECT;
    }
    if (false) {
      if (!isFunction(view))
        die("Autorun expects a function as first argument");
      if (isAction(view))
        die("Autorun does not accept actions since actions are untrackable");
    }
    var name = opts && opts.name || view.name || "Autorun@" + getNextId();
    var runSync = !opts.scheduler && !opts.delay;
    var reaction;
    if (runSync) {
      reaction = new Reaction(name, function() {
        this.track(reactionRunner);
      }, opts.onError, opts.requiresObservable);
    } else {
      var scheduler = createSchedulerFromOptions(opts);
      var isScheduled = false;
      reaction = new Reaction(name, function() {
        if (!isScheduled) {
          isScheduled = true;
          scheduler(function() {
            isScheduled = false;
            if (!reaction.isDisposed_)
              reaction.track(reactionRunner);
          });
        }
      }, opts.onError, opts.requiresObservable);
    }
    function reactionRunner() {
      view(reaction);
    }
    reaction.schedule_();
    return reaction.getDisposer_();
  }
  var run = function run2(f) {
    return f();
  };
  function createSchedulerFromOptions(opts) {
    return opts.scheduler ? opts.scheduler : opts.delay ? function(f) {
      return setTimeout(f, opts.delay);
    } : run;
  }
  var ON_BECOME_OBSERVED = "onBO";
  var ON_BECOME_UNOBSERVED = "onBUO";
  function onBecomeObserved(thing, arg2, arg3) {
    return interceptHook(ON_BECOME_OBSERVED, thing, arg2, arg3);
  }
  function onBecomeUnobserved(thing, arg2, arg3) {
    return interceptHook(ON_BECOME_UNOBSERVED, thing, arg2, arg3);
  }
  function interceptHook(hook, thing, arg2, arg3) {
    var atom = typeof arg3 === "function" ? getAtom(thing, arg2) : getAtom(thing);
    var cb = isFunction(arg3) ? arg3 : arg2;
    var listenersKey = hook + "L";
    if (atom[listenersKey]) {
      atom[listenersKey].add(cb);
    } else {
      atom[listenersKey] = new Set([cb]);
    }
    return function() {
      var hookListeners = atom[listenersKey];
      if (hookListeners) {
        hookListeners["delete"](cb);
        if (hookListeners.size === 0) {
          delete atom[listenersKey];
        }
      }
    };
  }
  var NEVER = "never";
  var ALWAYS = "always";
  var OBSERVED = "observed";
  function configure(options) {
    if (options.isolateGlobalState === true) {
      isolateGlobalState();
    }
    var useProxies = options.useProxies, enforceActions = options.enforceActions;
    if (useProxies !== void 0) {
      globalState.useProxies = useProxies === ALWAYS ? true : useProxies === NEVER ? false : typeof Proxy !== "undefined";
    }
    if (useProxies === "ifavailable")
      globalState.verifyProxies = true;
    if (enforceActions !== void 0) {
      var ea = enforceActions === ALWAYS ? ALWAYS : enforceActions === OBSERVED;
      globalState.enforceActions = ea;
      globalState.allowStateChanges = ea === true || ea === ALWAYS ? false : true;
    }
    ["computedRequiresReaction", "reactionRequiresObservable", "observableRequiresReaction", "disableErrorBoundaries"].forEach(function(key) {
      if (key in options)
        globalState[key] = !!options[key];
    });
    globalState.allowStateReads = !globalState.observableRequiresReaction;
    if (false) {
      console.warn("WARNING: Debug feature only. MobX will NOT recover from errors when `disableErrorBoundaries` is enabled.");
    }
    if (options.reactionScheduler) {
      setReactionScheduler(options.reactionScheduler);
    }
  }
  function extendObservable(target, properties, annotations, options) {
    if (false) {
      if (arguments.length > 4)
        die("'extendObservable' expected 2-4 arguments");
      if (typeof target !== "object")
        die("'extendObservable' expects an object as first argument");
      if (isObservableMap(target))
        die("'extendObservable' should not be used on maps, use map.merge instead");
      if (!isPlainObject(properties))
        die("'extendObservabe' only accepts plain objects as second argument");
      if (isObservable(properties) || isObservable(annotations))
        die("Extending an object with another observable (object) is not supported");
    }
    var o = asCreateObservableOptions(options);
    var adm = asObservableObject(target, o.name, getEnhancerFromOption(o));
    startBatch();
    try {
      var descs = getOwnPropertyDescriptors(properties);
      getPlainObjectKeys(descs).forEach(function(key) {
        makeProperty(adm, target, key, descs[key], !annotations ? true : key in annotations ? annotations[key] : true, true, !!(options === null || options === void 0 ? void 0 : options.autoBind));
      });
    } finally {
      endBatch();
    }
    return target;
  }
  function getDependencyTree(thing, property) {
    return nodeToDependencyTree(getAtom(thing, property));
  }
  function nodeToDependencyTree(node) {
    var result = {
      name: node.name_
    };
    if (node.observing_ && node.observing_.length > 0)
      result.dependencies = unique(node.observing_).map(nodeToDependencyTree);
    return result;
  }
  function unique(list) {
    return Array.from(new Set(list));
  }
  var FLOW = "flow";
  var generatorId = 0;
  function FlowCancellationError() {
    this.message = "FLOW_CANCELLED";
  }
  FlowCancellationError.prototype = /* @__PURE__ */ Object.create(Error.prototype);
  var flow = /* @__PURE__ */ Object.assign(function flow2(arg1, arg2) {
    if (isStringish(arg2)) {
      return storeDecorator(arg1, arg2, "flow");
    }
    if (false)
      die("Flow expects 1 argument and cannot be used as decorator");
    var generator = arg1;
    var name = generator.name || "<unnamed flow>";
    var res = function res2() {
      var ctx = this;
      var args = arguments;
      var runId = ++generatorId;
      var gen = action(name + " - runid: " + runId + " - init", generator).apply(ctx, args);
      var rejector;
      var pendingPromise = void 0;
      var promise = new Promise(function(resolve, reject) {
        var stepId = 0;
        rejector = reject;
        function onFulfilled(res3) {
          pendingPromise = void 0;
          var ret;
          try {
            ret = action(name + " - runid: " + runId + " - yield " + stepId++, gen.next).call(gen, res3);
          } catch (e) {
            return reject(e);
          }
          next(ret);
        }
        function onRejected(err) {
          pendingPromise = void 0;
          var ret;
          try {
            ret = action(name + " - runid: " + runId + " - yield " + stepId++, gen["throw"]).call(gen, err);
          } catch (e) {
            return reject(e);
          }
          next(ret);
        }
        function next(ret) {
          if (isFunction(ret === null || ret === void 0 ? void 0 : ret.then)) {
            ret.then(next, reject);
            return;
          }
          if (ret.done)
            return resolve(ret.value);
          pendingPromise = Promise.resolve(ret.value);
          return pendingPromise.then(onFulfilled, onRejected);
        }
        onFulfilled(void 0);
      });
      promise.cancel = action(name + " - runid: " + runId + " - cancel", function() {
        try {
          if (pendingPromise)
            cancelPromise(pendingPromise);
          var _res = gen["return"](void 0);
          var yieldedPromise = Promise.resolve(_res.value);
          yieldedPromise.then(noop, noop);
          cancelPromise(yieldedPromise);
          rejector(new FlowCancellationError());
        } catch (e) {
          rejector(e);
        }
      });
      return promise;
    };
    res.isMobXFlow = true;
    return res;
  }, {
    annotationType_: "flow"
  });
  function cancelPromise(promise) {
    if (isFunction(promise.cancel))
      promise.cancel();
  }
  function isFlow(fn) {
    return (fn === null || fn === void 0 ? void 0 : fn.isMobXFlow) === true;
  }
  function _isObservable(value, property) {
    if (!value)
      return false;
    if (property !== void 0) {
      if (false)
        return die("isObservable(object, propertyName) is not supported for arrays and maps. Use map.has or array.length instead.");
      if (isObservableObject(value)) {
        return value[$mobx].values_.has(property);
      }
      return false;
    }
    return isObservableObject(value) || !!value[$mobx] || isAtom(value) || isReaction(value) || isComputedValue(value);
  }
  function isObservable(value) {
    if (false)
      die("isObservable expects only 1 argument. Use isObservableProp to inspect the observability of a property");
    return _isObservable(value);
  }
  function isObservableProp(value, propName) {
    if (false)
      return die("expected a property name as second argument");
    return _isObservable(value, propName);
  }
  function set2(obj, key, value) {
    if (arguments.length === 2 && !isObservableSet(obj)) {
      startBatch();
      var _values = key;
      try {
        for (var _key in _values) {
          set2(obj, _key, _values[_key]);
        }
      } finally {
        endBatch();
      }
      return;
    }
    if (isObservableObject(obj)) {
      var adm = obj[$mobx];
      var existingObservable = adm.values_.get(key);
      if (existingObservable) {
        adm.write_(key, value);
      } else {
        adm.addObservableProp_(key, value, adm.defaultEnhancer_);
      }
    } else if (isObservableMap(obj)) {
      obj.set(key, value);
    } else if (isObservableSet(obj)) {
      obj.add(key);
    } else if (isObservableArray(obj)) {
      if (typeof key !== "number")
        key = parseInt(key, 10);
      if (key < 0)
        die("Invalid index: '" + key + "'");
      startBatch();
      if (key >= obj.length)
        obj.length = key + 1;
      obj[key] = value;
      endBatch();
    } else
      die(8);
  }
  function trace() {
    if (true)
      die("trace() is not available in production builds");
    var enterBreakPoint = false;
    for (var _len = arguments.length, args = new Array(_len), _key = 0; _key < _len; _key++) {
      args[_key] = arguments[_key];
    }
    if (typeof args[args.length - 1] === "boolean")
      enterBreakPoint = args.pop();
    var derivation = getAtomFromArgs(args);
    if (!derivation) {
      return die("'trace(break?)' can only be used inside a tracked computed value or a Reaction. Consider passing in the computed value or reaction explicitly");
    }
    if (derivation.isTracing_ === TraceMode.NONE) {
      console.log("[mobx.trace] '" + derivation.name_ + "' tracing enabled");
    }
    derivation.isTracing_ = enterBreakPoint ? TraceMode.BREAK : TraceMode.LOG;
  }
  function getAtomFromArgs(args) {
    switch (args.length) {
      case 0:
        return globalState.trackingDerivation;
      case 1:
        return getAtom(args[0]);
      case 2:
        return getAtom(args[0], args[1]);
    }
  }
  function transaction(action2, thisArg) {
    if (thisArg === void 0) {
      thisArg = void 0;
    }
    startBatch();
    try {
      return action2.apply(thisArg);
    } finally {
      endBatch();
    }
  }
  function getAdm(target) {
    return target[$mobx];
  }
  var objectProxyTraps = {
    has: function has(target, name) {
      if (name === $mobx || name === "constructor")
        return true;
      if (false)
        warnAboutProxyRequirement("detect new properties using the 'in' operator. Use 'has' from 'mobx' instead.");
      var adm = getAdm(target);
      if (isStringish(name))
        return adm.has_(name);
      return name in target;
    },
    get: function get(target, name) {
      if (name === $mobx || name === "constructor")
        return target[name];
      var adm = getAdm(target);
      var observable2 = adm.values_.get(name);
      if (observable2 instanceof Atom) {
        var result = observable2.get();
        if (result === void 0) {
          adm.has_(name);
        }
        return result;
      }
      if (isStringish(name))
        adm.has_(name);
      return target[name];
    },
    set: function set$1(target, name, value) {
      if (!isStringish(name))
        return false;
      if (false) {
        warnAboutProxyRequirement("add a new observable property through direct assignment. Use 'set' from 'mobx' instead.");
      }
      set2(target, name, value);
      return true;
    },
    deleteProperty: function deleteProperty(target, name) {
      if (false)
        warnAboutProxyRequirement("delete properties from an observable object. Use 'remove' from 'mobx' instead.");
      if (!isStringish(name))
        return false;
      var adm = getAdm(target);
      adm.remove_(name);
      return true;
    },
    ownKeys: function ownKeys2(target) {
      if (false)
        warnAboutProxyRequirement("iterate keys to detect added / removed properties. Use `keys` from 'mobx' instead.");
      var adm = getAdm(target);
      adm.keysAtom_.reportObserved();
      return Reflect.ownKeys(target);
    },
    preventExtensions: function preventExtensions(target) {
      die(13);
    }
  };
  function createDynamicObservableObject(base) {
    assertProxies();
    var proxy = new Proxy(base, objectProxyTraps);
    base[$mobx].proxy_ = proxy;
    return proxy;
  }
  function hasInterceptors(interceptable) {
    return interceptable.interceptors_ !== void 0 && interceptable.interceptors_.length > 0;
  }
  function registerInterceptor(interceptable, handler) {
    var interceptors = interceptable.interceptors_ || (interceptable.interceptors_ = []);
    interceptors.push(handler);
    return once(function() {
      var idx = interceptors.indexOf(handler);
      if (idx !== -1)
        interceptors.splice(idx, 1);
    });
  }
  function interceptChange(interceptable, change) {
    var prevU = untrackedStart();
    try {
      var interceptors = [].concat(interceptable.interceptors_ || []);
      for (var i = 0, l = interceptors.length; i < l; i++) {
        change = interceptors[i](change);
        if (change && !change.type)
          die(14);
        if (!change)
          break;
      }
      return change;
    } finally {
      untrackedEnd(prevU);
    }
  }
  function hasListeners(listenable) {
    return listenable.changeListeners_ !== void 0 && listenable.changeListeners_.length > 0;
  }
  function registerListener(listenable, handler) {
    var listeners = listenable.changeListeners_ || (listenable.changeListeners_ = []);
    listeners.push(handler);
    return once(function() {
      var idx = listeners.indexOf(handler);
      if (idx !== -1)
        listeners.splice(idx, 1);
    });
  }
  function notifyListeners(listenable, change) {
    var prevU = untrackedStart();
    var listeners = listenable.changeListeners_;
    if (!listeners)
      return;
    listeners = listeners.slice();
    for (var i = 0, l = listeners.length; i < l; i++) {
      listeners[i](change);
    }
    untrackedEnd(prevU);
  }
  var CACHED_ANNOTATIONS = /* @__PURE__ */ Symbol("mobx-cached-annotations");
  function makeAction(target, key, name, fn, asAutoAction) {
    addHiddenProp(target, key, asAutoAction ? autoAction(name || key, fn) : action(name || key, fn));
  }
  function getInferredAnnotation(desc, defaultAnnotation, autoBind) {
    if (desc.get)
      return computed;
    if (desc.set)
      return false;
    if (isFunction(desc.value))
      return isGenerator(desc.value) ? flow : isAction(desc.value) ? false : autoBind ? autoAction.bound : autoAction;
    return defaultAnnotation !== null && defaultAnnotation !== void 0 ? defaultAnnotation : observable.deep;
  }
  function getDescriptorInChain(target, prop) {
    var current = target;
    while (current && current !== objectPrototype) {
      var desc = getDescriptor(current, prop);
      if (desc) {
        return [desc, current];
      }
      current = Object.getPrototypeOf(current);
    }
    die(1, prop);
  }
  function makeProperty(adm, owner, key, descriptor, annotation, forceCopy, autoBind) {
    var _annotation$annotatio;
    var target = adm.target_;
    var defaultAnnotation = observable;
    var originAnnotation = annotation;
    if (annotation === true) {
      annotation = getInferredAnnotation(descriptor, defaultAnnotation, autoBind);
    }
    if (annotation === false) {
      if (forceCopy) {
        defineProperty(target, key, descriptor);
      }
      return;
    }
    if (!annotation || annotation === true || !annotation.annotationType_) {
      return die(2, key);
    }
    var type2 = annotation.annotationType_;
    switch (type2) {
      case AUTOACTION:
      case ACTION: {
        var fn = descriptor.value;
        if (!isFunction(fn))
          die(3, key);
        if (owner !== target && !forceCopy) {
          if (!isAction(owner[key]))
            makeAction(owner, key, annotation.arg_, fn, type2 === AUTOACTION);
        } else {
          makeAction(target, key, annotation.arg_, fn, type2 === AUTOACTION);
        }
        break;
      }
      case AUTOACTION_BOUND:
      case ACTION_BOUND: {
        var _fn = descriptor.value;
        if (!isFunction(_fn))
          die(3, key);
        makeAction(target, key, annotation.arg_, _fn.bind(adm.proxy_ || target), type2 === AUTOACTION_BOUND);
        break;
      }
      case FLOW: {
        if (owner !== target && !forceCopy) {
          if (!isFlow(owner[key]))
            addHiddenProp(owner, key, flow(descriptor.value));
        } else {
          addHiddenProp(target, key, flow(descriptor.value));
        }
        break;
      }
      case COMPUTED:
      case COMPUTED_STRUCT: {
        if (!descriptor.get)
          die(4, key);
        adm.addComputedProp_(target, key, _extends({
          get: descriptor.get,
          set: descriptor.set,
          compareStructural: annotation.annotationType_ === COMPUTED_STRUCT
        }, annotation.arg_));
        break;
      }
      case OBSERVABLE:
      case OBSERVABLE_REF:
      case OBSERVABLE_SHALLOW:
      case OBSERVABLE_STRUCT: {
        if (false)
          die("Cannot decorate '" + key.toString() + "': the property is already decorated as observable.");
        if (false)
          die("Cannot decorate '" + key.toString() + "': observable cannot be used on setter / getter properties.");
        var enhancer = originAnnotation === true ? adm.defaultEnhancer_ : getEnhancerFromAnnotation(annotation);
        adm.addObservableProp_(key, descriptor.value, enhancer);
        break;
      }
      default:
        if (false)
          die("invalid decorator '" + ((_annotation$annotatio = annotation.annotationType_) !== null && _annotation$annotatio !== void 0 ? _annotation$annotatio : annotation) + "' for '" + key.toString() + "'");
    }
  }
  function makeObservable(target, annotations, options) {
    var autoBind = !!(options === null || options === void 0 ? void 0 : options.autoBind);
    var adm = asObservableObject(target, options === null || options === void 0 ? void 0 : options.name, getEnhancerFromAnnotation(options === null || options === void 0 ? void 0 : options.defaultDecorator));
    startBatch();
    try {
      if (!annotations) {
        var didDecorate = applyDecorators(target);
        if (false)
          die("No annotations were passed to makeObservable, but no decorator members have been found either");
        return target;
      }
      var make = function make2(key) {
        var annotation = annotations[key];
        var _getDescriptorInChain = getDescriptorInChain(target, key), desc = _getDescriptorInChain[0], owner = _getDescriptorInChain[1];
        makeProperty(adm, owner, key, desc, annotation, false, autoBind);
      };
      ownKeys(annotations).forEach(make);
    } finally {
      endBatch();
    }
    return target;
  }
  function makeAutoObservable(target, overrides, options) {
    var proto = Object.getPrototypeOf(target);
    var isPlain = proto == null || proto === objectPrototype;
    if (false) {
      if (!isPlain && !isPlainObject(proto))
        die("'makeAutoObservable' can only be used for classes that don't have a superclass");
      if (isObservableObject(target))
        die("makeAutoObservable can only be used on objects not already made observable");
    }
    var annotations;
    if (!isPlain && hasProp(proto, CACHED_ANNOTATIONS)) {
      annotations = proto[CACHED_ANNOTATIONS];
    } else {
      annotations = _extends({}, overrides);
      extractAnnotationsFromObject(target, annotations, options);
      if (!isPlain) {
        extractAnnotationsFromProto(proto, annotations, options);
        addHiddenProp(proto, CACHED_ANNOTATIONS, annotations);
      }
    }
    makeObservable(target, annotations, options);
    return target;
  }
  function extractAnnotationsFromObject(target, collector, options) {
    var _options$defaultDecor;
    var autoBind = !!(options === null || options === void 0 ? void 0 : options.autoBind);
    var defaultAnnotation = (options === null || options === void 0 ? void 0 : options.deep) ? observable.deep : (_options$defaultDecor = options === null || options === void 0 ? void 0 : options.defaultDecorator) !== null && _options$defaultDecor !== void 0 ? _options$defaultDecor : observable.deep;
    Object.entries(getOwnPropertyDescriptors(target)).forEach(function(_ref) {
      var key = _ref[0], descriptor = _ref[1];
      if (key in collector || key === "constructor")
        return;
      collector[key] = getInferredAnnotation(descriptor, defaultAnnotation, autoBind);
    });
  }
  function extractAnnotationsFromProto(proto, collector, options) {
    Object.entries(getOwnPropertyDescriptors(proto)).forEach(function(_ref2) {
      var key = _ref2[0], prop = _ref2[1];
      if (key in collector || key === "constructor")
        return;
      if (prop.get) {
        collector[key] = computed;
      } else if (isFunction(prop.value)) {
        collector[key] = isGenerator(prop.value) ? flow : (options === null || options === void 0 ? void 0 : options.autoBind) ? autoAction.bound : autoAction;
      }
    });
  }
  var SPLICE = "splice";
  var UPDATE = "update";
  var MAX_SPLICE_SIZE = 1e4;
  var arrayTraps = {
    get: function get2(target, name) {
      var adm = target[$mobx];
      if (name === $mobx)
        return adm;
      if (name === "length")
        return adm.getArrayLength_();
      if (typeof name === "string" && !isNaN(name)) {
        return adm.get_(parseInt(name));
      }
      if (hasProp(arrayExtensions, name)) {
        return arrayExtensions[name];
      }
      return target[name];
    },
    set: function set3(target, name, value) {
      var adm = target[$mobx];
      if (name === "length") {
        adm.setArrayLength_(value);
      }
      if (typeof name === "symbol" || isNaN(name)) {
        target[name] = value;
      } else {
        adm.set_(parseInt(name), value);
      }
      return true;
    },
    preventExtensions: function preventExtensions2() {
      die(15);
    }
  };
  var ObservableArrayAdministration = /* @__PURE__ */ function() {
    function ObservableArrayAdministration2(name, enhancer, owned_, legacyMode_) {
      this.owned_ = void 0;
      this.legacyMode_ = void 0;
      this.atom_ = void 0;
      this.values_ = [];
      this.interceptors_ = void 0;
      this.changeListeners_ = void 0;
      this.enhancer_ = void 0;
      this.dehancer = void 0;
      this.proxy_ = void 0;
      this.lastKnownLength_ = 0;
      this.owned_ = owned_;
      this.legacyMode_ = legacyMode_;
      this.atom_ = new Atom(name || "ObservableArray@" + getNextId());
      this.enhancer_ = function(newV, oldV) {
        return enhancer(newV, oldV, name + "[..]");
      };
    }
    var _proto = ObservableArrayAdministration2.prototype;
    _proto.dehanceValue_ = function dehanceValue_(value) {
      if (this.dehancer !== void 0)
        return this.dehancer(value);
      return value;
    };
    _proto.dehanceValues_ = function dehanceValues_(values) {
      if (this.dehancer !== void 0 && values.length > 0)
        return values.map(this.dehancer);
      return values;
    };
    _proto.intercept_ = function intercept_(handler) {
      return registerInterceptor(this, handler);
    };
    _proto.observe_ = function observe_(listener, fireImmediately) {
      if (fireImmediately === void 0) {
        fireImmediately = false;
      }
      if (fireImmediately) {
        listener({
          observableKind: "array",
          object: this.proxy_,
          debugObjectName: this.atom_.name_,
          type: "splice",
          index: 0,
          added: this.values_.slice(),
          addedCount: this.values_.length,
          removed: [],
          removedCount: 0
        });
      }
      return registerListener(this, listener);
    };
    _proto.getArrayLength_ = function getArrayLength_() {
      this.atom_.reportObserved();
      return this.values_.length;
    };
    _proto.setArrayLength_ = function setArrayLength_(newLength) {
      if (typeof newLength !== "number" || newLength < 0)
        die("Out of range: " + newLength);
      var currentLength = this.values_.length;
      if (newLength === currentLength)
        return;
      else if (newLength > currentLength) {
        var newItems = new Array(newLength - currentLength);
        for (var i = 0; i < newLength - currentLength; i++) {
          newItems[i] = void 0;
        }
        this.spliceWithArray_(currentLength, 0, newItems);
      } else
        this.spliceWithArray_(newLength, currentLength - newLength);
    };
    _proto.updateArrayLength_ = function updateArrayLength_(oldLength, delta) {
      if (oldLength !== this.lastKnownLength_)
        die(16);
      this.lastKnownLength_ += delta;
      if (this.legacyMode_ && delta > 0)
        reserveArrayBuffer(oldLength + delta + 1);
    };
    _proto.spliceWithArray_ = function spliceWithArray_(index2, deleteCount, newItems) {
      var _this = this;
      checkIfStateModificationsAreAllowed(this.atom_);
      var length = this.values_.length;
      if (index2 === void 0)
        index2 = 0;
      else if (index2 > length)
        index2 = length;
      else if (index2 < 0)
        index2 = Math.max(0, length + index2);
      if (arguments.length === 1)
        deleteCount = length - index2;
      else if (deleteCount === void 0 || deleteCount === null)
        deleteCount = 0;
      else
        deleteCount = Math.max(0, Math.min(deleteCount, length - index2));
      if (newItems === void 0)
        newItems = EMPTY_ARRAY;
      if (hasInterceptors(this)) {
        var change = interceptChange(this, {
          object: this.proxy_,
          type: SPLICE,
          index: index2,
          removedCount: deleteCount,
          added: newItems
        });
        if (!change)
          return EMPTY_ARRAY;
        deleteCount = change.removedCount;
        newItems = change.added;
      }
      newItems = newItems.length === 0 ? newItems : newItems.map(function(v) {
        return _this.enhancer_(v, void 0);
      });
      if (this.legacyMode_ || false) {
        var lengthDelta = newItems.length - deleteCount;
        this.updateArrayLength_(length, lengthDelta);
      }
      var res = this.spliceItemsIntoValues_(index2, deleteCount, newItems);
      if (deleteCount !== 0 || newItems.length !== 0)
        this.notifyArraySplice_(index2, newItems, res);
      return this.dehanceValues_(res);
    };
    _proto.spliceItemsIntoValues_ = function spliceItemsIntoValues_(index2, deleteCount, newItems) {
      if (newItems.length < MAX_SPLICE_SIZE) {
        var _this$values_;
        return (_this$values_ = this.values_).splice.apply(_this$values_, [index2, deleteCount].concat(newItems));
      } else {
        var res = this.values_.slice(index2, index2 + deleteCount);
        var oldItems = this.values_.slice(index2 + deleteCount);
        this.values_.length = index2 + newItems.length - deleteCount;
        for (var i = 0; i < newItems.length; i++) {
          this.values_[index2 + i] = newItems[i];
        }
        for (var _i = 0; _i < oldItems.length; _i++) {
          this.values_[index2 + newItems.length + _i] = oldItems[_i];
        }
        return res;
      }
    };
    _proto.notifyArrayChildUpdate_ = function notifyArrayChildUpdate_(index2, newValue, oldValue) {
      var notifySpy = !this.owned_ && isSpyEnabled();
      var notify = hasListeners(this);
      var change = notify || notifySpy ? {
        observableKind: "array",
        object: this.proxy_,
        type: UPDATE,
        debugObjectName: this.atom_.name_,
        index: index2,
        newValue,
        oldValue
      } : null;
      if (false)
        spyReportStart(change);
      this.atom_.reportChanged();
      if (notify)
        notifyListeners(this, change);
      if (false)
        spyReportEnd();
    };
    _proto.notifyArraySplice_ = function notifyArraySplice_(index2, added, removed) {
      var notifySpy = !this.owned_ && isSpyEnabled();
      var notify = hasListeners(this);
      var change = notify || notifySpy ? {
        observableKind: "array",
        object: this.proxy_,
        debugObjectName: this.atom_.name_,
        type: SPLICE,
        index: index2,
        removed,
        added,
        removedCount: removed.length,
        addedCount: added.length
      } : null;
      if (false)
        spyReportStart(change);
      this.atom_.reportChanged();
      if (notify)
        notifyListeners(this, change);
      if (false)
        spyReportEnd();
    };
    _proto.get_ = function get_(index2) {
      if (index2 < this.values_.length) {
        this.atom_.reportObserved();
        return this.dehanceValue_(this.values_[index2]);
      }
      console.warn("[mobx.array] Attempt to read an array index (" + index2 + ") that is out of bounds (" + this.values_.length + "). Please check length first. Out of bound indices will not be tracked by MobX");
    };
    _proto.set_ = function set_(index2, newValue) {
      var values = this.values_;
      if (index2 < values.length) {
        checkIfStateModificationsAreAllowed(this.atom_);
        var oldValue = values[index2];
        if (hasInterceptors(this)) {
          var change = interceptChange(this, {
            type: UPDATE,
            object: this.proxy_,
            index: index2,
            newValue
          });
          if (!change)
            return;
          newValue = change.newValue;
        }
        newValue = this.enhancer_(newValue, oldValue);
        var changed = newValue !== oldValue;
        if (changed) {
          values[index2] = newValue;
          this.notifyArrayChildUpdate_(index2, newValue, oldValue);
        }
      } else if (index2 === values.length) {
        this.spliceWithArray_(index2, 0, [newValue]);
      } else {
        die(17, index2, values.length);
      }
    };
    return ObservableArrayAdministration2;
  }();
  function createObservableArray(initialValues, enhancer, name, owned) {
    if (name === void 0) {
      name = "ObservableArray@" + getNextId();
    }
    if (owned === void 0) {
      owned = false;
    }
    assertProxies();
    var adm = new ObservableArrayAdministration(name, enhancer, owned, false);
    addHiddenFinalProp(adm.values_, $mobx, adm);
    var proxy = new Proxy(adm.values_, arrayTraps);
    adm.proxy_ = proxy;
    if (initialValues && initialValues.length) {
      var prev = allowStateChangesStart(true);
      adm.spliceWithArray_(0, 0, initialValues);
      allowStateChangesEnd(prev);
    }
    return proxy;
  }
  var arrayExtensions = {
    clear: function clear() {
      return this.splice(0);
    },
    replace: function replace(newItems) {
      var adm = this[$mobx];
      return adm.spliceWithArray_(0, adm.values_.length, newItems);
    },
    toJSON: function toJSON() {
      return this.slice();
    },
    splice: function splice(index2, deleteCount) {
      for (var _len = arguments.length, newItems = new Array(_len > 2 ? _len - 2 : 0), _key = 2; _key < _len; _key++) {
        newItems[_key - 2] = arguments[_key];
      }
      var adm = this[$mobx];
      switch (arguments.length) {
        case 0:
          return [];
        case 1:
          return adm.spliceWithArray_(index2);
        case 2:
          return adm.spliceWithArray_(index2, deleteCount);
      }
      return adm.spliceWithArray_(index2, deleteCount, newItems);
    },
    spliceWithArray: function spliceWithArray(index2, deleteCount, newItems) {
      return this[$mobx].spliceWithArray_(index2, deleteCount, newItems);
    },
    push: function push() {
      var adm = this[$mobx];
      for (var _len2 = arguments.length, items = new Array(_len2), _key2 = 0; _key2 < _len2; _key2++) {
        items[_key2] = arguments[_key2];
      }
      adm.spliceWithArray_(adm.values_.length, 0, items);
      return adm.values_.length;
    },
    pop: function pop() {
      return this.splice(Math.max(this[$mobx].values_.length - 1, 0), 1)[0];
    },
    shift: function shift() {
      return this.splice(0, 1)[0];
    },
    unshift: function unshift() {
      var adm = this[$mobx];
      for (var _len3 = arguments.length, items = new Array(_len3), _key3 = 0; _key3 < _len3; _key3++) {
        items[_key3] = arguments[_key3];
      }
      adm.spliceWithArray_(0, 0, items);
      return adm.values_.length;
    },
    reverse: function reverse() {
      if (globalState.trackingDerivation) {
        die(37, "reverse");
      }
      this.replace(this.slice().reverse());
      return this;
    },
    sort: function sort() {
      if (globalState.trackingDerivation) {
        die(37, "sort");
      }
      var copy = this.slice();
      copy.sort.apply(copy, arguments);
      this.replace(copy);
      return this;
    },
    remove: function remove(value) {
      var adm = this[$mobx];
      var idx = adm.dehanceValues_(adm.values_).indexOf(value);
      if (idx > -1) {
        this.splice(idx, 1);
        return true;
      }
      return false;
    }
  };
  addArrayExtension("concat", simpleFunc);
  addArrayExtension("flat", simpleFunc);
  addArrayExtension("includes", simpleFunc);
  addArrayExtension("indexOf", simpleFunc);
  addArrayExtension("join", simpleFunc);
  addArrayExtension("lastIndexOf", simpleFunc);
  addArrayExtension("slice", simpleFunc);
  addArrayExtension("toString", simpleFunc);
  addArrayExtension("toLocaleString", simpleFunc);
  addArrayExtension("every", mapLikeFunc);
  addArrayExtension("filter", mapLikeFunc);
  addArrayExtension("find", mapLikeFunc);
  addArrayExtension("findIndex", mapLikeFunc);
  addArrayExtension("flatMap", mapLikeFunc);
  addArrayExtension("forEach", mapLikeFunc);
  addArrayExtension("map", mapLikeFunc);
  addArrayExtension("some", mapLikeFunc);
  addArrayExtension("reduce", reduceLikeFunc);
  addArrayExtension("reduceRight", reduceLikeFunc);
  function addArrayExtension(funcName, funcFactory) {
    if (typeof Array.prototype[funcName] === "function") {
      arrayExtensions[funcName] = funcFactory(funcName);
    }
  }
  function simpleFunc(funcName) {
    return function() {
      var adm = this[$mobx];
      adm.atom_.reportObserved();
      var dehancedValues = adm.dehanceValues_(adm.values_);
      return dehancedValues[funcName].apply(dehancedValues, arguments);
    };
  }
  function mapLikeFunc(funcName) {
    return function(callback, thisArg) {
      var _this2 = this;
      var adm = this[$mobx];
      adm.atom_.reportObserved();
      var dehancedValues = adm.dehanceValues_(adm.values_);
      return dehancedValues[funcName](function(element, index2) {
        return callback.call(thisArg, element, index2, _this2);
      });
    };
  }
  function reduceLikeFunc(funcName) {
    return function() {
      var _this3 = this;
      var adm = this[$mobx];
      adm.atom_.reportObserved();
      var dehancedValues = adm.dehanceValues_(adm.values_);
      var callback = arguments[0];
      arguments[0] = function(accumulator, currentValue, index2) {
        return callback(accumulator, currentValue, index2, _this3);
      };
      return dehancedValues[funcName].apply(dehancedValues, arguments);
    };
  }
  var isObservableArrayAdministration = /* @__PURE__ */ createInstanceofPredicate("ObservableArrayAdministration", ObservableArrayAdministration);
  function isObservableArray(thing) {
    return isObject(thing) && isObservableArrayAdministration(thing[$mobx]);
  }
  var _Symbol$iterator;
  var _Symbol$toStringTag;
  var ObservableMapMarker = {};
  var ADD = "add";
  var DELETE = "delete";
  _Symbol$iterator = Symbol.iterator;
  _Symbol$toStringTag = Symbol.toStringTag;
  var ObservableMap = /* @__PURE__ */ function() {
    function ObservableMap2(initialData, enhancer_, name_) {
      if (enhancer_ === void 0) {
        enhancer_ = deepEnhancer;
      }
      if (name_ === void 0) {
        name_ = "ObservableMap@" + getNextId();
      }
      this.enhancer_ = void 0;
      this.name_ = void 0;
      this[$mobx] = ObservableMapMarker;
      this.data_ = void 0;
      this.hasMap_ = void 0;
      this.keysAtom_ = void 0;
      this.interceptors_ = void 0;
      this.changeListeners_ = void 0;
      this.dehancer = void 0;
      this.enhancer_ = enhancer_;
      this.name_ = name_;
      if (!isFunction(Map)) {
        die(18);
      }
      this.keysAtom_ = createAtom(this.name_ + ".keys()");
      this.data_ = new Map();
      this.hasMap_ = new Map();
      this.merge(initialData);
    }
    var _proto = ObservableMap2.prototype;
    _proto.has_ = function has_(key) {
      return this.data_.has(key);
    };
    _proto.has = function has2(key) {
      var _this = this;
      if (!globalState.trackingDerivation)
        return this.has_(key);
      var entry = this.hasMap_.get(key);
      if (!entry) {
        var newEntry = entry = new ObservableValue(this.has_(key), referenceEnhancer, this.name_ + "." + stringifyKey(key) + "?", false);
        this.hasMap_.set(key, newEntry);
        onBecomeUnobserved(newEntry, function() {
          return _this.hasMap_["delete"](key);
        });
      }
      return entry.get();
    };
    _proto.set = function set4(key, value) {
      var hasKey = this.has_(key);
      if (hasInterceptors(this)) {
        var change = interceptChange(this, {
          type: hasKey ? UPDATE : ADD,
          object: this,
          newValue: value,
          name: key
        });
        if (!change)
          return this;
        value = change.newValue;
      }
      if (hasKey) {
        this.updateValue_(key, value);
      } else {
        this.addValue_(key, value);
      }
      return this;
    };
    _proto["delete"] = function _delete(key) {
      var _this2 = this;
      checkIfStateModificationsAreAllowed(this.keysAtom_);
      if (hasInterceptors(this)) {
        var change = interceptChange(this, {
          type: DELETE,
          object: this,
          name: key
        });
        if (!change)
          return false;
      }
      if (this.has_(key)) {
        var notifySpy = isSpyEnabled();
        var notify = hasListeners(this);
        var _change = notify || notifySpy ? {
          observableKind: "map",
          debugObjectName: this.name_,
          type: DELETE,
          object: this,
          oldValue: this.data_.get(key).value_,
          name: key
        } : null;
        if (false)
          spyReportStart(_change);
        transaction(function() {
          _this2.keysAtom_.reportChanged();
          _this2.updateHasMapEntry_(key, false);
          var observable2 = _this2.data_.get(key);
          observable2.setNewValue_(void 0);
          _this2.data_["delete"](key);
        });
        if (notify)
          notifyListeners(this, _change);
        if (false)
          spyReportEnd();
        return true;
      }
      return false;
    };
    _proto.updateHasMapEntry_ = function updateHasMapEntry_(key, value) {
      var entry = this.hasMap_.get(key);
      if (entry) {
        entry.setNewValue_(value);
      }
    };
    _proto.updateValue_ = function updateValue_(key, newValue) {
      var observable2 = this.data_.get(key);
      newValue = observable2.prepareNewValue_(newValue);
      if (newValue !== globalState.UNCHANGED) {
        var notifySpy = isSpyEnabled();
        var notify = hasListeners(this);
        var change = notify || notifySpy ? {
          observableKind: "map",
          debugObjectName: this.name_,
          type: UPDATE,
          object: this,
          oldValue: observable2.value_,
          name: key,
          newValue
        } : null;
        if (false)
          spyReportStart(change);
        observable2.setNewValue_(newValue);
        if (notify)
          notifyListeners(this, change);
        if (false)
          spyReportEnd();
      }
    };
    _proto.addValue_ = function addValue_(key, newValue) {
      var _this3 = this;
      checkIfStateModificationsAreAllowed(this.keysAtom_);
      transaction(function() {
        var observable2 = new ObservableValue(newValue, _this3.enhancer_, _this3.name_ + "." + stringifyKey(key), false);
        _this3.data_.set(key, observable2);
        newValue = observable2.value_;
        _this3.updateHasMapEntry_(key, true);
        _this3.keysAtom_.reportChanged();
      });
      var notifySpy = isSpyEnabled();
      var notify = hasListeners(this);
      var change = notify || notifySpy ? {
        observableKind: "map",
        debugObjectName: this.name_,
        type: ADD,
        object: this,
        name: key,
        newValue
      } : null;
      if (false)
        spyReportStart(change);
      if (notify)
        notifyListeners(this, change);
      if (false)
        spyReportEnd();
    };
    _proto.get = function get3(key) {
      if (this.has(key))
        return this.dehanceValue_(this.data_.get(key).get());
      return this.dehanceValue_(void 0);
    };
    _proto.dehanceValue_ = function dehanceValue_(value) {
      if (this.dehancer !== void 0) {
        return this.dehancer(value);
      }
      return value;
    };
    _proto.keys = function keys() {
      this.keysAtom_.reportObserved();
      return this.data_.keys();
    };
    _proto.values = function values() {
      var self = this;
      var keys = this.keys();
      return makeIterable({
        next: function next() {
          var _keys$next = keys.next(), done = _keys$next.done, value = _keys$next.value;
          return {
            done,
            value: done ? void 0 : self.get(value)
          };
        }
      });
    };
    _proto.entries = function entries() {
      var self = this;
      var keys = this.keys();
      return makeIterable({
        next: function next() {
          var _keys$next2 = keys.next(), done = _keys$next2.done, value = _keys$next2.value;
          return {
            done,
            value: done ? void 0 : [value, self.get(value)]
          };
        }
      });
    };
    _proto[_Symbol$iterator] = function() {
      return this.entries();
    };
    _proto.forEach = function forEach(callback, thisArg) {
      for (var _iterator = _createForOfIteratorHelperLoose(this), _step; !(_step = _iterator()).done; ) {
        var _step$value = _step.value, key = _step$value[0], value = _step$value[1];
        callback.call(thisArg, value, key, this);
      }
    };
    _proto.merge = function merge2(other) {
      var _this4 = this;
      if (isObservableMap(other)) {
        other = new Map(other);
      }
      transaction(function() {
        if (isPlainObject(other))
          getPlainObjectKeys(other).forEach(function(key) {
            return _this4.set(key, other[key]);
          });
        else if (Array.isArray(other))
          other.forEach(function(_ref) {
            var key = _ref[0], value = _ref[1];
            return _this4.set(key, value);
          });
        else if (isES6Map(other)) {
          if (other.constructor !== Map)
            die(19, other);
          other.forEach(function(value, key) {
            return _this4.set(key, value);
          });
        } else if (other !== null && other !== void 0)
          die(20, other);
      });
      return this;
    };
    _proto.clear = function clear2() {
      var _this5 = this;
      transaction(function() {
        untracked(function() {
          for (var _iterator2 = _createForOfIteratorHelperLoose(_this5.keys()), _step2; !(_step2 = _iterator2()).done; ) {
            var key = _step2.value;
            _this5["delete"](key);
          }
        });
      });
    };
    _proto.replace = function replace2(values) {
      var _this6 = this;
      transaction(function() {
        var replacementMap = convertToMap(values);
        var orderedData = new Map();
        var keysReportChangedCalled = false;
        for (var _iterator3 = _createForOfIteratorHelperLoose(_this6.data_.keys()), _step3; !(_step3 = _iterator3()).done; ) {
          var key = _step3.value;
          if (!replacementMap.has(key)) {
            var deleted = _this6["delete"](key);
            if (deleted) {
              keysReportChangedCalled = true;
            } else {
              var value = _this6.data_.get(key);
              orderedData.set(key, value);
            }
          }
        }
        for (var _iterator4 = _createForOfIteratorHelperLoose(replacementMap.entries()), _step4; !(_step4 = _iterator4()).done; ) {
          var _step4$value = _step4.value, _key = _step4$value[0], _value = _step4$value[1];
          var keyExisted = _this6.data_.has(_key);
          _this6.set(_key, _value);
          if (_this6.data_.has(_key)) {
            var _value2 = _this6.data_.get(_key);
            orderedData.set(_key, _value2);
            if (!keyExisted) {
              keysReportChangedCalled = true;
            }
          }
        }
        if (!keysReportChangedCalled) {
          if (_this6.data_.size !== orderedData.size) {
            _this6.keysAtom_.reportChanged();
          } else {
            var iter1 = _this6.data_.keys();
            var iter2 = orderedData.keys();
            var next1 = iter1.next();
            var next2 = iter2.next();
            while (!next1.done) {
              if (next1.value !== next2.value) {
                _this6.keysAtom_.reportChanged();
                break;
              }
              next1 = iter1.next();
              next2 = iter2.next();
            }
          }
        }
        _this6.data_ = orderedData;
      });
      return this;
    };
    _proto.toString = function toString2() {
      return "[object ObservableMap]";
    };
    _proto.toJSON = function toJSON2() {
      return Array.from(this);
    };
    _proto.observe_ = function observe_(listener, fireImmediately) {
      if (false)
        die("`observe` doesn't support fireImmediately=true in combination with maps.");
      return registerListener(this, listener);
    };
    _proto.intercept_ = function intercept_(handler) {
      return registerInterceptor(this, handler);
    };
    _createClass(ObservableMap2, [{
      key: "size",
      get: function get3() {
        this.keysAtom_.reportObserved();
        return this.data_.size;
      }
    }, {
      key: _Symbol$toStringTag,
      get: function get3() {
        return "Map";
      }
    }]);
    return ObservableMap2;
  }();
  var isObservableMap = /* @__PURE__ */ createInstanceofPredicate("ObservableMap", ObservableMap);
  function convertToMap(dataStructure) {
    if (isES6Map(dataStructure) || isObservableMap(dataStructure)) {
      return dataStructure;
    } else if (Array.isArray(dataStructure)) {
      return new Map(dataStructure);
    } else if (isPlainObject(dataStructure)) {
      var map2 = new Map();
      for (var key in dataStructure) {
        map2.set(key, dataStructure[key]);
      }
      return map2;
    } else {
      return die(21, dataStructure);
    }
  }
  var _Symbol$iterator$1;
  var _Symbol$toStringTag$1;
  var ObservableSetMarker = {};
  _Symbol$iterator$1 = Symbol.iterator;
  _Symbol$toStringTag$1 = Symbol.toStringTag;
  var ObservableSet = /* @__PURE__ */ function() {
    function ObservableSet2(initialData, enhancer, name_) {
      if (enhancer === void 0) {
        enhancer = deepEnhancer;
      }
      if (name_ === void 0) {
        name_ = "ObservableSet@" + getNextId();
      }
      this.name_ = void 0;
      this[$mobx] = ObservableSetMarker;
      this.data_ = new Set();
      this.atom_ = void 0;
      this.changeListeners_ = void 0;
      this.interceptors_ = void 0;
      this.dehancer = void 0;
      this.enhancer_ = void 0;
      this.name_ = name_;
      if (!isFunction(Set)) {
        die(22);
      }
      this.atom_ = createAtom(this.name_);
      this.enhancer_ = function(newV, oldV) {
        return enhancer(newV, oldV, name_);
      };
      if (initialData) {
        this.replace(initialData);
      }
    }
    var _proto = ObservableSet2.prototype;
    _proto.dehanceValue_ = function dehanceValue_(value) {
      if (this.dehancer !== void 0) {
        return this.dehancer(value);
      }
      return value;
    };
    _proto.clear = function clear2() {
      var _this = this;
      transaction(function() {
        untracked(function() {
          for (var _iterator = _createForOfIteratorHelperLoose(_this.data_.values()), _step; !(_step = _iterator()).done; ) {
            var value = _step.value;
            _this["delete"](value);
          }
        });
      });
    };
    _proto.forEach = function forEach(callbackFn, thisArg) {
      for (var _iterator2 = _createForOfIteratorHelperLoose(this), _step2; !(_step2 = _iterator2()).done; ) {
        var value = _step2.value;
        callbackFn.call(thisArg, value, value, this);
      }
    };
    _proto.add = function add(value) {
      var _this2 = this;
      checkIfStateModificationsAreAllowed(this.atom_);
      if (hasInterceptors(this)) {
        var change = interceptChange(this, {
          type: ADD,
          object: this,
          newValue: value
        });
        if (!change)
          return this;
      }
      if (!this.has(value)) {
        transaction(function() {
          _this2.data_.add(_this2.enhancer_(value, void 0));
          _this2.atom_.reportChanged();
        });
        var notifySpy = false;
        var notify = hasListeners(this);
        var _change = notify || notifySpy ? {
          observableKind: "set",
          debugObjectName: this.name_,
          type: ADD,
          object: this,
          newValue: value
        } : null;
        if (notifySpy && false)
          spyReportStart(_change);
        if (notify)
          notifyListeners(this, _change);
        if (notifySpy && false)
          spyReportEnd();
      }
      return this;
    };
    _proto["delete"] = function _delete(value) {
      var _this3 = this;
      if (hasInterceptors(this)) {
        var change = interceptChange(this, {
          type: DELETE,
          object: this,
          oldValue: value
        });
        if (!change)
          return false;
      }
      if (this.has(value)) {
        var notifySpy = false;
        var notify = hasListeners(this);
        var _change2 = notify || notifySpy ? {
          observableKind: "set",
          debugObjectName: this.name_,
          type: DELETE,
          object: this,
          oldValue: value
        } : null;
        if (notifySpy && false)
          spyReportStart(_change2);
        transaction(function() {
          _this3.atom_.reportChanged();
          _this3.data_["delete"](value);
        });
        if (notify)
          notifyListeners(this, _change2);
        if (notifySpy && false)
          spyReportEnd();
        return true;
      }
      return false;
    };
    _proto.has = function has2(value) {
      this.atom_.reportObserved();
      return this.data_.has(this.dehanceValue_(value));
    };
    _proto.entries = function entries() {
      var nextIndex = 0;
      var keys = Array.from(this.keys());
      var values = Array.from(this.values());
      return makeIterable({
        next: function next() {
          var index2 = nextIndex;
          nextIndex += 1;
          return index2 < values.length ? {
            value: [keys[index2], values[index2]],
            done: false
          } : {
            done: true
          };
        }
      });
    };
    _proto.keys = function keys() {
      return this.values();
    };
    _proto.values = function values() {
      this.atom_.reportObserved();
      var self = this;
      var nextIndex = 0;
      var observableValues = Array.from(this.data_.values());
      return makeIterable({
        next: function next() {
          return nextIndex < observableValues.length ? {
            value: self.dehanceValue_(observableValues[nextIndex++]),
            done: false
          } : {
            done: true
          };
        }
      });
    };
    _proto.replace = function replace2(other) {
      var _this4 = this;
      if (isObservableSet(other)) {
        other = new Set(other);
      }
      transaction(function() {
        if (Array.isArray(other)) {
          _this4.clear();
          other.forEach(function(value) {
            return _this4.add(value);
          });
        } else if (isES6Set(other)) {
          _this4.clear();
          other.forEach(function(value) {
            return _this4.add(value);
          });
        } else if (other !== null && other !== void 0) {
          die("Cannot initialize set from " + other);
        }
      });
      return this;
    };
    _proto.observe_ = function observe_(listener, fireImmediately) {
      if (false)
        die("`observe` doesn't support fireImmediately=true in combination with sets.");
      return registerListener(this, listener);
    };
    _proto.intercept_ = function intercept_(handler) {
      return registerInterceptor(this, handler);
    };
    _proto.toJSON = function toJSON2() {
      return Array.from(this);
    };
    _proto.toString = function toString2() {
      return "[object ObservableSet]";
    };
    _proto[_Symbol$iterator$1] = function() {
      return this.values();
    };
    _createClass(ObservableSet2, [{
      key: "size",
      get: function get3() {
        this.atom_.reportObserved();
        return this.data_.size;
      }
    }, {
      key: _Symbol$toStringTag$1,
      get: function get3() {
        return "Set";
      }
    }]);
    return ObservableSet2;
  }();
  var isObservableSet = /* @__PURE__ */ createInstanceofPredicate("ObservableSet", ObservableSet);
  var REMOVE = "remove";
  var ObservableObjectAdministration = /* @__PURE__ */ function() {
    function ObservableObjectAdministration2(target_, values_, name_, defaultEnhancer_) {
      if (values_ === void 0) {
        values_ = new Map();
      }
      this.target_ = void 0;
      this.values_ = void 0;
      this.name_ = void 0;
      this.defaultEnhancer_ = void 0;
      this.keysAtom_ = void 0;
      this.changeListeners_ = void 0;
      this.interceptors_ = void 0;
      this.proxy_ = void 0;
      this.pendingKeys_ = void 0;
      this.keysValue_ = [];
      this.isStaledKeysValue_ = true;
      this.target_ = target_;
      this.values_ = values_;
      this.name_ = name_;
      this.defaultEnhancer_ = defaultEnhancer_;
      this.keysAtom_ = new Atom(name_ + ".keys");
    }
    var _proto = ObservableObjectAdministration2.prototype;
    _proto.read_ = function read_(key) {
      return this.values_.get(key).get();
    };
    _proto.write_ = function write_(key, newValue) {
      var instance = this.target_;
      var observable2 = this.values_.get(key);
      if (observable2 instanceof ComputedValue) {
        observable2.set(newValue);
        return;
      }
      if (hasInterceptors(this)) {
        var change = interceptChange(this, {
          type: UPDATE,
          object: this.proxy_ || instance,
          name: key,
          newValue
        });
        if (!change)
          return;
        newValue = change.newValue;
      }
      newValue = observable2.prepareNewValue_(newValue);
      if (newValue !== globalState.UNCHANGED) {
        var notify = hasListeners(this);
        var notifySpy = false;
        var _change = notify || notifySpy ? {
          type: UPDATE,
          observableKind: "object",
          debugObjectName: this.name_,
          object: this.proxy_ || instance,
          oldValue: observable2.value_,
          name: key,
          newValue
        } : null;
        if (false)
          spyReportStart(_change);
        observable2.setNewValue_(newValue);
        if (notify)
          notifyListeners(this, _change);
        if (false)
          spyReportEnd();
      }
    };
    _proto.has_ = function has_(key) {
      var map2 = this.pendingKeys_ || (this.pendingKeys_ = new Map());
      var entry = map2.get(key);
      if (entry)
        return entry.get();
      else {
        var exists = !!this.values_.get(key);
        entry = new ObservableValue(exists, referenceEnhancer, this.name_ + "." + stringifyKey(key) + "?", false);
        map2.set(key, entry);
        return entry.get();
      }
    };
    _proto.addObservableProp_ = function addObservableProp_(propName, newValue, enhancer) {
      if (enhancer === void 0) {
        enhancer = this.defaultEnhancer_;
      }
      var target = this.target_;
      if (false)
        assertPropertyConfigurable(target, propName);
      if (hasInterceptors(this)) {
        var change = interceptChange(this, {
          object: this.proxy_ || target,
          name: propName,
          type: ADD,
          newValue
        });
        if (!change)
          return;
        newValue = change.newValue;
      }
      var observable2 = new ObservableValue(newValue, enhancer, this.name_ + "." + stringifyKey(propName), false);
      this.values_.set(propName, observable2);
      newValue = observable2.value_;
      defineProperty(target, propName, generateObservablePropConfig(propName));
      this.notifyPropertyAddition_(propName, newValue);
    };
    _proto.addComputedProp_ = function addComputedProp_(propertyOwner, propName, options) {
      var target = this.target_;
      options.name = options.name || this.name_ + "." + stringifyKey(propName);
      options.context = this.proxy_ || target;
      this.values_.set(propName, new ComputedValue(options));
      defineProperty(propertyOwner, propName, generateComputedPropConfig(propName));
    };
    _proto.remove_ = function remove_(key) {
      if (!this.values_.has(key))
        return;
      var target = this.target_;
      if (hasInterceptors(this)) {
        var change = interceptChange(this, {
          object: this.proxy_ || target,
          name: key,
          type: REMOVE
        });
        if (!change)
          return;
      }
      try {
        startBatch();
        var notify = hasListeners(this);
        var notifySpy = false;
        var oldObservable = this.values_.get(key);
        var oldValue = oldObservable && oldObservable.get();
        oldObservable && oldObservable.set(void 0);
        this.reportKeysChanged();
        this.values_["delete"](key);
        if (this.pendingKeys_) {
          var entry = this.pendingKeys_.get(key);
          if (entry)
            entry.set(false);
        }
        delete this.target_[key];
        var _change2 = notify || notifySpy ? {
          type: REMOVE,
          observableKind: "object",
          object: this.proxy_ || target,
          debugObjectName: this.name_,
          oldValue,
          name: key
        } : null;
        if (false)
          spyReportStart(_change2);
        if (notify)
          notifyListeners(this, _change2);
        if (false)
          spyReportEnd();
      } finally {
        endBatch();
      }
    };
    _proto.observe_ = function observe_(callback, fireImmediately) {
      if (false)
        die("`observe` doesn't support the fire immediately property for observable objects.");
      return registerListener(this, callback);
    };
    _proto.intercept_ = function intercept_(handler) {
      return registerInterceptor(this, handler);
    };
    _proto.notifyPropertyAddition_ = function notifyPropertyAddition_(key, newValue) {
      var notify = hasListeners(this);
      var notifySpy = false;
      var change = notify || notifySpy ? {
        type: ADD,
        observableKind: "object",
        debugObjectName: this.name_,
        object: this.proxy_ || this.target_,
        name: key,
        newValue
      } : null;
      if (false)
        spyReportStart(change);
      if (notify)
        notifyListeners(this, change);
      if (false)
        spyReportEnd();
      if (this.pendingKeys_) {
        var entry = this.pendingKeys_.get(key);
        if (entry)
          entry.set(true);
      }
      this.reportKeysChanged();
    };
    _proto.getKeys_ = function getKeys_() {
      this.keysAtom_.reportObserved();
      if (!this.isStaledKeysValue_) {
        return this.keysValue_;
      }
      this.keysValue_ = [];
      for (var _iterator = _createForOfIteratorHelperLoose(this.values_), _step; !(_step = _iterator()).done; ) {
        var _step$value = _step.value, key = _step$value[0], value = _step$value[1];
        if (value instanceof ObservableValue)
          this.keysValue_.push(key);
      }
      if (false)
        Object.freeze(this.keysValue_);
      this.isStaledKeysValue_ = false;
      return this.keysValue_;
    };
    _proto.reportKeysChanged = function reportKeysChanged() {
      this.isStaledKeysValue_ = true;
      this.keysAtom_.reportChanged();
    };
    return ObservableObjectAdministration2;
  }();
  function asObservableObject(target, name, defaultEnhancer) {
    if (name === void 0) {
      name = "";
    }
    if (defaultEnhancer === void 0) {
      defaultEnhancer = deepEnhancer;
    }
    if (hasProp(target, $mobx))
      return target[$mobx];
    if (false)
      die("Cannot make the designated object observable; it is not extensible");
    if (!isPlainObject(target))
      name = (target.constructor.name || "ObservableObject") + "@" + getNextId();
    if (!name)
      name = "ObservableObject@" + getNextId();
    var adm = new ObservableObjectAdministration(target, new Map(), stringifyKey(name), defaultEnhancer);
    addHiddenProp(target, $mobx, adm);
    return adm;
  }
  var observablePropertyConfigs = /* @__PURE__ */ Object.create(null);
  var computedPropertyConfigs = /* @__PURE__ */ Object.create(null);
  function generateObservablePropConfig(propName) {
    return observablePropertyConfigs[propName] || (observablePropertyConfigs[propName] = {
      configurable: true,
      enumerable: true,
      get: function get3() {
        return this[$mobx].read_(propName);
      },
      set: function set4(v) {
        this[$mobx].write_(propName, v);
      }
    });
  }
  function generateComputedPropConfig(propName) {
    return computedPropertyConfigs[propName] || (computedPropertyConfigs[propName] = {
      configurable: true,
      enumerable: false,
      get: function get3() {
        return this[$mobx].read_(propName);
      },
      set: function set4(v) {
        this[$mobx].write_(propName, v);
      }
    });
  }
  var isObservableObjectAdministration = /* @__PURE__ */ createInstanceofPredicate("ObservableObjectAdministration", ObservableObjectAdministration);
  function isObservableObject(thing) {
    if (isObject(thing)) {
      return isObservableObjectAdministration(thing[$mobx]);
    }
    return false;
  }
  var OBSERVABLE_ARRAY_BUFFER_SIZE = 0;
  var StubArray = function StubArray2() {
  };
  function inherit(ctor, proto) {
    if (Object.setPrototypeOf) {
      Object.setPrototypeOf(ctor.prototype, proto);
    } else if (ctor.prototype.__proto__ !== void 0) {
      ctor.prototype.__proto__ = proto;
    } else {
      ctor.prototype = proto;
    }
  }
  inherit(StubArray, Array.prototype);
  var LegacyObservableArray = /* @__PURE__ */ function(_StubArray) {
    _inheritsLoose(LegacyObservableArray2, _StubArray);
    function LegacyObservableArray2(initialValues, enhancer, name, owned) {
      var _this;
      if (name === void 0) {
        name = "ObservableArray@" + getNextId();
      }
      if (owned === void 0) {
        owned = false;
      }
      _this = _StubArray.call(this) || this;
      var adm = new ObservableArrayAdministration(name, enhancer, owned, true);
      adm.proxy_ = _assertThisInitialized(_this);
      addHiddenFinalProp(_assertThisInitialized(_this), $mobx, adm);
      if (initialValues && initialValues.length) {
        var prev = allowStateChangesStart(true);
        _this.spliceWithArray(0, 0, initialValues);
        allowStateChangesEnd(prev);
      }
      return _this;
    }
    var _proto = LegacyObservableArray2.prototype;
    _proto.concat = function concat() {
      this[$mobx].atom_.reportObserved();
      for (var _len = arguments.length, arrays = new Array(_len), _key = 0; _key < _len; _key++) {
        arrays[_key] = arguments[_key];
      }
      return Array.prototype.concat.apply(this.slice(), arrays.map(function(a) {
        return isObservableArray(a) ? a.slice() : a;
      }));
    };
    _proto[Symbol.iterator] = function() {
      var self = this;
      var nextIndex = 0;
      return makeIterable({
        next: function next() {
          return nextIndex < self.length ? {
            value: self[nextIndex++],
            done: false
          } : {
            done: true,
            value: void 0
          };
        }
      });
    };
    _createClass(LegacyObservableArray2, [{
      key: "length",
      get: function get3() {
        return this[$mobx].getArrayLength_();
      },
      set: function set4(newLength) {
        this[$mobx].setArrayLength_(newLength);
      }
    }, {
      key: Symbol.toStringTag,
      get: function get3() {
        return "Array";
      }
    }]);
    return LegacyObservableArray2;
  }(StubArray);
  Object.entries(arrayExtensions).forEach(function(_ref) {
    var prop = _ref[0], fn = _ref[1];
    if (prop !== "concat")
      addHiddenProp(LegacyObservableArray.prototype, prop, fn);
  });
  function createArrayEntryDescriptor(index2) {
    return {
      enumerable: false,
      configurable: true,
      get: function get3() {
        return this[$mobx].get_(index2);
      },
      set: function set4(value) {
        this[$mobx].set_(index2, value);
      }
    };
  }
  function createArrayBufferItem(index2) {
    defineProperty(LegacyObservableArray.prototype, "" + index2, createArrayEntryDescriptor(index2));
  }
  function reserveArrayBuffer(max) {
    if (max > OBSERVABLE_ARRAY_BUFFER_SIZE) {
      for (var index2 = OBSERVABLE_ARRAY_BUFFER_SIZE; index2 < max + 100; index2++) {
        createArrayBufferItem(index2);
      }
      OBSERVABLE_ARRAY_BUFFER_SIZE = max;
    }
  }
  reserveArrayBuffer(1e3);
  function createLegacyArray(initialValues, enhancer, name) {
    return new LegacyObservableArray(initialValues, enhancer, name);
  }
  function getAtom(thing, property) {
    if (typeof thing === "object" && thing !== null) {
      if (isObservableArray(thing)) {
        if (property !== void 0)
          die(23);
        return thing[$mobx].atom_;
      }
      if (isObservableSet(thing)) {
        return thing[$mobx];
      }
      if (isObservableMap(thing)) {
        if (property === void 0)
          return thing.keysAtom_;
        var observable2 = thing.data_.get(property) || thing.hasMap_.get(property);
        if (!observable2)
          die(25, property, getDebugName(thing));
        return observable2;
      }
      if (isObservableObject(thing)) {
        if (!property)
          return die(26);
        var _observable = thing[$mobx].values_.get(property);
        if (!_observable)
          die(27, property, getDebugName(thing));
        return _observable;
      }
      if (isAtom(thing) || isComputedValue(thing) || isReaction(thing)) {
        return thing;
      }
    } else if (isFunction(thing)) {
      if (isReaction(thing[$mobx])) {
        return thing[$mobx];
      }
    }
    die(28);
  }
  function getAdministration(thing, property) {
    if (!thing)
      die(29);
    if (property !== void 0)
      return getAdministration(getAtom(thing, property));
    if (isAtom(thing) || isComputedValue(thing) || isReaction(thing))
      return thing;
    if (isObservableMap(thing) || isObservableSet(thing))
      return thing;
    if (thing[$mobx])
      return thing[$mobx];
    die(24, thing);
  }
  function getDebugName(thing, property) {
    var named;
    if (property !== void 0)
      named = getAtom(thing, property);
    else if (isObservableObject(thing) || isObservableMap(thing) || isObservableSet(thing))
      named = getAdministration(thing);
    else
      named = getAtom(thing);
    return named.name_;
  }
  var toString = objectPrototype.toString;
  function deepEqual(a, b, depth) {
    if (depth === void 0) {
      depth = -1;
    }
    return eq(a, b, depth);
  }
  function eq(a, b, depth, aStack, bStack) {
    if (a === b)
      return a !== 0 || 1 / a === 1 / b;
    if (a == null || b == null)
      return false;
    if (a !== a)
      return b !== b;
    var type2 = typeof a;
    if (!isFunction(type2) && type2 !== "object" && typeof b != "object")
      return false;
    var className = toString.call(a);
    if (className !== toString.call(b))
      return false;
    switch (className) {
      case "[object RegExp]":
      case "[object String]":
        return "" + a === "" + b;
      case "[object Number]":
        if (+a !== +a)
          return +b !== +b;
        return +a === 0 ? 1 / +a === 1 / b : +a === +b;
      case "[object Date]":
      case "[object Boolean]":
        return +a === +b;
      case "[object Symbol]":
        return typeof Symbol !== "undefined" && Symbol.valueOf.call(a) === Symbol.valueOf.call(b);
      case "[object Map]":
      case "[object Set]":
        if (depth >= 0) {
          depth++;
        }
        break;
    }
    a = unwrap(a);
    b = unwrap(b);
    var areArrays = className === "[object Array]";
    if (!areArrays) {
      if (typeof a != "object" || typeof b != "object")
        return false;
      var aCtor = a.constructor, bCtor = b.constructor;
      if (aCtor !== bCtor && !(isFunction(aCtor) && aCtor instanceof aCtor && isFunction(bCtor) && bCtor instanceof bCtor) && "constructor" in a && "constructor" in b) {
        return false;
      }
    }
    if (depth === 0) {
      return false;
    } else if (depth < 0) {
      depth = -1;
    }
    aStack = aStack || [];
    bStack = bStack || [];
    var length = aStack.length;
    while (length--) {
      if (aStack[length] === a)
        return bStack[length] === b;
    }
    aStack.push(a);
    bStack.push(b);
    if (areArrays) {
      length = a.length;
      if (length !== b.length)
        return false;
      while (length--) {
        if (!eq(a[length], b[length], depth - 1, aStack, bStack))
          return false;
      }
    } else {
      var keys = Object.keys(a);
      var key;
      length = keys.length;
      if (Object.keys(b).length !== length)
        return false;
      while (length--) {
        key = keys[length];
        if (!(hasProp(b, key) && eq(a[key], b[key], depth - 1, aStack, bStack)))
          return false;
      }
    }
    aStack.pop();
    bStack.pop();
    return true;
  }
  function unwrap(a) {
    if (isObservableArray(a))
      return a.slice();
    if (isES6Map(a) || isObservableMap(a))
      return Array.from(a.entries());
    if (isES6Set(a) || isObservableSet(a))
      return Array.from(a.entries());
    return a;
  }
  function makeIterable(iterator) {
    iterator[Symbol.iterator] = getSelf;
    return iterator;
  }
  function getSelf() {
    return this;
  }
  ["Symbol", "Map", "Set", "Symbol"].forEach(function(m) {
    var g = getGlobal();
    if (typeof g[m] === "undefined") {
      die("MobX requires global '" + m + "' to be available or polyfilled");
    }
  });
  if (typeof __MOBX_DEVTOOLS_GLOBAL_HOOK__ === "object") {
    __MOBX_DEVTOOLS_GLOBAL_HOOK__.injectMobx({
      spy,
      extras: {
        getDebugName
      },
      $mobx
    });
  }

  // node_modules/mobx-react/dist/mobxreact.esm.js
  const react8 = __toModule(require_react());

  // node_modules/mobx-react-lite/es/utils/assertEnvironment.js
  const react = __toModule(require_react());
  if (!react.useState) {
    throw new Error("mobx-react-lite requires React with Hooks support");
  }
  if (!makeObservable) {
    throw new Error("mobx-react-lite@3 requires mobx at least version 6 to be available");
  }

  // node_modules/mobx-react-lite/es/utils/reactBatchedUpdates.js
  const react_dom = __toModule(require_react_dom());

  // node_modules/mobx-react-lite/es/utils/observerBatching.js
  function defaultNoopBatch(callback) {
    callback();
  }
  function observerBatching(reactionScheduler3) {
    if (!reactionScheduler3) {
      reactionScheduler3 = defaultNoopBatch;
      if (false) {
        console.warn("[MobX] Failed to get unstable_batched updates from react-dom / react-native");
      }
    }
    configure({reactionScheduler: reactionScheduler3});
  }

  // node_modules/mobx-react-lite/es/utils/utils.js
  const react2 = __toModule(require_react());
  var __read = function(o, n) {
    var m = typeof Symbol === "function" && o[Symbol.iterator];
    if (!m)
      return o;
    var i = m.call(o), r, ar = [], e;
    try {
      while ((n === void 0 || n-- > 0) && !(r = i.next()).done)
        ar.push(r.value);
    } catch (error) {
      e = {error};
    } finally {
      try {
        if (r && !r.done && (m = i["return"]))
          m.call(i);
      } finally {
        if (e)
          throw e.error;
      }
    }
    return ar;
  };
  var EMPTY_ARRAY2 = [];
  function useForceUpdate() {
    var _a = __read(react2.useState(0), 2), setTick = _a[1];
    var update = react2.useCallback(function() {
      setTick(function(tick) {
        return tick + 1;
      });
    }, EMPTY_ARRAY2);
    return update;
  }

  // node_modules/mobx-react-lite/es/useObserver.js
  const react3 = __toModule(require_react());

  // node_modules/mobx-react-lite/es/utils/printDebugValue.js
  function printDebugValue(v) {
    return getDependencyTree(v);
  }

  // node_modules/mobx-react-lite/es/utils/reactionCleanupTracking.js
  function createTrackingData(reaction) {
    var trackingData = {
      reaction,
      mounted: false,
      changedBeforeMount: false,
      cleanAt: Date.now() + CLEANUP_LEAKED_REACTIONS_AFTER_MILLIS
    };
    return trackingData;
  }
  var CLEANUP_LEAKED_REACTIONS_AFTER_MILLIS = 1e4;
  var CLEANUP_TIMER_LOOP_MILLIS = 1e4;
  var uncommittedReactionRefs = new Set();
  var reactionCleanupHandle;
  function ensureCleanupTimerRunning() {
    if (reactionCleanupHandle === void 0) {
      reactionCleanupHandle = setTimeout(cleanUncommittedReactions, CLEANUP_TIMER_LOOP_MILLIS);
    }
  }
  function scheduleCleanupOfReactionIfLeaked(ref) {
    uncommittedReactionRefs.add(ref);
    ensureCleanupTimerRunning();
  }
  function recordReactionAsCommitted(reactionRef) {
    uncommittedReactionRefs.delete(reactionRef);
  }
  function cleanUncommittedReactions() {
    reactionCleanupHandle = void 0;
    var now = Date.now();
    uncommittedReactionRefs.forEach(function(ref) {
      var tracking = ref.current;
      if (tracking) {
        if (now >= tracking.cleanAt) {
          tracking.reaction.dispose();
          ref.current = null;
          uncommittedReactionRefs.delete(ref);
        }
      }
    });
    if (uncommittedReactionRefs.size > 0) {
      ensureCleanupTimerRunning();
    }
  }

  // node_modules/mobx-react-lite/es/staticRendering.js
  var globalIsUsingStaticRendering = false;
  function isUsingStaticRendering() {
    return globalIsUsingStaticRendering;
  }

  // node_modules/mobx-react-lite/es/useObserver.js
  function observerComponentNameFor(baseComponentName) {
    return "observer" + baseComponentName;
  }
  function useObserver(fn, baseComponentName) {
    if (baseComponentName === void 0) {
      baseComponentName = "observed";
    }
    if (isUsingStaticRendering()) {
      return fn();
    }
    var forceUpdate = useForceUpdate();
    var reactionTrackingRef = react3.default.useRef(null);
    if (!reactionTrackingRef.current) {
      var newReaction = new Reaction(observerComponentNameFor(baseComponentName), function() {
        if (trackingData_1.mounted) {
          forceUpdate();
        } else {
          trackingData_1.changedBeforeMount = true;
        }
      });
      var trackingData_1 = createTrackingData(newReaction);
      reactionTrackingRef.current = trackingData_1;
      scheduleCleanupOfReactionIfLeaked(reactionTrackingRef);
    }
    var reaction = reactionTrackingRef.current.reaction;
    react3.default.useDebugValue(reaction, printDebugValue);
    react3.default.useEffect(function() {
      recordReactionAsCommitted(reactionTrackingRef);
      if (reactionTrackingRef.current) {
        reactionTrackingRef.current.mounted = true;
        if (reactionTrackingRef.current.changedBeforeMount) {
          reactionTrackingRef.current.changedBeforeMount = false;
          forceUpdate();
        }
      } else {
        reactionTrackingRef.current = {
          reaction: new Reaction(observerComponentNameFor(baseComponentName), function() {
            forceUpdate();
          }),
          mounted: true,
          changedBeforeMount: false,
          cleanAt: Infinity
        };
        forceUpdate();
      }
      return function() {
        reactionTrackingRef.current.reaction.dispose();
        reactionTrackingRef.current = null;
      };
    }, []);
    var rendering;
    var exception;
    reaction.track(function() {
      try {
        rendering = fn();
      } catch (e) {
        exception = e;
      }
    });
    if (exception) {
      throw exception;
    }
    return rendering;
  }

  // node_modules/mobx-react-lite/es/observer.js
  const react4 = __toModule(require_react());
  var __assign = function() {
    __assign = Object.assign || function(t2) {
      for (var s, i = 1, n = arguments.length; i < n; i++) {
        s = arguments[i];
        for (var p in s)
          if (Object.prototype.hasOwnProperty.call(s, p))
            t2[p] = s[p];
      }
      return t2;
    };
    return __assign.apply(this, arguments);
  };
  function observer(baseComponent, options) {
    if (isUsingStaticRendering()) {
      return baseComponent;
    }
    var realOptions = __assign({forwardRef: false}, options);
    var baseComponentName = baseComponent.displayName || baseComponent.name;
    var wrappedComponent = function(props, ref) {
      return useObserver(function() {
        return baseComponent(props, ref);
      }, baseComponentName);
    };
    wrappedComponent.displayName = baseComponentName;
    var memoComponent;
    if (realOptions.forwardRef) {
      memoComponent = react4.memo(react4.forwardRef(wrappedComponent));
    } else {
      memoComponent = react4.memo(wrappedComponent);
    }
    copyStaticProperties(baseComponent, memoComponent);
    memoComponent.displayName = baseComponentName;
    return memoComponent;
  }
  var hoistBlackList = {
    $$typeof: true,
    render: true,
    compare: true,
    type: true
  };
  function copyStaticProperties(base, target) {
    Object.keys(base).forEach(function(key) {
      if (!hoistBlackList[key]) {
        Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(base, key));
      }
    });
  }

  // node_modules/mobx-react-lite/es/ObserverComponent.js
  function ObserverComponent(_a) {
    var children = _a.children, render2 = _a.render;
    var component = children || render2;
    if (typeof component !== "function") {
      return null;
    }
    return useObserver(component);
  }
  if (false) {
    ObserverComponent.propTypes = {
      children: ObserverPropsCheck,
      render: ObserverPropsCheck
    };
  }
  ObserverComponent.displayName = "Observer";

  // node_modules/mobx-react-lite/es/useLocalObservable.js
  const react5 = __toModule(require_react());

  // node_modules/mobx-react-lite/es/useLocalStore.js
  const react7 = __toModule(require_react());

  // node_modules/mobx-react-lite/es/useAsObservableSource.js
  const react6 = __toModule(require_react());

  // node_modules/mobx-react-lite/es/index.js
  observerBatching(react_dom.unstable_batchedUpdates);

  // node_modules/mobx-react/dist/mobxreact.esm.js
  var symbolId = 0;
  function createSymbol(name) {
    if (typeof Symbol === "function") {
      return Symbol(name);
    }
    var symbol = "__$mobx-react " + name + " (" + symbolId + ")";
    symbolId++;
    return symbol;
  }
  var createdSymbols = {};
  function newSymbol(name) {
    if (!createdSymbols[name]) {
      createdSymbols[name] = createSymbol(name);
    }
    return createdSymbols[name];
  }
  function shallowEqual(objA, objB) {
    if (is(objA, objB))
      return true;
    if (typeof objA !== "object" || objA === null || typeof objB !== "object" || objB === null) {
      return false;
    }
    var keysA = Object.keys(objA);
    var keysB = Object.keys(objB);
    if (keysA.length !== keysB.length)
      return false;
    for (var i = 0; i < keysA.length; i++) {
      if (!Object.hasOwnProperty.call(objB, keysA[i]) || !is(objA[keysA[i]], objB[keysA[i]])) {
        return false;
      }
    }
    return true;
  }
  function is(x, y) {
    if (x === y) {
      return x !== 0 || 1 / x === 1 / y;
    } else {
      return x !== x && y !== y;
    }
  }
  function setHiddenProp(target, prop, value) {
    if (!Object.hasOwnProperty.call(target, prop)) {
      Object.defineProperty(target, prop, {
        enumerable: false,
        configurable: true,
        writable: true,
        value
      });
    } else {
      target[prop] = value;
    }
  }
  var mobxMixins = /* @__PURE__ */ newSymbol("patchMixins");
  var mobxPatchedDefinition = /* @__PURE__ */ newSymbol("patchedDefinition");
  function getMixins(target, methodName) {
    var mixins = target[mobxMixins] = target[mobxMixins] || {};
    var methodMixins = mixins[methodName] = mixins[methodName] || {};
    methodMixins.locks = methodMixins.locks || 0;
    methodMixins.methods = methodMixins.methods || [];
    return methodMixins;
  }
  function wrapper(realMethod, mixins) {
    var _this = this;
    for (var _len = arguments.length, args = new Array(_len > 2 ? _len - 2 : 0), _key = 2; _key < _len; _key++) {
      args[_key - 2] = arguments[_key];
    }
    mixins.locks++;
    try {
      var retVal;
      if (realMethod !== void 0 && realMethod !== null) {
        retVal = realMethod.apply(this, args);
      }
      return retVal;
    } finally {
      mixins.locks--;
      if (mixins.locks === 0) {
        mixins.methods.forEach(function(mx) {
          mx.apply(_this, args);
        });
      }
    }
  }
  function wrapFunction(realMethod, mixins) {
    var fn = function fn2() {
      for (var _len2 = arguments.length, args = new Array(_len2), _key2 = 0; _key2 < _len2; _key2++) {
        args[_key2] = arguments[_key2];
      }
      wrapper.call.apply(wrapper, [this, realMethod, mixins].concat(args));
    };
    return fn;
  }
  function patch(target, methodName, mixinMethod) {
    var mixins = getMixins(target, methodName);
    if (mixins.methods.indexOf(mixinMethod) < 0) {
      mixins.methods.push(mixinMethod);
    }
    var oldDefinition = Object.getOwnPropertyDescriptor(target, methodName);
    if (oldDefinition && oldDefinition[mobxPatchedDefinition]) {
      return;
    }
    var originalMethod = target[methodName];
    var newDefinition = createDefinition(target, methodName, oldDefinition ? oldDefinition.enumerable : void 0, mixins, originalMethod);
    Object.defineProperty(target, methodName, newDefinition);
  }
  function createDefinition(target, methodName, enumerable, mixins, originalMethod) {
    var _ref;
    var wrappedFunc = wrapFunction(originalMethod, mixins);
    return _ref = {}, _ref[mobxPatchedDefinition] = true, _ref.get = function get3() {
      return wrappedFunc;
    }, _ref.set = function set4(value) {
      if (this === target) {
        wrappedFunc = wrapFunction(value, mixins);
      } else {
        var newDefinition = createDefinition(this, methodName, enumerable, mixins, value);
        Object.defineProperty(this, methodName, newDefinition);
      }
    }, _ref.configurable = true, _ref.enumerable = enumerable, _ref;
  }
  var mobxAdminProperty = $mobx || "$mobx";
  var mobxObserverProperty = /* @__PURE__ */ newSymbol("isMobXReactObserver");
  var mobxIsUnmounted = /* @__PURE__ */ newSymbol("isUnmounted");
  var skipRenderKey = /* @__PURE__ */ newSymbol("skipRender");
  var isForcingUpdateKey = /* @__PURE__ */ newSymbol("isForcingUpdate");
  function makeClassComponentObserver(componentClass) {
    var target = componentClass.prototype;
    if (componentClass[mobxObserverProperty]) {
      var displayName = getDisplayName(target);
      console.warn("The provided component class (" + displayName + ") \n                has already been declared as an observer component.");
    } else {
      componentClass[mobxObserverProperty] = true;
    }
    if (target.componentWillReact)
      throw new Error("The componentWillReact life-cycle event is no longer supported");
    if (componentClass["__proto__"] !== react8.PureComponent) {
      if (!target.shouldComponentUpdate)
        target.shouldComponentUpdate = observerSCU;
      else if (target.shouldComponentUpdate !== observerSCU)
        throw new Error("It is not allowed to use shouldComponentUpdate in observer based components.");
    }
    makeObservableProp(target, "props");
    makeObservableProp(target, "state");
    var baseRender = target.render;
    target.render = function() {
      return makeComponentReactive.call(this, baseRender);
    };
    patch(target, "componentWillUnmount", function() {
      var _this$render$mobxAdmi;
      if (isUsingStaticRendering() === true)
        return;
      (_this$render$mobxAdmi = this.render[mobxAdminProperty]) === null || _this$render$mobxAdmi === void 0 ? void 0 : _this$render$mobxAdmi.dispose();
      this[mobxIsUnmounted] = true;
      if (!this.render[mobxAdminProperty]) {
        var _displayName = getDisplayName(this);
        console.warn("The reactive render of an observer class component (" + _displayName + ") \n                was overriden after MobX attached. This may result in a memory leak if the \n                overriden reactive render was not properly disposed.");
      }
    });
    return componentClass;
  }
  function getDisplayName(comp) {
    return comp.displayName || comp.name || comp.constructor && (comp.constructor.displayName || comp.constructor.name) || "<component>";
  }
  function makeComponentReactive(render2) {
    var _this = this;
    if (isUsingStaticRendering() === true)
      return render2.call(this);
    setHiddenProp(this, skipRenderKey, false);
    setHiddenProp(this, isForcingUpdateKey, false);
    var initialName = getDisplayName(this);
    var baseRender = render2.bind(this);
    var isRenderingPending = false;
    var reaction = new Reaction(initialName + ".render()", function() {
      if (!isRenderingPending) {
        isRenderingPending = true;
        if (_this[mobxIsUnmounted] !== true) {
          var hasError = true;
          try {
            setHiddenProp(_this, isForcingUpdateKey, true);
            if (!_this[skipRenderKey])
              react8.Component.prototype.forceUpdate.call(_this);
            hasError = false;
          } finally {
            setHiddenProp(_this, isForcingUpdateKey, false);
            if (hasError)
              reaction.dispose();
          }
        }
      }
    });
    reaction["reactComponent"] = this;
    reactiveRender[mobxAdminProperty] = reaction;
    this.render = reactiveRender;
    function reactiveRender() {
      isRenderingPending = false;
      var exception = void 0;
      var rendering = void 0;
      reaction.track(function() {
        try {
          rendering = allowStateChanges(false, baseRender);
        } catch (e) {
          exception = e;
        }
      });
      if (exception) {
        throw exception;
      }
      return rendering;
    }
    return reactiveRender.call(this);
  }
  function observerSCU(nextProps, nextState) {
    if (isUsingStaticRendering()) {
      console.warn("[mobx-react] It seems that a re-rendering of a React component is triggered while in static (server-side) mode. Please make sure components are rendered only once server-side.");
    }
    if (this.state !== nextState) {
      return true;
    }
    return !shallowEqual(this.props, nextProps);
  }
  function makeObservableProp(target, propName) {
    var valueHolderKey = newSymbol("reactProp_" + propName + "_valueHolder");
    var atomHolderKey = newSymbol("reactProp_" + propName + "_atomHolder");
    function getAtom2() {
      if (!this[atomHolderKey]) {
        setHiddenProp(this, atomHolderKey, createAtom("reactive " + propName));
      }
      return this[atomHolderKey];
    }
    Object.defineProperty(target, propName, {
      configurable: true,
      enumerable: true,
      get: function get3() {
        var prevReadState = false;
        if (allowStateReadsStart && allowStateReadsEnd) {
          prevReadState = allowStateReadsStart(true);
        }
        getAtom2.call(this).reportObserved();
        if (allowStateReadsStart && allowStateReadsEnd) {
          allowStateReadsEnd(prevReadState);
        }
        return this[valueHolderKey];
      },
      set: function set4(v) {
        if (!this[isForcingUpdateKey] && !shallowEqual(this[valueHolderKey], v)) {
          setHiddenProp(this, valueHolderKey, v);
          setHiddenProp(this, skipRenderKey, true);
          getAtom2.call(this).reportChanged();
          setHiddenProp(this, skipRenderKey, false);
        } else {
          setHiddenProp(this, valueHolderKey, v);
        }
      }
    });
  }
  var hasSymbol = typeof Symbol === "function" && Symbol.for;
  var ReactForwardRefSymbol = hasSymbol ? /* @__PURE__ */ Symbol.for("react.forward_ref") : typeof react8.forwardRef === "function" && react8.forwardRef(function(props) {
    return null;
  })["$$typeof"];
  var ReactMemoSymbol = hasSymbol ? /* @__PURE__ */ Symbol.for("react.memo") : typeof react8.memo === "function" && react8.memo(function(props) {
    return null;
  })["$$typeof"];
  function observer3(component) {
    if (component["isMobxInjector"] === true) {
      console.warn("Mobx observer: You are trying to use 'observer' on a component that already has 'inject'. Please apply 'observer' before applying 'inject'");
    }
    if (ReactMemoSymbol && component["$$typeof"] === ReactMemoSymbol) {
      throw new Error("Mobx observer: You are trying to use 'observer' on a function component wrapped in either another observer or 'React.memo'. The observer already applies 'React.memo' for you.");
    }
    if (ReactForwardRefSymbol && component["$$typeof"] === ReactForwardRefSymbol) {
      var baseRender = component["render"];
      if (typeof baseRender !== "function")
        throw new Error("render property of ForwardRef was not a function");
      return react8.forwardRef(function ObserverForwardRef() {
        var args = arguments;
        return react8.createElement(ObserverComponent, null, function() {
          return baseRender.apply(void 0, args);
        });
      });
    }
    if (typeof component === "function" && (!component.prototype || !component.prototype.render) && !component["isReactClass"] && !Object.prototype.isPrototypeOf.call(react8.Component, component)) {
      return observer(component);
    }
    return makeClassComponentObserver(component);
  }
  function _extends2() {
    _extends2 = Object.assign || function(target) {
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
    return _extends2.apply(this, arguments);
  }
  function _objectWithoutPropertiesLoose(source, excluded) {
    if (source == null)
      return {};
    var target = {};
    var sourceKeys = Object.keys(source);
    var key, i;
    for (i = 0; i < sourceKeys.length; i++) {
      key = sourceKeys[i];
      if (excluded.indexOf(key) >= 0)
        continue;
      target[key] = source[key];
    }
    return target;
  }
  var MobXProviderContext = /* @__PURE__ */ react8.default.createContext({});
  function Provider2(props) {
    var children = props.children, stores = _objectWithoutPropertiesLoose(props, ["children"]);
    var parentValue = react8.default.useContext(MobXProviderContext);
    var mutableProviderRef = react8.default.useRef(_extends2({}, parentValue, stores));
    var value = mutableProviderRef.current;
    if (false) {
      var newValue = _extends2({}, value, stores);
      if (!shallowEqual(value, newValue)) {
        throw new Error("MobX Provider: The set of provided stores has changed. See: https://github.com/mobxjs/mobx-react#the-set-of-provided-stores-has-changed-error.");
      }
    }
    return react8.default.createElement(MobXProviderContext.Provider, {
      value
    }, children);
  }
  Provider2.displayName = "MobXProvider";
  if (!react8.Component)
    throw new Error("mobx-react requires React to be available");
  if (!observable)
    throw new Error("mobx-react requires mobx to be available");

  // node_modules/@emotion/styled-base/dist/styled-base.browser.esm.js
  const defineProperty2 = __toModule(require_defineProperty());
  const react11 = __toModule(require_react());

  // node_modules/@emotion/memoize/dist/memoize.browser.esm.js
  function memoize(fn) {
    var cache3 = {};
    return function(arg) {
      if (cache3[arg] === void 0)
        cache3[arg] = fn(arg);
      return cache3[arg];
    };
  }
  var memoize_browser_esm_default = memoize;

  // node_modules/@emotion/is-prop-valid/dist/is-prop-valid.browser.esm.js
  var reactPropsRegex = /^((children|dangerouslySetInnerHTML|key|ref|autoFocus|defaultValue|defaultChecked|innerHTML|suppressContentEditableWarning|suppressHydrationWarning|valueLink|accept|acceptCharset|accessKey|action|allow|allowUserMedia|allowPaymentRequest|allowFullScreen|allowTransparency|alt|async|autoComplete|autoPlay|capture|cellPadding|cellSpacing|challenge|charSet|checked|cite|classID|className|cols|colSpan|content|contentEditable|contextMenu|controls|controlsList|coords|crossOrigin|data|dateTime|decoding|default|defer|dir|disabled|disablePictureInPicture|download|draggable|encType|form|formAction|formEncType|formMethod|formNoValidate|formTarget|frameBorder|headers|height|hidden|high|href|hrefLang|htmlFor|httpEquiv|id|inputMode|integrity|is|keyParams|keyType|kind|label|lang|list|loading|loop|low|marginHeight|marginWidth|max|maxLength|media|mediaGroup|method|min|minLength|multiple|muted|name|nonce|noValidate|open|optimum|pattern|placeholder|playsInline|poster|preload|profile|radioGroup|readOnly|referrerPolicy|rel|required|reversed|role|rows|rowSpan|sandbox|scope|scoped|scrolling|seamless|selected|shape|size|sizes|slot|span|spellCheck|src|srcDoc|srcLang|srcSet|start|step|style|summary|tabIndex|target|title|type|useMap|value|width|wmode|wrap|about|datatype|inlist|prefix|property|resource|typeof|vocab|autoCapitalize|autoCorrect|autoSave|color|inert|itemProp|itemScope|itemType|itemID|itemRef|on|results|security|unselectable|accentHeight|accumulate|additive|alignmentBaseline|allowReorder|alphabetic|amplitude|arabicForm|ascent|attributeName|attributeType|autoReverse|azimuth|baseFrequency|baselineShift|baseProfile|bbox|begin|bias|by|calcMode|capHeight|clip|clipPathUnits|clipPath|clipRule|colorInterpolation|colorInterpolationFilters|colorProfile|colorRendering|contentScriptType|contentStyleType|cursor|cx|cy|d|decelerate|descent|diffuseConstant|direction|display|divisor|dominantBaseline|dur|dx|dy|edgeMode|elevation|enableBackground|end|exponent|externalResourcesRequired|fill|fillOpacity|fillRule|filter|filterRes|filterUnits|floodColor|floodOpacity|focusable|fontFamily|fontSize|fontSizeAdjust|fontStretch|fontStyle|fontVariant|fontWeight|format|from|fr|fx|fy|g1|g2|glyphName|glyphOrientationHorizontal|glyphOrientationVertical|glyphRef|gradientTransform|gradientUnits|hanging|horizAdvX|horizOriginX|ideographic|imageRendering|in|in2|intercept|k|k1|k2|k3|k4|kernelMatrix|kernelUnitLength|kerning|keyPoints|keySplines|keyTimes|lengthAdjust|letterSpacing|lightingColor|limitingConeAngle|local|markerEnd|markerMid|markerStart|markerHeight|markerUnits|markerWidth|mask|maskContentUnits|maskUnits|mathematical|mode|numOctaves|offset|opacity|operator|order|orient|orientation|origin|overflow|overlinePosition|overlineThickness|panose1|paintOrder|pathLength|patternContentUnits|patternTransform|patternUnits|pointerEvents|points|pointsAtX|pointsAtY|pointsAtZ|preserveAlpha|preserveAspectRatio|primitiveUnits|r|radius|refX|refY|renderingIntent|repeatCount|repeatDur|requiredExtensions|requiredFeatures|restart|result|rotate|rx|ry|scale|seed|shapeRendering|slope|spacing|specularConstant|specularExponent|speed|spreadMethod|startOffset|stdDeviation|stemh|stemv|stitchTiles|stopColor|stopOpacity|strikethroughPosition|strikethroughThickness|string|stroke|strokeDasharray|strokeDashoffset|strokeLinecap|strokeLinejoin|strokeMiterlimit|strokeOpacity|strokeWidth|surfaceScale|systemLanguage|tableValues|targetX|targetY|textAnchor|textDecoration|textRendering|textLength|to|transform|u1|u2|underlinePosition|underlineThickness|unicode|unicodeBidi|unicodeRange|unitsPerEm|vAlphabetic|vHanging|vIdeographic|vMathematical|values|vectorEffect|version|vertAdvY|vertOriginX|vertOriginY|viewBox|viewTarget|visibility|widths|wordSpacing|writingMode|x|xHeight|x1|x2|xChannelSelector|xlinkActuate|xlinkArcrole|xlinkHref|xlinkRole|xlinkShow|xlinkTitle|xlinkType|xmlBase|xmlns|xmlnsXlink|xmlLang|xmlSpace|y|y1|y2|yChannelSelector|z|zoomAndPan|for|class|autofocus)|(([Dd][Aa][Tt][Aa]|[Aa][Rr][Ii][Aa]|x)-.*))$/;
  var index = memoize_browser_esm_default(function(prop) {
    return reactPropsRegex.test(prop) || prop.charCodeAt(0) === 111 && prop.charCodeAt(1) === 110 && prop.charCodeAt(2) < 91;
  });
  var is_prop_valid_browser_esm_default = index;

  // node_modules/@babel/runtime/helpers/esm/inheritsLoose.js
  function _inheritsLoose2(subClass, superClass) {
    subClass.prototype = Object.create(superClass.prototype);
    subClass.prototype.constructor = subClass;
    subClass.__proto__ = superClass;
  }

  // node_modules/@emotion/core/dist/core.browser.esm.js
  const react10 = __toModule(require_react());

  // node_modules/@emotion/sheet/dist/sheet.browser.esm.js
  function sheetForTag(tag) {
    if (tag.sheet) {
      return tag.sheet;
    }
    for (var i = 0; i < document.styleSheets.length; i++) {
      if (document.styleSheets[i].ownerNode === tag) {
        return document.styleSheets[i];
      }
    }
  }
  function createStyleElement(options) {
    var tag = document.createElement("style");
    tag.setAttribute("data-emotion", options.key);
    if (options.nonce !== void 0) {
      tag.setAttribute("nonce", options.nonce);
    }
    tag.appendChild(document.createTextNode(""));
    return tag;
  }
  var StyleSheet = /* @__PURE__ */ function() {
    function StyleSheet2(options) {
      this.isSpeedy = options.speedy === void 0 ? true : options.speedy;
      this.tags = [];
      this.ctr = 0;
      this.nonce = options.nonce;
      this.key = options.key;
      this.container = options.container;
      this.before = null;
    }
    var _proto = StyleSheet2.prototype;
    _proto.insert = function insert(rule) {
      if (this.ctr % (this.isSpeedy ? 65e3 : 1) === 0) {
        var _tag = createStyleElement(this);
        var before;
        if (this.tags.length === 0) {
          before = this.before;
        } else {
          before = this.tags[this.tags.length - 1].nextSibling;
        }
        this.container.insertBefore(_tag, before);
        this.tags.push(_tag);
      }
      var tag = this.tags[this.tags.length - 1];
      if (this.isSpeedy) {
        var sheet3 = sheetForTag(tag);
        try {
          var isImportRule = rule.charCodeAt(1) === 105 && rule.charCodeAt(0) === 64;
          sheet3.insertRule(rule, isImportRule ? 0 : sheet3.cssRules.length);
        } catch (e) {
          if (false) {
            console.warn('There was a problem inserting the following rule: "' + rule + '"', e);
          }
        }
      } else {
        tag.appendChild(document.createTextNode(rule));
      }
      this.ctr++;
    };
    _proto.flush = function flush() {
      this.tags.forEach(function(tag) {
        return tag.parentNode.removeChild(tag);
      });
      this.tags = [];
      this.ctr = 0;
    };
    return StyleSheet2;
  }();

  // node_modules/@emotion/stylis/dist/stylis.browser.esm.js
  function stylis_min(W) {
    function M(d, c, e, h, a) {
      for (var m = 0, b = 0, v = 0, n = 0, q, g, x = 0, K = 0, k, u = k = q = 0, l = 0, r = 0, I = 0, t2 = 0, B2 = e.length, J = B2 - 1, y, f = "", p = "", F2 = "", G2 = "", C; l < B2; ) {
        g = e.charCodeAt(l);
        l === J && b + n + v + m !== 0 && (b !== 0 && (g = b === 47 ? 10 : 47), n = v = m = 0, B2++, J++);
        if (b + n + v + m === 0) {
          if (l === J && (0 < r && (f = f.replace(N, "")), 0 < f.trim().length)) {
            switch (g) {
              case 32:
              case 9:
              case 59:
              case 13:
              case 10:
                break;
              default:
                f += e.charAt(l);
            }
            g = 59;
          }
          switch (g) {
            case 123:
              f = f.trim();
              q = f.charCodeAt(0);
              k = 1;
              for (t2 = ++l; l < B2; ) {
                switch (g = e.charCodeAt(l)) {
                  case 123:
                    k++;
                    break;
                  case 125:
                    k--;
                    break;
                  case 47:
                    switch (g = e.charCodeAt(l + 1)) {
                      case 42:
                      case 47:
                        a: {
                          for (u = l + 1; u < J; ++u) {
                            switch (e.charCodeAt(u)) {
                              case 47:
                                if (g === 42 && e.charCodeAt(u - 1) === 42 && l + 2 !== u) {
                                  l = u + 1;
                                  break a;
                                }
                                break;
                              case 10:
                                if (g === 47) {
                                  l = u + 1;
                                  break a;
                                }
                            }
                          }
                          l = u;
                        }
                    }
                    break;
                  case 91:
                    g++;
                  case 40:
                    g++;
                  case 34:
                  case 39:
                    for (; l++ < J && e.charCodeAt(l) !== g; ) {
                    }
                }
                if (k === 0)
                  break;
                l++;
              }
              k = e.substring(t2, l);
              q === 0 && (q = (f = f.replace(ca, "").trim()).charCodeAt(0));
              switch (q) {
                case 64:
                  0 < r && (f = f.replace(N, ""));
                  g = f.charCodeAt(1);
                  switch (g) {
                    case 100:
                    case 109:
                    case 115:
                    case 45:
                      r = c;
                      break;
                    default:
                      r = O;
                  }
                  k = M(c, r, k, g, a + 1);
                  t2 = k.length;
                  0 < A && (r = X(O, f, I), C = H(3, k, r, c, D, z, t2, g, a, h), f = r.join(""), C !== void 0 && (t2 = (k = C.trim()).length) === 0 && (g = 0, k = ""));
                  if (0 < t2)
                    switch (g) {
                      case 115:
                        f = f.replace(da, ea);
                      case 100:
                      case 109:
                      case 45:
                        k = f + "{" + k + "}";
                        break;
                      case 107:
                        f = f.replace(fa, "$1 $2");
                        k = f + "{" + k + "}";
                        k = w === 1 || w === 2 && L("@" + k, 3) ? "@-webkit-" + k + "@" + k : "@" + k;
                        break;
                      default:
                        k = f + k, h === 112 && (k = (p += k, ""));
                    }
                  else
                    k = "";
                  break;
                default:
                  k = M(c, X(c, f, I), k, h, a + 1);
              }
              F2 += k;
              k = I = r = u = q = 0;
              f = "";
              g = e.charCodeAt(++l);
              break;
            case 125:
            case 59:
              f = (0 < r ? f.replace(N, "") : f).trim();
              if (1 < (t2 = f.length))
                switch (u === 0 && (q = f.charCodeAt(0), q === 45 || 96 < q && 123 > q) && (t2 = (f = f.replace(" ", ":")).length), 0 < A && (C = H(1, f, c, d, D, z, p.length, h, a, h)) !== void 0 && (t2 = (f = C.trim()).length) === 0 && (f = "\0\0"), q = f.charCodeAt(0), g = f.charCodeAt(1), q) {
                  case 0:
                    break;
                  case 64:
                    if (g === 105 || g === 99) {
                      G2 += f + e.charAt(l);
                      break;
                    }
                  default:
                    f.charCodeAt(t2 - 1) !== 58 && (p += P(f, q, g, f.charCodeAt(2)));
                }
              I = r = u = q = 0;
              f = "";
              g = e.charCodeAt(++l);
          }
        }
        switch (g) {
          case 13:
          case 10:
            b === 47 ? b = 0 : 1 + q === 0 && h !== 107 && 0 < f.length && (r = 1, f += "\0");
            0 < A * Y && H(0, f, c, d, D, z, p.length, h, a, h);
            z = 1;
            D++;
            break;
          case 59:
          case 125:
            if (b + n + v + m === 0) {
              z++;
              break;
            }
          default:
            z++;
            y = e.charAt(l);
            switch (g) {
              case 9:
              case 32:
                if (n + m + b === 0)
                  switch (x) {
                    case 44:
                    case 58:
                    case 9:
                    case 32:
                      y = "";
                      break;
                    default:
                      g !== 32 && (y = " ");
                  }
                break;
              case 0:
                y = "\\0";
                break;
              case 12:
                y = "\\f";
                break;
              case 11:
                y = "\\v";
                break;
              case 38:
                n + b + m === 0 && (r = I = 1, y = "\f" + y);
                break;
              case 108:
                if (n + b + m + E === 0 && 0 < u)
                  switch (l - u) {
                    case 2:
                      x === 112 && e.charCodeAt(l - 3) === 58 && (E = x);
                    case 8:
                      K === 111 && (E = K);
                  }
                break;
              case 58:
                n + b + m === 0 && (u = l);
                break;
              case 44:
                b + v + n + m === 0 && (r = 1, y += "\r");
                break;
              case 34:
              case 39:
                b === 0 && (n = n === g ? 0 : n === 0 ? g : n);
                break;
              case 91:
                n + b + v === 0 && m++;
                break;
              case 93:
                n + b + v === 0 && m--;
                break;
              case 41:
                n + b + m === 0 && v--;
                break;
              case 40:
                if (n + b + m === 0) {
                  if (q === 0)
                    switch (2 * x + 3 * K) {
                      case 533:
                        break;
                      default:
                        q = 1;
                    }
                  v++;
                }
                break;
              case 64:
                b + v + n + m + u + k === 0 && (k = 1);
                break;
              case 42:
              case 47:
                if (!(0 < n + m + v))
                  switch (b) {
                    case 0:
                      switch (2 * g + 3 * e.charCodeAt(l + 1)) {
                        case 235:
                          b = 47;
                          break;
                        case 220:
                          t2 = l, b = 42;
                      }
                      break;
                    case 42:
                      g === 47 && x === 42 && t2 + 2 !== l && (e.charCodeAt(t2 + 2) === 33 && (p += e.substring(t2, l + 1)), y = "", b = 0);
                  }
            }
            b === 0 && (f += y);
        }
        K = x;
        x = g;
        l++;
      }
      t2 = p.length;
      if (0 < t2) {
        r = c;
        if (0 < A && (C = H(2, p, r, d, D, z, t2, h, a, h), C !== void 0 && (p = C).length === 0))
          return G2 + p + F2;
        p = r.join(",") + "{" + p + "}";
        if (w * E !== 0) {
          w !== 2 || L(p, 2) || (E = 0);
          switch (E) {
            case 111:
              p = p.replace(ha, ":-moz-$1") + p;
              break;
            case 112:
              p = p.replace(Q, "::-webkit-input-$1") + p.replace(Q, "::-moz-$1") + p.replace(Q, ":-ms-input-$1") + p;
          }
          E = 0;
        }
      }
      return G2 + p + F2;
    }
    function X(d, c, e) {
      var h = c.trim().split(ia);
      c = h;
      var a = h.length, m = d.length;
      switch (m) {
        case 0:
        case 1:
          var b = 0;
          for (d = m === 0 ? "" : d[0] + " "; b < a; ++b) {
            c[b] = Z(d, c[b], e).trim();
          }
          break;
        default:
          var v = b = 0;
          for (c = []; b < a; ++b) {
            for (var n = 0; n < m; ++n) {
              c[v++] = Z(d[n] + " ", h[b], e).trim();
            }
          }
      }
      return c;
    }
    function Z(d, c, e) {
      var h = c.charCodeAt(0);
      33 > h && (h = (c = c.trim()).charCodeAt(0));
      switch (h) {
        case 38:
          return c.replace(F, "$1" + d.trim());
        case 58:
          return d.trim() + c.replace(F, "$1" + d.trim());
        default:
          if (0 < 1 * e && 0 < c.indexOf("\f"))
            return c.replace(F, (d.charCodeAt(0) === 58 ? "" : "$1") + d.trim());
      }
      return d + c;
    }
    function P(d, c, e, h) {
      var a = d + ";", m = 2 * c + 3 * e + 4 * h;
      if (m === 944) {
        d = a.indexOf(":", 9) + 1;
        var b = a.substring(d, a.length - 1).trim();
        b = a.substring(0, d).trim() + b + ";";
        return w === 1 || w === 2 && L(b, 1) ? "-webkit-" + b + b : b;
      }
      if (w === 0 || w === 2 && !L(a, 1))
        return a;
      switch (m) {
        case 1015:
          return a.charCodeAt(10) === 97 ? "-webkit-" + a + a : a;
        case 951:
          return a.charCodeAt(3) === 116 ? "-webkit-" + a + a : a;
        case 963:
          return a.charCodeAt(5) === 110 ? "-webkit-" + a + a : a;
        case 1009:
          if (a.charCodeAt(4) !== 100)
            break;
        case 969:
        case 942:
          return "-webkit-" + a + a;
        case 978:
          return "-webkit-" + a + "-moz-" + a + a;
        case 1019:
        case 983:
          return "-webkit-" + a + "-moz-" + a + "-ms-" + a + a;
        case 883:
          if (a.charCodeAt(8) === 45)
            return "-webkit-" + a + a;
          if (0 < a.indexOf("image-set(", 11))
            return a.replace(ja, "$1-webkit-$2") + a;
          break;
        case 932:
          if (a.charCodeAt(4) === 45)
            switch (a.charCodeAt(5)) {
              case 103:
                return "-webkit-box-" + a.replace("-grow", "") + "-webkit-" + a + "-ms-" + a.replace("grow", "positive") + a;
              case 115:
                return "-webkit-" + a + "-ms-" + a.replace("shrink", "negative") + a;
              case 98:
                return "-webkit-" + a + "-ms-" + a.replace("basis", "preferred-size") + a;
            }
          return "-webkit-" + a + "-ms-" + a + a;
        case 964:
          return "-webkit-" + a + "-ms-flex-" + a + a;
        case 1023:
          if (a.charCodeAt(8) !== 99)
            break;
          b = a.substring(a.indexOf(":", 15)).replace("flex-", "").replace("space-between", "justify");
          return "-webkit-box-pack" + b + "-webkit-" + a + "-ms-flex-pack" + b + a;
        case 1005:
          return ka.test(a) ? a.replace(aa, ":-webkit-") + a.replace(aa, ":-moz-") + a : a;
        case 1e3:
          b = a.substring(13).trim();
          c = b.indexOf("-") + 1;
          switch (b.charCodeAt(0) + b.charCodeAt(c)) {
            case 226:
              b = a.replace(G, "tb");
              break;
            case 232:
              b = a.replace(G, "tb-rl");
              break;
            case 220:
              b = a.replace(G, "lr");
              break;
            default:
              return a;
          }
          return "-webkit-" + a + "-ms-" + b + a;
        case 1017:
          if (a.indexOf("sticky", 9) === -1)
            break;
        case 975:
          c = (a = d).length - 10;
          b = (a.charCodeAt(c) === 33 ? a.substring(0, c) : a).substring(d.indexOf(":", 7) + 1).trim();
          switch (m = b.charCodeAt(0) + (b.charCodeAt(7) | 0)) {
            case 203:
              if (111 > b.charCodeAt(8))
                break;
            case 115:
              a = a.replace(b, "-webkit-" + b) + ";" + a;
              break;
            case 207:
            case 102:
              a = a.replace(b, "-webkit-" + (102 < m ? "inline-" : "") + "box") + ";" + a.replace(b, "-webkit-" + b) + ";" + a.replace(b, "-ms-" + b + "box") + ";" + a;
          }
          return a + ";";
        case 938:
          if (a.charCodeAt(5) === 45)
            switch (a.charCodeAt(6)) {
              case 105:
                return b = a.replace("-items", ""), "-webkit-" + a + "-webkit-box-" + b + "-ms-flex-" + b + a;
              case 115:
                return "-webkit-" + a + "-ms-flex-item-" + a.replace(ba, "") + a;
              default:
                return "-webkit-" + a + "-ms-flex-line-pack" + a.replace("align-content", "").replace(ba, "") + a;
            }
          break;
        case 973:
        case 989:
          if (a.charCodeAt(3) !== 45 || a.charCodeAt(4) === 122)
            break;
        case 931:
        case 953:
          if (la.test(d) === true)
            return (b = d.substring(d.indexOf(":") + 1)).charCodeAt(0) === 115 ? P(d.replace("stretch", "fill-available"), c, e, h).replace(":fill-available", ":stretch") : a.replace(b, "-webkit-" + b) + a.replace(b, "-moz-" + b.replace("fill-", "")) + a;
          break;
        case 962:
          if (a = "-webkit-" + a + (a.charCodeAt(5) === 102 ? "-ms-" + a : "") + a, e + h === 211 && a.charCodeAt(13) === 105 && 0 < a.indexOf("transform", 10))
            return a.substring(0, a.indexOf(";", 27) + 1).replace(ma, "$1-webkit-$2") + a;
      }
      return a;
    }
    function L(d, c) {
      var e = d.indexOf(c === 1 ? ":" : "{"), h = d.substring(0, c !== 3 ? e : 10);
      e = d.substring(e + 1, d.length - 1);
      return R(c !== 2 ? h : h.replace(na, "$1"), e, c);
    }
    function ea(d, c) {
      var e = P(c, c.charCodeAt(0), c.charCodeAt(1), c.charCodeAt(2));
      return e !== c + ";" ? e.replace(oa, " or ($1)").substring(4) : "(" + c + ")";
    }
    function H(d, c, e, h, a, m, b, v, n, q) {
      for (var g = 0, x = c, w2; g < A; ++g) {
        switch (w2 = S[g].call(B, d, x, e, h, a, m, b, v, n, q)) {
          case void 0:
          case false:
          case true:
          case null:
            break;
          default:
            x = w2;
        }
      }
      if (x !== c)
        return x;
    }
    function T(d) {
      switch (d) {
        case void 0:
        case null:
          A = S.length = 0;
          break;
        default:
          if (typeof d === "function")
            S[A++] = d;
          else if (typeof d === "object")
            for (var c = 0, e = d.length; c < e; ++c) {
              T(d[c]);
            }
          else
            Y = !!d | 0;
      }
      return T;
    }
    function U(d) {
      d = d.prefix;
      d !== void 0 && (R = null, d ? typeof d !== "function" ? w = 1 : (w = 2, R = d) : w = 0);
      return U;
    }
    function B(d, c) {
      var e = d;
      33 > e.charCodeAt(0) && (e = e.trim());
      V = e;
      e = [V];
      if (0 < A) {
        var h = H(-1, c, e, e, D, z, 0, 0, 0, 0);
        h !== void 0 && typeof h === "string" && (c = h);
      }
      var a = M(O, e, c, 0, 0);
      0 < A && (h = H(-2, a, e, e, D, z, a.length, 0, 0, 0), h !== void 0 && (a = h));
      V = "";
      E = 0;
      z = D = 1;
      return a;
    }
    var ca = /^\0+/g, N = /[\0\r\f]/g, aa = /: */g, ka = /zoo|gra/, ma = /([,: ])(transform)/g, ia = /,\r+?/g, F = /([\t\r\n ])*\f?&/g, fa = /@(k\w+)\s*(\S*)\s*/, Q = /::(place)/g, ha = /:(read-only)/g, G = /[svh]\w+-[tblr]{2}/, da = /\(\s*(.*)\s*\)/g, oa = /([\s\S]*?);/g, ba = /-self|flex-/g, na = /[^]*?(:[rp][el]a[\w-]+)[^]*/, la = /stretch|:\s*\w+\-(?:conte|avail)/, ja = /([^-])(image-set\()/, z = 1, D = 1, E = 0, w = 1, O = [], S = [], A = 0, R = null, Y = 0, V = "";
    B.use = T;
    B.set = U;
    W !== void 0 && U(W);
    return B;
  }
  var stylis_browser_esm_default = stylis_min;

  // node_modules/@emotion/cache/dist/cache.browser.esm.js
  var delimiter = "/*|*/";
  var needle = delimiter + "}";
  function toSheet(block) {
    if (block) {
      Sheet.current.insert(block + "}");
    }
  }
  var Sheet = {
    current: null
  };
  var ruleSheet = function ruleSheet2(context, content, selectors, parents, line, column, length, ns, depth, at) {
    switch (context) {
      case 1: {
        switch (content.charCodeAt(0)) {
          case 64: {
            Sheet.current.insert(content + ";");
            return "";
          }
          case 108: {
            if (content.charCodeAt(2) === 98) {
              return "";
            }
          }
        }
        break;
      }
      case 2: {
        if (ns === 0)
          return content + delimiter;
        break;
      }
      case 3: {
        switch (ns) {
          case 102:
          case 112: {
            Sheet.current.insert(selectors[0] + content);
            return "";
          }
          default: {
            return content + (at === 0 ? delimiter : "");
          }
        }
      }
      case -2: {
        content.split(needle).forEach(toSheet);
      }
    }
  };
  var createCache = function createCache2(options) {
    if (options === void 0)
      options = {};
    var key = options.key || "css";
    var stylisOptions;
    if (options.prefix !== void 0) {
      stylisOptions = {
        prefix: options.prefix
      };
    }
    var stylis2 = new stylis_browser_esm_default(stylisOptions);
    if (false) {
      if (/[^a-z-]/.test(key)) {
        throw new Error('Emotion key must only contain lower case alphabetical characters and - but "' + key + '" was passed');
      }
    }
    var inserted = {};
    var container;
    {
      container = options.container || document.head;
      var nodes = document.querySelectorAll("style[data-emotion-" + key + "]");
      Array.prototype.forEach.call(nodes, function(node) {
        var attrib = node.getAttribute("data-emotion-" + key);
        attrib.split(" ").forEach(function(id) {
          inserted[id] = true;
        });
        if (node.parentNode !== container) {
          container.appendChild(node);
        }
      });
    }
    var _insert;
    {
      stylis2.use(options.stylisPlugins)(ruleSheet);
      _insert = function insert(selector, serialized, sheet3, shouldCache) {
        var name = serialized.name;
        Sheet.current = sheet3;
        if (false) {
          var map2 = serialized.map;
          Sheet.current = {
            insert: function insert2(rule) {
              sheet3.insert(rule + map2);
            }
          };
        }
        stylis2(selector, serialized.styles);
        if (shouldCache) {
          cache3.inserted[name] = true;
        }
      };
    }
    if (false) {
      var commentStart = /\/\*/g;
      var commentEnd = /\*\//g;
      stylis2.use(function(context, content) {
        switch (context) {
          case -1: {
            while (commentStart.test(content)) {
              commentEnd.lastIndex = commentStart.lastIndex;
              if (commentEnd.test(content)) {
                commentStart.lastIndex = commentEnd.lastIndex;
                continue;
              }
              throw new Error('Your styles have an unterminated comment ("/*" without corresponding "*/").');
            }
            commentStart.lastIndex = 0;
            break;
          }
        }
      });
      stylis2.use(function(context, content, selectors) {
        switch (context) {
          case -1: {
            var flag = "emotion-disable-server-rendering-unsafe-selector-warning-please-do-not-use-this-the-warning-exists-for-a-reason";
            var unsafePseudoClasses = content.match(/(:first|:nth|:nth-last)-child/g);
            if (unsafePseudoClasses && cache3.compat !== true) {
              unsafePseudoClasses.forEach(function(unsafePseudoClass) {
                var ignoreRegExp = new RegExp(unsafePseudoClass + ".*\\/\\* " + flag + " \\*\\/");
                var ignore = ignoreRegExp.test(content);
                if (unsafePseudoClass && !ignore) {
                  console.error('The pseudo class "' + unsafePseudoClass + '" is potentially unsafe when doing server-side rendering. Try changing it to "' + unsafePseudoClass.split("-child")[0] + '-of-type".');
                }
              });
            }
            break;
          }
        }
      });
    }
    var cache3 = {
      key,
      sheet: new StyleSheet({
        key,
        container,
        nonce: options.nonce,
        speedy: options.speedy
      }),
      nonce: options.nonce,
      inserted,
      registered: {},
      insert: _insert
    };
    return cache3;
  };
  var cache_browser_esm_default = createCache;

  // node_modules/@emotion/core/dist/emotion-element-57a3a7a3.browser.esm.js
  const inheritsLoose = __toModule(require_inheritsLoose());
  const react9 = __toModule(require_react());

  // node_modules/@emotion/utils/dist/utils.browser.esm.js
  var isBrowser = true;
  function getRegisteredStyles(registered, registeredStyles, classNames) {
    var rawClassName = "";
    classNames.split(" ").forEach(function(className) {
      if (registered[className] !== void 0) {
        registeredStyles.push(registered[className]);
      } else {
        rawClassName += className + " ";
      }
    });
    return rawClassName;
  }
  var insertStyles = function insertStyles2(cache3, serialized, isStringTag) {
    var className = cache3.key + "-" + serialized.name;
    if ((isStringTag === false || isBrowser === false && cache3.compat !== void 0) && cache3.registered[className] === void 0) {
      cache3.registered[className] = serialized.styles;
    }
    if (cache3.inserted[serialized.name] === void 0) {
      var current = serialized;
      do {
        var maybeStyles = cache3.insert("." + className, current, cache3.sheet, true);
        current = current.next;
      } while (current !== void 0);
    }
  };

  // node_modules/@emotion/hash/dist/hash.browser.esm.js
  function murmur2(str) {
    var h = 0;
    var k, i = 0, len = str.length;
    for (; len >= 4; ++i, len -= 4) {
      k = str.charCodeAt(i) & 255 | (str.charCodeAt(++i) & 255) << 8 | (str.charCodeAt(++i) & 255) << 16 | (str.charCodeAt(++i) & 255) << 24;
      k = (k & 65535) * 1540483477 + ((k >>> 16) * 59797 << 16);
      k ^= k >>> 24;
      h = (k & 65535) * 1540483477 + ((k >>> 16) * 59797 << 16) ^ (h & 65535) * 1540483477 + ((h >>> 16) * 59797 << 16);
    }
    switch (len) {
      case 3:
        h ^= (str.charCodeAt(i + 2) & 255) << 16;
      case 2:
        h ^= (str.charCodeAt(i + 1) & 255) << 8;
      case 1:
        h ^= str.charCodeAt(i) & 255;
        h = (h & 65535) * 1540483477 + ((h >>> 16) * 59797 << 16);
    }
    h ^= h >>> 13;
    h = (h & 65535) * 1540483477 + ((h >>> 16) * 59797 << 16);
    return ((h ^ h >>> 15) >>> 0).toString(36);
  }
  var hash_browser_esm_default = murmur2;

  // node_modules/@emotion/unitless/dist/unitless.browser.esm.js
  var unitlessKeys = {
    animationIterationCount: 1,
    borderImageOutset: 1,
    borderImageSlice: 1,
    borderImageWidth: 1,
    boxFlex: 1,
    boxFlexGroup: 1,
    boxOrdinalGroup: 1,
    columnCount: 1,
    columns: 1,
    flex: 1,
    flexGrow: 1,
    flexPositive: 1,
    flexShrink: 1,
    flexNegative: 1,
    flexOrder: 1,
    gridRow: 1,
    gridRowEnd: 1,
    gridRowSpan: 1,
    gridRowStart: 1,
    gridColumn: 1,
    gridColumnEnd: 1,
    gridColumnSpan: 1,
    gridColumnStart: 1,
    msGridRow: 1,
    msGridRowSpan: 1,
    msGridColumn: 1,
    msGridColumnSpan: 1,
    fontWeight: 1,
    lineHeight: 1,
    opacity: 1,
    order: 1,
    orphans: 1,
    tabSize: 1,
    widows: 1,
    zIndex: 1,
    zoom: 1,
    WebkitLineClamp: 1,
    fillOpacity: 1,
    floodOpacity: 1,
    stopOpacity: 1,
    strokeDasharray: 1,
    strokeDashoffset: 1,
    strokeMiterlimit: 1,
    strokeOpacity: 1,
    strokeWidth: 1
  };
  var unitless_browser_esm_default = unitlessKeys;

  // node_modules/@emotion/serialize/dist/serialize.browser.esm.js
  var hyphenateRegex = /[A-Z]|^ms/g;
  var animationRegex = /_EMO_([^_]+?)_([^]*?)_EMO_/g;
  var isCustomProperty = function isCustomProperty2(property) {
    return property.charCodeAt(1) === 45;
  };
  var isProcessableValue = function isProcessableValue2(value) {
    return value != null && typeof value !== "boolean";
  };
  var processStyleName = memoize_browser_esm_default(function(styleName) {
    return isCustomProperty(styleName) ? styleName : styleName.replace(hyphenateRegex, "-$&").toLowerCase();
  });
  var processStyleValue = function processStyleValue2(key, value) {
    switch (key) {
      case "animation":
      case "animationName": {
        if (typeof value === "string") {
          return value.replace(animationRegex, function(match, p1, p2) {
            cursor = {
              name: p1,
              styles: p2,
              next: cursor
            };
            return p1;
          });
        }
      }
    }
    if (unitless_browser_esm_default[key] !== 1 && !isCustomProperty(key) && typeof value === "number" && value !== 0) {
      return value + "px";
    }
    return value;
  };
  if (false) {
    var contentValuePattern = /(attr|calc|counters?|url)\(/;
    var contentValues = ["normal", "none", "counter", "open-quote", "close-quote", "no-open-quote", "no-close-quote", "initial", "inherit", "unset"];
    var oldProcessStyleValue = processStyleValue;
    var msPattern = /^-ms-/;
    var hyphenPattern = /-(.)/g;
    var hyphenatedCache = {};
    processStyleValue = function processStyleValue3(key, value) {
      if (key === "content") {
        if (typeof value !== "string" || contentValues.indexOf(value) === -1 && !contentValuePattern.test(value) && (value.charAt(0) !== value.charAt(value.length - 1) || value.charAt(0) !== '"' && value.charAt(0) !== "'")) {
          console.error("You seem to be using a value for 'content' without quotes, try replacing it with `content: '\"" + value + "\"'`");
        }
      }
      var processed = oldProcessStyleValue(key, value);
      if (processed !== "" && !isCustomProperty(key) && key.indexOf("-") !== -1 && hyphenatedCache[key] === void 0) {
        hyphenatedCache[key] = true;
        console.error("Using kebab-case for css properties in objects is not supported. Did you mean " + key.replace(msPattern, "ms-").replace(hyphenPattern, function(str, _char) {
          return _char.toUpperCase();
        }) + "?");
      }
      return processed;
    };
  }
  var shouldWarnAboutInterpolatingClassNameFromCss = true;
  function handleInterpolation(mergedProps, registered, interpolation, couldBeSelectorInterpolation) {
    if (interpolation == null) {
      return "";
    }
    if (interpolation.__emotion_styles !== void 0) {
      if (false) {
        throw new Error("Component selectors can only be used in conjunction with babel-plugin-emotion.");
      }
      return interpolation;
    }
    switch (typeof interpolation) {
      case "boolean": {
        return "";
      }
      case "object": {
        if (interpolation.anim === 1) {
          cursor = {
            name: interpolation.name,
            styles: interpolation.styles,
            next: cursor
          };
          return interpolation.name;
        }
        if (interpolation.styles !== void 0) {
          var next = interpolation.next;
          if (next !== void 0) {
            while (next !== void 0) {
              cursor = {
                name: next.name,
                styles: next.styles,
                next: cursor
              };
              next = next.next;
            }
          }
          var styles = interpolation.styles + ";";
          if (false) {
            styles += interpolation.map;
          }
          return styles;
        }
        return createStringFromObject(mergedProps, registered, interpolation);
      }
      case "function": {
        if (mergedProps !== void 0) {
          var previousCursor = cursor;
          var result = interpolation(mergedProps);
          cursor = previousCursor;
          return handleInterpolation(mergedProps, registered, result, couldBeSelectorInterpolation);
        } else if (false) {
          console.error("Functions that are interpolated in css calls will be stringified.\nIf you want to have a css call based on props, create a function that returns a css call like this\nlet dynamicStyle = (props) => css`color: ${props.color}`\nIt can be called directly with props or interpolated in a styled call like this\nlet SomeComponent = styled('div')`${dynamicStyle}`");
        }
        break;
      }
      case "string":
        if (false) {
          var matched = [];
          var replaced = interpolation.replace(animationRegex, function(match, p1, p2) {
            var fakeVarName = "animation" + matched.length;
            matched.push("const " + fakeVarName + " = keyframes`" + p2.replace(/^@keyframes animation-\w+/, "") + "`");
            return "${" + fakeVarName + "}";
          });
          if (matched.length) {
            console.error("`keyframes` output got interpolated into plain string, please wrap it with `css`.\n\nInstead of doing this:\n\n" + [].concat(matched, ["`" + replaced + "`"]).join("\n") + "\n\nYou should wrap it with `css` like this:\n\n" + ("css`" + replaced + "`"));
          }
        }
        break;
    }
    if (registered == null) {
      return interpolation;
    }
    var cached = registered[interpolation];
    if (false) {
      console.error("Interpolating a className from css`` is not recommended and will cause problems with composition.\nInterpolating a className from css`` will be completely unsupported in a future major version of Emotion");
      shouldWarnAboutInterpolatingClassNameFromCss = false;
    }
    return cached !== void 0 && !couldBeSelectorInterpolation ? cached : interpolation;
  }
  function createStringFromObject(mergedProps, registered, obj) {
    var string2 = "";
    if (Array.isArray(obj)) {
      for (var i = 0; i < obj.length; i++) {
        string2 += handleInterpolation(mergedProps, registered, obj[i], false);
      }
    } else {
      for (var _key in obj) {
        var value = obj[_key];
        if (typeof value !== "object") {
          if (registered != null && registered[value] !== void 0) {
            string2 += _key + "{" + registered[value] + "}";
          } else if (isProcessableValue(value)) {
            string2 += processStyleName(_key) + ":" + processStyleValue(_key, value) + ";";
          }
        } else {
          if (_key === "NO_COMPONENT_SELECTOR" && false) {
            throw new Error("Component selectors can only be used in conjunction with babel-plugin-emotion.");
          }
          if (Array.isArray(value) && typeof value[0] === "string" && (registered == null || registered[value[0]] === void 0)) {
            for (var _i = 0; _i < value.length; _i++) {
              if (isProcessableValue(value[_i])) {
                string2 += processStyleName(_key) + ":" + processStyleValue(_key, value[_i]) + ";";
              }
            }
          } else {
            var interpolated = handleInterpolation(mergedProps, registered, value, false);
            switch (_key) {
              case "animation":
              case "animationName": {
                string2 += processStyleName(_key) + ":" + interpolated + ";";
                break;
              }
              default: {
                if (false) {
                  console.error(UNDEFINED_AS_OBJECT_KEY_ERROR);
                }
                string2 += _key + "{" + interpolated + "}";
              }
            }
          }
        }
      }
    }
    return string2;
  }
  var labelPattern = /label:\s*([^\s;\n{]+)\s*;/g;
  var sourceMapPattern;
  if (false) {
    sourceMapPattern = /\/\*#\ssourceMappingURL=data:application\/json;\S+\s+\*\//;
  }
  var cursor;
  var serializeStyles = function serializeStyles2(args, registered, mergedProps) {
    if (args.length === 1 && typeof args[0] === "object" && args[0] !== null && args[0].styles !== void 0) {
      return args[0];
    }
    var stringMode = true;
    var styles = "";
    cursor = void 0;
    var strings = args[0];
    if (strings == null || strings.raw === void 0) {
      stringMode = false;
      styles += handleInterpolation(mergedProps, registered, strings, false);
    } else {
      if (false) {
        console.error(ILLEGAL_ESCAPE_SEQUENCE_ERROR);
      }
      styles += strings[0];
    }
    for (var i = 1; i < args.length; i++) {
      styles += handleInterpolation(mergedProps, registered, args[i], styles.charCodeAt(styles.length - 1) === 46);
      if (stringMode) {
        if (false) {
          console.error(ILLEGAL_ESCAPE_SEQUENCE_ERROR);
        }
        styles += strings[i];
      }
    }
    var sourceMap;
    if (false) {
      styles = styles.replace(sourceMapPattern, function(match2) {
        sourceMap = match2;
        return "";
      });
    }
    labelPattern.lastIndex = 0;
    var identifierName = "";
    var match;
    while ((match = labelPattern.exec(styles)) !== null) {
      identifierName += "-" + match[1];
    }
    var name = hash_browser_esm_default(styles) + identifierName;
    if (false) {
      return {
        name,
        styles,
        map: sourceMap,
        next: cursor,
        toString: function toString2() {
          return "You have tried to stringify object returned from `css` function. It isn't supposed to be used directly (e.g. as value of the `className` prop), but rather handed to emotion so it can handle it (e.g. as value of `css` prop).";
        }
      };
    }
    return {
      name,
      styles,
      next: cursor
    };
  };

  // node_modules/@emotion/core/dist/emotion-element-57a3a7a3.browser.esm.js
  var EmotionCacheContext = /* @__PURE__ */ react9.createContext(typeof HTMLElement !== "undefined" ? cache_browser_esm_default() : null);
  var ThemeContext = /* @__PURE__ */ react9.createContext({});
  var CacheProvider = EmotionCacheContext.Provider;
  var withEmotionCache = function withEmotionCache2(func) {
    var render2 = function render3(props, ref) {
      return /* @__PURE__ */ react9.createElement(EmotionCacheContext.Consumer, null, function(cache3) {
        return func(props, cache3, ref);
      });
    };
    return /* @__PURE__ */ react9.forwardRef(render2);
  };
  if (false) {
    Emotion.displayName = "EmotionCssPropInternal";
  }

  // node_modules/@emotion/core/dist/core.browser.esm.js
  var InnerGlobal = /* @__PURE__ */ function(_React$Component) {
    _inheritsLoose2(InnerGlobal2, _React$Component);
    function InnerGlobal2(props, context, updater) {
      return _React$Component.call(this, props, context, updater) || this;
    }
    var _proto = InnerGlobal2.prototype;
    _proto.componentDidMount = function componentDidMount() {
      this.sheet = new StyleSheet({
        key: this.props.cache.key + "-global",
        nonce: this.props.cache.sheet.nonce,
        container: this.props.cache.sheet.container
      });
      var node = document.querySelector("style[data-emotion-" + this.props.cache.key + '="' + this.props.serialized.name + '"]');
      if (node !== null) {
        this.sheet.tags.push(node);
      }
      if (this.props.cache.sheet.tags.length) {
        this.sheet.before = this.props.cache.sheet.tags[0];
      }
      this.insertStyles();
    };
    _proto.componentDidUpdate = function componentDidUpdate(prevProps) {
      if (prevProps.serialized.name !== this.props.serialized.name) {
        this.insertStyles();
      }
    };
    _proto.insertStyles = function insertStyles$1() {
      if (this.props.serialized.next !== void 0) {
        insertStyles(this.props.cache, this.props.serialized.next, true);
      }
      if (this.sheet.tags.length) {
        var element = this.sheet.tags[this.sheet.tags.length - 1].nextElementSibling;
        this.sheet.before = element;
        this.sheet.flush();
      }
      this.props.cache.insert("", this.props.serialized, this.sheet, false);
    };
    _proto.componentWillUnmount = function componentWillUnmount() {
      this.sheet.flush();
    };
    _proto.render = function render2() {
      return null;
    };
    return InnerGlobal2;
  }(react10.Component);
  var classnames = function classnames2(args) {
    var len = args.length;
    var i = 0;
    var cls = "";
    for (; i < len; i++) {
      var arg = args[i];
      if (arg == null)
        continue;
      var toAdd = void 0;
      switch (typeof arg) {
        case "boolean":
          break;
        case "object": {
          if (Array.isArray(arg)) {
            toAdd = classnames2(arg);
          } else {
            toAdd = "";
            for (var k in arg) {
              if (arg[k] && k) {
                toAdd && (toAdd += " ");
                toAdd += k;
              }
            }
          }
          break;
        }
        default: {
          toAdd = arg;
        }
      }
      if (toAdd) {
        cls && (cls += " ");
        cls += toAdd;
      }
    }
    return cls;
  };
  function merge(registered, css3, className) {
    var registeredStyles = [];
    var rawClassName = getRegisteredStyles(registered, registeredStyles, className);
    if (registeredStyles.length < 2) {
      return className;
    }
    return rawClassName + css3(registeredStyles);
  }
  var ClassNames = withEmotionCache(function(props, context) {
    return /* @__PURE__ */ react10.createElement(ThemeContext.Consumer, null, function(theme) {
      var hasRendered = false;
      var css3 = function css4() {
        if (hasRendered && false) {
          throw new Error("css can only be used during render");
        }
        for (var _len = arguments.length, args = new Array(_len), _key = 0; _key < _len; _key++) {
          args[_key] = arguments[_key];
        }
        var serialized = serializeStyles(args, context.registered);
        {
          insertStyles(context, serialized, false);
        }
        return context.key + "-" + serialized.name;
      };
      var cx = function cx2() {
        if (hasRendered && false) {
          throw new Error("cx can only be used during render");
        }
        for (var _len2 = arguments.length, args = new Array(_len2), _key2 = 0; _key2 < _len2; _key2++) {
          args[_key2] = arguments[_key2];
        }
        return merge(context.registered, css3, classnames(args));
      };
      var content = {
        css: css3,
        cx,
        theme
      };
      var ele = props.children(content);
      hasRendered = true;
      return ele;
    });
  });

  // node_modules/@emotion/styled-base/dist/styled-base.browser.esm.js
  var testOmitPropsOnStringTag = is_prop_valid_browser_esm_default;
  var testOmitPropsOnComponent = function testOmitPropsOnComponent2(key) {
    return key !== "theme" && key !== "innerRef";
  };
  var getDefaultShouldForwardProp = function getDefaultShouldForwardProp2(tag) {
    return typeof tag === "string" && tag.charCodeAt(0) > 96 ? testOmitPropsOnStringTag : testOmitPropsOnComponent;
  };
  function ownKeys3(object3, enumerableOnly) {
    var keys = Object.keys(object3);
    if (Object.getOwnPropertySymbols) {
      var symbols = Object.getOwnPropertySymbols(object3);
      if (enumerableOnly)
        symbols = symbols.filter(function(sym) {
          return Object.getOwnPropertyDescriptor(object3, sym).enumerable;
        });
      keys.push.apply(keys, symbols);
    }
    return keys;
  }
  function _objectSpread(target) {
    for (var i = 1; i < arguments.length; i++) {
      var source = arguments[i] != null ? arguments[i] : {};
      if (i % 2) {
        ownKeys3(source, true).forEach(function(key) {
          defineProperty2.default(target, key, source[key]);
        });
      } else if (Object.getOwnPropertyDescriptors) {
        Object.defineProperties(target, Object.getOwnPropertyDescriptors(source));
      } else {
        ownKeys3(source).forEach(function(key) {
          Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key));
        });
      }
    }
    return target;
  }
  var createStyled = function createStyled2(tag, options) {
    if (false) {
      if (tag === void 0) {
        throw new Error("You are trying to create a styled element with an undefined component.\nYou may have forgotten to import it.");
      }
    }
    var identifierName;
    var shouldForwardProp;
    var targetClassName;
    if (options !== void 0) {
      identifierName = options.label;
      targetClassName = options.target;
      shouldForwardProp = tag.__emotion_forwardProp && options.shouldForwardProp ? function(propName) {
        return tag.__emotion_forwardProp(propName) && options.shouldForwardProp(propName);
      } : options.shouldForwardProp;
    }
    var isReal = tag.__emotion_real === tag;
    var baseTag = isReal && tag.__emotion_base || tag;
    if (typeof shouldForwardProp !== "function" && isReal) {
      shouldForwardProp = tag.__emotion_forwardProp;
    }
    var defaultShouldForwardProp = shouldForwardProp || getDefaultShouldForwardProp(baseTag);
    var shouldUseAs = !defaultShouldForwardProp("as");
    return function() {
      var args = arguments;
      var styles = isReal && tag.__emotion_styles !== void 0 ? tag.__emotion_styles.slice(0) : [];
      if (identifierName !== void 0) {
        styles.push("label:" + identifierName + ";");
      }
      if (args[0] == null || args[0].raw === void 0) {
        styles.push.apply(styles, args);
      } else {
        if (false) {
          console.error(ILLEGAL_ESCAPE_SEQUENCE_ERROR);
        }
        styles.push(args[0][0]);
        var len = args.length;
        var i = 1;
        for (; i < len; i++) {
          if (false) {
            console.error(ILLEGAL_ESCAPE_SEQUENCE_ERROR);
          }
          styles.push(args[i], args[0][i]);
        }
      }
      var Styled = withEmotionCache(function(props, context, ref) {
        return react11.createElement(ThemeContext.Consumer, null, function(theme) {
          var finalTag = shouldUseAs && props.as || baseTag;
          var className = "";
          var classInterpolations = [];
          var mergedProps = props;
          if (props.theme == null) {
            mergedProps = {};
            for (var key in props) {
              mergedProps[key] = props[key];
            }
            mergedProps.theme = theme;
          }
          if (typeof props.className === "string") {
            className = getRegisteredStyles(context.registered, classInterpolations, props.className);
          } else if (props.className != null) {
            className = props.className + " ";
          }
          var serialized = serializeStyles(styles.concat(classInterpolations), context.registered, mergedProps);
          var rules = insertStyles(context, serialized, typeof finalTag === "string");
          className += context.key + "-" + serialized.name;
          if (targetClassName !== void 0) {
            className += " " + targetClassName;
          }
          var finalShouldForwardProp = shouldUseAs && shouldForwardProp === void 0 ? getDefaultShouldForwardProp(finalTag) : defaultShouldForwardProp;
          var newProps = {};
          for (var _key in props) {
            if (shouldUseAs && _key === "as")
              continue;
            if (finalShouldForwardProp(_key)) {
              newProps[_key] = props[_key];
            }
          }
          newProps.className = className;
          newProps.ref = ref || props.innerRef;
          if (false) {
            console.error("`innerRef` is deprecated and will be removed in a future major version of Emotion, please use the `ref` prop instead" + (identifierName === void 0 ? "" : " in the usage of `" + identifierName + "`"));
          }
          var ele = react11.createElement(finalTag, newProps);
          return ele;
        });
      });
      Styled.displayName = identifierName !== void 0 ? identifierName : "Styled(" + (typeof baseTag === "string" ? baseTag : baseTag.displayName || baseTag.name || "Component") + ")";
      Styled.defaultProps = tag.defaultProps;
      Styled.__emotion_real = Styled;
      Styled.__emotion_base = baseTag;
      Styled.__emotion_styles = styles;
      Styled.__emotion_forwardProp = shouldForwardProp;
      Object.defineProperty(Styled, "toString", {
        value: function value() {
          if (targetClassName === void 0 && false) {
            return "NO_COMPONENT_SELECTOR";
          }
          return "." + targetClassName;
        }
      });
      Styled.withComponent = function(nextTag, nextOptions) {
        return createStyled2(nextTag, nextOptions !== void 0 ? _objectSpread({}, options || {}, {}, nextOptions) : options).apply(void 0, styles);
      };
      return Styled;
    };
  };
  var styled_base_browser_esm_default = createStyled;

  // node_modules/@emotion/styled/dist/styled.browser.esm.js
  var tags = [
    "a",
    "abbr",
    "address",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "bdi",
    "bdo",
    "big",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "img",
    "input",
    "ins",
    "kbd",
    "keygen",
    "label",
    "legend",
    "li",
    "link",
    "main",
    "map",
    "mark",
    "marquee",
    "menu",
    "menuitem",
    "meta",
    "meter",
    "nav",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "pre",
    "progress",
    "q",
    "rp",
    "rt",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "small",
    "source",
    "span",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
    "circle",
    "clipPath",
    "defs",
    "ellipse",
    "foreignObject",
    "g",
    "image",
    "line",
    "linearGradient",
    "mask",
    "path",
    "pattern",
    "polygon",
    "polyline",
    "radialGradient",
    "rect",
    "stop",
    "svg",
    "text",
    "tspan"
  ];
  var newStyled = styled_base_browser_esm_default.bind();
  tags.forEach(function(tagName) {
    newStyled[tagName] = newStyled(tagName);
  });
  var styled_browser_esm_default = newStyled;

  // src/client/App.tsx
  const PathWrapper = styled_browser_esm_default("div")`
    border: 1px solid red;
    padding: 10px;
    margin-top: 10px;
`;
  const Path = observer3(() => {
    const appState = useAppStateContext();
    const chunks = appState.currentPath.pathChunks;
    if (chunks.length === 0) {
      return /* @__PURE__ */ React3.createElement(PathWrapper, null, "--root--");
    }
    return /* @__PURE__ */ React3.createElement(PathWrapper, null, chunks.map((item) => {
      return /* @__PURE__ */ React3.createElement("div", null, item);
    }));
  });
  const ListWrapper = styled_browser_esm_default("div")`
    border: 1px solid blue;
    padding: 10px;
    margin-top: 10px;
`;
  const List = observer3(() => {
    const appState = useAppStateContext();
    return /* @__PURE__ */ React3.createElement(ListWrapper, null, "lista plik\xF3w todo");
  });
  const App = observer3(() => {
    const appState = useAppStateContext();
    const [state, setState] = React3.useState(false);
    return /* @__PURE__ */ React3.createElement("div", null, /* @__PURE__ */ React3.createElement("div", {
      onClick: () => setState(!state)
    }, "toogle ", state), state ? /* @__PURE__ */ React3.createElement("div", null, "to jest applikacja ", appState.counterValue) : null, /* @__PURE__ */ React3.createElement(Path, null), /* @__PURE__ */ React3.createElement(List, null));
  });

  // src/common/MobxMapAutoNew.ts
  class MobxMapAutoNew {
    constructor(fnBuild) {
      this.fnBuild = fnBuild;
      this.data = new Map();
    }
    get(key) {
      const value = this.data.get(key);
      if (typeof value !== "undefined") {
        return value;
      }
      const newValue = this.fnBuild(key);
      this.data.set(key, newValue);
      return newValue;
    }
  }

  // src/common/MobxValueConnect.ts
  class MobxValueConnect {
    constructor(id, value, onConnect, onDisconnect) {
      this.subscription = null;
      this.id = id;
      this.value = value;
      this.onConnect = onConnect;
      this.onDisconnect = onDisconnect;
      makeObservable(this, {
        value: observable
      });
      onBecomeObserved(this, "value", () => {
        console.info("value start observe", this.id);
        if (this.subscription !== null) {
          throw Error("onBecomeObserved - Nieprawid\u0142owy stan");
        }
        this.subscription = this.onConnect(this);
      });
      onBecomeUnobserved(this, "value", () => {
        console.info("value stop observe", this.id);
        if (this.subscription === null) {
          throw Error("onBecomeUnobserved - Nieprawid\u0142owy stan");
        }
        this.onDisconnect(this.subscription);
        this.subscription = null;
      });
    }
    setValue(value) {
      this.value = value;
    }
  }

  // src/client/AppState/ContentState.ts
  const startCounter = (_mobxValue) => {
    return setInterval(() => {
    }, 1e3);
  };
  const stopCounter = (timer) => {
    clearInterval(timer);
  };
  class ContentState {
    constructor() {
      this.data = new MobxMapAutoNew((path) => {
        return new MobxValueConnect(path, null, startCounter, stopCounter);
      });
    }
  }

  // src/client/AppState/inne/CounterState.ts
  const startCounter2 = (mobxValue) => {
    return setInterval(() => {
      mobxValue.setValue(mobxValue.value + 1);
    }, 1e3);
  };
  const stopCounter2 = (timer) => {
    clearInterval(timer);
  };
  class CounterState {
    constructor() {
      this.counter = new MobxValueConnect(void 0, 0, startCounter2, stopCounter2);
    }
  }

  // src/common/MobxValue.ts
  class MobxValue {
    constructor(value) {
      this.value = value;
      makeObservable(this, {
        value: observable
      });
    }
    setValue(value) {
      this.value = value;
    }
  }

  // src/client/AppState/CurrentPath.ts
  class CurrentPath {
    constructor(path, content) {
      makeAutoObservable(this);
      this.path = new MobxValue(path);
      this.content = content;
    }
    get pathChunks() {
      return this.path.value.split("/").map((item) => item.trim()).filter((item) => item !== "");
    }
  }

  // node_modules/fp-ts/es6/function.js
  function identity(a) {
    return a;
  }

  // node_modules/fp-ts/es6/Either.js
  var isLeft = function(ma) {
    return ma._tag === "Left";
  };
  var isRight = function(ma) {
    return ma._tag === "Right";
  };
  var left = function(e) {
    return {_tag: "Left", left: e};
  };
  var right = function(a) {
    return {_tag: "Right", right: a};
  };
  function fold(onLeft, onRight) {
    return function(ma) {
      return isLeft(ma) ? onLeft(ma.left) : onRight(ma.right);
    };
  }
  var chainW = function(f) {
    return function(ma) {
      return isLeft(ma) ? ma : f(ma.right);
    };
  };
  var chain = chainW;
  var flatten = /* @__PURE__ */ chain(identity);
  var extend = function(f) {
    return function(wa) {
      return isLeft(wa) ? wa : right(f(wa));
    };
  };
  var duplicate = /* @__PURE__ */ extend(identity);

  // node_modules/io-ts/es6/index.js
  var __extends = function() {
    var extendStatics = function(d, b) {
      extendStatics = Object.setPrototypeOf || {__proto__: []} instanceof Array && function(d2, b2) {
        d2.__proto__ = b2;
      } || function(d2, b2) {
        for (var p in b2)
          if (Object.prototype.hasOwnProperty.call(b2, p))
            d2[p] = b2[p];
      };
      return extendStatics(d, b);
    };
    return function(d, b) {
      extendStatics(d, b);
      function __() {
        this.constructor = d;
      }
      d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
  }();
  var __assign2 = function() {
    __assign2 = Object.assign || function(t2) {
      for (var s, i = 1, n = arguments.length; i < n; i++) {
        s = arguments[i];
        for (var p in s)
          if (Object.prototype.hasOwnProperty.call(s, p))
            t2[p] = s[p];
      }
      return t2;
    };
    return __assign2.apply(this, arguments);
  };
  var Type = function() {
    function Type2(name, is2, validate, encode) {
      this.name = name;
      this.is = is2;
      this.validate = validate;
      this.encode = encode;
      this.decode = this.decode.bind(this);
    }
    Type2.prototype.pipe = function(ab, name) {
      var _this = this;
      if (name === void 0) {
        name = "pipe(" + this.name + ", " + ab.name + ")";
      }
      return new Type2(name, ab.is, function(i, c) {
        var e = _this.validate(i, c);
        if (isLeft(e)) {
          return e;
        }
        return ab.validate(e.right, c);
      }, this.encode === identity2 && ab.encode === identity2 ? identity2 : function(b) {
        return _this.encode(ab.encode(b));
      });
    };
    Type2.prototype.asDecoder = function() {
      return this;
    };
    Type2.prototype.asEncoder = function() {
      return this;
    };
    Type2.prototype.decode = function(i) {
      return this.validate(i, [{key: "", type: this, actual: i}]);
    };
    return Type2;
  }();
  var identity2 = function(a) {
    return a;
  };
  var getFunctionName = function(f) {
    return f.displayName || f.name || "<function" + f.length + ">";
  };
  var appendContext = function(c, key, decoder, actual) {
    var len = c.length;
    var r = Array(len + 1);
    for (var i = 0; i < len; i++) {
      r[i] = c[i];
    }
    r[len] = {key, type: decoder, actual};
    return r;
  };
  var failures = left;
  var failure = function(value, context, message) {
    return failures([{value, context, message}]);
  };
  var success = right;
  var pushAll = function(xs, ys) {
    var l = ys.length;
    for (var i = 0; i < l; i++) {
      xs.push(ys[i]);
    }
  };
  var NullType = function(_super) {
    __extends(NullType2, _super);
    function NullType2() {
      var _this = _super.call(this, "null", function(u) {
        return u === null;
      }, function(u, c) {
        return _this.is(u) ? success(u) : failure(u, c);
      }, identity2) || this;
      _this._tag = "NullType";
      return _this;
    }
    return NullType2;
  }(Type);
  var nullType = new NullType();
  var UndefinedType = function(_super) {
    __extends(UndefinedType2, _super);
    function UndefinedType2() {
      var _this = _super.call(this, "undefined", function(u) {
        return u === void 0;
      }, function(u, c) {
        return _this.is(u) ? success(u) : failure(u, c);
      }, identity2) || this;
      _this._tag = "UndefinedType";
      return _this;
    }
    return UndefinedType2;
  }(Type);
  var undefinedType = new UndefinedType();
  var VoidType = function(_super) {
    __extends(VoidType2, _super);
    function VoidType2() {
      var _this = _super.call(this, "void", undefinedType.is, undefinedType.validate, identity2) || this;
      _this._tag = "VoidType";
      return _this;
    }
    return VoidType2;
  }(Type);
  var voidType = new VoidType();
  var UnknownType = function(_super) {
    __extends(UnknownType2, _super);
    function UnknownType2() {
      var _this = _super.call(this, "unknown", function(_) {
        return true;
      }, success, identity2) || this;
      _this._tag = "UnknownType";
      return _this;
    }
    return UnknownType2;
  }(Type);
  var unknown = new UnknownType();
  var StringType = function(_super) {
    __extends(StringType2, _super);
    function StringType2() {
      var _this = _super.call(this, "string", function(u) {
        return typeof u === "string";
      }, function(u, c) {
        return _this.is(u) ? success(u) : failure(u, c);
      }, identity2) || this;
      _this._tag = "StringType";
      return _this;
    }
    return StringType2;
  }(Type);
  var string = new StringType();
  var NumberType = function(_super) {
    __extends(NumberType2, _super);
    function NumberType2() {
      var _this = _super.call(this, "number", function(u) {
        return typeof u === "number";
      }, function(u, c) {
        return _this.is(u) ? success(u) : failure(u, c);
      }, identity2) || this;
      _this._tag = "NumberType";
      return _this;
    }
    return NumberType2;
  }(Type);
  var number = new NumberType();
  var BigIntType = function(_super) {
    __extends(BigIntType2, _super);
    function BigIntType2() {
      var _this = _super.call(this, "bigint", function(u) {
        return typeof u === "bigint";
      }, function(u, c) {
        return _this.is(u) ? success(u) : failure(u, c);
      }, identity2) || this;
      _this._tag = "BigIntType";
      return _this;
    }
    return BigIntType2;
  }(Type);
  var bigint = new BigIntType();
  var BooleanType = function(_super) {
    __extends(BooleanType2, _super);
    function BooleanType2() {
      var _this = _super.call(this, "boolean", function(u) {
        return typeof u === "boolean";
      }, function(u, c) {
        return _this.is(u) ? success(u) : failure(u, c);
      }, identity2) || this;
      _this._tag = "BooleanType";
      return _this;
    }
    return BooleanType2;
  }(Type);
  var boolean = new BooleanType();
  var AnyArrayType = function(_super) {
    __extends(AnyArrayType2, _super);
    function AnyArrayType2() {
      var _this = _super.call(this, "UnknownArray", Array.isArray, function(u, c) {
        return _this.is(u) ? success(u) : failure(u, c);
      }, identity2) || this;
      _this._tag = "AnyArrayType";
      return _this;
    }
    return AnyArrayType2;
  }(Type);
  var UnknownArray = new AnyArrayType();
  var AnyDictionaryType = function(_super) {
    __extends(AnyDictionaryType2, _super);
    function AnyDictionaryType2() {
      var _this = _super.call(this, "UnknownRecord", function(u) {
        var s = Object.prototype.toString.call(u);
        return s === "[object Object]" || s === "[object Window]";
      }, function(u, c) {
        return _this.is(u) ? success(u) : failure(u, c);
      }, identity2) || this;
      _this._tag = "AnyDictionaryType";
      return _this;
    }
    return AnyDictionaryType2;
  }(Type);
  var UnknownRecord = new AnyDictionaryType();
  var FunctionType = function(_super) {
    __extends(FunctionType2, _super);
    function FunctionType2() {
      var _this = _super.call(this, "Function", function(u) {
        return typeof u === "function";
      }, function(u, c) {
        return _this.is(u) ? success(u) : failure(u, c);
      }, identity2) || this;
      _this._tag = "FunctionType";
      return _this;
    }
    return FunctionType2;
  }(Type);
  var Function2 = new FunctionType();
  var RefinementType = function(_super) {
    __extends(RefinementType2, _super);
    function RefinementType2(name, is2, validate, encode, type2, predicate) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.type = type2;
      _this.predicate = predicate;
      _this._tag = "RefinementType";
      return _this;
    }
    return RefinementType2;
  }(Type);
  var brand = function(codec, predicate, name) {
    return refinement(codec, predicate, name);
  };
  var Int = brand(number, function(n) {
    return Number.isInteger(n);
  }, "Int");
  var LiteralType = function(_super) {
    __extends(LiteralType2, _super);
    function LiteralType2(name, is2, validate, encode, value) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.value = value;
      _this._tag = "LiteralType";
      return _this;
    }
    return LiteralType2;
  }(Type);
  var literal = function(value, name) {
    if (name === void 0) {
      name = JSON.stringify(value);
    }
    var is2 = function(u) {
      return u === value;
    };
    return new LiteralType(name, is2, function(u, c) {
      return is2(u) ? success(value) : failure(u, c);
    }, identity2, value);
  };
  var KeyofType = function(_super) {
    __extends(KeyofType2, _super);
    function KeyofType2(name, is2, validate, encode, keys) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.keys = keys;
      _this._tag = "KeyofType";
      return _this;
    }
    return KeyofType2;
  }(Type);
  var hasOwnProperty2 = Object.prototype.hasOwnProperty;
  var RecursiveType = function(_super) {
    __extends(RecursiveType2, _super);
    function RecursiveType2(name, is2, validate, encode, runDefinition) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.runDefinition = runDefinition;
      _this._tag = "RecursiveType";
      return _this;
    }
    return RecursiveType2;
  }(Type);
  Object.defineProperty(RecursiveType.prototype, "type", {
    get: function() {
      return this.runDefinition();
    },
    enumerable: true,
    configurable: true
  });
  var ArrayType = function(_super) {
    __extends(ArrayType2, _super);
    function ArrayType2(name, is2, validate, encode, type2) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.type = type2;
      _this._tag = "ArrayType";
      return _this;
    }
    return ArrayType2;
  }(Type);
  var array2 = function(item, name) {
    if (name === void 0) {
      name = "Array<" + item.name + ">";
    }
    return new ArrayType(name, function(u) {
      return UnknownArray.is(u) && u.every(item.is);
    }, function(u, c) {
      var e = UnknownArray.validate(u, c);
      if (isLeft(e)) {
        return e;
      }
      var us = e.right;
      var len = us.length;
      var as = us;
      var errors = [];
      for (var i = 0; i < len; i++) {
        var ui = us[i];
        var result = item.validate(ui, appendContext(c, String(i), item, ui));
        if (isLeft(result)) {
          pushAll(errors, result.left);
        } else {
          var ai = result.right;
          if (ai !== ui) {
            if (as === us) {
              as = us.slice();
            }
            as[i] = ai;
          }
        }
      }
      return errors.length > 0 ? failures(errors) : success(as);
    }, item.encode === identity2 ? identity2 : function(a) {
      return a.map(item.encode);
    }, item);
  };
  var InterfaceType = function(_super) {
    __extends(InterfaceType2, _super);
    function InterfaceType2(name, is2, validate, encode, props) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.props = props;
      _this._tag = "InterfaceType";
      return _this;
    }
    return InterfaceType2;
  }(Type);
  var getNameFromProps = function(props) {
    return Object.keys(props).map(function(k) {
      return k + ": " + props[k].name;
    }).join(", ");
  };
  var useIdentity = function(codecs) {
    for (var i = 0; i < codecs.length; i++) {
      if (codecs[i].encode !== identity2) {
        return false;
      }
    }
    return true;
  };
  var getInterfaceTypeName = function(props) {
    return "{ " + getNameFromProps(props) + " }";
  };
  var type = function(props, name) {
    if (name === void 0) {
      name = getInterfaceTypeName(props);
    }
    var keys = Object.keys(props);
    var types = keys.map(function(key) {
      return props[key];
    });
    var len = keys.length;
    return new InterfaceType(name, function(u) {
      if (UnknownRecord.is(u)) {
        for (var i = 0; i < len; i++) {
          var k = keys[i];
          var uk = u[k];
          if (uk === void 0 && !hasOwnProperty2.call(u, k) || !types[i].is(uk)) {
            return false;
          }
        }
        return true;
      }
      return false;
    }, function(u, c) {
      var e = UnknownRecord.validate(u, c);
      if (isLeft(e)) {
        return e;
      }
      var o = e.right;
      var a = o;
      var errors = [];
      for (var i = 0; i < len; i++) {
        var k = keys[i];
        var ak = a[k];
        var type_1 = types[i];
        var result = type_1.validate(ak, appendContext(c, k, type_1, ak));
        if (isLeft(result)) {
          pushAll(errors, result.left);
        } else {
          var vak = result.right;
          if (vak !== ak || vak === void 0 && !hasOwnProperty2.call(a, k)) {
            if (a === o) {
              a = __assign2({}, o);
            }
            a[k] = vak;
          }
        }
      }
      return errors.length > 0 ? failures(errors) : success(a);
    }, useIdentity(types) ? identity2 : function(a) {
      var s = __assign2({}, a);
      for (var i = 0; i < len; i++) {
        var k = keys[i];
        var encode = types[i].encode;
        if (encode !== identity2) {
          s[k] = encode(a[k]);
        }
      }
      return s;
    }, props);
  };
  var PartialType = function(_super) {
    __extends(PartialType2, _super);
    function PartialType2(name, is2, validate, encode, props) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.props = props;
      _this._tag = "PartialType";
      return _this;
    }
    return PartialType2;
  }(Type);
  var DictionaryType = function(_super) {
    __extends(DictionaryType2, _super);
    function DictionaryType2(name, is2, validate, encode, domain, codomain) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.domain = domain;
      _this.codomain = codomain;
      _this._tag = "DictionaryType";
      return _this;
    }
    return DictionaryType2;
  }(Type);
  var UnionType = function(_super) {
    __extends(UnionType2, _super);
    function UnionType2(name, is2, validate, encode, types) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.types = types;
      _this._tag = "UnionType";
      return _this;
    }
    return UnionType2;
  }(Type);
  var getUnionName = function(codecs) {
    return "(" + codecs.map(function(type2) {
      return type2.name;
    }).join(" | ") + ")";
  };
  var union = function(codecs, name) {
    if (name === void 0) {
      name = getUnionName(codecs);
    }
    var index2 = getIndex(codecs);
    if (index2 !== void 0 && codecs.length > 0) {
      var tag_1 = index2[0], groups_1 = index2[1];
      var len_1 = groups_1.length;
      var find_1 = function(value) {
        for (var i = 0; i < len_1; i++) {
          if (groups_1[i].indexOf(value) !== -1) {
            return i;
          }
        }
        return void 0;
      };
      return new TaggedUnionType(name, function(u) {
        if (UnknownRecord.is(u)) {
          var i = find_1(u[tag_1]);
          return i !== void 0 ? codecs[i].is(u) : false;
        }
        return false;
      }, function(u, c) {
        var e = UnknownRecord.validate(u, c);
        if (isLeft(e)) {
          return e;
        }
        var r = e.right;
        var i = find_1(r[tag_1]);
        if (i === void 0) {
          return failure(u, c);
        }
        var codec = codecs[i];
        return codec.validate(r, appendContext(c, String(i), codec, r));
      }, useIdentity(codecs) ? identity2 : function(a) {
        var i = find_1(a[tag_1]);
        if (i === void 0) {
          throw new Error("no codec found to encode value in union codec " + name);
        } else {
          return codecs[i].encode(a);
        }
      }, codecs, tag_1);
    } else {
      return new UnionType(name, function(u) {
        return codecs.some(function(type2) {
          return type2.is(u);
        });
      }, function(u, c) {
        var errors = [];
        for (var i = 0; i < codecs.length; i++) {
          var codec = codecs[i];
          var result = codec.validate(u, appendContext(c, String(i), codec, u));
          if (isLeft(result)) {
            pushAll(errors, result.left);
          } else {
            return success(result.right);
          }
        }
        return failures(errors);
      }, useIdentity(codecs) ? identity2 : function(a) {
        for (var _i = 0, codecs_1 = codecs; _i < codecs_1.length; _i++) {
          var codec = codecs_1[_i];
          if (codec.is(a)) {
            return codec.encode(a);
          }
        }
        throw new Error("no codec found to encode value in union type " + name);
      }, codecs);
    }
  };
  var IntersectionType = function(_super) {
    __extends(IntersectionType2, _super);
    function IntersectionType2(name, is2, validate, encode, types) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.types = types;
      _this._tag = "IntersectionType";
      return _this;
    }
    return IntersectionType2;
  }(Type);
  var TupleType = function(_super) {
    __extends(TupleType2, _super);
    function TupleType2(name, is2, validate, encode, types) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.types = types;
      _this._tag = "TupleType";
      return _this;
    }
    return TupleType2;
  }(Type);
  var ReadonlyType = function(_super) {
    __extends(ReadonlyType2, _super);
    function ReadonlyType2(name, is2, validate, encode, type2) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.type = type2;
      _this._tag = "ReadonlyType";
      return _this;
    }
    return ReadonlyType2;
  }(Type);
  var ReadonlyArrayType = function(_super) {
    __extends(ReadonlyArrayType2, _super);
    function ReadonlyArrayType2(name, is2, validate, encode, type2) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.type = type2;
      _this._tag = "ReadonlyArrayType";
      return _this;
    }
    return ReadonlyArrayType2;
  }(Type);
  var TaggedUnionType = function(_super) {
    __extends(TaggedUnionType2, _super);
    function TaggedUnionType2(name, is2, validate, encode, codecs, tag) {
      var _this = _super.call(this, name, is2, validate, encode, codecs) || this;
      _this.tag = tag;
      return _this;
    }
    return TaggedUnionType2;
  }(UnionType);
  var ExactType = function(_super) {
    __extends(ExactType2, _super);
    function ExactType2(name, is2, validate, encode, type2) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.type = type2;
      _this._tag = "ExactType";
      return _this;
    }
    return ExactType2;
  }(Type);
  var NeverType = function(_super) {
    __extends(NeverType2, _super);
    function NeverType2() {
      var _this = _super.call(this, "never", function(_) {
        return false;
      }, function(u, c) {
        return failure(u, c);
      }, function() {
        throw new Error("cannot encode never");
      }) || this;
      _this._tag = "NeverType";
      return _this;
    }
    return NeverType2;
  }(Type);
  var never = new NeverType();
  var AnyType = function(_super) {
    __extends(AnyType2, _super);
    function AnyType2() {
      var _this = _super.call(this, "any", function(_) {
        return true;
      }, success, identity2) || this;
      _this._tag = "AnyType";
      return _this;
    }
    return AnyType2;
  }(Type);
  var any = new AnyType();
  var ObjectType = function(_super) {
    __extends(ObjectType2, _super);
    function ObjectType2() {
      var _this = _super.call(this, "object", function(u) {
        return u !== null && typeof u === "object";
      }, function(u, c) {
        return _this.is(u) ? success(u) : failure(u, c);
      }, identity2) || this;
      _this._tag = "ObjectType";
      return _this;
    }
    return ObjectType2;
  }(Type);
  var object2 = new ObjectType();
  function refinement(codec, predicate, name) {
    if (name === void 0) {
      name = "(" + codec.name + " | " + getFunctionName(predicate) + ")";
    }
    return new RefinementType(name, function(u) {
      return codec.is(u) && predicate(u);
    }, function(i, c) {
      var e = codec.validate(i, c);
      if (isLeft(e)) {
        return e;
      }
      var a = e.right;
      return predicate(a) ? success(a) : failure(a, c);
    }, codec.encode, codec, predicate);
  }
  var Integer = refinement(number, Number.isInteger, "Integer");
  var StrictType = function(_super) {
    __extends(StrictType2, _super);
    function StrictType2(name, is2, validate, encode, props) {
      var _this = _super.call(this, name, is2, validate, encode) || this;
      _this.props = props;
      _this._tag = "StrictType";
      return _this;
    }
    return StrictType2;
  }(Type);
  var isNonEmpty = function(as) {
    return as.length > 0;
  };
  var emptyTags = {};
  function intersect(a, b) {
    var r = [];
    for (var _i = 0, a_1 = a; _i < a_1.length; _i++) {
      var v = a_1[_i];
      if (b.indexOf(v) !== -1) {
        r.push(v);
      }
    }
    return r;
  }
  function mergeTags(a, b) {
    if (a === emptyTags) {
      return b;
    }
    if (b === emptyTags) {
      return a;
    }
    var r = Object.assign({}, a);
    for (var k in b) {
      if (a.hasOwnProperty(k)) {
        var intersection_1 = intersect(a[k], b[k]);
        if (isNonEmpty(intersection_1)) {
          r[k] = intersection_1;
        } else {
          r = emptyTags;
          break;
        }
      } else {
        r[k] = b[k];
      }
    }
    return r;
  }
  function intersectTags(a, b) {
    if (a === emptyTags || b === emptyTags) {
      return emptyTags;
    }
    var r = emptyTags;
    for (var k in a) {
      if (b.hasOwnProperty(k)) {
        var intersection_2 = intersect(a[k], b[k]);
        if (intersection_2.length === 0) {
          if (r === emptyTags) {
            r = {};
          }
          r[k] = a[k].concat(b[k]);
        }
      }
    }
    return r;
  }
  function isLiteralC(codec) {
    return codec._tag === "LiteralType";
  }
  function isTypeC(codec) {
    return codec._tag === "InterfaceType";
  }
  function isStrictC(codec) {
    return codec._tag === "StrictType";
  }
  function isExactC(codec) {
    return codec._tag === "ExactType";
  }
  function isRefinementC(codec) {
    return codec._tag === "RefinementType";
  }
  function isIntersectionC(codec) {
    return codec._tag === "IntersectionType";
  }
  function isUnionC(codec) {
    return codec._tag === "UnionType";
  }
  function isRecursiveC(codec) {
    return codec._tag === "RecursiveType";
  }
  var lazyCodecs = [];
  function getTags(codec) {
    if (lazyCodecs.indexOf(codec) !== -1) {
      return emptyTags;
    }
    if (isTypeC(codec) || isStrictC(codec)) {
      var index2 = emptyTags;
      for (var k in codec.props) {
        var prop = codec.props[k];
        if (isLiteralC(prop)) {
          if (index2 === emptyTags) {
            index2 = {};
          }
          index2[k] = [prop.value];
        }
      }
      return index2;
    } else if (isExactC(codec) || isRefinementC(codec)) {
      return getTags(codec.type);
    } else if (isIntersectionC(codec)) {
      return codec.types.reduce(function(tags3, codec2) {
        return mergeTags(tags3, getTags(codec2));
      }, emptyTags);
    } else if (isUnionC(codec)) {
      return codec.types.slice(1).reduce(function(tags3, codec2) {
        return intersectTags(tags3, getTags(codec2));
      }, getTags(codec.types[0]));
    } else if (isRecursiveC(codec)) {
      lazyCodecs.push(codec);
      var tags2 = getTags(codec.type);
      lazyCodecs.pop();
      return tags2;
    }
    return emptyTags;
  }
  function getIndex(codecs) {
    var tags2 = getTags(codecs[0]);
    var keys = Object.keys(tags2);
    var len = codecs.length;
    var _loop_1 = function(k2) {
      var all = tags2[k2].slice();
      var index2 = [tags2[k2]];
      for (var i = 1; i < len; i++) {
        var codec = codecs[i];
        var ctags = getTags(codec);
        var values = ctags[k2];
        if (values === void 0) {
          return "continue-keys";
        } else {
          if (values.some(function(v) {
            return all.indexOf(v) !== -1;
          })) {
            return "continue-keys";
          } else {
            all.push.apply(all, values);
            index2.push(values);
          }
        }
      }
      return {value: [k2, index2]};
    };
    keys:
      for (var _i = 0, keys_1 = keys; _i < keys_1.length; _i++) {
        var k = keys_1[_i];
        var state_1 = _loop_1(k);
        if (typeof state_1 === "object")
          return state_1.value;
        switch (state_1) {
          case "continue-keys":
            continue keys;
        }
      }
    return void 0;
  }

  // src/common/buildValidator.ts
  const getContextPath = (context) => context.map(({key}, index2) => {
    if (index2 === 0 && key === "") {
      return ".";
    }
    return key;
  }).join("/");
  const failure2 = (es) => es.map((e) => {
    const lastTypeName = e.context[e.context.length - 1].type.name;
    return {
      path: getContextPath(e.context),
      value: e.value,
      expected: lastTypeName
    };
  });
  const success2 = () => {
    return [];
  };
  const buildValidator = (decoder) => {
    return (dataIn) => {
      const decodeResult = decoder.decode(dataIn);
      if (isRight(decodeResult)) {
        return {
          type: "ok",
          data: decodeResult.right
        };
      }
      const errorDecodeInfo = fold(failure2, success2)(decodeResult);
      return {
        type: "error",
        message: errorDecodeInfo
      };
    };
  };
  const buildValidatorWithUnwrap = (label, decoder) => {
    const validator = buildValidator(decoder);
    return (dataIn) => {
      const result = validator(dataIn);
      if (result.type === "ok") {
        return result.data;
      }
      console.error({
        label: `Decoder '${label}'`,
        dataIn,
        errorDecodeInfo: result.message
      });
      throw new Error(JSON.stringify({
        label: `Decoder '${label}'`,
        errorDecodeInfo: result.message
      }));
    };
  };

  // src/common/fetchRequest.ts
  const axios = __toModule(require_axios2());
  const TIMEOUT = 10 * 1e3;
  const jsonParse = (body) => {
    try {
      const bodyJson = JSON.parse(body);
      return {
        type: "json",
        json: bodyJson
      };
    } catch (err) {
      return {
        type: "text",
        text: body
      };
    }
  };
  const fetchRequest = async (method, url, params, decode) => {
    const response = await axios.default.request({
      method,
      url,
      data: params === void 0 ? void 0 : JSON.stringify(params),
      transformResponse: [],
      validateStatus: () => true,
      timeout: TIMEOUT
    });
    const respData = response.data;
    if (typeof respData !== "string") {
      console.error(respData);
      throw Error("String expected");
    }
    const parsedResponse = jsonParse(respData);
    return decode(response.status, parsedResponse);
  };

  // src/client/api/apiGetPath.ts
  const ApiGetPathResponseIO = union([
    type({
      type: literal("file"),
      lastWrite: number
    }),
    type({
      type: literal("dir"),
      list: array2(string)
    })
  ]);
  const decodeApiGetPathResponse = buildValidatorWithUnwrap("ApiGetPathResponseIO", ApiGetPathResponseIO);
  const apiGetPath = async (path) => {
    const params = {
      mathod: "getPath",
      path
    };
    return await fetchRequest("POST", "/api", params, (status, data) => {
      if (status === 200 && data.type === "json") {
        return decodeApiGetPathResponse(data.json);
      }
      throw Error(`Niespodziewana zwrotka z api ${status} ${data.type}`);
    });
  };

  // src/client/AppState/AppState.ts
  class AppState {
    constructor() {
      makeAutoObservable(this);
      this.counter = new CounterState();
      this.currentPath = new CurrentPath("", new ContentState());
      (async () => {
        const resp = await apiGetPath("aaa/ffff");
        console.info("AAAPI", resp);
      })().catch((error) => {
        console.error(error);
      });
    }
    get counterValue() {
      return this.counter.counter.value;
    }
  }

  // src/client.tsx
  console.info("client init");
  const root = document.getElementById("root");
  if (root === null) {
    console.error("Brakuje root-a");
  } else {
    const appState = new AppState();
    const domJsx = /* @__PURE__ */ React4.createElement(Provider, {
      value: appState
    }, /* @__PURE__ */ React4.createElement(App, null));
    ReactDOM.render(domJsx, root);
  }
})();
