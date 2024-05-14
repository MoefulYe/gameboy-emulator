<template>
  <div id="emulator-main">
    <div class="wrapper">
      <div
        @click="power = !power"
        :class="{
          power: true,
          on: power
        }"
      ></div>
      <div class="gameboy">
        <div class="top">
          <div class="corner left"></div>
          <div class="top">
            <span>◁ OFF·ON ▷</span>
          </div>
          <div class="corner right"></div>
        </div>
        <div class="screen">
          <div class="top">
            <span>DOT MATRIX WITH STEREO SOUND</span>
          </div>
          <div class="bottom">
            <div class="battery">
              <div class="led on"></div>
              <span>BATTERY</span>
            </div>
            <div class="gamescreen">
              <span>Nintendo<sup>®</sup></span>
            </div>
          </div>
        </div>
        <div class="brand">
          <span>Nintendo</span>
          <span>GAME BOY</span>
          <sub>™</sub>
        </div>

        <div class="controls">
          <div class="cross">
            <div class="cursor up"></div>
            <div class="cursor left"></div>
            <div class="cursor center">
              <div class="circle"></div>
            </div>
            <div class="cursor right"></div>
            <div class="cursor down"></div>
          </div>
          <div class="buttons">
            <div class="button B" data-button="B"></div>
            <div class="button A" data-button="A"></div>
          </div>
        </div>

        <div class="speaker">
          <div class="band"></div>
          <div class="band"></div>
          <div class="band"></div>
          <div class="band"></div>
          <div class="band"></div>
          <div class="band"></div>
        </div>

        <div class="bottom">
          <div class="gamecontrols">
            <div class="gap"><div class="button select" data-button="SELECT"></div></div>
            <div class="gap"><div class="button start" data-button="START"></div></div>
          </div>
          <div class="phones">🎧PHONES</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { EmulatorState } from '@/emulator/state'
import { computed } from 'vue'
import { ref } from 'vue'
const emu = useEmulator()
const state = emu.useState()
const power = computed({
  get: () => state.value !== EmulatorState.Shutdown,
  set: (val) => (state.value = val ? EmulatorState.Running : EmulatorState.Shutdown)
})
</script>

<style scoped lang="scss">
#emulator-main {
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-items: center;
  align-items: center;
  user-select: none;
}

.wrapper {
  width: 375px;
  height: 625px;
  position: relative;
}

.gameboy {
  background: #d3ccd3 linear-gradient(#d3ccd3 95%, #bbb 98%);
  overflow: hidden;
  border-radius: 12px 12px 75px 12px;
  box-shadow:
    0 0 2px rgba(0, 0, 0, 0.5),
    0 0 2px rgba(0, 0, 0, 0.65) inset;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  position: relative;
  > .top {
    display: flex;
    padding-bottom: 5px;
    margin-bottom: 5px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    .corner {
      width: 25px;
      height: 20px;
      &.left {
        margin-right: 5px;
      }
      &.right {
        margin-left: 5px;
      }
    }
    .top {
      width: 100%;
    }

    > div {
      border-radius: 0 0 2px 2px;
      border: 1px solid rgba(0, 0, 0, 0.1);
      box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.2);
      span {
        font-family: Arial, sans-serif;
        font-size: 12px;
        box-shadow: 2px 2px 2px rgba(0, 0, 0, 0.5) inset;
        text-shadow: 2px 1px 2px rgba(0, 0, 0, 1);
        color: #eee;
        text-shadow: 1px 1px 1px rgba(0, 0, 0, 0.5);
        border-radius: 15px;
        margin: 0 6px;
        padding: 2px 5px;
        opacity: 0.25;
      }
    }
  }
}

.screen {
  background: #767189;
  width: 312.5px;
  box-shadow: 0 0 1px #514c65;
  border-radius: 10px 10px 35px 10px;
  height: 250px;
  margin: 0.1em auto;
  span {
    font-family: Arial, sans-serif;
    font-size: 10px;
    color: rgba(#fff, 0.9);
    text-shadow: 0px 0px 1px rgba(#eee, 0.4);
  }
  .top {
    margin: 0 15px;
    height: 30px;
    background: linear-gradient(
      transparent 10px,
      #7d1a4a 10px 12px,
      transparent 12px 16px,
      #35224e 16px 18px,
      transparent 18px
    );
    position: relative;
    span {
      padding: 0 8px;
      background: #767189;
      position: absolute;
      right: 30px;
      top: 8px;
    }
  }
}

.bottom {
  display: flex;
  .led {
    width: 10px;
    height: 10px;
    background: #4a4748;
    border-radius: 50%;
    margin: 6px;
    &.on {
      background: #d81e07;
      box-shadow: 0 0 5px #d81e077f;
    }

    .battery {
      padding: 0 10px;
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: flex-start;
    }
  }
}

.gamescreen {
  background: #9ca04c;
  width: 200px;
  height: 190px;
  box-shadow:
    5px 5px 10px rgba(0, 0, 0, 0.5) inset,
    -2px -2px 10px rgba(0, 0, 0, 0.25) inset;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  overflow: hidden;
  span {
    display: none;
    font-family: 'Press Start 2P';
    font-weight: bold;
    font-size: 18px;
    letter-spacing: -1px;
    color: #0f380f;
    sup {
      font-weight: normal;
      font-size: 12px;
    }
  }

  &.startup span {
    display: block;
    animation: startup 2s linear forwards;
    transform: translate(0, -25px);
  }
}

@keyframes startup {
  0% {
    transform: translate(0, -25px);
  }
  100% {
    transform: translate(0, 80px);
  }
}

.power {
  width: 30px;
  height: 15px;
  border-radius: 50%;
  background: linear-gradient(to right, #eee 10%, #d3ccd3 30% 70%, #eee 90%);
  box-shadow: 0 0 4px rgba(0, 0, 0, 0.5) inset;
  position: absolute;
  top: -7px;
  left: 50px;
  cursor: pointer;
  transition: left 0.2s ease-in-out;

  &.on {
    left: 75px;
  }
}
</style>
