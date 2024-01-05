<template>
  <div class="home">
    <a-space>
      <a-button
        type="primary"
        size="small"
        @click="handleScanLoop"
        :icon="h(PlayCircleOutlined)"
      >扫描</a-button>
      <a-button
        type="primary"
        danger
        size="small"
        @click="handleStop"
        :icon="h(PauseCircleOutlined)"
      >终止</a-button>
    </a-space>
    <a-divider>设置</a-divider>
    <a-input-number size="small" v-model:value="scanInterval" addon-before="扫描间隔" addon-after="毫秒" />
    <a-divider>任务</a-divider>
    <a-collapse v-if="startX.length" size="small" accordion v-model:activeKey="activeKey">
      <a-collapse-panel
        :header="`任务${index + 1} x: ${startX[index] + endX[index] / 2} y: ${y[index]}`"
        v-for="(_item, index) in startX.length"
        :key="index"
      >
        <a-space direction="vertical">
          <a-input-number size="small" v-model:value="startX[index]" addon-before="起始 x" />
          <a-input-number size="small" v-model:value="endX[index]" addon-before="结束 x" />
          <a-input-number size="small" v-model:value="y[index]" addon-before="高度 y" />
        </a-space>
        <template #extra>
          <a-space>
            <CreateScreenshot size="small" :index="index" />
            <a-button size="small" :icon="h(CloseOutlined)" @click="deleteTask(index)"></a-button>
          </a-space>
        </template>
      </a-collapse-panel>
    </a-collapse>
    <a-space style="margin-top: 12px;">
      <a-button size="small" @click="addProject">新增任务</a-button>
      <a-button size="small" @click="reset">重置</a-button>
    </a-space>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { listen } from '@tauri-apps/api/event';
import { useStorage } from "@vueuse/core";
import CreateScreenshot from '../components/createScreenshot.vue';
import { ref, h } from "vue";
import { PlayCircleOutlined, PauseCircleOutlined } from '@ant-design/icons-vue'
import { register } from '@tauri-apps/api/globalShortcut';
import { getCurrent } from '@tauri-apps/api/window';
import { CloseOutlined } from '@ant-design/icons-vue';
// import ScreenImage from '../components/screenImage.vue';

const activeKey = ref('0')

const startX = useStorage<number[]>('startX', [0])
const endX = useStorage<number[]>('endX', [0])
const y = useStorage<number[]>('y', [0])

const colors = useStorage<number[][][]>('colors', [[]])
const scanInterval = useStorage<number>('scanInterval', 1000)

async function handleScanLoop() {
  startX.value.forEach((_item, index) => {
    setTimeout(() => {
      const data = {
        colors: colors.value[index],
        startX: startX.value[index],
        endX: endX.value[index],
        y: y.value[index],
        interval: scanInterval.value * startX.value.length,
      }
      invoke("scan_loop", data)
    }, index * scanInterval.value);
  })
}

listen<{ startX: number, endX: number, y: number, index: number}>("location", async (event) => {
  const index = event.payload.index;
  startX.value[index] = event.payload.startX
  endX.value[index] = event.payload.endX
  y.value[index] = event.payload.y
  colors.value[index] = await invoke('scan_colors', { 
    startX: startX.value[index],
    endX: endX.value[index],
    y: y.value[index] 
  })
  console.log(colors.value[index])
})

function addProject() {
  startX.value.push(0)
  endX.value.push(0)
  y.value.push(0)
  colors.value.push([])
}

function handleStop() {
  invoke('stop_scan')
}

register('CommandOrControl+Shift+D', () => {
  handleScanLoop()
  const win = getCurrent()
  win.hide()
})

register('CommandOrControl+Shift+X', () => {
  handleStop()
  const win = getCurrent()
  win.show()
})

function deleteTask(index: number) {
  startX.value.splice(index, 1)
  endX.value.splice(index, 1)
  y.value.splice(index, 1)
  colors.value.splice(index, 1)
}

function reset() {
  localStorage.clear()
  window.location.reload()
}
</script>

<style lang="scss" scoped>
.home {
  min-height: 100vh;
  background-color: #fff;
  padding: 16px;
}
</style>