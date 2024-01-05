<script setup lang="ts">
import { WebviewWindow, getCurrent } from '@tauri-apps/api/window';
import { h } from 'vue';
import { ScanOutlined } from '@ant-design/icons-vue';

const props = defineProps<{
  index: number
}>()

async function handleScreenshot() {
  // 创建截屏窗口
  const screenshot = new WebviewWindow("screenshot", {
    title: "screenshot",
    decorations: false,
    // 对应 views/screenshot.vue
    url: `/#/screenshot?index=${props.index}`,
    alwaysOnTop: true,
    transparent: true,
    hiddenTitle: true,
    maximized: true,
    visible: false,
    resizable: false,
    skipTaskbar: false,
  })

  const currentWindow = getCurrent();
  await currentWindow.hide();

  await screenshot.show()
}
</script>

<template>
  <a-button
    @click="handleScreenshot"
    :icon="h(ScanOutlined)"
  ></a-button>
</template>
