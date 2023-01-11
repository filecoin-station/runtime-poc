// @generated file from wasmbuild -- do not edit
// deno-lint-ignore-file
// deno-fmt-ignore-file
// source-hash: 3376cf624c49eda3d2aea3a00216910f7518b151
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) {
  return heap[idx];
}

let heap_next = heap.length;

function dropObject(idx) {
  if (idx < 36) return;
  heap[idx] = heap_next;
  heap_next = idx;
}

function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}

const cachedTextDecoder = new TextDecoder("utf-8", {
  ignoreBOM: true,
  fatal: true,
});

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
  if (cachedUint8Memory0.byteLength === 0) {
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
  if (heap_next === heap.length) heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];

  heap[idx] = obj;
  return idx;
}

function debugString(val) {
  // primitive types
  const type = typeof val;
  if (type == "number" || type == "boolean" || val == null) {
    return `${val}`;
  }
  if (type == "string") {
    return `"${val}"`;
  }
  if (type == "symbol") {
    const description = val.description;
    if (description == null) {
      return "Symbol";
    } else {
      return `Symbol(${description})`;
    }
  }
  if (type == "function") {
    const name = val.name;
    if (typeof name == "string" && name.length > 0) {
      return `Function(${name})`;
    } else {
      return "Function";
    }
  }
  // objects
  if (Array.isArray(val)) {
    const length = val.length;
    let debug = "[";
    if (length > 0) {
      debug += debugString(val[0]);
    }
    for (let i = 1; i < length; i++) {
      debug += ", " + debugString(val[i]);
    }
    debug += "]";
    return debug;
  }
  // Test for built-in
  const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
  let className;
  if (builtInMatches.length > 1) {
    className = builtInMatches[1];
  } else {
    // Failed to match the standard '[object ClassName]'
    return toString.call(val);
  }
  if (className == "Object") {
    // we're a user defined class or Object
    // JSON.stringify avoids problems with cycles, and is generally much
    // easier than looping through ownProperties of `val`.
    try {
      return "Object(" + JSON.stringify(val) + ")";
    } catch (_) {
      return "Object";
    }
  }
  // errors
  if (val instanceof Error) {
    return `${val.name}: ${val.message}\n${val.stack}`;
  }
  // TODO we could test for more things here, like `Set`s and `Map`s.
  return className;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder("utf-8");

const encodeString = function (arg, view) {
  return cachedTextEncoder.encodeInto(arg, view);
};

function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr = malloc(buf.length);
    getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
  }

  let len = arg.length;
  let ptr = malloc(len);

  const mem = getUint8Memory0();

  let offset = 0;

  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7F) break;
    mem[ptr + offset] = code;
  }

  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, len = offset + arg.length * 3);
    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);

    offset += ret.written;
  }

  WASM_VECTOR_LEN = offset;
  return ptr;
}

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
  if (cachedInt32Memory0.byteLength === 0) {
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  }
  return cachedInt32Memory0;
}

const CLOSURE_DTORS = new FinalizationRegistry((state) => {
  wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
});

function makeMutClosure(arg0, arg1, dtor, f) {
  const state = { a: arg0, b: arg1, cnt: 1, dtor };
  const real = (...args) => {
    // First up with a closure we increment the internal reference
    // count. This ensures that the Rust closure environment won't
    // be deallocated while we're invoking it.
    state.cnt++;
    const a = state.a;
    state.a = 0;
    try {
      return f(a, state.b, ...args);
    } finally {
      if (--state.cnt === 0) {
        wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);
        CLOSURE_DTORS.unregister(state);
      } else {
        state.a = a;
      }
    }
  };
  real.original = state;
  CLOSURE_DTORS.register(real, state, state);
  return real;
}
function __wbg_adapter_26(arg0, arg1, arg2) {
  wasm
    ._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hae3e076dfdadd60f(
      arg0,
      arg1,
      addHeapObject(arg2),
    );
}

function __wbg_adapter_29(arg0, arg1, arg2) {
  wasm
    ._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hfec852ae0e02d3c3(
      arg0,
      arg1,
      addHeapObject(arg2),
    );
}

