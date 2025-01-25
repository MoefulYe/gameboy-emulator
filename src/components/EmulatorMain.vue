<template>
  <div ref="container" class="pt-4">
    <div id="gameboy" ref="gameboy">
      <div class="reflex">
        <canvas id="screen" ref="screen" width="160" height="144"></canvas>
        <span class="diod"></span>
      </div>
      <ul class="buttons">
        <li @pointerdown="emu.down(EmulatorButton.A)" @pointerup="emu.up(EmulatorButton.A)">
          <span>A</span>
        </li>
        <li @pointerdown="emu.down(EmulatorButton.B)" @pointerup="emu.up(EmulatorButton.B)">
          <span>B</span>
        </li>
      </ul>
      <ul id="gamecontrol">
        <li
          @pointerdown="emu.down(EmulatorButton.Select)"
          @pointerup="emu.up(EmulatorButton.Select)"
        >
          <span>SELECT</span>
        </li>
        <li @pointerdown="emu.down(EmulatorButton.Start)" @pointerup="emu.up(EmulatorButton.Start)">
          <span>START</span>
        </li>
      </ul>
      <ul id="speaker">
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
      </ul>
      <div class="stick">
        <ul
          class="left"
          @pointerdown="emu.down(EmulatorButton.Left)"
          @pointerup="emu.up(EmulatorButton.Left)"
        >
          <li></li>
          <li></li>
          <li></li>
          <li></li>
          <li></li>
        </ul>
        <ul
          class="right"
          @pointerdown="emu.down(EmulatorButton.Right)"
          @pointerup="emu.up(EmulatorButton.Right)"
        >
          <li></li>
          <li></li>
          <li></li>
          <li></li>
          <li></li>
        </ul>
        <ul class="circle">
          <li></li>
        </ul>
        <ul
          class="top"
          @pointerdown="emu.down(EmulatorButton.Up)"
          @pointerup="emu.up(EmulatorButton.Up)"
        >
          <li></li>
          <li></li>
          <li></li>
          <li></li>
          <li></li>
        </ul>
        <ul
          class="bottom sm:size-12 md:me-4 lg:text-2xl"
          @pointerdown="emu.down(EmulatorButton.Down)"
          @pointerup="emu.up(EmulatorButton.Down)"
        >
          <li></li>
          <li></li>
          <li></li>
          <li></li>
          <li></li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { EmulatorButton, useEmulator } from '@/emulator'
import { useElementWidth } from '@/utils/hooks'
import { useTemplateRef } from 'vue'
const gameboy = useTemplateRef('gameboy')
const container = useTemplateRef('container')
const screen = useTemplateRef('screen')

const emu = useEmulator()
emu.useCanvas(screen)
useElementWidth(container, (w) => {
  const s = (w * 0.9) / 265
  const scale = s > 3 ? 3 : s
  gameboy.value!.style.transform = `scale(${scale})`
})
</script>

<style scoped lang="scss">
#gameboy {
  margin-inline: auto;
  transform-origin: top center;
  user-select: none;
  position: relative;
  width: 265px;
  height: 433px;
  border: 2px solid rgb(153, 153, 153);
  border-radius: 10px 10px 77px 10px;
  background-image: linear-gradient(#adb6b3, #bec2c1 50%);
  box-shadow:
    -5px 6px 20px rgba(0, 0, 0, 0.37),
    -21px 17px 65px rgba(0, 0, 0, 0.46),
    inset -3px 0 4px #8e918e,
    inset -7px 0 7px #dfdfdf,
    inset 1px 0 3px #66696a,
    inset 3px 0 10px #616465,
    inset 0 3px 7px #919496,
    inset 0 4px 10px white;
  font-family: Arial, Helvetica, sans-serif;
  transform-origin: top center;
}

.reflex {
  position: relative;
  top: 36px;
  left: 15px;
  overflow: hidden;
  width: 232px;
  height: 177px;
  border-top: 1px solid #3d4844;
  border-right: 1px solid #3d4844;
  border-bottom: 1px solid #1d2120;
  border-left: 2px solid rgba(29, 33, 32, 0.57);
  border-radius: 10px 10px 53px 10px;
  background: #485055;
  box-shadow:
    inset 0 1px 1px #aab3b0,
    0 1px 0 #5b5e63,
    0 2px 0 #ebebeb,
    1px 0 2px #d3d3d3,
    -1px 0 2px #cfcfcf,
    inset -1px 0 2px rgba(255, 255, 255, 0.44);
}

#screen {
  position: absolute;
  top: 21px;
  left: 43px;
  z-index: 2;
  width: 147px;
  height: 134px;
  border-top: 1px solid #394b21;
  border-right: 1px solid #36472f;
  background: #5f953d;
  box-shadow: inset -2px 5px 10px #2a421a;
  opacity: 0.5;
}

.diod {
  position: absolute;
  top: 61px;
  left: 12px;
  display: block;
  width: 9px;
  height: 9px;
  border: 1px solid #ac774d;
  border-radius: 50px;
  background: #e4b443;
  box-shadow:
    0 0 5px #ac774d,
    -1px 1px 1px rgba(83, 53, 41, 0.99),
    inset 0 1px 2px #fcb251;
}

