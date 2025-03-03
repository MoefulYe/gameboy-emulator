/* tslint:disable */
/* eslint-disable */
export type LoadCartResult = { status: "ok"; info: CartInfo } | { status: "error"; msg: string };

export interface CartInfo {
    title: string;
    cartType: string;
    romSize: number;
    ramSize: number;
    dest: string;
    publisher: string;
    version: number;
}

export interface EmulatorUpdateResult {
    cycles: ClockCycle;
    cpu: CPUStateDump;
    err: string | null;
}

export interface EmulatorUpdateInput {
    btns: number;
    cycles: ClockCycle;
    timestamp: number;
}

export interface EmulatorStepInput {
    btns: number;
    timestamp: number;
}


export type Byte = number;
export type Word = number;
export type DWord = number;
export type Addr = DWord;
export type OpCode = Word;
export type ClockCycle = number;


export interface CPUStateDump {
    ime: boolean;
    halted: boolean;
    a: Word;
    f: Word;
    b: Word;
    c: Word;
    d: Word;
    e: Word;
    h: Word;
    l: Word;
    af: DWord;
    bc: DWord;
    de: DWord;
    hl: DWord;
    pc: DWord;
    sp: DWord;
    zeroFlag: boolean;
    negativeFlag: boolean;
    halfFlag: boolean;
    carryFlag: boolean;
    inst: string;
    threeWordsAtPc: [Word, Word, Word];
}

export class WasmEmulator {
  free(): void;
  constructor(freq_scale: number, volume: number);
  static initLogger(): void;
  update(arg0: EmulatorUpdateInput): EmulatorUpdateResult;
  step(arg0: EmulatorStepInput): EmulatorUpdateResult;
  loadCart(rom: Uint8Array, timestamp: number): LoadCartResult;
  save(): Uint8Array | undefined;
  load(save: Uint8Array): boolean;
  reset(): void;
  setScreenCanvas(canvas: OffscreenCanvasRenderingContext2D): void;
  setTilesCanvas(canvas: OffscreenCanvasRenderingContext2D): void;
  setVolume(volume: number): void;
  setFreqScale(freq_scale: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmemulator_free: (a: number, b: number) => void;
  readonly wasmemulator_new: (a: number, b: number) => number;
  readonly wasmemulator_initLogger: () => void;
  readonly wasmemulator_update: (a: number, b: number) => number;
  readonly wasmemulator_step: (a: number, b: number) => number;
  readonly wasmemulator_loadCart: (a: number, b: number, c: number, d: number) => number;
  readonly wasmemulator_save: (a: number, b: number) => void;
  readonly wasmemulator_load: (a: number, b: number, c: number) => number;
  readonly wasmemulator_reset: (a: number) => void;
  readonly wasmemulator_setScreenCanvas: (a: number, b: number) => void;
  readonly wasmemulator_setTilesCanvas: (a: number, b: number) => void;
  readonly wasmemulator_setVolume: (a: number, b: number) => void;
  readonly wasmemulator_setFreqScale: (a: number, b: number) => void;
  readonly __wbindgen_export_0: (a: number) => void;
  readonly __wbindgen_export_1: (a: number, b: number) => number;
  readonly __wbindgen_export_2: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_export_3: (a: number, b: number, c: number) => void;
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