/**
 * @returns {Promise<any>}
 */
export function wget() {
  const ret = wasm.wget();
  return takeObject(ret);
}

function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
  }
}

function isLikeNone(x) {
  return x === undefined || x === null;
}
function __wbg_adapter_111(arg0, arg1, arg2, arg3) {
  wasm.wasm_bindgen__convert__closures__invoke2_mut__h55e78ee8650a4c51(
    arg0,
    arg1,
    addHeapObject(arg2),
    addHeapObject(arg3),
  );
}

const IntoUnderlyingByteSourceFinalization = new FinalizationRegistry((ptr) =>
  wasm.__wbg_intounderlyingbytesource_free(ptr)
);
/** */
export class IntoUnderlyingByteSource {
  __destroy_into_raw() {
    const ptr = this.ptr;
    this.ptr = 0;
    IntoUnderlyingByteSourceFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_intounderlyingbytesource_free(ptr);
  }
  /**
   * @returns {string}
   */
  get type() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.intounderlyingbytesource_type(retptr, this.ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(r0, r1);
    }
  }
  /**
   * @returns {number}
   */
  get autoAllocateChunkSize() {
    const ret = wasm.intounderlyingbytesource_autoAllocateChunkSize(this.ptr);
    return ret >>> 0;
  }
  /**
   * @param {any} controller
   */
  start(controller) {
    wasm.intounderlyingbytesource_start(this.ptr, addHeapObject(controller));
  }
  /**
   * @param {any} controller
   * @returns {Promise<any>}
   */
  pull(controller) {
    const ret = wasm.intounderlyingbytesource_pull(
      this.ptr,
      addHeapObject(controller),
    );
    return takeObject(ret);
  }
  /** */
  cancel() {
    const ptr = this.__destroy_into_raw();
    wasm.intounderlyingbytesource_cancel(ptr);
  }
}

const IntoUnderlyingSinkFinalization = new FinalizationRegistry((ptr) =>
  wasm.__wbg_intounderlyingsink_free(ptr)
);
/** */
export class IntoUnderlyingSink {
  __destroy_into_raw() {
    const ptr = this.ptr;
    this.ptr = 0;
    IntoUnderlyingSinkFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_intounderlyingsink_free(ptr);
  }
  /**
   * @param {any} chunk
   * @returns {Promise<any>}
   */
  write(chunk) {
    const ret = wasm.intounderlyingsink_write(this.ptr, addHeapObject(chunk));
    return takeObject(ret);
  }
  /**
   * @returns {Promise<any>}
   */
  close() {
    const ptr = this.__destroy_into_raw();
    const ret = wasm.intounderlyingsink_close(ptr);
    return takeObject(ret);
  }
  /**
   * @param {any} reason
   * @returns {Promise<any>}
   */
  abort(reason) {
    const ptr = this.__destroy_into_raw();
    const ret = wasm.intounderlyingsink_abort(ptr, addHeapObject(reason));
    return takeObject(ret);
  }
}

const IntoUnderlyingSourceFinalization = new FinalizationRegistry((ptr) =>
  wasm.__wbg_intounderlyingsource_free(ptr)
);
/** */
export class IntoUnderlyingSource {
  __destroy_into_raw() {
    const ptr = this.ptr;
    this.ptr = 0;
    IntoUnderlyingSourceFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_intounderlyingsource_free(ptr);
  }
  /**
   * @param {any} controller
   * @returns {Promise<any>}
   */
  pull(controller) {
    const ret = wasm.intounderlyingsource_pull(
      this.ptr,
      addHeapObject(controller),
    );
    return takeObject(ret);
  }
  /** */
  cancel() {
    const ptr = this.__destroy_into_raw();
    wasm.intounderlyingsource_cancel(ptr);
  }
}

const PipeOptionsFinalization = new FinalizationRegistry((ptr) =>
  wasm.__wbg_pipeoptions_free(ptr)
);
/**
 * Raw options for [`pipeTo()`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeTo).
 */
