export class BugField {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BugFieldFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bugfield_free(ptr, 0);
    }
    /**
     * @param {number} timestamp
     */
    frame(timestamp) {
        const ret = wasm.bugfield_frame(this.__wbg_ptr, timestamp);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {HTMLCanvasElement} canvas
     * @param {number} seed
     */
    constructor(canvas, seed) {
        const ret = wasm.bugfield_new(canvas, seed);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        BugFieldFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} dpr
     */
    resize(width, height, dpr) {
        const ret = wasm.bugfield_resize(this.__wbg_ptr, width, height, dpr);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {boolean} dark_mode
     */
    set_dark_mode(dark_mode) {
        wasm.bugfield_set_dark_mode(this.__wbg_ptr, dark_mode);
    }
    /**
     * @param {Float32Array} rects
     */
    set_obstacles(rects) {
        const ptr0 = passArrayF32ToWasm0(rects, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.bugfield_set_obstacles(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) BugField.prototype[Symbol.dispose] = BugField.prototype.free;
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_6b64449b9b9ed33c: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_beginPath_6d95cc267dd3e88f: function(arg0) {
            arg0.beginPath();
        },
        __wbg_clearRect_5fb1d6b44e6b6738: function(arg0, arg1, arg2, arg3, arg4) {
            arg0.clearRect(arg1, arg2, arg3, arg4);
        },
        __wbg_ellipse_0303410b3d05bb97: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
            arg0.ellipse(arg1, arg2, arg3, arg4, arg5, arg6, arg7);
        }, arguments); },
        __wbg_fill_ec5da5f3916cf924: function(arg0) {
            arg0.fill();
        },
        __wbg_getContext_fc146f8ec021d074: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_instanceof_CanvasRenderingContext2d_24a3fe06e62b98d7: function(arg0) {
            let result;
            try {
                result = arg0 instanceof CanvasRenderingContext2D;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_lineTo_c9f1e0dd4824ae31: function(arg0, arg1, arg2) {
            arg0.lineTo(arg1, arg2);
        },
        __wbg_moveTo_d3deaceb55dc2d80: function(arg0, arg1, arg2) {
            arg0.moveTo(arg1, arg2);
        },
        __wbg_restore_f103803ad0dc390b: function(arg0) {
            arg0.restore();
        },
        __wbg_rotate_fb8a7a0e39ad85a6: function() { return handleError(function (arg0, arg1) {
            arg0.rotate(arg1);
        }, arguments); },
        __wbg_save_5b07d6d1028c3e4d: function(arg0) {
            arg0.save();
        },
        __wbg_setTransform_e43c6ac3207fe112: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            arg0.setTransform(arg1, arg2, arg3, arg4, arg5, arg6);
        }, arguments); },
        __wbg_set_fillStyle_6816843dfcfe7a30: function(arg0, arg1) {
            arg0.fillStyle = arg1;
        },
        __wbg_set_height_be9b2b920bd68401: function(arg0, arg1) {
            arg0.height = arg1 >>> 0;
        },
        __wbg_set_lineCap_63624e83cb57fb8b: function(arg0, arg1, arg2) {
            arg0.lineCap = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_lineJoin_e6a1ec1a93ea92cb: function(arg0, arg1, arg2) {
            arg0.lineJoin = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_lineWidth_2fae105117e1a89f: function(arg0, arg1) {
            arg0.lineWidth = arg1;
        },
        __wbg_set_shadowBlur_f9653d1a9e1573ef: function(arg0, arg1) {
            arg0.shadowBlur = arg1;
        },
        __wbg_set_shadowColor_5882e8ce2ded629b: function(arg0, arg1, arg2) {
            arg0.shadowColor = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_strokeStyle_1eb6e7e03b340920: function(arg0, arg1) {
            arg0.strokeStyle = arg1;
        },
        __wbg_set_width_5cda41d4d06a14dd: function(arg0, arg1) {
            arg0.width = arg1 >>> 0;
        },
        __wbg_stroke_42f07013960cf81b: function(arg0) {
            arg0.stroke();
        },
        __wbg_translate_34989493d69eaecd: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.translate(arg1, arg2);
        }, arguments); },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./ambient_bugs_bg.js": import0,
    };
}

const BugFieldFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bugfield_free(ptr >>> 0, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedFloat32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('ambient_bugs_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
