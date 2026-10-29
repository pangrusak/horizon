/* tslint:disable */
/* eslint-disable */

export function present_test_frame(): Promise<void>;

export function probe_gpu(): Promise<string>;

export function start(): void;

export function start_renderer(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly present_test_frame: () => any;
    readonly probe_gpu: () => any;
    readonly start: () => void;
    readonly start_renderer: () => [number, number];
    readonly ring_core_0_17_14__bn_mul_mont: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___js_sys_5987fc9e5362308f___Array__web_sys_d4f5e9135230afbd___features__gen_ResizeObserver__ResizeObserver______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___js_sys_5987fc9e5362308f___Function_fn_wasm_bindgen_99b6d9e459273f30___JsValue_____wasm_bindgen_99b6d9e459273f30___sys__Undefined___js_sys_5987fc9e5362308f___Function_fn_wasm_bindgen_99b6d9e459273f30___JsValue_____wasm_bindgen_99b6d9e459273f30___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___wasm_bindgen_99b6d9e459273f30___JsValue__core_37ee0ce0c4cd57d5___result__Result_____wasm_bindgen_99b6d9e459273f30___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___wasm_bindgen_99b6d9e459273f30___sys__JsNullable_wgpu_b2f491b0e21f7f39___backend__webgpu__webgpu_sys__gen_GpuAdapter__GpuAdapter___core_37ee0ce0c4cd57d5___result__Result_____wasm_bindgen_99b6d9e459273f30___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___wasm_bindgen_99b6d9e459273f30___sys__JsNullable_wgpu_b2f491b0e21f7f39___backend__webgpu__webgpu_sys__gen_GpuError__GpuError___core_37ee0ce0c4cd57d5___result__Result_____wasm_bindgen_99b6d9e459273f30___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___wasm_bindgen_99b6d9e459273f30___sys__Undefined__core_37ee0ce0c4cd57d5___result__Result_____wasm_bindgen_99b6d9e459273f30___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___wgpu_b2f491b0e21f7f39___backend__webgpu__webgpu_sys__gen_GpuDevice__GpuDevice__core_37ee0ce0c4cd57d5___result__Result_____wasm_bindgen_99b6d9e459273f30___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___js_sys_5987fc9e5362308f___Array______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___wasm_bindgen_99b6d9e459273f30___JsValue______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___web_sys_d4f5e9135230afbd___features__gen_Event__Event______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___web_sys_d4f5e9135230afbd___features__gen_FocusEvent__FocusEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___web_sys_d4f5e9135230afbd___features__gen_KeyboardEvent__KeyboardEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___web_sys_d4f5e9135230afbd___features__gen_PageTransitionEvent__PageTransitionEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___web_sys_d4f5e9135230afbd___features__gen_PointerEvent__PointerEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___web_sys_d4f5e9135230afbd___features__gen_WheelEvent__WheelEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___wgpu_b2f491b0e21f7f39___backend__webgpu__webgpu_sys__gen_GpuDeviceLostInfo__GpuDeviceLostInfo______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___wgpu_b2f491b0e21f7f39___backend__webgpu__webgpu_sys__gen_GpuUncapturedErrorEvent__GpuUncapturedErrorEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke___bool__true_: (a: number, b: number) => number;
    readonly wasm_bindgen_99b6d9e459273f30___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