export class PipeOptions {
  __destroy_into_raw() {
    const ptr = this.ptr;
    this.ptr = 0;
    PipeOptionsFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_pipeoptions_free(ptr);
  }
  /**
   * @returns {boolean}
   */
  get preventClose() {
    const ret = wasm.pipeoptions_preventClose(this.ptr);
    return ret !== 0;
  }
  /**
   * @returns {boolean}
   */
  get preventCancel() {
    const ret = wasm.pipeoptions_preventCancel(this.ptr);
    return ret !== 0;
  }
  /**
   * @returns {boolean}
   */
  get preventAbort() {
    const ret = wasm.pipeoptions_preventAbort(this.ptr);
    return ret !== 0;
  }
  /**
   * @returns {AbortSignal | undefined}
   */
  get signal() {
    const ret = wasm.pipeoptions_signal(this.ptr);
    return takeObject(ret);
  }
}

const QueuingStrategyFinalization = new FinalizationRegistry((ptr) =>
  wasm.__wbg_queuingstrategy_free(ptr)
);
/** */
export class QueuingStrategy {
  __destroy_into_raw() {
    const ptr = this.ptr;
    this.ptr = 0;
    QueuingStrategyFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_queuingstrategy_free(ptr);
  }
  /**
   * @returns {number}
   */
  get highWaterMark() {
    const ret = wasm.queuingstrategy_highWaterMark(this.ptr);
    return ret;
  }
}

const ReadableStreamGetReaderOptionsFinalization = new FinalizationRegistry(
  (ptr) => wasm.__wbg_readablestreamgetreaderoptions_free(ptr)
);
/**
 * Raw options for [`getReader()`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader).
 */
export class ReadableStreamGetReaderOptions {
  __destroy_into_raw() {
    const ptr = this.ptr;
    this.ptr = 0;
    ReadableStreamGetReaderOptionsFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_readablestreamgetreaderoptions_free(ptr);
  }
  /**
   * @returns {any}
   */
  get mode() {
    const ret = wasm.readablestreamgetreaderoptions_mode(this.ptr);
    return takeObject(ret);
  }
}

