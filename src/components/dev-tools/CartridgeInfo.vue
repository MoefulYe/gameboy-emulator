<template>
  <p v-if="cartridgeInfo !== undefined">
    <strong>{{ t('title') }}:</strong> {{ cartridgeInfo.title }}<br />
    <strong>{{ t('cartType') }}:</strong> {{ cartridgeInfo.cartType }}<br />
    <strong>{{ t('romSize') }}:</strong> {{ cartridgeInfo.romSize }}<br />
    <strong>{{ t('ramSize') }}:</strong> {{ cartridgeInfo.ramSize }}<br />
    <strong>{{ t('dest') }}:</strong> {{ cartridgeInfo.dest }}<br />
    <strong>{{ t('publisher') }}:</strong> {{ cartridgeInfo.publisher }}<br />
    <strong>{{ t('version') }}:</strong> {{ cartridgeInfo.version }}<br />
  </p>
  <p v-else>
    {{ t('nocart') }}
  </p>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { useI18n } from 'vue-i18n'

const { cartridgeInfo } = useEmulator()
const { t } = useI18n()
</script>

<i18n>
{
  "en": {
    "title": "Title",
    "cartType": "Cartridge Type",
    "romSize": "ROM Size",
    "ramSize": "RAM Size",
    "dest": "Destination",
    "publisher": "Publisher",
    "version": "Version",
    "nocart": "No cartridge inserted"
  },
  "zh": {
    "title": "标题",
    "cartType": "卡带类型",
    "romSize": "ROM 大小",
    "ramSize": "RAM 大小",
    "dest": "发行地区",
    "publisher": "发行商",
    "version": "版本",
    "nocart": "未插入卡带"
  }
}
</i18n>
