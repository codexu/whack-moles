<template>
  <canvas ref="image" class="image"></canvas>
</template>

<script lang="ts" setup>
import { nextTick, ref, watch } from 'vue';

const props = defineProps<{
  colors: number[][],
}>()

const image = ref<HTMLCanvasElement>()

// 将颜色数组转换为图片
function colorsToImage() {
  const colors = props.colors
  if (image.value) {
    const canvas = image.value
    canvas.width = colors.length
    canvas.height = 1
    const ctx = canvas.getContext('2d')
    if (ctx) {
      const imageData = ctx.createImageData(canvas.width, canvas.height)
      for (let i = 0; i < colors.length; i++) {
        for (let j = 0; j < colors[i].length; j++) {
          const index = (i + j * canvas.width) * 4
          imageData.data[index] = colors[i][j]
          imageData.data[index + 1] = colors[i][j]
          imageData.data[index + 2] = colors[i][j]
          imageData.data[index + 3] = 255
        }
      }
      ctx.putImageData(imageData, 0, 0)
    }
  }
}

watch(() => props.colors,
  async() => {
    await nextTick()
    if (image.value) {
      colorsToImage()
    }
  },
  {
    immediate: true
  }
)
</script>

<style lang="scss" scoped>
.image {
  transform: scale(5);
}
</style>