const imports = {
  __wbindgen_placeholder__: {
    __wbindgen_object_drop_ref: function (arg0) {
      takeObject(arg0);
    },
    __wbindgen_is_object: function (arg0) {
      const val = getObject(arg0);
      const ret = typeof (val) === "object" && val !== null;
      return ret;
    },
    __wbindgen_string_new: function (arg0, arg1) {
      const ret = getStringFromWasm0(arg0, arg1);
      return addHeapObject(ret);
    },
    __wbindgen_number_new: function (arg0) {
      const ret = arg0;
      return addHeapObject(ret);
    },
    __wbg_create_67d5d07e69d65530: function (arg0, arg1) {
      const ret = Deno.create(getStringFromWasm0(arg0, arg1));
      return addHeapObject(ret);
    },
    __wbindgen_is_function: function (arg0) {
      const ret = typeof (getObject(arg0)) === "function";
      return ret;
    },
    __wbindgen_cb_drop: function (arg0) {
      const obj = takeObject(arg0).original;
      if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
      }
      const ret = false;
      return ret;
    },
    __wbg_instanceof_ReadableStream_723f1212419028dc: function (arg0) {
      let result;
      try {
        result = getObject(arg0) instanceof ReadableStream;
      } catch {
        result = false;
      }
      const ret = result;
      return ret;
    },
    __wbg_getReader_8ecba87d8003e950: function () {
      return handleError(function (arg0) {
        const ret = getObject(arg0).getReader();
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_close_e9110ca16e2567db: function (arg0) {
      getObject(arg0).close();
    },
    __wbg_enqueue_d71a1a518e21f5c3: function (arg0, arg1) {
      getObject(arg0).enqueue(getObject(arg1));
    },
    __wbg_byobRequest_08c18cee35def1f4: function (arg0) {
      const ret = getObject(arg0).byobRequest;
      return isLikeNone(ret) ? 0 : addHeapObject(ret);
    },
    __wbg_close_da7e6fb9d9851e5a: function (arg0) {
      getObject(arg0).close();
    },
    __wbg_view_231340b0dd8a2484: function (arg0) {
      const ret = getObject(arg0).view;
      return isLikeNone(ret) ? 0 : addHeapObject(ret);
    },
    __wbg_respond_8fadc5f5c9d95422: function (arg0, arg1) {
      getObject(arg0).respond(arg1 >>> 0);
    },
    __wbg_buffer_4e79326814bdd393: function (arg0) {
      const ret = getObject(arg0).buffer;
      return addHeapObject(ret);
    },
    __wbg_byteOffset_b69b0a07afccce19: function (arg0) {
      const ret = getObject(arg0).byteOffset;
      return ret;
    },
    __wbg_byteLength_5299848ed3264181: function (arg0) {
      const ret = getObject(arg0).byteLength;
      return ret;
    },
    __wbg_cancel_7f202496da02cd45: function (arg0) {
      const ret = getObject(arg0).cancel();
      return addHeapObject(ret);
    },
    __wbg_releaseLock_9ae075576f54bf0b: function () {
      return handleError(function (arg0) {
        getObject(arg0).releaseLock();
      }, arguments);
    },
    __wbg_read_88c96573fc8b3b01: function (arg0) {
      const ret = getObject(arg0).read();
      return addHeapObject(ret);
    },
    __wbg_done_76252d32deca186b: function (arg0) {
      const ret = getObject(arg0).done;
      return ret;
    },
    __wbg_value_ff3741eb46856618: function (arg0) {
      const ret = getObject(arg0).value;
      return addHeapObject(ret);
    },
    __wbindgen_object_clone_ref: function (arg0) {
      const ret = getObject(arg0);
      return addHeapObject(ret);
    },
    __wbg_getWriter_8bddd57284927f28: function () {
      return handleError(function (arg0) {
        const ret = getObject(arg0).getWriter();
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_releaseLock_e5b6c3837ff62f0b: function (arg0) {
      getObject(arg0).releaseLock();
    },
    __wbg_write_d258588d7e849d84: function (arg0, arg1) {
      const ret = getObject(arg0).write(takeObject(arg1));
      return addHeapObject(ret);
    },
    __wbg_instanceof_WritableStream_5e42a26dfe7f5406: function (arg0) {
      let result;
      try {
        result = getObject(arg0) instanceof WritableStream;
      } catch {
        result = false;
      }
      const ret = result;
      return ret;
    },
    __wbg_instanceof_Window_acc97ff9f5d2c7b4: function (arg0) {
      let result;
      try {
        result = getObject(arg0) instanceof Window;
      } catch {
        result = false;
      }
      const ret = result;
      return ret;
    },
    __wbg_fetch_c5d08af59be0ee7d: function (arg0, arg1, arg2) {
      const ret = getObject(arg0).fetch(getStringFromWasm0(arg1, arg2));
      return addHeapObject(ret);
    },
    __wbg_log_4b5638ad60bdc54a: function (arg0) {
      console.log(getObject(arg0));
    },
    __wbg_log_89ca282a8a49b121: function (arg0, arg1) {
      console.log(getObject(arg0), getObject(arg1));
    },
    __wbg_instanceof_Response_eaa426220848a39e: function (arg0) {
      let result;
      try {
        result = getObject(arg0) instanceof Response;
      } catch {
        result = false;
      }
      const ret = result;
      return ret;
    },
    __wbg_status_c4ef3dd591e63435: function (arg0) {
      const ret = getObject(arg0).status;
      return ret;
    },
    __wbg_body_7bf1a45a7ee13f62: function (arg0) {
      const ret = getObject(arg0).body;
      return isLikeNone(ret) ? 0 : addHeapObject(ret);
    },
    __wbg_newnoargs_b5b063fc6c2f0376: function (arg0, arg1) {
      const ret = new Function(getStringFromWasm0(arg0, arg1));
      return addHeapObject(ret);
    },
    __wbg_get_765201544a2b6869: function () {
      return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_call_97ae9d8645dc388b: function () {
      return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_self_6d479506f72c6a71: function () {
      return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_window_f2557cc78490aceb: function () {
      return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_globalThis_7f206bda628d5286: function () {
      return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_global_ba75c50d1cf384f4: function () {
      return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
      }, arguments);
    },
    __wbindgen_is_undefined: function (arg0) {
      const ret = getObject(arg0) === undefined;
      return ret;
    },
    __wbg_new_8d2af00bc1e329ee: function (arg0, arg1) {
      const ret = new Error(getStringFromWasm0(arg0, arg1));
      return addHeapObject(ret);
    },
    __wbg_call_168da88779e35f61: function () {
      return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_new_9962f939219f1820: function (arg0, arg1) {
      try {
        var state0 = { a: arg0, b: arg1 };
        var cb0 = (arg0, arg1) => {
          const a = state0.a;
          state0.a = 0;
          try {
            return __wbg_adapter_111(a, state0.b, arg0, arg1);
          } finally {
            state0.a = a;
          }
        };
        const ret = new Promise(cb0);
        return addHeapObject(ret);
      } finally {
        state0.a = state0.b = 0;
      }
    },
    __wbg_resolve_99fe17964f31ffc0: function (arg0) {
      const ret = Promise.resolve(getObject(arg0));
      return addHeapObject(ret);
    },
    __wbg_catch_4eaf75e3e2d27d00: function (arg0, arg1) {
      const ret = getObject(arg0).catch(getObject(arg1));
      return addHeapObject(ret);
    },
    __wbg_then_11f7a54d67b4bfad: function (arg0, arg1) {
      const ret = getObject(arg0).then(getObject(arg1));
      return addHeapObject(ret);
    },
    __wbg_then_cedad20fbbd9418a: function (arg0, arg1, arg2) {
      const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
      return addHeapObject(ret);
    },
    __wbg_buffer_3f3d764d4747d564: function (arg0) {
      const ret = getObject(arg0).buffer;
      return addHeapObject(ret);
    },
    __wbg_newwithbyteoffsetandlength_d9aa266703cb98be: function (
      arg0,
      arg1,
      arg2,
    ) {
      const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
      return addHeapObject(ret);
    },
    __wbg_set_83db9690f9353e79: function (arg0, arg1, arg2) {
      getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    },
    __wbg_length_9e1ae1900cb0fbd5: function (arg0) {
      const ret = getObject(arg0).length;
      return ret;
    },
    __wbindgen_debug_string: function (arg0, arg1) {
      const ret = debugString(getObject(arg1));
      const ptr0 = passStringToWasm0(
        ret,
        wasm.__wbindgen_malloc,
        wasm.__wbindgen_realloc,
      );
      const len0 = WASM_VECTOR_LEN;
      getInt32Memory0()[arg0 / 4 + 1] = len0;
      getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    },
    __wbindgen_throw: function (arg0, arg1) {
      throw new Error(getStringFromWasm0(arg0, arg1));
    },
    __wbindgen_rethrow: function (arg0) {
      throw takeObject(arg0);
    },
    __wbindgen_memory: function () {
      const ret = wasm.memory;
      return addHeapObject(ret);
    },
    __wbindgen_closure_wrapper136: function (arg0, arg1, arg2) {
      const ret = makeMutClosure(arg0, arg1, 31, __wbg_adapter_26);
      return addHeapObject(ret);
    },
    __wbindgen_closure_wrapper380: function (arg0, arg1, arg2) {
      const ret = makeMutClosure(arg0, arg1, 68, __wbg_adapter_29);
      return addHeapObject(ret);
    },
  },
};

/**
 * Decompression callback
 *
 * @callback DecompressCallback
 * @param {Uint8Array} compressed
 * @return {Uint8Array} decompressed
 */

/**
 * Options for instantiating a Wasm instance.
 * @typedef {Object} InstantiateOptions
 * @property {URL=} url - Optional url to the Wasm file to instantiate.
 * @property {DecompressCallback=} decompress - Callback to decompress the
 * raw Wasm file bytes before instantiating.
 */

/** Instantiates an instance of the Wasm module returning its functions.
 * @remarks It is safe to call this multiple times and once successfully
 * loaded it will always return a reference to the same object.
 * @param {InstantiateOptions=} opts
 */
export async function instantiate(opts) {
  return (await instantiateWithInstance(opts)).exports;
}

let instanceWithExports;
let lastLoadPromise;

/** Instantiates an instance of the Wasm module along with its exports.
 * @remarks It is safe to call this multiple times and once successfully
 * loaded it will always return a reference to the same object.
 * @param {InstantiateOptions=} opts
 * @returns {Promise<{
 *   instance: WebAssembly.Instance;
 *   exports: { wget: typeof wget; IntoUnderlyingByteSource : typeof IntoUnderlyingByteSource ; IntoUnderlyingSink : typeof IntoUnderlyingSink ; IntoUnderlyingSource : typeof IntoUnderlyingSource ; PipeOptions : typeof PipeOptions ; QueuingStrategy : typeof QueuingStrategy ; ReadableStreamGetReaderOptions : typeof ReadableStreamGetReaderOptions  }
 * }>}
 */
export function instantiateWithInstance(opts) {
  if (instanceWithExports != null) {
    return Promise.resolve(instanceWithExports);
  }
  if (lastLoadPromise == null) {
    lastLoadPromise = (async () => {
      try {
        const instance = (await instantiateModule(opts ?? {})).instance;
        wasm = instance.exports;
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        instanceWithExports = {
          instance,
          exports: getWasmInstanceExports(),
        };
        return instanceWithExports;
      } finally {
        lastLoadPromise = null;
      }
    })();
  }
  return lastLoadPromise;
}

function getWasmInstanceExports() {
  return {
    wget,
    IntoUnderlyingByteSource,
    IntoUnderlyingSink,
    IntoUnderlyingSource,
    PipeOptions,
    QueuingStrategy,
    ReadableStreamGetReaderOptions,
  };
}

/** Gets if the Wasm module has been instantiated. */
export function isInstantiated() {
  return instanceWithExports != null;
}

/**
 * @param {InstantiateOptions} opts
 */
async function instantiateModule(opts) {
  const wasmUrl = opts.url ?? new URL("rs_lib_bg.wasm", import.meta.url);
  const decompress = opts.decompress;
  const isFile = wasmUrl.protocol === "file:";

  // make file urls work in Node via dnt
  const isNode = globalThis.process?.versions?.node != null;
  if (isNode && isFile) {
    // the deno global will be shimmed by dnt
    const wasmCode = await Deno.readFile(wasmUrl);
    return WebAssembly.instantiate(
      decompress ? decompress(wasmCode) : wasmCode,
      imports,
    );
  }

  switch (wasmUrl.protocol) {
    case "file:":
    case "https:":
    case "http:": {
      if (isFile) {
        if (typeof Deno !== "object") {
          throw new Error("file urls are not supported in this environment");
        }
        if ("permissions" in Deno) {
          await Deno.permissions.request({ name: "read", path: wasmUrl });
        }
      } else if (typeof Deno === "object" && "permissions" in Deno) {
        await Deno.permissions.request({ name: "net", host: wasmUrl.host });
      }
      const wasmResponse = await fetch(wasmUrl);
      if (decompress) {
        const wasmCode = new Uint8Array(await wasmResponse.arrayBuffer());
        return WebAssembly.instantiate(decompress(wasmCode), imports);
      }
      if (
        isFile ||
        wasmResponse.headers.get("content-type")?.toLowerCase()
          .startsWith("application/wasm")
      ) {
        return WebAssembly.instantiateStreaming(wasmResponse, imports);
      } else {
        return WebAssembly.instantiate(
          await wasmResponse.arrayBuffer(),
          imports,
        );
      }
    }
    default:
      throw new Error(`Unsupported protocol: ${wasmUrl.protocol}`);
  }
}
