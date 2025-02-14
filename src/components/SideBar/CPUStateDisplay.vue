<template>
  <div class="flex flex-col gap-2">
    <div>
      <label>regs:</label>
    </div>
    <div class="b-1 b-coolgray-2 p-2 rounded text-sm">
      <div>
        <label> AF: </label>
        <span>{{ toHex4(state.af) }}</span>
        <label> A: </label>
        <span>{{ toHex2(state.a) }}</span>
        <label> F: </label>
        <span>{{ toHex2(state.f) }}</span>
      </div>
      <div>
        <label> BC: </label>
        <span>{{ toHex4(state.bc) }}</span>
        <label> B: </label>
        <span>{{ toHex2(state.b) }}</span>
        <label> C: </label>
        <span>{{ toHex2(state.c) }}</span>
      </div>
      <div>
        <label> DE: </label>
        <span>{{ toHex4(state.de) }}</span>
        <label> D: </label>
        <span>{{ toHex2(state.d) }}</span>
        <label> E: </label>
        <span>{{ toHex2(state.e) }}</span>
      </div>
      <div>
        <label> HL: </label>
        <span>{{ toHex4(state.hl) }}</span>
        <label> H: </label>
        <span>{{ toHex2(state.h) }}</span>
        <label> L: </label>
        <span>{{ toHex2(state.l) }}</span>
      </div>
      <div>
        <label> PC: </label>
        <span>{{ toHex4(state.pc) }}</span>
        <label> SP: </label>
        <span>{{ toHex4(state.sp) }}</span>
      </div>
    </div>
    <div>
      <label>flags:</label>
    </div>
    <div class="b-1 b-coolgray-2 p-2 rounded text-sm">
      <label> Z: </label>
      <span> {{ toBin(state.zeroFlag) }}</span>
      <label> N: </label>
      <span> {{ toBin(state.negativeFlag) }}</span>
      <label> H: </label>
      <span> {{ toBin(state.halfFlag) }}</span>
      <label> C: </label>
      <span> {{ toBin(state.carryFlag) }}</span>
    </div>
    <div>
      <label>pc:</label>
    </div>
    <div class="b-1 b-coolgray-2 p-2 rounded text-sm">
      <div>
        <label>inst: </label>
        <span>{{ state.inst }}</span>
      </div>
      <div>
        <label>words: </label>
        <span>{{ toHex2(state.threeWordsAtPc[0]) }}</span>
        <span>{{ toHex2(state.threeWordsAtPc[1]) }}</span>
        <span>{{ toHex2(state.threeWordsAtPc[2]) }}</span>
      </div>
    </div>
    <div>
      <label>other:</label>
    </div>
    <div class="b-1 b-coolgray-2 p-2 rounded text-sm">
      <div>
        <label>ime: </label>
        <span>{{ toBin(state.ime) }}</span>
        <label> halted: </label>
        <span>{{ toBin(state.halted) }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'

const state = useEmulator().stat.cpu
</script>

<script lang="ts">
const toHex4 = (v: number) => v.toString(16).padStart(4, '0')
const toHex2 = (v: number) => v.toString(16).padStart(2, '0')
const toBin = (v: boolean) => (v ? 1 : 0)
</script>
