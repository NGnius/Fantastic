/* tslint:disable */
/* eslint-disable */
/**
* Initialize the front-end library
* @param {number} port
*/
export function init_usdpl(port: number): void;
/**
* Get the targeted plugin framework, or "any" if unknown
* @returns {string}
*/
export function target(): string;
/**
* Call a function on the back-end.
* Returns null (None) if this fails for any reason.
* @param {string} name
* @param {any[]} parameters
* @returns {Promise<any>}
*/
export function call_backend(name: string, parameters: any[]): Promise<any>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_usdpl: (a: number) => void;
  readonly target: (a: number) => void;
  readonly call_backend: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h2bd47cee569ae4c6: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__hfe1195d34914cc54: (a: number, b: number, c: number, d: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;


// USDPL customization
export function init_embedded();
