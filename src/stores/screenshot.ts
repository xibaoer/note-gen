import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface ScreenshotListStatus {
  id: string
  tabId: number
  path: string
  keywords: string[]
  description: string;
  screenshotStatus: boolean
  screenshotProgress: string
}

export default defineStore('screenshotStore', () => {
  // 截图进度
  const screenshotList = ref<ScreenshotListStatus[]>([])

  function updateStatus(data: ScreenshotListStatus) {
    const item = screenshotList.value.find(item => item.id === data.id)
    if (item) {
      item.path = data.path
      item.keywords = data.keywords
      item.description = data.description
      item.screenshotProgress = data.screenshotProgress
      item.screenshotStatus = data.screenshotStatus
    }
  }

  function complete(id: string) {
    const index = screenshotList.value.findIndex(item => item.id === id)
    screenshotList.value.splice(index, 1)
  }

  return {
    screenshotList,
    updateStatus,
    complete
  }
})