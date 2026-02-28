let wasm_bindgen = (function(exports) {
    let script_src;
    if (typeof document !== 'undefined' && document.currentScript !== null) {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }

    /**
     * Our handle to the web app from JavaScript.
     */
    class WebHandle {
        __destroy_into_raw() {
            const ptr = this.__wbg_ptr;
            this.__wbg_ptr = 0;
            WebHandleFinalization.unregister(this);
            return ptr;
        }
        free() {
            const ptr = this.__destroy_into_raw();
            wasm.__wbg_webhandle_free(ptr, 0);
        }
        destroy() {
            wasm.webhandle_destroy(this.__wbg_ptr);
        }
        /**
         * The JavaScript can check whether or not your app has crashed:
         * @returns {boolean}
         */
        has_panicked() {
            const ret = wasm.webhandle_has_panicked(this.__wbg_ptr);
            return ret !== 0;
        }
        /**
         * Installs a panic hook, then returns.
         */
        constructor() {
            const ret = wasm.webhandle_new();
            this.__wbg_ptr = ret >>> 0;
            WebHandleFinalization.register(this, this.__wbg_ptr, this);
            return this;
        }
        /**
         * @returns {string | undefined}
         */
        panic_callstack() {
            const ret = wasm.webhandle_panic_callstack(this.__wbg_ptr);
            let v1;
            if (ret[0] !== 0) {
                v1 = getStringFromWasm0(ret[0], ret[1]).slice();
                wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
            }
            return v1;
        }
        /**
         * @returns {string | undefined}
         */
        panic_message() {
            const ret = wasm.webhandle_panic_message(this.__wbg_ptr);
            let v1;
            if (ret[0] !== 0) {
                v1 = getStringFromWasm0(ret[0], ret[1]).slice();
                wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
            }
            return v1;
        }
        /**
         * Call this once from JavaScript to start your app.
         *
         * # Errors
         * Returns an error if the app could not start.
         * @param {HTMLCanvasElement} canvas
         * @returns {Promise<void>}
         */
        start(canvas) {
            const ret = wasm.webhandle_start(this.__wbg_ptr, canvas);
            return ret;
        }
    }
    if (Symbol.dispose) WebHandle.prototype[Symbol.dispose] = WebHandle.prototype.free;
    exports.WebHandle = WebHandle;

    function __wbg_get_imports() {
        const import0 = {
            __proto__: null,
            __wbg_Window_e0df001eddf1d3fa: function(arg0) {
                const ret = arg0.Window;
                return ret;
            },
            __wbg_WorkerGlobalScope_d731e9136c6c49a0: function(arg0) {
                const ret = arg0.WorkerGlobalScope;
                return ret;
            },
            __wbg___wbindgen_boolean_get_18c4ed9422296fff: function(arg0) {
                const v = arg0;
                const ret = typeof(v) === 'boolean' ? v : undefined;
                return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
            },
            __wbg___wbindgen_debug_string_ddde1867f49c2442: function(arg0, arg1) {
                const ret = debugString(arg1);
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg___wbindgen_in_1064a108f4d18b9e: function(arg0, arg1) {
                const ret = arg0 in arg1;
                return ret;
            },
            __wbg___wbindgen_is_function_d633e708baf0d146: function(arg0) {
                const ret = typeof(arg0) === 'function';
                return ret;
            },
            __wbg___wbindgen_is_null_a2a19127c13e7126: function(arg0) {
                const ret = arg0 === null;
                return ret;
            },
            __wbg___wbindgen_is_undefined_c18285b9fc34cb7d: function(arg0) {
                const ret = arg0 === undefined;
                return ret;
            },
            __wbg___wbindgen_number_get_5854912275df1894: function(arg0, arg1) {
                const obj = arg1;
                const ret = typeof(obj) === 'number' ? obj : undefined;
                getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
            },
            __wbg___wbindgen_string_get_3e5751597f39a112: function(arg0, arg1) {
                const obj = arg1;
                const ret = typeof(obj) === 'string' ? obj : undefined;
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg___wbindgen_throw_39bc967c0e5a9b58: function(arg0, arg1) {
                throw new Error(getStringFromWasm0(arg0, arg1));
            },
            __wbg__wbg_cb_unref_b6d832240a919168: function(arg0) {
                arg0._wbg_cb_unref();
            },
            __wbg_activeElement_00429e22d5138dde: function(arg0) {
                const ret = arg0.activeElement;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_activeElement_31e766ce04adbe5e: function(arg0) {
                const ret = arg0.activeElement;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_activeTexture_64564eacfd432771: function(arg0, arg1) {
                arg0.activeTexture(arg1 >>> 0);
            },
            __wbg_activeTexture_a687cd190b65ed8b: function(arg0, arg1) {
                arg0.activeTexture(arg1 >>> 0);
            },
            __wbg_addEventListener_ba87252e1a7558be: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
                arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);
            }, arguments); },
            __wbg_altKey_6f89a54e91c24976: function(arg0) {
                const ret = arg0.altKey;
                return ret;
            },
            __wbg_altKey_dd23e9838cbfcfc8: function(arg0) {
                const ret = arg0.altKey;
                return ret;
            },
            __wbg_appendChild_f8784f6270d097cd: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.appendChild(arg1);
                return ret;
            }, arguments); },
            __wbg_arrayBuffer_31c5b2ee4b318cbb: function(arg0) {
                const ret = arg0.arrayBuffer();
                return ret;
            },
            __wbg_at_7e9de736cf76d6a9: function(arg0, arg1) {
                const ret = arg0.at(arg1);
                return ret;
            },
            __wbg_attachShader_7f668aa1a21c15af: function(arg0, arg1, arg2) {
                arg0.attachShader(arg1, arg2);
            },
            __wbg_attachShader_a565d8643aae908a: function(arg0, arg1, arg2) {
                arg0.attachShader(arg1, arg2);
            },
            __wbg_beginQuery_74dabe32811c05c4: function(arg0, arg1, arg2) {
                arg0.beginQuery(arg1 >>> 0, arg2);
            },
            __wbg_beginRenderPass_373f34636d157c43: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.beginRenderPass(arg1);
                return ret;
            }, arguments); },
            __wbg_bindAttribLocation_76ae089d6b166433: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.bindAttribLocation(arg1, arg2 >>> 0, getStringFromWasm0(arg3, arg4));
            },
            __wbg_bindAttribLocation_bd33c70e34f80567: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.bindAttribLocation(arg1, arg2 >>> 0, getStringFromWasm0(arg3, arg4));
            },
            __wbg_bindBufferRange_b040289410dbef36: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.bindBufferRange(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5);
            },
            __wbg_bindBuffer_27ccb09ea9f2e98e: function(arg0, arg1, arg2) {
                arg0.bindBuffer(arg1 >>> 0, arg2);
            },
            __wbg_bindBuffer_b2db7cf886fca9a3: function(arg0, arg1, arg2) {
                arg0.bindBuffer(arg1 >>> 0, arg2);
            },
            __wbg_bindFramebuffer_0bdaf595cb433987: function(arg0, arg1, arg2) {
                arg0.bindFramebuffer(arg1 >>> 0, arg2);
            },
            __wbg_bindFramebuffer_8ccbd2e95d6ce81a: function(arg0, arg1, arg2) {
                arg0.bindFramebuffer(arg1 >>> 0, arg2);
            },
            __wbg_bindRenderbuffer_68ece4fd48a0d656: function(arg0, arg1, arg2) {
                arg0.bindRenderbuffer(arg1 >>> 0, arg2);
            },
            __wbg_bindRenderbuffer_94d3d58eac9f9f42: function(arg0, arg1, arg2) {
                arg0.bindRenderbuffer(arg1 >>> 0, arg2);
            },
            __wbg_bindSampler_097bac338125a419: function(arg0, arg1, arg2) {
                arg0.bindSampler(arg1 >>> 0, arg2);
            },
            __wbg_bindTexture_dddcc41c895ae918: function(arg0, arg1, arg2) {
                arg0.bindTexture(arg1 >>> 0, arg2);
            },
            __wbg_bindTexture_e67d6c16a2fe173f: function(arg0, arg1, arg2) {
                arg0.bindTexture(arg1 >>> 0, arg2);
            },
            __wbg_bindVertexArrayOES_44978e7c3a0f85a8: function(arg0, arg1) {
                arg0.bindVertexArrayOES(arg1);
            },
            __wbg_bindVertexArray_2d06af8aef338098: function(arg0, arg1) {
                arg0.bindVertexArray(arg1);
            },
            __wbg_blendColor_0f4cf6fee9ff50b0: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.blendColor(arg1, arg2, arg3, arg4);
            },
            __wbg_blendColor_ae31bdfd2de5f2f8: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.blendColor(arg1, arg2, arg3, arg4);
            },
            __wbg_blendEquationSeparate_1a52ede955d119ed: function(arg0, arg1, arg2) {
                arg0.blendEquationSeparate(arg1 >>> 0, arg2 >>> 0);
            },
            __wbg_blendEquationSeparate_3a3ac73f0ab4801a: function(arg0, arg1, arg2) {
                arg0.blendEquationSeparate(arg1 >>> 0, arg2 >>> 0);
            },
            __wbg_blendEquation_49cfef9544d6bf88: function(arg0, arg1) {
                arg0.blendEquation(arg1 >>> 0);
            },
            __wbg_blendEquation_b1d67585f9808cb0: function(arg0, arg1) {
                arg0.blendEquation(arg1 >>> 0);
            },
            __wbg_blendFuncSeparate_233d42517c0ae9a0: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.blendFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
            },
            __wbg_blendFuncSeparate_c13a09db80a687f4: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.blendFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
            },
            __wbg_blendFunc_9cfc908a927238ed: function(arg0, arg1, arg2) {
                arg0.blendFunc(arg1 >>> 0, arg2 >>> 0);
            },
            __wbg_blendFunc_c48e9f82e35e0d29: function(arg0, arg1, arg2) {
                arg0.blendFunc(arg1 >>> 0, arg2 >>> 0);
            },
            __wbg_blitFramebuffer_e82a343f3e2b8c49: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
                arg0.blitFramebuffer(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0);
            },
            __wbg_blockSize_8d4d3e3ebf6496bd: function(arg0) {
                const ret = arg0.blockSize;
                return ret;
            },
            __wbg_blur_a7cd60502d4b9cf8: function() { return handleError(function (arg0) {
                arg0.blur();
            }, arguments); },
            __wbg_body_4eb4906314b12ac0: function(arg0) {
                const ret = arg0.body;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_bottom_a78bc68536ba8b51: function(arg0) {
                const ret = arg0.bottom;
                return ret;
            },
            __wbg_bufferData_39cda6976f6c5628: function(arg0, arg1, arg2, arg3) {
                arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
            },
            __wbg_bufferData_4fdcf2da17845f17: function(arg0, arg1, arg2, arg3) {
                arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
            },
            __wbg_bufferData_df231ca3b030fcb3: function(arg0, arg1, arg2, arg3) {
                arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
            },
            __wbg_bufferData_e5478d2317e9784b: function(arg0, arg1, arg2, arg3) {
                arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
            },
            __wbg_bufferSubData_93ac6d0fde964171: function(arg0, arg1, arg2, arg3) {
                arg0.bufferSubData(arg1 >>> 0, arg2, arg3);
            },
            __wbg_bufferSubData_c08a65dc878612f6: function(arg0, arg1, arg2, arg3) {
                arg0.bufferSubData(arg1 >>> 0, arg2, arg3);
            },
            __wbg_buffer_b47db24bb1e1d5fd: function(arg0) {
                const ret = arg0.buffer;
                return ret;
            },
            __wbg_button_048e9cbb8b27af8e: function(arg0) {
                const ret = arg0.button;
                return ret;
            },
            __wbg_call_08ad0d89caa7cb79: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.call(arg1, arg2);
                return ret;
            }, arguments); },
            __wbg_cancelAnimationFrame_e9e0714e1db5b8f4: function() { return handleError(function (arg0, arg1) {
                arg0.cancelAnimationFrame(arg1);
            }, arguments); },
            __wbg_cancel_7cb5ff04e77d0216: function(arg0) {
                arg0.cancel();
            },
            __wbg_changedTouches_1da9f495ff1b7d40: function(arg0) {
                const ret = arg0.changedTouches;
                return ret;
            },
            __wbg_clearBufferfv_ec80930351b38bfb: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.clearBufferfv(arg1 >>> 0, arg2, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_clearBufferiv_f3b3a12829310bf9: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.clearBufferiv(arg1 >>> 0, arg2, getArrayI32FromWasm0(arg3, arg4));
            },
            __wbg_clearBufferuiv_c063ac933c5cf4eb: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.clearBufferuiv(arg1 >>> 0, arg2, getArrayU32FromWasm0(arg3, arg4));
            },
            __wbg_clearDepth_841f9783e50a00f5: function(arg0, arg1) {
                arg0.clearDepth(arg1);
            },
            __wbg_clearDepth_d0d1d116cf29a818: function(arg0, arg1) {
                arg0.clearDepth(arg1);
            },
            __wbg_clearInterval_7d974f351c09852d: function(arg0, arg1) {
                arg0.clearInterval(arg1);
            },
            __wbg_clearStencil_dc03587dd5c3700a: function(arg0, arg1) {
                arg0.clearStencil(arg1);
            },
            __wbg_clearStencil_fabda6095a014612: function(arg0, arg1) {
                arg0.clearStencil(arg1);
            },
            __wbg_clear_1ba9d21e0b0d5b09: function(arg0, arg1) {
                arg0.clear(arg1 >>> 0);
            },
            __wbg_clear_634214bdd9b127ef: function(arg0, arg1) {
                arg0.clear(arg1 >>> 0);
            },
            __wbg_clientWaitSync_909530be7c352352: function(arg0, arg1, arg2, arg3) {
                const ret = arg0.clientWaitSync(arg1, arg2 >>> 0, arg3 >>> 0);
                return ret;
            },
            __wbg_clientX_810042c308568d39: function(arg0) {
                const ret = arg0.clientX;
                return ret;
            },
            __wbg_clientX_bcafee300d1d801d: function(arg0) {
                const ret = arg0.clientX;
                return ret;
            },
            __wbg_clientY_b6f7fe76e1e90406: function(arg0) {
                const ret = arg0.clientY;
                return ret;
            },
            __wbg_clientY_ddce2da9c948105f: function(arg0) {
                const ret = arg0.clientY;
                return ret;
            },
            __wbg_clipboardData_d9a09cba44040af7: function(arg0) {
                const ret = arg0.clipboardData;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_clipboard_0c2c0e1482d29c8a: function(arg0) {
                const ret = arg0.clipboard;
                return ret;
            },
            __wbg_colorMask_44e896b29859f8c9: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.colorMask(arg1 !== 0, arg2 !== 0, arg3 !== 0, arg4 !== 0);
            },
            __wbg_colorMask_464d785152d1cdef: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.colorMask(arg1 !== 0, arg2 !== 0, arg3 !== 0, arg4 !== 0);
            },
            __wbg_compileShader_4ad30ffb94e34efc: function(arg0, arg1) {
                arg0.compileShader(arg1);
            },
            __wbg_compileShader_f1cddf9f87e77812: function(arg0, arg1) {
                arg0.compileShader(arg1);
            },
            __wbg_compressedTexSubImage2D_22c42aff2113ffeb: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
                arg0.compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8);
            },
            __wbg_compressedTexSubImage2D_658ea6635713fa4f: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
                arg0.compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8);
            },
            __wbg_compressedTexSubImage2D_f820b398b2ca1c59: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8, arg9);
            },
            __wbg_compressedTexSubImage3D_204f12758d7dab5a: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
                arg0.compressedTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10, arg11);
            },
            __wbg_compressedTexSubImage3D_c809cf85e73b8dc9: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
                arg0.compressedTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10);
            },
            __wbg_configure_b39d6ec9527208fd: function() { return handleError(function (arg0, arg1) {
                arg0.configure(arg1);
            }, arguments); },
            __wbg_contentBoxSize_c6aba0e8faa68952: function(arg0) {
                const ret = arg0.contentBoxSize;
                return ret;
            },
            __wbg_contentRect_b2c72b468836067d: function(arg0) {
                const ret = arg0.contentRect;
                return ret;
            },
            __wbg_copyBufferSubData_93f996c53d5adbbf: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.copyBufferSubData(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5);
            },
            __wbg_copyTexSubImage2D_861a9b4f862ede63: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
                arg0.copyTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
            },
            __wbg_copyTexSubImage2D_d5a9efea86f19fdd: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
                arg0.copyTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
            },
            __wbg_copyTexSubImage3D_2bcebcd3156634fb: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.copyTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
            },
            __wbg_copyTextureToBuffer_f5501895b13306e1: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                arg0.copyTextureToBuffer(arg1, arg2, arg3);
            }, arguments); },
            __wbg_createBindGroupLayout_f5bb5a31b2ac11bf: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.createBindGroupLayout(arg1);
                return ret;
            }, arguments); },
            __wbg_createBindGroup_2290306cfa413c74: function(arg0, arg1) {
                const ret = arg0.createBindGroup(arg1);
                return ret;
            },
            __wbg_createBuffer_3b74e7e1fffdc0cd: function(arg0) {
                const ret = arg0.createBuffer();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createBuffer_7336552d8cc9b64d: function(arg0) {
                const ret = arg0.createBuffer();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createBuffer_e2b25dd1471f92f7: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.createBuffer(arg1);
                return ret;
            }, arguments); },
            __wbg_createCommandEncoder_80578730e7314357: function(arg0, arg1) {
                const ret = arg0.createCommandEncoder(arg1);
                return ret;
            },
            __wbg_createElement_c28be812ac2ffe84: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
                return ret;
            }, arguments); },
            __wbg_createFramebuffer_3baafc390f09183d: function(arg0) {
                const ret = arg0.createFramebuffer();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createFramebuffer_9347bd917769c54c: function(arg0) {
                const ret = arg0.createFramebuffer();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createPipelineLayout_0ef251301bed0c34: function(arg0, arg1) {
                const ret = arg0.createPipelineLayout(arg1);
                return ret;
            },
            __wbg_createProgram_297462ff29413ff9: function(arg0) {
                const ret = arg0.createProgram();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createProgram_f15bb178f64abe76: function(arg0) {
                const ret = arg0.createProgram();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createQuery_d2db9b50d71aa734: function(arg0) {
                const ret = arg0.createQuery();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createRenderPipeline_f9f8aa23f50f8a9c: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.createRenderPipeline(arg1);
                return ret;
            }, arguments); },
            __wbg_createRenderbuffer_2b30b056597aafad: function(arg0) {
                const ret = arg0.createRenderbuffer();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createRenderbuffer_af079391e4490df2: function(arg0) {
                const ret = arg0.createRenderbuffer();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createSampler_27c37a8245da51a4: function(arg0, arg1) {
                const ret = arg0.createSampler(arg1);
                return ret;
            },
            __wbg_createSampler_29b1b4802073604b: function(arg0) {
                const ret = arg0.createSampler();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createShaderModule_eb21a131dfb0d4dc: function(arg0, arg1) {
                const ret = arg0.createShaderModule(arg1);
                return ret;
            },
            __wbg_createShader_09f6467c8809e6b7: function(arg0, arg1) {
                const ret = arg0.createShader(arg1 >>> 0);
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createShader_db3e548c131dd109: function(arg0, arg1) {
                const ret = arg0.createShader(arg1 >>> 0);
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createTexture_284160f981e0075f: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.createTexture(arg1);
                return ret;
            }, arguments); },
            __wbg_createTexture_80f7867d3560dc55: function(arg0) {
                const ret = arg0.createTexture();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createTexture_b96824dd5b9eac15: function(arg0) {
                const ret = arg0.createTexture();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createVertexArrayOES_1abd49ebd4675f56: function(arg0) {
                const ret = arg0.createVertexArrayOES();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createVertexArray_96dfeee9b03b8759: function(arg0) {
                const ret = arg0.createVertexArray();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createView_b09749798973b0f5: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.createView(arg1);
                return ret;
            }, arguments); },
            __wbg_ctrlKey_c66665e9d705f967: function(arg0) {
                const ret = arg0.ctrlKey;
                return ret;
            },
            __wbg_ctrlKey_ff524c2e8a33ea2a: function(arg0) {
                const ret = arg0.ctrlKey;
                return ret;
            },
            __wbg_cullFace_620e8e59ab3f7ea9: function(arg0, arg1) {
                arg0.cullFace(arg1 >>> 0);
            },
            __wbg_cullFace_a05bb403025a43c2: function(arg0, arg1) {
                arg0.cullFace(arg1 >>> 0);
            },
            __wbg_dataTransfer_cb01f93a82e217c0: function(arg0) {
                const ret = arg0.dataTransfer;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_data_c04d7396b33c2fac: function(arg0, arg1) {
                const ret = arg1.data;
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_debug_b26db6c23bf072f6: function(arg0, arg1) {
                console.debug(getStringFromWasm0(arg0, arg1));
            },
            __wbg_deleteBuffer_87d51c2118381c4d: function(arg0, arg1) {
                arg0.deleteBuffer(arg1);
            },
            __wbg_deleteBuffer_d7d8e9ac0e6edee5: function(arg0, arg1) {
                arg0.deleteBuffer(arg1);
            },
            __wbg_deleteFramebuffer_2c8f287491b96ba8: function(arg0, arg1) {
                arg0.deleteFramebuffer(arg1);
            },
            __wbg_deleteFramebuffer_bf64f68c155c9b5b: function(arg0, arg1) {
                arg0.deleteFramebuffer(arg1);
            },
            __wbg_deleteProgram_37dbc48826d2cd51: function(arg0, arg1) {
                arg0.deleteProgram(arg1);
            },
            __wbg_deleteProgram_f72adfda762d5d08: function(arg0, arg1) {
                arg0.deleteProgram(arg1);
            },
            __wbg_deleteQuery_2e29965d93bcab81: function(arg0, arg1) {
                arg0.deleteQuery(arg1);
            },
            __wbg_deleteRenderbuffer_1aa08905d179d6f1: function(arg0, arg1) {
                arg0.deleteRenderbuffer(arg1);
            },
            __wbg_deleteRenderbuffer_25cd089b7d406e57: function(arg0, arg1) {
                arg0.deleteRenderbuffer(arg1);
            },
            __wbg_deleteSampler_754f0882540780dc: function(arg0, arg1) {
                arg0.deleteSampler(arg1);
            },
            __wbg_deleteShader_5bcb07b673e64bb7: function(arg0, arg1) {
                arg0.deleteShader(arg1);
            },
            __wbg_deleteShader_8cb16c774b41f231: function(arg0, arg1) {
                arg0.deleteShader(arg1);
            },
            __wbg_deleteSync_ae842921adc9e753: function(arg0, arg1) {
                arg0.deleteSync(arg1);
            },
            __wbg_deleteTexture_7df8c7b546df0b7f: function(arg0, arg1) {
                arg0.deleteTexture(arg1);
            },
            __wbg_deleteTexture_dbcc5d1517d8e813: function(arg0, arg1) {
                arg0.deleteTexture(arg1);
            },
            __wbg_deleteVertexArrayOES_6fc0d67738c79d3d: function(arg0, arg1) {
                arg0.deleteVertexArrayOES(arg1);
            },
            __wbg_deleteVertexArray_61a8364b98fd6eb3: function(arg0, arg1) {
                arg0.deleteVertexArray(arg1);
            },
            __wbg_deltaMode_ba6d094b7940d738: function(arg0) {
                const ret = arg0.deltaMode;
                return ret;
            },
            __wbg_deltaX_c0b6729a7e4606dd: function(arg0) {
                const ret = arg0.deltaX;
                return ret;
            },
            __wbg_deltaY_8c99bdb9344f3932: function(arg0) {
                const ret = arg0.deltaY;
                return ret;
            },
            __wbg_depthFunc_2ecb179aea3ea9ec: function(arg0, arg1) {
                arg0.depthFunc(arg1 >>> 0);
            },
            __wbg_depthFunc_5e62d8faac8e3ece: function(arg0, arg1) {
                arg0.depthFunc(arg1 >>> 0);
            },
            __wbg_depthMask_01ad7bcbad667215: function(arg0, arg1) {
                arg0.depthMask(arg1 !== 0);
            },
            __wbg_depthMask_985f99a66d3306b6: function(arg0, arg1) {
                arg0.depthMask(arg1 !== 0);
            },
            __wbg_depthRange_164b435c41d20b5a: function(arg0, arg1, arg2) {
                arg0.depthRange(arg1, arg2);
            },
            __wbg_depthRange_5f731a354cbb6fcd: function(arg0, arg1, arg2) {
                arg0.depthRange(arg1, arg2);
            },
            __wbg_destroy_7456ec4b2359cbe8: function(arg0) {
                arg0.destroy();
            },
            __wbg_destroy_ebf527bbd86ae58b: function(arg0) {
                arg0.destroy();
            },
            __wbg_devicePixelContentBoxSize_567b1d0782eba230: function(arg0) {
                const ret = arg0.devicePixelContentBoxSize;
                return ret;
            },
            __wbg_devicePixelRatio_48fab2b0d76ee308: function(arg0) {
                const ret = arg0.devicePixelRatio;
                return ret;
            },
            __wbg_disableVertexAttribArray_b1b8bdb62459d86a: function(arg0, arg1) {
                arg0.disableVertexAttribArray(arg1 >>> 0);
            },
            __wbg_disableVertexAttribArray_f0cffca07b1200e2: function(arg0, arg1) {
                arg0.disableVertexAttribArray(arg1 >>> 0);
            },
            __wbg_disable_5dd99ea39c3a626e: function(arg0, arg1) {
                arg0.disable(arg1 >>> 0);
            },
            __wbg_disable_f6643aaf3c8e61c7: function(arg0, arg1) {
                arg0.disable(arg1 >>> 0);
            },
            __wbg_disconnect_be838b47ecbe793a: function(arg0) {
                arg0.disconnect();
            },
            __wbg_document_0b7613236d782ccc: function(arg0) {
                const ret = arg0.document;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_drawArraysInstancedANGLE_7db751170e7fb7e9: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.drawArraysInstancedANGLE(arg1 >>> 0, arg2, arg3, arg4);
            },
            __wbg_drawArraysInstanced_1e6674637ea9a352: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.drawArraysInstanced(arg1 >>> 0, arg2, arg3, arg4);
            },
            __wbg_drawArrays_7d205c1a425d655e: function(arg0, arg1, arg2, arg3) {
                arg0.drawArrays(arg1 >>> 0, arg2, arg3);
            },
            __wbg_drawArrays_ab0d55c13e7c682e: function(arg0, arg1, arg2, arg3) {
                arg0.drawArrays(arg1 >>> 0, arg2, arg3);
            },
            __wbg_drawBuffersWEBGL_fc94498a71ddcfa1: function(arg0, arg1) {
                arg0.drawBuffersWEBGL(arg1);
            },
            __wbg_drawBuffers_c176bf79c7473103: function(arg0, arg1) {
                arg0.drawBuffers(arg1);
            },
            __wbg_drawElementsInstancedANGLE_86005a953de3ca8f: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.drawElementsInstancedANGLE(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
            },
            __wbg_drawElementsInstanced_653f99f5abef21ba: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.drawElementsInstanced(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
            },
            __wbg_drawIndexed_a60a41b2b0ffdadf: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.drawIndexed(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5 >>> 0);
            },
            __wbg_draw_bcc050d6677121b5: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.draw(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
            },
            __wbg_elementFromPoint_202715c736408a6e: function(arg0, arg1, arg2) {
                const ret = arg0.elementFromPoint(arg1, arg2);
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_elementFromPoint_f03d1dbb44a36609: function(arg0, arg1, arg2) {
                const ret = arg0.elementFromPoint(arg1, arg2);
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_enableVertexAttribArray_40936ba18e230cf5: function(arg0, arg1) {
                arg0.enableVertexAttribArray(arg1 >>> 0);
            },
            __wbg_enableVertexAttribArray_9afddc114ba7e8da: function(arg0, arg1) {
                arg0.enableVertexAttribArray(arg1 >>> 0);
            },
            __wbg_enable_78a8f9933e5177b4: function(arg0, arg1) {
                arg0.enable(arg1 >>> 0);
            },
            __wbg_enable_dbad6974e120a23b: function(arg0, arg1) {
                arg0.enable(arg1 >>> 0);
            },
            __wbg_endQuery_906b566f245f25cd: function(arg0, arg1) {
                arg0.endQuery(arg1 >>> 0);
            },
            __wbg_end_c269ebd826210ed1: function(arg0) {
                arg0.end();
            },
            __wbg_error_2cdb790dce31b44d: function(arg0, arg1) {
                let deferred0_0;
                let deferred0_1;
                try {
                    deferred0_0 = arg0;
                    deferred0_1 = arg1;
                    console.error(getStringFromWasm0(arg0, arg1));
                } finally {
                    wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
                }
            },
            __wbg_fenceSync_51ee1809f789e2db: function(arg0, arg1, arg2) {
                const ret = arg0.fenceSync(arg1 >>> 0, arg2 >>> 0);
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_files_20a727e8d5dd5f5b: function(arg0) {
                const ret = arg0.files;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_finish_073e2bc456a4b625: function(arg0) {
                const ret = arg0.finish();
                return ret;
            },
            __wbg_finish_e43b1b48427f2db0: function(arg0, arg1) {
                const ret = arg0.finish(arg1);
                return ret;
            },
            __wbg_flush_79cb8df0e7e2243a: function(arg0) {
                arg0.flush();
            },
            __wbg_flush_93eac7bf0068ddf7: function(arg0) {
                arg0.flush();
            },
            __wbg_focus_a5756e69ecf69851: function() { return handleError(function (arg0) {
                arg0.focus();
            }, arguments); },
            __wbg_force_ea27108ff639c895: function(arg0) {
                const ret = arg0.force;
                return ret;
            },
            __wbg_framebufferRenderbuffer_6f864b125d6f45d4: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4);
            },
            __wbg_framebufferRenderbuffer_7d6565c3de02e312: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4);
            },
            __wbg_framebufferTexture2D_7f01c9c09cb05faf: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5);
            },
            __wbg_framebufferTexture2D_c44b6acceac86932: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5);
            },
            __wbg_framebufferTextureLayer_01df85420b7cce3f: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.framebufferTextureLayer(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5);
            },
            __wbg_framebufferTextureMultiviewOVR_863e10841ebdc1a6: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
                arg0.framebufferTextureMultiviewOVR(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5, arg6);
            },
            __wbg_frontFace_e202e9acb6e8875e: function(arg0, arg1) {
                arg0.frontFace(arg1 >>> 0);
            },
            __wbg_frontFace_fa79a69163e02778: function(arg0, arg1) {
                arg0.frontFace(arg1 >>> 0);
            },
            __wbg_getBindGroupLayout_5e833af835634af6: function(arg0, arg1) {
                const ret = arg0.getBindGroupLayout(arg1 >>> 0);
                return ret;
            },
            __wbg_getBoundingClientRect_881837f23b7e503b: function(arg0) {
                const ret = arg0.getBoundingClientRect();
                return ret;
            },
            __wbg_getBufferSubData_eaa6168d5a6b40ae: function(arg0, arg1, arg2, arg3) {
                arg0.getBufferSubData(arg1 >>> 0, arg2, arg3);
            },
            __wbg_getComputedStyle_e94bad8b9bf17baf: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.getComputedStyle(arg1);
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_getContext_04fd91bf79400077: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_getContext_d222f91a7648c2a6: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                const ret = arg0.getContext(getStringFromWasm0(arg1, arg2), arg3);
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_getContext_e1dd2651f26091f4: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                const ret = arg0.getContext(getStringFromWasm0(arg1, arg2), arg3);
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_getContext_f63e0cc3b9d1dc24: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_getCurrentTexture_7edbea16b438c9fc: function() { return handleError(function (arg0) {
                const ret = arg0.getCurrentTexture();
                return ret;
            }, arguments); },
            __wbg_getData_2a86a2ab154c0650: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                const ret = arg1.getData(getStringFromWasm0(arg2, arg3));
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_getExtension_138346d952a1dac2: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.getExtension(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_getIndexedParameter_8868a4c14e8757a3: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.getIndexedParameter(arg1 >>> 0, arg2 >>> 0);
                return ret;
            }, arguments); },
            __wbg_getItem_d794ea14168dbca6: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                const ret = arg1.getItem(getStringFromWasm0(arg2, arg3));
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_getMappedRange_191c0084744858f0: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.getMappedRange(arg1, arg2);
                return ret;
            }, arguments); },
            __wbg_getParameter_61a012daa3c4b99d: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.getParameter(arg1 >>> 0);
                return ret;
            }, arguments); },
            __wbg_getParameter_883d72fffb8ed209: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.getParameter(arg1 >>> 0);
                return ret;
            }, arguments); },
            __wbg_getPreferredCanvasFormat_56e30944cc798353: function(arg0) {
                const ret = arg0.getPreferredCanvasFormat();
                return (__wbindgen_enum_GpuTextureFormat.indexOf(ret) + 1 || 96) - 1;
            },
            __wbg_getProgramInfoLog_5756ac0562379f54: function(arg0, arg1, arg2) {
                const ret = arg1.getProgramInfoLog(arg2);
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_getProgramInfoLog_b4dc30e835c1d5cd: function(arg0, arg1, arg2) {
                const ret = arg1.getProgramInfoLog(arg2);
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_getProgramParameter_3cfccd1428215c8e: function(arg0, arg1, arg2) {
                const ret = arg0.getProgramParameter(arg1, arg2 >>> 0);
                return ret;
            },
            __wbg_getProgramParameter_c6f164a68c3ae570: function(arg0, arg1, arg2) {
                const ret = arg0.getProgramParameter(arg1, arg2 >>> 0);
                return ret;
            },
            __wbg_getPropertyValue_08a611fc5d42f15d: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                const ret = arg1.getPropertyValue(getStringFromWasm0(arg2, arg3));
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_getQueryParameter_64a385ad047b4d6e: function(arg0, arg1, arg2) {
                const ret = arg0.getQueryParameter(arg1, arg2 >>> 0);
                return ret;
            },
            __wbg_getRootNode_779adc0ff6fc99c2: function(arg0) {
                const ret = arg0.getRootNode();
                return ret;
            },
            __wbg_getShaderInfoLog_18f68e3c08cab42c: function(arg0, arg1, arg2) {
                const ret = arg1.getShaderInfoLog(arg2);
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_getShaderInfoLog_6ee75e4dcf854f76: function(arg0, arg1, arg2) {
                const ret = arg1.getShaderInfoLog(arg2);
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_getShaderParameter_cb466becc20c6e27: function(arg0, arg1, arg2) {
                const ret = arg0.getShaderParameter(arg1, arg2 >>> 0);
                return ret;
            },
            __wbg_getShaderParameter_f04db7dce5e7d819: function(arg0, arg1, arg2) {
                const ret = arg0.getShaderParameter(arg1, arg2 >>> 0);
                return ret;
            },
            __wbg_getSupportedExtensions_ba68e5beb8d84331: function(arg0) {
                const ret = arg0.getSupportedExtensions();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_getSupportedProfiles_f39b445857083740: function(arg0) {
                const ret = arg0.getSupportedProfiles();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_getSyncParameter_90c32aa50a3e230e: function(arg0, arg1, arg2) {
                const ret = arg0.getSyncParameter(arg1, arg2 >>> 0);
                return ret;
            },
            __wbg_getUniformBlockIndex_d9ea984aa1b04197: function(arg0, arg1, arg2, arg3) {
                const ret = arg0.getUniformBlockIndex(arg1, getStringFromWasm0(arg2, arg3));
                return ret;
            },
            __wbg_getUniformLocation_0ad7d432d4038db7: function(arg0, arg1, arg2, arg3) {
                const ret = arg0.getUniformLocation(arg1, getStringFromWasm0(arg2, arg3));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_getUniformLocation_30446f6d89535e6c: function(arg0, arg1, arg2, arg3) {
                const ret = arg0.getUniformLocation(arg1, getStringFromWasm0(arg2, arg3));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_get_01b80713f61639c9: function(arg0, arg1) {
                const ret = arg0[arg1 >>> 0];
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_get_18349afdb36339a9: function() { return handleError(function (arg0, arg1) {
                const ret = Reflect.get(arg0, arg1);
                return ret;
            }, arguments); },
            __wbg_get_a7c870070b1c9a67: function(arg0, arg1) {
                const ret = arg0[arg1 >>> 0];
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_get_b90fa2dace08a1af: function(arg0, arg1) {
                const ret = arg0[arg1 >>> 0];
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_get_f3675880fc3be783: function(arg0, arg1) {
                const ret = arg0[arg1 >>> 0];
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_get_unchecked_3d0f4b91c8eca4f0: function(arg0, arg1) {
                const ret = arg0[arg1 >>> 0];
                return ret;
            },
            __wbg_gpu_7c0927abcc96dd45: function(arg0) {
                const ret = arg0.gpu;
                return ret;
            },
            __wbg_hash_d749d1249013fd4d: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.hash;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_height_28939e1616041ee0: function(arg0) {
                const ret = arg0.height;
                return ret;
            },
            __wbg_height_a2a793f8a2363a46: function(arg0) {
                const ret = arg0.height;
                return ret;
            },
            __wbg_hidden_88a81e3595db6a9e: function(arg0) {
                const ret = arg0.hidden;
                return ret;
            },
            __wbg_host_72ced3a086d285e2: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.host;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_hostname_5f0dc31def0fb6fc: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.hostname;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_href_a4a9bcd105d14884: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.href;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_id_9f9ac8e79d78f45b: function(arg0, arg1) {
                const ret = arg1.id;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_identifier_ea9d33ffaf5bec21: function(arg0) {
                const ret = arg0.identifier;
                return ret;
            },
            __wbg_includes_1614a18b57f4761d: function(arg0, arg1, arg2) {
                const ret = arg0.includes(arg1, arg2);
                return ret;
            },
            __wbg_info_3d31f9abbce09c0d: function(arg0, arg1) {
                console.info(getStringFromWasm0(arg0, arg1));
            },
            __wbg_inlineSize_2ac8eb21e19ddffc: function(arg0) {
                const ret = arg0.inlineSize;
                return ret;
            },
            __wbg_instanceof_Document_3ad6587538294d2f: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof Document;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_Element_3db3f20fca28f450: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof Element;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_GpuAdapter_5e451ad6596e2784: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof GPUAdapter;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_GpuCanvasContext_f70ee27f49f4f884: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof GPUCanvasContext;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_HtmlCanvasElement_d8fa699a8663ca1b: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof HTMLCanvasElement;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_HtmlElement_4e9f5820ff28f6f0: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof HTMLElement;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_HtmlInputElement_fae00d2f3c8ad77f: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof HTMLInputElement;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_ResizeObserverEntry_264f14ace1812c97: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof ResizeObserverEntry;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_ResizeObserverSize_ce5b1a8425a81c6d: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof ResizeObserverSize;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_ShadowRoot_845d1aa5e4cd52ce: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof ShadowRoot;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_WebGl2RenderingContext_8bea2dcac1bc6243: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof WebGL2RenderingContext;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_Window_4aba49e4d1a12365: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof Window;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_invalidateFramebuffer_800ed360d87911e0: function() { return handleError(function (arg0, arg1, arg2) {
                arg0.invalidateFramebuffer(arg1 >>> 0, arg2);
            }, arguments); },
            __wbg_isComposing_0c2995ea41db2701: function(arg0) {
                const ret = arg0.isComposing;
                return ret;
            },
            __wbg_isComposing_e65e3f3f39805b9e: function(arg0) {
                const ret = arg0.isComposing;
                return ret;
            },
            __wbg_isSecureContext_1dac27b103968653: function(arg0) {
                const ret = arg0.isSecureContext;
                return ret;
            },
            __wbg_is_1ad0660d6042ae31: function(arg0, arg1) {
                const ret = Object.is(arg0, arg1);
                return ret;
            },
            __wbg_item_661f7924f6d003c6: function(arg0, arg1) {
                const ret = arg0.item(arg1 >>> 0);
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_items_9ca1084182e2eaad: function(arg0) {
                const ret = arg0.items;
                return ret;
            },
            __wbg_keyCode_20448a5ffa7a043a: function(arg0) {
                const ret = arg0.keyCode;
                return ret;
            },
            __wbg_key_659f8d2f3a028e75: function(arg0, arg1) {
                const ret = arg1.key;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_label_0abc44bf8d3a3e99: function(arg0, arg1) {
                const ret = arg1.label;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_lastModified_33c12a57486cc618: function(arg0) {
                const ret = arg0.lastModified;
                return ret;
            },
            __wbg_left_271b4450c739c266: function(arg0) {
                const ret = arg0.left;
                return ret;
            },
            __wbg_length_191308a415c1c26a: function(arg0) {
                const ret = arg0.length;
                return ret;
            },
            __wbg_length_2cd86f7a61d731a6: function(arg0) {
                const ret = arg0.length;
                return ret;
            },
            __wbg_length_345296cefb19a95e: function(arg0) {
                const ret = arg0.length;
                return ret;
            },
            __wbg_length_5855c1f289dfffc1: function(arg0) {
                const ret = arg0.length;
                return ret;
            },
            __wbg_length_a31e05262e09b7f8: function(arg0) {
                const ret = arg0.length;
                return ret;
            },
            __wbg_limits_ea7aa423b3575ea6: function(arg0) {
                const ret = arg0.limits;
                return ret;
            },
            __wbg_linkProgram_131eeff5c8b4b322: function(arg0, arg1) {
                arg0.linkProgram(arg1);
            },
            __wbg_linkProgram_81931e457dc6efc0: function(arg0, arg1) {
                arg0.linkProgram(arg1);
            },
            __wbg_localStorage_71373fb4bbe7cb23: function() { return handleError(function (arg0) {
                const ret = arg0.localStorage;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_location_e18b1d47af5ae20f: function(arg0) {
                const ret = arg0.location;
                return ret;
            },
            __wbg_mapAsync_1be2f9e8f464f69e: function(arg0, arg1, arg2, arg3) {
                const ret = arg0.mapAsync(arg1 >>> 0, arg2, arg3);
                return ret;
            },
            __wbg_matchMedia_060878840a0816a7: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.matchMedia(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_matches_eff69dc4a8d9ddf8: function(arg0) {
                const ret = arg0.matches;
                return ret;
            },
            __wbg_maxBindGroups_c439abd1498fc924: function(arg0) {
                const ret = arg0.maxBindGroups;
                return ret;
            },
            __wbg_maxBindingsPerBindGroup_186292f383c7b982: function(arg0) {
                const ret = arg0.maxBindingsPerBindGroup;
                return ret;
            },
            __wbg_maxBufferSize_87b76aa2842d0e8e: function(arg0) {
                const ret = arg0.maxBufferSize;
                return ret;
            },
            __wbg_maxColorAttachmentBytesPerSample_2ba81ae1e2742413: function(arg0) {
                const ret = arg0.maxColorAttachmentBytesPerSample;
                return ret;
            },
            __wbg_maxColorAttachments_1ec5191521ef0d22: function(arg0) {
                const ret = arg0.maxColorAttachments;
                return ret;
            },
            __wbg_maxComputeInvocationsPerWorkgroup_ee67a82206d412d2: function(arg0) {
                const ret = arg0.maxComputeInvocationsPerWorkgroup;
                return ret;
            },
            __wbg_maxComputeWorkgroupSizeX_0b2b16b802f85a14: function(arg0) {
                const ret = arg0.maxComputeWorkgroupSizeX;
                return ret;
            },
            __wbg_maxComputeWorkgroupSizeY_00d8aeba9472fdb2: function(arg0) {
                const ret = arg0.maxComputeWorkgroupSizeY;
                return ret;
            },
            __wbg_maxComputeWorkgroupSizeZ_351fd9dab4c07321: function(arg0) {
                const ret = arg0.maxComputeWorkgroupSizeZ;
                return ret;
            },
            __wbg_maxComputeWorkgroupStorageSize_881d2b675868eb68: function(arg0) {
                const ret = arg0.maxComputeWorkgroupStorageSize;
                return ret;
            },
            __wbg_maxComputeWorkgroupsPerDimension_21c223eca6bd6d6b: function(arg0) {
                const ret = arg0.maxComputeWorkgroupsPerDimension;
                return ret;
            },
            __wbg_maxDynamicStorageBuffersPerPipelineLayout_7155d3f7a514a157: function(arg0) {
                const ret = arg0.maxDynamicStorageBuffersPerPipelineLayout;
                return ret;
            },
            __wbg_maxDynamicUniformBuffersPerPipelineLayout_76dee9028eaa5322: function(arg0) {
                const ret = arg0.maxDynamicUniformBuffersPerPipelineLayout;
                return ret;
            },
            __wbg_maxSampledTexturesPerShaderStage_78d018dcd0b999c8: function(arg0) {
                const ret = arg0.maxSampledTexturesPerShaderStage;
                return ret;
            },
            __wbg_maxSamplersPerShaderStage_0e3ad4d70194a7c2: function(arg0) {
                const ret = arg0.maxSamplersPerShaderStage;
                return ret;
            },
            __wbg_maxStorageBufferBindingSize_30a1e5c0b8fcd992: function(arg0) {
                const ret = arg0.maxStorageBufferBindingSize;
                return ret;
            },
            __wbg_maxStorageBuffersPerShaderStage_d77703e9a0d5960e: function(arg0) {
                const ret = arg0.maxStorageBuffersPerShaderStage;
                return ret;
            },
            __wbg_maxStorageTexturesPerShaderStage_c09e7daf1141067e: function(arg0) {
                const ret = arg0.maxStorageTexturesPerShaderStage;
                return ret;
            },
            __wbg_maxTextureArrayLayers_44d8badedb4e5245: function(arg0) {
                const ret = arg0.maxTextureArrayLayers;
                return ret;
            },
            __wbg_maxTextureDimension1D_6d1ff8e56b9cf824: function(arg0) {
                const ret = arg0.maxTextureDimension1D;
                return ret;
            },
            __wbg_maxTextureDimension2D_5ef5830837d92b7c: function(arg0) {
                const ret = arg0.maxTextureDimension2D;
                return ret;
            },
            __wbg_maxTextureDimension3D_cfdebbf2b20068cd: function(arg0) {
                const ret = arg0.maxTextureDimension3D;
                return ret;
            },
            __wbg_maxUniformBufferBindingSize_63dc0c714d2fcebe: function(arg0) {
                const ret = arg0.maxUniformBufferBindingSize;
                return ret;
            },
            __wbg_maxUniformBuffersPerShaderStage_a52382f8a7dfc816: function(arg0) {
                const ret = arg0.maxUniformBuffersPerShaderStage;
                return ret;
            },
            __wbg_maxVertexAttributes_4c83ac8c1d442e1c: function(arg0) {
                const ret = arg0.maxVertexAttributes;
                return ret;
            },
            __wbg_maxVertexBufferArrayStride_955879053ec672f8: function(arg0) {
                const ret = arg0.maxVertexBufferArrayStride;
                return ret;
            },
            __wbg_maxVertexBuffers_0bb014e62f100c6c: function(arg0) {
                const ret = arg0.maxVertexBuffers;
                return ret;
            },
            __wbg_metaKey_3159d9a0a1070899: function(arg0) {
                const ret = arg0.metaKey;
                return ret;
            },
            __wbg_metaKey_665ebd3312e5ed58: function(arg0) {
                const ret = arg0.metaKey;
                return ret;
            },
            __wbg_minStorageBufferOffsetAlignment_6ed09762e603ac3a: function(arg0) {
                const ret = arg0.minStorageBufferOffsetAlignment;
                return ret;
            },
            __wbg_minUniformBufferOffsetAlignment_02579f79815cf83c: function(arg0) {
                const ret = arg0.minUniformBufferOffsetAlignment;
                return ret;
            },
            __wbg_name_947bdbce2d7f9ec1: function(arg0, arg1) {
                const ret = arg1.name;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_navigator_bb9bf52d5003ebaa: function(arg0) {
                const ret = arg0.navigator;
                return ret;
            },
            __wbg_navigator_c088813b66e0b67c: function(arg0) {
                const ret = arg0.navigator;
                return ret;
            },
            __wbg_new_09959f7b4c92c246: function(arg0) {
                const ret = new Uint8Array(arg0);
                return ret;
            },
            __wbg_new_2f79084d4c1a6fc4: function() { return handleError(function (arg0) {
                const ret = new ResizeObserver(arg0);
                return ret;
            }, arguments); },
            __wbg_new_cbee8c0d5c479eac: function() {
                const ret = new Array();
                return ret;
            },
            __wbg_new_e3c739e35c80b60d: function() {
                const ret = new Error();
                return ret;
            },
            __wbg_new_ed69e637b553a997: function() {
                const ret = new Object();
                return ret;
            },
            __wbg_new_from_slice_d7e202fdbee3c396: function(arg0, arg1) {
                const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
                return ret;
            },
            __wbg_new_typed_8258a0d8488ef2a2: function(arg0, arg1) {
                try {
                    var state0 = {a: arg0, b: arg1};
                    var cb0 = (arg0, arg1) => {
                        const a = state0.a;
                        state0.a = 0;
                        try {
                            return wasm_bindgen__convert__closures_____invoke__h4f0b2f7052c09c9c(a, state0.b, arg0, arg1);
                        } finally {
                            state0.a = a;
                        }
                    };
                    const ret = new Promise(cb0);
                    return ret;
                } finally {
                    state0.a = state0.b = 0;
                }
            },
            __wbg_new_typed_e8cd930b75161ad3: function() {
                const ret = new Array();
                return ret;
            },
            __wbg_new_with_byte_offset_and_length_3e6cc05444a2f3c5: function(arg0, arg1, arg2) {
                const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
                return ret;
            },
            __wbg_new_with_record_from_str_to_blob_promise_c00bc5f0199d32ad: function() { return handleError(function (arg0) {
                const ret = new ClipboardItem(arg0);
                return ret;
            }, arguments); },
            __wbg_new_with_text_36b5b68dc2ae71a5: function() { return handleError(function (arg0, arg1) {
                const ret = new SpeechSynthesisUtterance(getStringFromWasm0(arg0, arg1));
                return ret;
            }, arguments); },
            __wbg_new_with_u8_array_sequence_and_options_5c9dbead0aaecd18: function() { return handleError(function (arg0, arg1) {
                const ret = new Blob(arg0, arg1);
                return ret;
            }, arguments); },
            __wbg_now_b134ec02cd6d8b88: function(arg0) {
                const ret = arg0.now();
                return ret;
            },
            __wbg_now_e7c6795a7f81e10f: function(arg0) {
                const ret = arg0.now();
                return ret;
            },
            __wbg_observe_432b8a8326ccea0b: function(arg0, arg1, arg2) {
                arg0.observe(arg1, arg2);
            },
            __wbg_of_25a3bcb86f9d51ab: function(arg0) {
                const ret = Array.of(arg0);
                return ret;
            },
            __wbg_offsetTop_719c9a279c5b1c98: function(arg0) {
                const ret = arg0.offsetTop;
                return ret;
            },
            __wbg_onSubmittedWorkDone_7d532ba1f20a64b3: function(arg0) {
                const ret = arg0.onSubmittedWorkDone();
                return ret;
            },
            __wbg_open_57e3229b59a978a6: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
                const ret = arg0.open(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_origin_a7a87aa0de1545b0: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.origin;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_performance_14d1bb70cebe5f40: function(arg0) {
                const ret = arg0.performance;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_performance_3fcf6e32a7e1ed0a: function(arg0) {
                const ret = arg0.performance;
                return ret;
            },
            __wbg_pixelStorei_a8cd1dddec011730: function(arg0, arg1, arg2) {
                arg0.pixelStorei(arg1 >>> 0, arg2);
            },
            __wbg_pixelStorei_e9e82f9ad573b089: function(arg0, arg1, arg2) {
                arg0.pixelStorei(arg1 >>> 0, arg2);
            },
            __wbg_polygonOffset_1807bbb1882764ca: function(arg0, arg1, arg2) {
                arg0.polygonOffset(arg1, arg2);
            },
            __wbg_polygonOffset_d063a0ac4022a31d: function(arg0, arg1, arg2) {
                arg0.polygonOffset(arg1, arg2);
            },
            __wbg_port_605824fbbd0490b7: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.port;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_preventDefault_d8dbb4013b32560a: function(arg0) {
                arg0.preventDefault();
            },
            __wbg_protocol_d9b33827ebe302c6: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.protocol;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_prototypesetcall_f034d444741426c3: function(arg0, arg1, arg2) {
                Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
            },
            __wbg_push_a6f9488ffd3fae3b: function(arg0, arg1) {
                const ret = arg0.push(arg1);
                return ret;
            },
            __wbg_queryCounterEXT_d005c653c16c094a: function(arg0, arg1, arg2) {
                arg0.queryCounterEXT(arg1, arg2 >>> 0);
            },
            __wbg_querySelectorAll_0553d7ba7491befc: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.querySelectorAll(getStringFromWasm0(arg1, arg2));
                return ret;
            }, arguments); },
            __wbg_querySelector_5a9cd5c59506cf7a: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_queueMicrotask_2c8dfd1056f24fdc: function(arg0) {
                const ret = arg0.queueMicrotask;
                return ret;
            },
            __wbg_queueMicrotask_8985ad63815852e7: function(arg0) {
                queueMicrotask(arg0);
            },
            __wbg_queue_5eda23116e5d3adb: function(arg0) {
                const ret = arg0.queue;
                return ret;
            },
            __wbg_readBuffer_287911ef47f5a8f4: function(arg0, arg1) {
                arg0.readBuffer(arg1 >>> 0);
            },
            __wbg_readPixels_541ab272647b46e7: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
                arg0.readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7);
            }, arguments); },
            __wbg_readPixels_7344aeae85a918d6: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
                arg0.readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7);
            }, arguments); },
            __wbg_readPixels_c640414a3eb425c4: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
                arg0.readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7);
            }, arguments); },
            __wbg_removeEventListener_357b0bf9803ecae1: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3);
            }, arguments); },
            __wbg_remove_575c01e4788cfef7: function(arg0) {
                arg0.remove();
            },
            __wbg_renderbufferStorageMultisample_d985f90b9488db9f: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.renderbufferStorageMultisample(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
            },
            __wbg_renderbufferStorage_e411cc329e1cb3b6: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
            },
            __wbg_renderbufferStorage_fb0e1c9b5c1e2059: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
            },
            __wbg_requestAdapter_8efca1b953fd13aa: function(arg0, arg1) {
                const ret = arg0.requestAdapter(arg1);
                return ret;
            },
            __wbg_requestAdapter_9b4c482e978dd75f: function(arg0) {
                const ret = arg0.requestAdapter();
                return ret;
            },
            __wbg_requestAnimationFrame_a3d50e525d8e209e: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.requestAnimationFrame(arg1);
                return ret;
            }, arguments); },
            __wbg_requestDevice_290c73161fe959d5: function(arg0, arg1) {
                const ret = arg0.requestDevice(arg1);
                return ret;
            },
            __wbg_resolve_5d61e0d10c14730a: function(arg0) {
                const ret = Promise.resolve(arg0);
                return ret;
            },
            __wbg_right_95c8e20338bf429f: function(arg0) {
                const ret = arg0.right;
                return ret;
            },
            __wbg_samplerParameterf_2ecd0bf3d3e47590: function(arg0, arg1, arg2, arg3) {
                arg0.samplerParameterf(arg1, arg2 >>> 0, arg3);
            },
            __wbg_samplerParameteri_b962048082652f16: function(arg0, arg1, arg2, arg3) {
                arg0.samplerParameteri(arg1, arg2 >>> 0, arg3);
            },
            __wbg_scissor_078a1ef455687e27: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.scissor(arg1, arg2, arg3, arg4);
            },
            __wbg_scissor_1e24215d3fe96802: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.scissor(arg1, arg2, arg3, arg4);
            },
            __wbg_search_3b0bdaea662128ca: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.search;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_setAttribute_52376c4b548b7c58: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
                arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            }, arguments); },
            __wbg_setBindGroup_29f4a44dff76f1a4: function(arg0, arg1, arg2) {
                arg0.setBindGroup(arg1 >>> 0, arg2);
            },
            __wbg_setBindGroup_35a4830ac2c27742: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
                arg0.setBindGroup(arg1 >>> 0, arg2, getArrayU32FromWasm0(arg3, arg4), arg5, arg6 >>> 0);
            }, arguments); },
            __wbg_setIndexBuffer_924197dc97dbb679: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.setIndexBuffer(arg1, __wbindgen_enum_GpuIndexFormat[arg2], arg3, arg4);
            },
            __wbg_setIndexBuffer_a400322dea5437f7: function(arg0, arg1, arg2, arg3) {
                arg0.setIndexBuffer(arg1, __wbindgen_enum_GpuIndexFormat[arg2], arg3);
            },
            __wbg_setItem_0c9c2d583a540407: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
                arg0.setItem(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            }, arguments); },
            __wbg_setPipeline_e6ea6756d71b19a7: function(arg0, arg1) {
                arg0.setPipeline(arg1);
            },
            __wbg_setProperty_bdfc1b57fadc046e: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
                arg0.setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            }, arguments); },
            __wbg_setScissorRect_eeb4f61d4b860d7a: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.setScissorRect(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
            },
            __wbg_setVertexBuffer_58f30a4873b36907: function(arg0, arg1, arg2, arg3) {
                arg0.setVertexBuffer(arg1 >>> 0, arg2, arg3);
            },
            __wbg_setVertexBuffer_7aa508f017477005: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.setVertexBuffer(arg1 >>> 0, arg2, arg3, arg4);
            },
            __wbg_setViewport_014b4c4d1101ba6b: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
                arg0.setViewport(arg1, arg2, arg3, arg4, arg5, arg6);
            },
            __wbg_set_410caa03fd65d0ba: function(arg0, arg1, arg2) {
                arg0.set(arg1, arg2 >>> 0);
            },
            __wbg_set_a_6f1653ca7319cdcf: function(arg0, arg1) {
                arg0.a = arg1;
            },
            __wbg_set_access_cbee993a36feed10: function(arg0, arg1) {
                arg0.access = __wbindgen_enum_GpuStorageTextureAccess[arg1];
            },
            __wbg_set_address_mode_u_38e255cd89ce1977: function(arg0, arg1) {
                arg0.addressModeU = __wbindgen_enum_GpuAddressMode[arg1];
            },
            __wbg_set_address_mode_v_513f843d6e3c9dbd: function(arg0, arg1) {
                arg0.addressModeV = __wbindgen_enum_GpuAddressMode[arg1];
            },
            __wbg_set_address_mode_w_801f70901a90ed5a: function(arg0, arg1) {
                arg0.addressModeW = __wbindgen_enum_GpuAddressMode[arg1];
            },
            __wbg_set_alpha_0a28ffc800461787: function(arg0, arg1) {
                arg0.alpha = arg1;
            },
            __wbg_set_alpha_mode_55b4f33e93691fe8: function(arg0, arg1) {
                arg0.alphaMode = __wbindgen_enum_GpuCanvasAlphaMode[arg1];
            },
            __wbg_set_alpha_to_coverage_enabled_ec44695cc0d0e961: function(arg0, arg1) {
                arg0.alphaToCoverageEnabled = arg1 !== 0;
            },
            __wbg_set_array_layer_count_e774b6d4a5334e63: function(arg0, arg1) {
                arg0.arrayLayerCount = arg1 >>> 0;
            },
            __wbg_set_array_stride_11c840b41b728354: function(arg0, arg1) {
                arg0.arrayStride = arg1;
            },
            __wbg_set_aspect_2503cdfcdcc17373: function(arg0, arg1) {
                arg0.aspect = __wbindgen_enum_GpuTextureAspect[arg1];
            },
            __wbg_set_attributes_ac1030b589bf253a: function(arg0, arg1) {
                arg0.attributes = arg1;
            },
            __wbg_set_autofocus_7489c1c09bb9cbed: function() { return handleError(function (arg0, arg1) {
                arg0.autofocus = arg1 !== 0;
            }, arguments); },
            __wbg_set_b_d5b23064b0492744: function(arg0, arg1) {
                arg0.b = arg1;
            },
            __wbg_set_bad5c505cc70b5f8: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = Reflect.set(arg0, arg1, arg2);
                return ret;
            }, arguments); },
            __wbg_set_base_array_layer_f64cdadf250d1a9b: function(arg0, arg1) {
                arg0.baseArrayLayer = arg1 >>> 0;
            },
            __wbg_set_base_mip_level_74fc97c2aaf8fc33: function(arg0, arg1) {
                arg0.baseMipLevel = arg1 >>> 0;
            },
            __wbg_set_beginning_of_pass_write_index_348e7f2f53a86db0: function(arg0, arg1) {
                arg0.beginningOfPassWriteIndex = arg1 >>> 0;
            },
            __wbg_set_bind_group_layouts_6f13eb021a550053: function(arg0, arg1) {
                arg0.bindGroupLayouts = arg1;
            },
            __wbg_set_binding_2240d98479c0c256: function(arg0, arg1) {
                arg0.binding = arg1 >>> 0;
            },
            __wbg_set_binding_5296904f2a4c7e25: function(arg0, arg1) {
                arg0.binding = arg1 >>> 0;
            },
            __wbg_set_blend_4aea897cd7d3c0f8: function(arg0, arg1) {
                arg0.blend = arg1;
            },
            __wbg_set_box_1d483522c055116a: function(arg0, arg1) {
                arg0.box = __wbindgen_enum_ResizeObserverBoxOptions[arg1];
            },
            __wbg_set_buffer_2e7d1f7814caf92b: function(arg0, arg1) {
                arg0.buffer = arg1;
            },
            __wbg_set_buffer_ba8ed06078d347f7: function(arg0, arg1) {
                arg0.buffer = arg1;
            },
            __wbg_set_buffer_fc9285180932669f: function(arg0, arg1) {
                arg0.buffer = arg1;
            },
            __wbg_set_buffers_72754529595d4bc0: function(arg0, arg1) {
                arg0.buffers = arg1;
            },
            __wbg_set_bytes_per_row_5fedf5a2d44b8482: function(arg0, arg1) {
                arg0.bytesPerRow = arg1 >>> 0;
            },
            __wbg_set_bytes_per_row_9425e8d6a11b52dc: function(arg0, arg1) {
                arg0.bytesPerRow = arg1 >>> 0;
            },
            __wbg_set_clear_value_1171de96edbc21fe: function(arg0, arg1) {
                arg0.clearValue = arg1;
            },
            __wbg_set_code_27a25a855d3fbc6d: function(arg0, arg1, arg2) {
                arg0.code = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_color_attachments_4516b6dfb4ad987b: function(arg0, arg1) {
                arg0.colorAttachments = arg1;
            },
            __wbg_set_color_f2ac28bdc576c010: function(arg0, arg1) {
                arg0.color = arg1;
            },
            __wbg_set_compare_2c8ee8ccaa2b6b5d: function(arg0, arg1) {
                arg0.compare = __wbindgen_enum_GpuCompareFunction[arg1];
            },
            __wbg_set_compare_cbf49b43d3211833: function(arg0, arg1) {
                arg0.compare = __wbindgen_enum_GpuCompareFunction[arg1];
            },
            __wbg_set_count_53854513da5c0e04: function(arg0, arg1) {
                arg0.count = arg1 >>> 0;
            },
            __wbg_set_cull_mode_3852dd4cff56dd90: function(arg0, arg1) {
                arg0.cullMode = __wbindgen_enum_GpuCullMode[arg1];
            },
            __wbg_set_depth_bias_c20861a58fc2b8d9: function(arg0, arg1) {
                arg0.depthBias = arg1;
            },
            __wbg_set_depth_bias_clamp_eecc04d702f9402e: function(arg0, arg1) {
                arg0.depthBiasClamp = arg1;
            },
            __wbg_set_depth_bias_slope_scale_b2a251d3d4c65018: function(arg0, arg1) {
                arg0.depthBiasSlopeScale = arg1;
            },
            __wbg_set_depth_clear_value_fca9e379a0cdff8f: function(arg0, arg1) {
                arg0.depthClearValue = arg1;
            },
            __wbg_set_depth_compare_7883e52aad39b925: function(arg0, arg1) {
                arg0.depthCompare = __wbindgen_enum_GpuCompareFunction[arg1];
            },
            __wbg_set_depth_fail_op_1d11c8e03d061484: function(arg0, arg1) {
                arg0.depthFailOp = __wbindgen_enum_GpuStencilOperation[arg1];
            },
            __wbg_set_depth_load_op_7e95e67c69e09c5e: function(arg0, arg1) {
                arg0.depthLoadOp = __wbindgen_enum_GpuLoadOp[arg1];
            },
            __wbg_set_depth_or_array_layers_36ef1df107b6b651: function(arg0, arg1) {
                arg0.depthOrArrayLayers = arg1 >>> 0;
            },
            __wbg_set_depth_read_only_0c5e726b56520b08: function(arg0, arg1) {
                arg0.depthReadOnly = arg1 !== 0;
            },
            __wbg_set_depth_stencil_17e2d1710f4e07ae: function(arg0, arg1) {
                arg0.depthStencil = arg1;
            },
            __wbg_set_depth_stencil_attachment_a7b5eca74b7ddcfb: function(arg0, arg1) {
                arg0.depthStencilAttachment = arg1;
            },
            __wbg_set_depth_store_op_1b4cc257f121a4e7: function(arg0, arg1) {
                arg0.depthStoreOp = __wbindgen_enum_GpuStoreOp[arg1];
            },
            __wbg_set_depth_write_enabled_1551f99ae66d959e: function(arg0, arg1) {
                arg0.depthWriteEnabled = arg1 !== 0;
            },
            __wbg_set_device_846227515bb0301a: function(arg0, arg1) {
                arg0.device = arg1;
            },
            __wbg_set_dimension_7454baa9c745cf06: function(arg0, arg1) {
                arg0.dimension = __wbindgen_enum_GpuTextureDimension[arg1];
            },
            __wbg_set_dimension_9d314669636abc65: function(arg0, arg1) {
                arg0.dimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
            },
            __wbg_set_dst_factor_8397030245674624: function(arg0, arg1) {
                arg0.dstFactor = __wbindgen_enum_GpuBlendFactor[arg1];
            },
            __wbg_set_end_of_pass_write_index_4600a261d0317ecb: function(arg0, arg1) {
                arg0.endOfPassWriteIndex = arg1 >>> 0;
            },
            __wbg_set_entries_4d13c932343146c3: function(arg0, arg1) {
                arg0.entries = arg1;
            },
            __wbg_set_entries_7e6b569918b11bf4: function(arg0, arg1) {
                arg0.entries = arg1;
            },
            __wbg_set_entry_point_7248ed25fb9070c7: function(arg0, arg1, arg2) {
                arg0.entryPoint = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_entry_point_b01eb3970a1dcb95: function(arg0, arg1, arg2) {
                arg0.entryPoint = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_external_texture_cf6cf39036321145: function(arg0, arg1) {
                arg0.externalTexture = arg1;
            },
            __wbg_set_fail_op_ac8f2b4c077715b1: function(arg0, arg1) {
                arg0.failOp = __wbindgen_enum_GpuStencilOperation[arg1];
            },
            __wbg_set_format_12bcbdd3428cd4b5: function(arg0, arg1) {
                arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
            },
            __wbg_set_format_1fc8a436841b29c8: function(arg0, arg1) {
                arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
            },
            __wbg_set_format_2a42ed14de233ae5: function(arg0, arg1) {
                arg0.format = __wbindgen_enum_GpuVertexFormat[arg1];
            },
            __wbg_set_format_3759d043ddc658d4: function(arg0, arg1) {
                arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
            },
            __wbg_set_format_b08e529cc1612d7b: function(arg0, arg1) {
                arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
            },
            __wbg_set_format_e0cf5a237864edb6: function(arg0, arg1) {
                arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
            },
            __wbg_set_format_ffa0a97f114a945a: function(arg0, arg1) {
                arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
            },
            __wbg_set_fragment_703ddd6f5db6e4af: function(arg0, arg1) {
                arg0.fragment = arg1;
            },
            __wbg_set_front_face_17a3723085696d9a: function(arg0, arg1) {
                arg0.frontFace = __wbindgen_enum_GpuFrontFace[arg1];
            },
            __wbg_set_g_4cc3b3e3231ca6f8: function(arg0, arg1) {
                arg0.g = arg1;
            },
            __wbg_set_has_dynamic_offset_dc25aba64b9bd3ff: function(arg0, arg1) {
                arg0.hasDynamicOffset = arg1 !== 0;
            },
            __wbg_set_height_ac705ece3aa08c95: function(arg0, arg1) {
                arg0.height = arg1 >>> 0;
            },
            __wbg_set_height_b3ad521fb0d982ea: function(arg0, arg1) {
                arg0.height = arg1 >>> 0;
            },
            __wbg_set_height_ed13c7b896d93a3b: function(arg0, arg1) {
                arg0.height = arg1 >>> 0;
            },
            __wbg_set_label_10bd19b972ff1ba6: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_16cff4ff3c381368: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_343ceab4761679d7: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_403725ced930414e: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_62b82f9361718fb9: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_7d448e8a777d0d37: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_900e563567315063: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_98bef61fcbcecdde: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_9d2ce197e447a967: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_b5d7ff5f8e4fbaac: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_ba288fbac1259847: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_e1bd2437f39d21f3: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_label_e4debe6dc9ea319b: function(arg0, arg1, arg2) {
                arg0.label = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_layout_53be3643dc5dbbbe: function(arg0, arg1) {
                arg0.layout = arg1;
            },
            __wbg_set_layout_ca5f863d331bb6b4: function(arg0, arg1) {
                arg0.layout = arg1;
            },
            __wbg_set_load_op_91d2cbf2912c96fd: function(arg0, arg1) {
                arg0.loadOp = __wbindgen_enum_GpuLoadOp[arg1];
            },
            __wbg_set_lod_max_clamp_01800ff5df00cc8e: function(arg0, arg1) {
                arg0.lodMaxClamp = arg1;
            },
            __wbg_set_lod_min_clamp_fe71be084b04bd97: function(arg0, arg1) {
                arg0.lodMinClamp = arg1;
            },
            __wbg_set_mag_filter_a6df09d1943d5caa: function(arg0, arg1) {
                arg0.magFilter = __wbindgen_enum_GpuFilterMode[arg1];
            },
            __wbg_set_mapped_at_creation_eb954cf5fdb9bc25: function(arg0, arg1) {
                arg0.mappedAtCreation = arg1 !== 0;
            },
            __wbg_set_mask_47a41aae6631771f: function(arg0, arg1) {
                arg0.mask = arg1 >>> 0;
            },
            __wbg_set_max_anisotropy_418bd200a56097a0: function(arg0, arg1) {
                arg0.maxAnisotropy = arg1;
            },
            __wbg_set_min_binding_size_d0315b751370234c: function(arg0, arg1) {
                arg0.minBindingSize = arg1;
            },
            __wbg_set_min_filter_5b27a7eb3f5ea88a: function(arg0, arg1) {
                arg0.minFilter = __wbindgen_enum_GpuFilterMode[arg1];
            },
            __wbg_set_mip_level_b50dccbd04935c98: function(arg0, arg1) {
                arg0.mipLevel = arg1 >>> 0;
            },
            __wbg_set_mip_level_count_307eb64d9d29e3a6: function(arg0, arg1) {
                arg0.mipLevelCount = arg1 >>> 0;
            },
            __wbg_set_mip_level_count_fe7f73daa6021aaa: function(arg0, arg1) {
                arg0.mipLevelCount = arg1 >>> 0;
            },
            __wbg_set_mipmap_filter_e1543204e8199db0: function(arg0, arg1) {
                arg0.mipmapFilter = __wbindgen_enum_GpuMipmapFilterMode[arg1];
            },
            __wbg_set_module_9afd1b80ff72cee9: function(arg0, arg1) {
                arg0.module = arg1;
            },
            __wbg_set_module_ffe8f8e909e9fdcf: function(arg0, arg1) {
                arg0.module = arg1;
            },
            __wbg_set_multisample_957afdd96685c6f5: function(arg0, arg1) {
                arg0.multisample = arg1;
            },
            __wbg_set_multisampled_84e304d3a68838ea: function(arg0, arg1) {
                arg0.multisampled = arg1 !== 0;
            },
            __wbg_set_offset_157c6bc4fd6ec4b1: function(arg0, arg1) {
                arg0.offset = arg1;
            },
            __wbg_set_offset_3e78f3e530cf8049: function(arg0, arg1) {
                arg0.offset = arg1;
            },
            __wbg_set_offset_616ad7dfa51d50e0: function(arg0, arg1) {
                arg0.offset = arg1;
            },
            __wbg_set_offset_bea112c360dc7f2b: function(arg0, arg1) {
                arg0.offset = arg1;
            },
            __wbg_set_once_c5ce5b778ba9b1f8: function(arg0, arg1) {
                arg0.once = arg1 !== 0;
            },
            __wbg_set_operation_6c5fd88df90bc7b2: function(arg0, arg1) {
                arg0.operation = __wbindgen_enum_GpuBlendOperation[arg1];
            },
            __wbg_set_origin_dec4f4c36f9f79f6: function(arg0, arg1) {
                arg0.origin = arg1;
            },
            __wbg_set_pass_op_461dabd5ee4ea1b7: function(arg0, arg1) {
                arg0.passOp = __wbindgen_enum_GpuStencilOperation[arg1];
            },
            __wbg_set_pitch_fd25583966554248: function(arg0, arg1) {
                arg0.pitch = arg1;
            },
            __wbg_set_power_preference_a4ce891b22ea2b05: function(arg0, arg1) {
                arg0.powerPreference = __wbindgen_enum_GpuPowerPreference[arg1];
            },
            __wbg_set_primitive_eb8abbc5e7f278a4: function(arg0, arg1) {
                arg0.primitive = arg1;
            },
            __wbg_set_query_set_849fb32875f137d7: function(arg0, arg1) {
                arg0.querySet = arg1;
            },
            __wbg_set_r_5fa0f548248c394c: function(arg0, arg1) {
                arg0.r = arg1;
            },
            __wbg_set_rate_8c705d5cd260bb4d: function(arg0, arg1) {
                arg0.rate = arg1;
            },
            __wbg_set_required_features_98a83c7003fd73d5: function(arg0, arg1) {
                arg0.requiredFeatures = arg1;
            },
            __wbg_set_resolve_target_1ff405e060e2d32e: function(arg0, arg1) {
                arg0.resolveTarget = arg1;
            },
            __wbg_set_resource_1409c14d4d6b5a50: function(arg0, arg1) {
                arg0.resource = arg1;
            },
            __wbg_set_rows_per_image_8104dfe1b042a530: function(arg0, arg1) {
                arg0.rowsPerImage = arg1 >>> 0;
            },
            __wbg_set_rows_per_image_9cfda8920e669db0: function(arg0, arg1) {
                arg0.rowsPerImage = arg1 >>> 0;
            },
            __wbg_set_sample_count_95a9892a60894677: function(arg0, arg1) {
                arg0.sampleCount = arg1 >>> 0;
            },
            __wbg_set_sample_type_f8f7b39d62e7b29c: function(arg0, arg1) {
                arg0.sampleType = __wbindgen_enum_GpuTextureSampleType[arg1];
            },
            __wbg_set_sampler_a2277e90dfe7395f: function(arg0, arg1) {
                arg0.sampler = arg1;
            },
            __wbg_set_shader_location_cdbcf5cf84a6cbcb: function(arg0, arg1) {
                arg0.shaderLocation = arg1 >>> 0;
            },
            __wbg_set_size_6f271c4c28c18e1b: function(arg0, arg1) {
                arg0.size = arg1;
            },
            __wbg_set_size_7ec162511b3bad1f: function(arg0, arg1) {
                arg0.size = arg1;
            },
            __wbg_set_size_ca765d983baccefd: function(arg0, arg1) {
                arg0.size = arg1;
            },
            __wbg_set_src_factor_e96f05a25f8383ed: function(arg0, arg1) {
                arg0.srcFactor = __wbindgen_enum_GpuBlendFactor[arg1];
            },
            __wbg_set_stencil_back_5c8971274cbcddcf: function(arg0, arg1) {
                arg0.stencilBack = arg1;
            },
            __wbg_set_stencil_clear_value_89ba97b367fa1385: function(arg0, arg1) {
                arg0.stencilClearValue = arg1 >>> 0;
            },
            __wbg_set_stencil_front_69f85bf4a6f02cb2: function(arg0, arg1) {
                arg0.stencilFront = arg1;
            },
            __wbg_set_stencil_load_op_a3e2c3a6f20d4da5: function(arg0, arg1) {
                arg0.stencilLoadOp = __wbindgen_enum_GpuLoadOp[arg1];
            },
            __wbg_set_stencil_read_mask_86a08afb2665c29b: function(arg0, arg1) {
                arg0.stencilReadMask = arg1 >>> 0;
            },
            __wbg_set_stencil_read_only_dd058fe8c6a1f6ae: function(arg0, arg1) {
                arg0.stencilReadOnly = arg1 !== 0;
            },
            __wbg_set_stencil_store_op_87c97415636844c9: function(arg0, arg1) {
                arg0.stencilStoreOp = __wbindgen_enum_GpuStoreOp[arg1];
            },
            __wbg_set_stencil_write_mask_7844d8a057a87a58: function(arg0, arg1) {
                arg0.stencilWriteMask = arg1 >>> 0;
            },
            __wbg_set_step_mode_285f2e428148f3b4: function(arg0, arg1) {
                arg0.stepMode = __wbindgen_enum_GpuVertexStepMode[arg1];
            },
            __wbg_set_storage_texture_373b9fc0e534dd33: function(arg0, arg1) {
                arg0.storageTexture = arg1;
            },
            __wbg_set_store_op_94575f47253d270d: function(arg0, arg1) {
                arg0.storeOp = __wbindgen_enum_GpuStoreOp[arg1];
            },
            __wbg_set_strip_index_format_aeb7aa0e95e6285d: function(arg0, arg1) {
                arg0.stripIndexFormat = __wbindgen_enum_GpuIndexFormat[arg1];
            },
            __wbg_set_tabIndex_e06b6f565e09ba96: function(arg0, arg1) {
                arg0.tabIndex = arg1;
            },
            __wbg_set_targets_93553735385af349: function(arg0, arg1) {
                arg0.targets = arg1;
            },
            __wbg_set_texture_6003a9e79918bf8a: function(arg0, arg1) {
                arg0.texture = arg1;
            },
            __wbg_set_texture_c5a457625c071b25: function(arg0, arg1) {
                arg0.texture = arg1;
            },
            __wbg_set_timestamp_writes_0603b32a31ee6205: function(arg0, arg1) {
                arg0.timestampWrites = arg1;
            },
            __wbg_set_topology_5e4eb809635ea291: function(arg0, arg1) {
                arg0.topology = __wbindgen_enum_GpuPrimitiveTopology[arg1];
            },
            __wbg_set_type_0e707d4c06fc2b7b: function(arg0, arg1) {
                arg0.type = __wbindgen_enum_GpuSamplerBindingType[arg1];
            },
            __wbg_set_type_1631880f22765d5c: function(arg0, arg1, arg2) {
                arg0.type = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_type_6fe4c5f460401ee0: function(arg0, arg1) {
                arg0.type = __wbindgen_enum_GpuBufferBindingType[arg1];
            },
            __wbg_set_type_f7c1c5bc543a4f95: function(arg0, arg1, arg2) {
                arg0.type = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_unclipped_depth_e9a2451e4fa0277a: function(arg0, arg1) {
                arg0.unclippedDepth = arg1 !== 0;
            },
            __wbg_set_usage_5abd566becc087bb: function(arg0, arg1) {
                arg0.usage = arg1 >>> 0;
            },
            __wbg_set_usage_61967f166fba5e13: function(arg0, arg1) {
                arg0.usage = arg1 >>> 0;
            },
            __wbg_set_usage_d0a75d4429098a06: function(arg0, arg1) {
                arg0.usage = arg1 >>> 0;
            },
            __wbg_set_usage_f0bb325677668e77: function(arg0, arg1) {
                arg0.usage = arg1 >>> 0;
            },
            __wbg_set_value_078f56ab8bf4ee14: function(arg0, arg1, arg2) {
                arg0.value = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_vertex_2525cfcd959b2add: function(arg0, arg1) {
                arg0.vertex = arg1;
            },
            __wbg_set_view_57d232eea19739c3: function(arg0, arg1) {
                arg0.view = arg1;
            },
            __wbg_set_view_dimension_49cfda500f1dea55: function(arg0, arg1) {
                arg0.viewDimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
            },
            __wbg_set_view_dimension_a669c29ec3b0813a: function(arg0, arg1) {
                arg0.viewDimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
            },
            __wbg_set_view_ffadd767d5e9b839: function(arg0, arg1) {
                arg0.view = arg1;
            },
            __wbg_set_view_formats_70a1fcabcd34282a: function(arg0, arg1) {
                arg0.viewFormats = arg1;
            },
            __wbg_set_view_formats_83865b9cdfda5cb6: function(arg0, arg1) {
                arg0.viewFormats = arg1;
            },
            __wbg_set_visibility_088046ee77c33b1d: function(arg0, arg1) {
                arg0.visibility = arg1 >>> 0;
            },
            __wbg_set_volume_6cafa29e1198f240: function(arg0, arg1) {
                arg0.volume = arg1;
            },
            __wbg_set_width_7f65ced2ffeee343: function(arg0, arg1) {
                arg0.width = arg1 >>> 0;
            },
            __wbg_set_width_ae28c0c10381c919: function(arg0, arg1) {
                arg0.width = arg1 >>> 0;
            },
            __wbg_set_width_e96e07f8255ad913: function(arg0, arg1) {
                arg0.width = arg1 >>> 0;
            },
            __wbg_set_write_mask_76041c03688571cd: function(arg0, arg1) {
                arg0.writeMask = arg1 >>> 0;
            },
            __wbg_set_x_fdd6aca9a2390926: function(arg0, arg1) {
                arg0.x = arg1 >>> 0;
            },
            __wbg_set_y_410a18c5811abf4c: function(arg0, arg1) {
                arg0.y = arg1 >>> 0;
            },
            __wbg_set_z_f7f1ae8afd3a9308: function(arg0, arg1) {
                arg0.z = arg1 >>> 0;
            },
            __wbg_shaderSource_5c87a16d08541975: function(arg0, arg1, arg2, arg3) {
                arg0.shaderSource(arg1, getStringFromWasm0(arg2, arg3));
            },
            __wbg_shaderSource_fd677c89c8b82c95: function(arg0, arg1, arg2, arg3) {
                arg0.shaderSource(arg1, getStringFromWasm0(arg2, arg3));
            },
            __wbg_shiftKey_a05e8e0faf05efa4: function(arg0) {
                const ret = arg0.shiftKey;
                return ret;
            },
            __wbg_shiftKey_f1de6c442d6b6562: function(arg0) {
                const ret = arg0.shiftKey;
                return ret;
            },
            __wbg_size_09f35345b4742a87: function(arg0) {
                const ret = arg0.size;
                return ret;
            },
            __wbg_size_551c19bddc4aeca7: function(arg0) {
                const ret = arg0.size;
                return ret;
            },
            __wbg_speak_7efc2f26b2fd1757: function(arg0, arg1) {
                arg0.speak(arg1);
            },
            __wbg_speechSynthesis_faea0d08817408d8: function() { return handleError(function (arg0) {
                const ret = arg0.speechSynthesis;
                return ret;
            }, arguments); },
            __wbg_stack_452d99d0c4dad9e1: function(arg0, arg1) {
                const ret = arg1.stack;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_static_accessor_GLOBAL_THIS_14325d8cca34bb77: function() {
                const ret = typeof globalThis === 'undefined' ? null : globalThis;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_GLOBAL_f3a1e69f9c5a7e8e: function() {
                const ret = typeof global === 'undefined' ? null : global;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_SELF_50cdb5b517789aca: function() {
                const ret = typeof self === 'undefined' ? null : self;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_WINDOW_d6c4126e4c244380: function() {
                const ret = typeof window === 'undefined' ? null : window;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_stencilFuncSeparate_1ba83bbf67cd09a1: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.stencilFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3, arg4 >>> 0);
            },
            __wbg_stencilFuncSeparate_c147b1030478577c: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.stencilFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3, arg4 >>> 0);
            },
            __wbg_stencilMaskSeparate_b95fb4da26b1f0ca: function(arg0, arg1, arg2) {
                arg0.stencilMaskSeparate(arg1 >>> 0, arg2 >>> 0);
            },
            __wbg_stencilMaskSeparate_e9506ba14d5a63af: function(arg0, arg1, arg2) {
                arg0.stencilMaskSeparate(arg1 >>> 0, arg2 >>> 0);
            },
            __wbg_stencilMask_4465f5e5289670c7: function(arg0, arg1) {
                arg0.stencilMask(arg1 >>> 0);
            },
            __wbg_stencilMask_b6216f0fa63cbef2: function(arg0, arg1) {
                arg0.stencilMask(arg1 >>> 0);
            },
            __wbg_stencilOpSeparate_1265a994bf83b591: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.stencilOpSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
            },
            __wbg_stencilOpSeparate_d5adcac036870822: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.stencilOpSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
            },
            __wbg_stopPropagation_189d059eaef22300: function(arg0) {
                arg0.stopPropagation();
            },
            __wbg_style_40817c2a3eeee400: function(arg0) {
                const ret = arg0.style;
                return ret;
            },
            __wbg_submit_21302eebe551e30d: function(arg0, arg1) {
                arg0.submit(arg1);
            },
            __wbg_texImage2D_169a1d2793c9b4d0: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
            }, arguments); },
            __wbg_texImage2D_a6c153f8f3f4a6fa: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
            }, arguments); },
            __wbg_texImage2D_cc6a47f9fc917114: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
            }, arguments); },
            __wbg_texImage3D_079737846c2b5087: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
                arg0.texImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8 >>> 0, arg9 >>> 0, arg10);
            }, arguments); },
            __wbg_texImage3D_0e00d8ea22772860: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
                arg0.texImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8 >>> 0, arg9 >>> 0, arg10);
            }, arguments); },
            __wbg_texParameteri_0fd9fa05e3edc812: function(arg0, arg1, arg2, arg3) {
                arg0.texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
            },
            __wbg_texParameteri_269da40546b830f2: function(arg0, arg1, arg2, arg3) {
                arg0.texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
            },
            __wbg_texStorage2D_83378704b8f8f379: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.texStorage2D(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
            },
            __wbg_texStorage3D_6bac59aaccb552d6: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
                arg0.texStorage3D(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5, arg6);
            },
            __wbg_texSubImage2D_56f3e2c69a807a5c: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
            }, arguments); },
            __wbg_texSubImage2D_8cdf712e9d8fa9a7: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
            }, arguments); },
            __wbg_texSubImage2D_970887d00df8aabc: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
            }, arguments); },
            __wbg_texSubImage2D_97cfc86cb45f59d3: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
            }, arguments); },
            __wbg_texSubImage2D_b94b8397ebb54802: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
            }, arguments); },
            __wbg_texSubImage2D_df85c28c3f512381: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
            }, arguments); },
            __wbg_texSubImage2D_eef4d80bbb449050: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
            }, arguments); },
            __wbg_texSubImage2D_f0eead2d81419472: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
                arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
            }, arguments); },
            __wbg_texSubImage3D_0fccea0343b7c2d2: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
                arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
            }, arguments); },
            __wbg_texSubImage3D_1d5e591b38114399: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
                arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
            }, arguments); },
            __wbg_texSubImage3D_31061cf43f114023: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
                arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
            }, arguments); },
            __wbg_texSubImage3D_4de7cff75d031ee9: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
                arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
            }, arguments); },
            __wbg_texSubImage3D_5953d9e69c2d982b: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
                arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
            }, arguments); },
            __wbg_texSubImage3D_9913b25dfee57acc: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
                arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
            }, arguments); },
            __wbg_texSubImage3D_e3f4c1fa03686530: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
                arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
            }, arguments); },
            __wbg_then_a5a891fa8b478d8d: function(arg0, arg1, arg2) {
                const ret = arg0.then(arg1, arg2);
                return ret;
            },
            __wbg_then_d4163530723f56f4: function(arg0, arg1, arg2) {
                const ret = arg0.then(arg1, arg2);
                return ret;
            },
            __wbg_then_f1c954fe00733701: function(arg0, arg1) {
                const ret = arg0.then(arg1);
                return ret;
            },
            __wbg_top_8b55ad858d42fd2e: function(arg0) {
                const ret = arg0.top;
                return ret;
            },
            __wbg_touches_e34c4876f28425b6: function(arg0) {
                const ret = arg0.touches;
                return ret;
            },
            __wbg_type_bee715f6264f94c2: function(arg0, arg1) {
                const ret = arg1.type;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_type_f32dd7c24b75e034: function(arg0, arg1) {
                const ret = arg1.type;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_uniform1f_5659911af16ec766: function(arg0, arg1, arg2) {
                arg0.uniform1f(arg1, arg2);
            },
            __wbg_uniform1f_60068e43759c0b8c: function(arg0, arg1, arg2) {
                arg0.uniform1f(arg1, arg2);
            },
            __wbg_uniform1i_052c6443a75c7366: function(arg0, arg1, arg2) {
                arg0.uniform1i(arg1, arg2);
            },
            __wbg_uniform1i_899e4dcca4f79778: function(arg0, arg1, arg2) {
                arg0.uniform1i(arg1, arg2);
            },
            __wbg_uniform1ui_305ec23a7959f5d9: function(arg0, arg1, arg2) {
                arg0.uniform1ui(arg1, arg2 >>> 0);
            },
            __wbg_uniform2fv_1f6882fe059eb80d: function(arg0, arg1, arg2, arg3) {
                arg0.uniform2fv(arg1, getArrayF32FromWasm0(arg2, arg3));
            },
            __wbg_uniform2fv_bce01decb209b02f: function(arg0, arg1, arg2, arg3) {
                arg0.uniform2fv(arg1, getArrayF32FromWasm0(arg2, arg3));
            },
            __wbg_uniform2iv_a16ca338795b24a9: function(arg0, arg1, arg2, arg3) {
                arg0.uniform2iv(arg1, getArrayI32FromWasm0(arg2, arg3));
            },
            __wbg_uniform2iv_b88952a5325ea197: function(arg0, arg1, arg2, arg3) {
                arg0.uniform2iv(arg1, getArrayI32FromWasm0(arg2, arg3));
            },
            __wbg_uniform2uiv_53ddc4bb01816864: function(arg0, arg1, arg2, arg3) {
                arg0.uniform2uiv(arg1, getArrayU32FromWasm0(arg2, arg3));
            },
            __wbg_uniform3fv_32b832e8077f4941: function(arg0, arg1, arg2, arg3) {
                arg0.uniform3fv(arg1, getArrayF32FromWasm0(arg2, arg3));
            },
            __wbg_uniform3fv_9980c262279e8ff6: function(arg0, arg1, arg2, arg3) {
                arg0.uniform3fv(arg1, getArrayF32FromWasm0(arg2, arg3));
            },
            __wbg_uniform3iv_6cb2e385ccac3311: function(arg0, arg1, arg2, arg3) {
                arg0.uniform3iv(arg1, getArrayI32FromWasm0(arg2, arg3));
            },
            __wbg_uniform3iv_c51c448e5ade5cb3: function(arg0, arg1, arg2, arg3) {
                arg0.uniform3iv(arg1, getArrayI32FromWasm0(arg2, arg3));
            },
            __wbg_uniform3uiv_76319e38565e60ef: function(arg0, arg1, arg2, arg3) {
                arg0.uniform3uiv(arg1, getArrayU32FromWasm0(arg2, arg3));
            },
            __wbg_uniform4f_26dc31991794a19f: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.uniform4f(arg1, arg2, arg3, arg4, arg5);
            },
            __wbg_uniform4f_d1d88a600a851b4a: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.uniform4f(arg1, arg2, arg3, arg4, arg5);
            },
            __wbg_uniform4fv_bb625975e5aa7d37: function(arg0, arg1, arg2, arg3) {
                arg0.uniform4fv(arg1, getArrayF32FromWasm0(arg2, arg3));
            },
            __wbg_uniform4fv_eaff9e835bdfecdc: function(arg0, arg1, arg2, arg3) {
                arg0.uniform4fv(arg1, getArrayF32FromWasm0(arg2, arg3));
            },
            __wbg_uniform4iv_ae91c947ade7ce01: function(arg0, arg1, arg2, arg3) {
                arg0.uniform4iv(arg1, getArrayI32FromWasm0(arg2, arg3));
            },
            __wbg_uniform4iv_bff2c54f3d252186: function(arg0, arg1, arg2, arg3) {
                arg0.uniform4iv(arg1, getArrayI32FromWasm0(arg2, arg3));
            },
            __wbg_uniform4uiv_e6b4d068cf56c502: function(arg0, arg1, arg2, arg3) {
                arg0.uniform4uiv(arg1, getArrayU32FromWasm0(arg2, arg3));
            },
            __wbg_uniformBlockBinding_6462fb1acaedfc4c: function(arg0, arg1, arg2, arg3) {
                arg0.uniformBlockBinding(arg1, arg2 >>> 0, arg3 >>> 0);
            },
            __wbg_uniformMatrix2fv_957138c1ade047ed: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix2fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_uniformMatrix2fv_af14028516dec70e: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix2fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_uniformMatrix2x3fv_4f27dcebb733b3a4: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix2x3fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_uniformMatrix2x4fv_d29fe6f5166fff1f: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix2x4fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_uniformMatrix3fv_390d1ef3042cd49c: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix3fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_uniformMatrix3fv_83cf507f6128f506: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix3fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_uniformMatrix3x2fv_2b76937b77214b3f: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix3x2fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_uniformMatrix3x4fv_56684a80298742ae: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix3x4fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_uniformMatrix4fv_291619e6434aab4d: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix4fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_uniformMatrix4fv_5f024e3e3fb7b6b1: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix4fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_uniformMatrix4x2fv_7f1b6f5878ebcb5a: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix4x2fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_uniformMatrix4x3fv_ca4357c4fd88e258: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniformMatrix4x3fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
            },
            __wbg_unmap_b819b8b402db13cc: function(arg0) {
                arg0.unmap();
            },
            __wbg_usage_34a9bc47ff4a3feb: function(arg0) {
                const ret = arg0.usage;
                return ret;
            },
            __wbg_useProgram_b9270da3c6fe321b: function(arg0, arg1) {
                arg0.useProgram(arg1);
            },
            __wbg_useProgram_c86286bf2a0c907a: function(arg0, arg1) {
                arg0.useProgram(arg1);
            },
            __wbg_userAgent_b04cea25c65e3f22: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.userAgent;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_value_161196e83c12d910: function(arg0, arg1) {
                const ret = arg1.value;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_vertexAttribDivisorANGLE_3a8d24c266270457: function(arg0, arg1, arg2) {
                arg0.vertexAttribDivisorANGLE(arg1 >>> 0, arg2 >>> 0);
            },
            __wbg_vertexAttribDivisor_08b24576542733b4: function(arg0, arg1, arg2) {
                arg0.vertexAttribDivisor(arg1 >>> 0, arg2 >>> 0);
            },
            __wbg_vertexAttribIPointer_2d38be749cda8ba5: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.vertexAttribIPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
            },
            __wbg_vertexAttribPointer_5fae23082137507b: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
                arg0.vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
            },
            __wbg_vertexAttribPointer_81ff6d983a8358fc: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
                arg0.vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
            },
            __wbg_viewport_3cb1161a95655499: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.viewport(arg1, arg2, arg3, arg4);
            },
            __wbg_viewport_62bfa3b78bd53d4d: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.viewport(arg1, arg2, arg3, arg4);
            },
            __wbg_warn_7eab3af0ebd26b76: function(arg0, arg1) {
                console.warn(getStringFromWasm0(arg0, arg1));
            },
            __wbg_width_60f44a816d7f9267: function(arg0) {
                const ret = arg0.width;
                return ret;
            },
            __wbg_width_d93904c25e940752: function(arg0) {
                const ret = arg0.width;
                return ret;
            },
            __wbg_writeBuffer_c6919ed0c4aaeef5: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.writeBuffer(arg1, arg2, arg3, arg4, arg5);
            }, arguments); },
            __wbg_writeText_9b14339000e655c8: function(arg0, arg1, arg2) {
                const ret = arg0.writeText(getStringFromWasm0(arg1, arg2));
                return ret;
            },
            __wbg_writeTexture_340cfbecd9544755: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
                arg0.writeTexture(arg1, arg2, arg3, arg4);
            }, arguments); },
            __wbg_write_979387ca6cc33ac0: function(arg0, arg1) {
                const ret = arg0.write(arg1);
                return ret;
            },
            __wbindgen_cast_0000000000000001: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { dtor_idx: 1507, function: Function { arguments: [NamedExternref("Array<any>")], shim_idx: 1510, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h1750c7be6d72d079, wasm_bindgen__convert__closures_____invoke__h76d95842a6218214);
                return ret;
            },
            __wbindgen_cast_0000000000000002: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { dtor_idx: 1507, function: Function { arguments: [NamedExternref("Event")], shim_idx: 1510, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h1750c7be6d72d079, wasm_bindgen__convert__closures_____invoke__h76d95842a6218214_1);
                return ret;
            },
            __wbindgen_cast_0000000000000003: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { dtor_idx: 1507, function: Function { arguments: [], shim_idx: 1508, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h1750c7be6d72d079, wasm_bindgen__convert__closures_____invoke__hacc4601c5043654b);
                return ret;
            },
            __wbindgen_cast_0000000000000004: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { dtor_idx: 2127, function: Function { arguments: [Externref], shim_idx: 2128, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__ha7f1522537e420d5, wasm_bindgen__convert__closures_____invoke__h00a79c105f336659);
                return ret;
            },
            __wbindgen_cast_0000000000000005: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { dtor_idx: 2302, function: Function { arguments: [Externref], shim_idx: 2303, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h0c327d1cfa66f6cc, wasm_bindgen__convert__closures_____invoke__heda35ba472fcc8de);
                return ret;
            },
            __wbindgen_cast_0000000000000006: function(arg0) {
                // Cast intrinsic for `F64 -> Externref`.
                const ret = arg0;
                return ret;
            },
            __wbindgen_cast_0000000000000007: function(arg0, arg1) {
                // Cast intrinsic for `Ref(Slice(F32)) -> NamedExternref("Float32Array")`.
                const ret = getArrayF32FromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_cast_0000000000000008: function(arg0, arg1) {
                // Cast intrinsic for `Ref(Slice(I16)) -> NamedExternref("Int16Array")`.
                const ret = getArrayI16FromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_cast_0000000000000009: function(arg0, arg1) {
                // Cast intrinsic for `Ref(Slice(I32)) -> NamedExternref("Int32Array")`.
                const ret = getArrayI32FromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_cast_000000000000000a: function(arg0, arg1) {
                // Cast intrinsic for `Ref(Slice(I8)) -> NamedExternref("Int8Array")`.
                const ret = getArrayI8FromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_cast_000000000000000b: function(arg0, arg1) {
                // Cast intrinsic for `Ref(Slice(U16)) -> NamedExternref("Uint16Array")`.
                const ret = getArrayU16FromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_cast_000000000000000c: function(arg0, arg1) {
                // Cast intrinsic for `Ref(Slice(U32)) -> NamedExternref("Uint32Array")`.
                const ret = getArrayU32FromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_cast_000000000000000d: function(arg0, arg1) {
                // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
                const ret = getArrayU8FromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_cast_000000000000000e: function(arg0, arg1) {
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
            "./blog_app_bg.js": import0,
        };
    }

    function wasm_bindgen__convert__closures_____invoke__hacc4601c5043654b(arg0, arg1) {
        const ret = wasm.wasm_bindgen__convert__closures_____invoke__hacc4601c5043654b(arg0, arg1);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }

    function wasm_bindgen__convert__closures_____invoke__h76d95842a6218214(arg0, arg1, arg2) {
        wasm.wasm_bindgen__convert__closures_____invoke__h76d95842a6218214(arg0, arg1, arg2);
    }

    function wasm_bindgen__convert__closures_____invoke__h76d95842a6218214_1(arg0, arg1, arg2) {
        wasm.wasm_bindgen__convert__closures_____invoke__h76d95842a6218214_1(arg0, arg1, arg2);
    }

    function wasm_bindgen__convert__closures_____invoke__h00a79c105f336659(arg0, arg1, arg2) {
        wasm.wasm_bindgen__convert__closures_____invoke__h00a79c105f336659(arg0, arg1, arg2);
    }

    function wasm_bindgen__convert__closures_____invoke__heda35ba472fcc8de(arg0, arg1, arg2) {
        const ret = wasm.wasm_bindgen__convert__closures_____invoke__heda35ba472fcc8de(arg0, arg1, arg2);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }

    function wasm_bindgen__convert__closures_____invoke__h4f0b2f7052c09c9c(arg0, arg1, arg2, arg3) {
        wasm.wasm_bindgen__convert__closures_____invoke__h4f0b2f7052c09c9c(arg0, arg1, arg2, arg3);
    }


    const __wbindgen_enum_GpuAddressMode = ["clamp-to-edge", "repeat", "mirror-repeat"];


    const __wbindgen_enum_GpuBlendFactor = ["zero", "one", "src", "one-minus-src", "src-alpha", "one-minus-src-alpha", "dst", "one-minus-dst", "dst-alpha", "one-minus-dst-alpha", "src-alpha-saturated", "constant", "one-minus-constant", "src1", "one-minus-src1", "src1-alpha", "one-minus-src1-alpha"];


    const __wbindgen_enum_GpuBlendOperation = ["add", "subtract", "reverse-subtract", "min", "max"];


    const __wbindgen_enum_GpuBufferBindingType = ["uniform", "storage", "read-only-storage"];


    const __wbindgen_enum_GpuCanvasAlphaMode = ["opaque", "premultiplied"];


    const __wbindgen_enum_GpuCompareFunction = ["never", "less", "equal", "less-equal", "greater", "not-equal", "greater-equal", "always"];


    const __wbindgen_enum_GpuCullMode = ["none", "front", "back"];


    const __wbindgen_enum_GpuFilterMode = ["nearest", "linear"];


    const __wbindgen_enum_GpuFrontFace = ["ccw", "cw"];


    const __wbindgen_enum_GpuIndexFormat = ["uint16", "uint32"];


    const __wbindgen_enum_GpuLoadOp = ["load", "clear"];


    const __wbindgen_enum_GpuMipmapFilterMode = ["nearest", "linear"];


    const __wbindgen_enum_GpuPowerPreference = ["low-power", "high-performance"];


    const __wbindgen_enum_GpuPrimitiveTopology = ["point-list", "line-list", "line-strip", "triangle-list", "triangle-strip"];


    const __wbindgen_enum_GpuSamplerBindingType = ["filtering", "non-filtering", "comparison"];


    const __wbindgen_enum_GpuStencilOperation = ["keep", "zero", "replace", "invert", "increment-clamp", "decrement-clamp", "increment-wrap", "decrement-wrap"];


    const __wbindgen_enum_GpuStorageTextureAccess = ["write-only", "read-only", "read-write"];


    const __wbindgen_enum_GpuStoreOp = ["store", "discard"];


    const __wbindgen_enum_GpuTextureAspect = ["all", "stencil-only", "depth-only"];


    const __wbindgen_enum_GpuTextureDimension = ["1d", "2d", "3d"];


    const __wbindgen_enum_GpuTextureFormat = ["r8unorm", "r8snorm", "r8uint", "r8sint", "r16uint", "r16sint", "r16float", "rg8unorm", "rg8snorm", "rg8uint", "rg8sint", "r32uint", "r32sint", "r32float", "rg16uint", "rg16sint", "rg16float", "rgba8unorm", "rgba8unorm-srgb", "rgba8snorm", "rgba8uint", "rgba8sint", "bgra8unorm", "bgra8unorm-srgb", "rgb9e5ufloat", "rgb10a2uint", "rgb10a2unorm", "rg11b10ufloat", "rg32uint", "rg32sint", "rg32float", "rgba16uint", "rgba16sint", "rgba16float", "rgba32uint", "rgba32sint", "rgba32float", "stencil8", "depth16unorm", "depth24plus", "depth24plus-stencil8", "depth32float", "depth32float-stencil8", "bc1-rgba-unorm", "bc1-rgba-unorm-srgb", "bc2-rgba-unorm", "bc2-rgba-unorm-srgb", "bc3-rgba-unorm", "bc3-rgba-unorm-srgb", "bc4-r-unorm", "bc4-r-snorm", "bc5-rg-unorm", "bc5-rg-snorm", "bc6h-rgb-ufloat", "bc6h-rgb-float", "bc7-rgba-unorm", "bc7-rgba-unorm-srgb", "etc2-rgb8unorm", "etc2-rgb8unorm-srgb", "etc2-rgb8a1unorm", "etc2-rgb8a1unorm-srgb", "etc2-rgba8unorm", "etc2-rgba8unorm-srgb", "eac-r11unorm", "eac-r11snorm", "eac-rg11unorm", "eac-rg11snorm", "astc-4x4-unorm", "astc-4x4-unorm-srgb", "astc-5x4-unorm", "astc-5x4-unorm-srgb", "astc-5x5-unorm", "astc-5x5-unorm-srgb", "astc-6x5-unorm", "astc-6x5-unorm-srgb", "astc-6x6-unorm", "astc-6x6-unorm-srgb", "astc-8x5-unorm", "astc-8x5-unorm-srgb", "astc-8x6-unorm", "astc-8x6-unorm-srgb", "astc-8x8-unorm", "astc-8x8-unorm-srgb", "astc-10x5-unorm", "astc-10x5-unorm-srgb", "astc-10x6-unorm", "astc-10x6-unorm-srgb", "astc-10x8-unorm", "astc-10x8-unorm-srgb", "astc-10x10-unorm", "astc-10x10-unorm-srgb", "astc-12x10-unorm", "astc-12x10-unorm-srgb", "astc-12x12-unorm", "astc-12x12-unorm-srgb"];


    const __wbindgen_enum_GpuTextureSampleType = ["float", "unfilterable-float", "depth", "sint", "uint"];


    const __wbindgen_enum_GpuTextureViewDimension = ["1d", "2d", "2d-array", "cube", "cube-array", "3d"];


    const __wbindgen_enum_GpuVertexFormat = ["uint8", "uint8x2", "uint8x4", "sint8", "sint8x2", "sint8x4", "unorm8", "unorm8x2", "unorm8x4", "snorm8", "snorm8x2", "snorm8x4", "uint16", "uint16x2", "uint16x4", "sint16", "sint16x2", "sint16x4", "unorm16", "unorm16x2", "unorm16x4", "snorm16", "snorm16x2", "snorm16x4", "float16", "float16x2", "float16x4", "float32", "float32x2", "float32x3", "float32x4", "uint32", "uint32x2", "uint32x3", "uint32x4", "sint32", "sint32x2", "sint32x3", "sint32x4", "unorm10-10-10-2", "unorm8x4-bgra"];


    const __wbindgen_enum_GpuVertexStepMode = ["vertex", "instance"];


    const __wbindgen_enum_ResizeObserverBoxOptions = ["border-box", "content-box", "device-pixel-content-box"];
    const WebHandleFinalization = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(ptr => wasm.__wbg_webhandle_free(ptr >>> 0, 1));

    function addToExternrefTable0(obj) {
        const idx = wasm.__externref_table_alloc();
        wasm.__wbindgen_externrefs.set(idx, obj);
        return idx;
    }

    const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(state => state.dtor(state.a, state.b));

    function debugString(val) {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debugString(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debugString(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches && builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
            return `${val.name}: ${val.message}\n${val.stack}`;
        }
        // TODO we could test for more things here, like `Set`s and `Map`s.
        return className;
    }

    function getArrayF32FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
    }

    function getArrayI16FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getInt16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
    }

    function getArrayI32FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getInt32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
    }

    function getArrayI8FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getInt8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
    }

    function getArrayU16FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getUint16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
    }

    function getArrayU32FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
    }

    function getArrayU8FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
    }

    let cachedDataViewMemory0 = null;
    function getDataViewMemory0() {
        if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
            cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
        }
        return cachedDataViewMemory0;
    }

    let cachedFloat32ArrayMemory0 = null;
    function getFloat32ArrayMemory0() {
        if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
            cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
        }
        return cachedFloat32ArrayMemory0;
    }

    let cachedInt16ArrayMemory0 = null;
    function getInt16ArrayMemory0() {
        if (cachedInt16ArrayMemory0 === null || cachedInt16ArrayMemory0.byteLength === 0) {
            cachedInt16ArrayMemory0 = new Int16Array(wasm.memory.buffer);
        }
        return cachedInt16ArrayMemory0;
    }

    let cachedInt32ArrayMemory0 = null;
    function getInt32ArrayMemory0() {
        if (cachedInt32ArrayMemory0 === null || cachedInt32ArrayMemory0.byteLength === 0) {
            cachedInt32ArrayMemory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachedInt32ArrayMemory0;
    }

    let cachedInt8ArrayMemory0 = null;
    function getInt8ArrayMemory0() {
        if (cachedInt8ArrayMemory0 === null || cachedInt8ArrayMemory0.byteLength === 0) {
            cachedInt8ArrayMemory0 = new Int8Array(wasm.memory.buffer);
        }
        return cachedInt8ArrayMemory0;
    }

    function getStringFromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return decodeText(ptr, len);
    }

    let cachedUint16ArrayMemory0 = null;
    function getUint16ArrayMemory0() {
        if (cachedUint16ArrayMemory0 === null || cachedUint16ArrayMemory0.byteLength === 0) {
            cachedUint16ArrayMemory0 = new Uint16Array(wasm.memory.buffer);
        }
        return cachedUint16ArrayMemory0;
    }

    let cachedUint32ArrayMemory0 = null;
    function getUint32ArrayMemory0() {
        if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
            cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
        }
        return cachedUint32ArrayMemory0;
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
                state.a = a;
                real._wbg_cb_unref();
            }
        };
        real._wbg_cb_unref = () => {
            if (--state.cnt === 0) {
                state.dtor(state.a, state.b);
                state.a = 0;
                CLOSURE_DTORS.unregister(state);
            }
        };
        CLOSURE_DTORS.register(real, state, state);
        return real;
    }

    function passStringToWasm0(arg, malloc, realloc) {
        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length, 1) >>> 0;
            getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len, 1) >>> 0;

        const mem = getUint8ArrayMemory0();

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
            ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
            const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
            ptr = realloc(ptr, len, offset, 1) >>> 0;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    function takeFromExternrefTable0(idx) {
        const value = wasm.__wbindgen_externrefs.get(idx);
        wasm.__externref_table_dealloc(idx);
        return value;
    }

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
    cachedTextDecoder.decode();
    function decodeText(ptr, len) {
        return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
    }

    const cachedTextEncoder = new TextEncoder();

    if (!('encodeInto' in cachedTextEncoder)) {
        cachedTextEncoder.encodeInto = function (arg, view) {
            const buf = cachedTextEncoder.encode(arg);
            view.set(buf);
            return {
                read: arg.length,
                written: buf.length
            };
        };
    }

    let WASM_VECTOR_LEN = 0;

    let wasmModule, wasm;
    function __wbg_finalize_init(instance, module) {
        wasm = instance.exports;
        wasmModule = module;
        cachedDataViewMemory0 = null;
        cachedFloat32ArrayMemory0 = null;
        cachedInt16ArrayMemory0 = null;
        cachedInt32ArrayMemory0 = null;
        cachedInt8ArrayMemory0 = null;
        cachedUint16ArrayMemory0 = null;
        cachedUint32ArrayMemory0 = null;
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

        if (module_or_path === undefined && script_src !== undefined) {
            module_or_path = script_src.replace(/\.js$/, "_bg.wasm");
        }
        const imports = __wbg_get_imports();

        if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
            module_or_path = fetch(module_or_path);
        }

        const { instance, module } = await __wbg_load(await module_or_path, imports);

        return __wbg_finalize_init(instance, module);
    }

    return Object.assign(__wbg_init, { initSync }, exports);
})({ __proto__: null });