.buttons {
  position: absolute;
  top: 242px;
  left: 179px;
  padding: 6px;
  border-radius: 50px;
  box-shadow:
    1px 1px 5px rgba(255, 255, 255, 0.21),
    -1px -1px 0px #babdbc,
    inset 0 12px 22px rgba(0, 0, 0, 0.12);
  transform: rotate(64deg);

  li {
    transition: ease-in-out 0.05s;
    width: 30px;
    height: 30px;
    border: 1px solid rgba(68, 32, 54, 1);
    border-radius: 100%;
    background-image: -webkit-linear-gradient(6deg, rgba(133, 50, 108, 0.61), #6d1851 89%);
    background-image: linear-gradient(6deg, rgba(133, 50, 108, 0.61), #6d1851 89%);
    box-shadow:
      inset 1px 2px 4px #a87493,
      -1px -1px 2px #000000,
      3px 6px 7px rgba(125, 136, 134, 0.79);
    &:active {
      transition: ease-in-out 0.3s;
      box-shadow:
        inset 0px 0px 0px #a87493,
        -1px -1px 2px #000000,
        0px 0px 0px rgba(125, 136, 134, 0.79);
    }

    &:hover {
      cursor: pointer;
    }

    &:first-child {
      margin-bottom: 17px;
    }

    span {
      transform: rotate(-90deg) translate(-4px, 40px);
      font-family: Arial, Helvetica, sans-serif;
      font-size: 12px;
      position: absolute;
      color: #302058;
    }
  }
}

#gamecontrol {
  position: absolute;
  top: 348px;
  left: 103px;
  transform: rotate(64deg);

  li {
    position: absolute;
    width: 9px;
    height: 34px;
    border-radius: 50px;
    background-image: linear-gradient(269deg, #6f7875, #c3c2c8 50%);
    box-shadow:
      1px 1px 1px rgba(226, 226, 226, 0.97),
      -1px -1px 2px #767a79,
      inset 1px 1px 1px #a3aca9;
    &:last-child {
      position: absolute;
      bottom: 7px;
      left: 20px;
    }

    &:hover {
      cursor: pointer;
    }

    span {
      position: absolute;
      transform: rotate(-90deg) translate(-12px, 8px);
      font-size: 8px;
      color: #302058;
    }
  }
}

#speaker {
  position: absolute;
  top: 344px;
  left: 174px;
  transform: rotate(64deg);

  li {
    margin-bottom: 8px;
    width: 42px;
    height: 6px;
    border-radius: 10px;
    box-shadow:
      1px 1px 1px rgba(255, 255, 255, 0.7),
      inset 2px 1px 3px #5b5a53,
      inset 1px -1px black;
  }
}

.stick {
  position: absolute;
  top: 252px;
  left: 14px;
  width: 93px;
  height: 93px;
  border-radius: 50%;
  box-shadow:
    inset -7px 10px 22px #c7cbca,
    inset 7px -10px 22px #b8b8b8;
}

.top {
  position: absolute;
  top: 15px;
  left: 34px;
  width: 24px;
  height: 21px;
  border-radius: 2px 2px 0 0;
  background: #282c2b;
  background-image: linear-gradient(90deg, #282c2b, #9b9b9b 119%);
  box-shadow:
    1px -1px 1px rgb(53, 53, 53),
    -4px 2px 7px rgba(96, 99, 98, 0.64);
  cursor: pointer;

  li {
    margin: 2px;
    width: 20px;
    height: 2px;
    border-radius: 2px;
    background: #353837;
    box-shadow: inset 0 -1px 1px #494c4b;
  }
}

.left {
  position: absolute;
  top: 36px;
  left: 56px;
  width: 24px;
  height: 22px;
  border-radius: 2px 2px 0px 0;
  background: #282c2b;
  background-image: linear-gradient(303deg, #282c2b, #9b9b9b 111%);
  box-shadow: -1px -1px 1px rgb(53, 53, 53);
  transform: rotate(90deg);
  cursor: pointer;

  li {
    margin: 2px;
    width: 20px;
    height: 2px;
    border-radius: 2px;
    background: #353837;
    box-shadow: inset 0 -1px 1px #494c4b;
  }
}

.bottom {
  position: absolute;
  top: 58px;
  left: 34px;
  width: 24px;
  height: 21px;
  border-radius: 0 0 2px 2px;
  background: #282c2b;
  background-image: linear-gradient(55deg, #131716, #6a706f 147%);
  box-shadow: -5px 0px 6px #606362;
  cursor: pointer;

  li {
    margin: 2px;
    width: 20px;
    height: 2px;
    border-radius: 2px;
    background: #050908;
    box-shadow: inset 0 -1px 1px #343839;
  }
}

.right {
  position: absolute;
  top: 36px;
  left: 12px;
  z-index: 2;
  width: 24px;
  height: 22px;
  border-radius: 0px 0px 2px 2px;
  background: #282c2b;
  background-image: linear-gradient(290deg, #131716, #6a706f 173%);
  box-shadow: 2px 3px 7px rgb(53, 53, 53);
  transform: rotate(90deg);
  cursor: pointer;

  li {
    margin: 2px;
    width: 20px;
    height: 2px;
    border-radius: 2px;
    background: #050908;
    box-shadow: inset 0 -1px 1px #343839;
  }
}

.circle {
  position: absolute;
  top: 36px;
  left: 35px;
  z-index: 5;
  width: 22px;
  height: 22px;
  background-image: linear-gradient(-134deg, #656666, #2e3231 60%);

  li {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 16px;
    height: 16px;
    border: 1px solid rgba(39, 42, 41, 0.91);
    border-radius: 16px;
    background: #4b4e4d;
    box-shadow:
      inset 2px -1px 8px #868787,
      inset 3px -1px 10px black;
  }
}

.ds {
  position: relative;
  z-index: 10;
  top: 300px;
  left: 580px;
  color: white;
  font-size: 15pxl;
}
</style>
