import { ElMessage } from 'element-plus'

export const infoTip = (message: string) => {
    ElMessage({
        message,
        type: 'info',
    })
}
export const successTip = (message: string) => {
    ElMessage({
        message,
        type: 'success',
    })
}
export const warnTip = (message: string) => {
    ElMessage({
        message,
        type: 'warning',
    })
}
export const errorTip = (message: string) => {
    ElMessage({
        message,
        type: 'error',
    })
}