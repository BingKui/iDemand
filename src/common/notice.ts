import { ElNotification } from 'element-plus'

export const successNotice = (message: string, title?: string) => {
  ElNotification({
    title: title || '成功',
    message,
    type: 'success',
  })
}

export const warnNotice = (message: string, title?: string) => {
  ElNotification({
    title: title || '警告',
    message,
    type: 'warning',
  })
}

export const infoNotice = (message: string, title?: string) => {
  ElNotification({
    title: title || '提示',
    message,
    type: 'info',
  })
}

export const errorNotice = (message: string, title?: string) => {
  ElNotification({
    title: title || '错误',
    message,
    type: 'error',
  })
}