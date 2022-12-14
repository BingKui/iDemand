<template>
    <el-button v-if="haveButton" size="small" type="primary" @click="handleCheckUpdate">检查更新</el-button>
    <el-dialog v-model="updateVisible" title="版本更新" width="40%" center :close-on-click-modal="false">
        <div class="font-size text-title">版本：<span class="font-weight-bold">v{{ updateInfo?.version }}</span></div>
        <div class="font-size text-title margin-top">发布时间：<span class="font-weight-bold">{{ updateInfo?.date }}</span>
        </div>
        <div class="font-size text-title margin-top">更新内容：<span class="font-weight-bold">{{ updateInfo?.body }}</span>
        </div>
        <template #footer>
            <span class="dialog-footer">
                <el-button @click="updateVisible = false">取消</el-button>
                <el-button type="primary" @click="updateAction">安装</el-button>
            </span>
        </template>
    </el-dialog>
    <el-dialog v-model="installVisible" title="重启提醒" width="40%" center :close-on-click-modal="false">
        <div class="font-size text-title">新版本已经下载完成，是否重启更新？</div>
        <template #footer>
            <span class="dialog-footer">
                <el-button @click="installVisible = false">取消</el-button>
                <el-button type="primary" @click="installAction">重启</el-button>
            </span>
        </template>
    </el-dialog>
</template>

<script setup lang="ts">
import { onMounted, ref, toRefs } from 'vue';
import { checkUpdate, installUpdate, UpdateManifest } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process';
import { listen } from '@tauri-apps/api/event'
import { ElNotification } from 'element-plus';

const props = defineProps<{ haveButton: boolean, isAuto: boolean }>();
const { haveButton, isAuto } = toRefs(props);

const updateVisible = ref(false);
const installVisible = ref(false);
const updateInfo = ref<UpdateManifest | undefined>();

onMounted(async () => {
    console.log('是否自动更新 -> ', isAuto.value);
    if (isAuto.value) {
        await handleCheckUpdate();
    }
    listen('tauri://update-status', (res: { payload: { status: string } }) => {
        const { status } = res.payload;
        switch(status) {
            case 'ERROR':
                ElNotification.closeAll();
                ElNotification({
                    title: '更新出错',
                    message: '更新出错，请稍后重试！',
                    type: 'error',
                    position: 'bottom-right',
                });
                break;
            case 'PENDING':
            case 'DOWNLOAD':
                ElNotification.closeAll();
                ElNotification({
                    title: '更新中',
                    message: '正在下载更新...',
                    type: 'info',
                    showClose: false,
                    position: 'bottom-right',
                    duration: 0,
                });
                break;
            case 'DONE':
                ElNotification.closeAll();
                installVisible.value = true;
                break;
                
        }
    })
});

const handleCheckUpdate = async () => {
    // 检查更新
    try {
        const { shouldUpdate, manifest } = await checkUpdate();
        if (shouldUpdate) {
            updateVisible.value = true;
            updateInfo.value = manifest;
        }
    } catch (error) {
        console.error('错误信息 ->', error)
    }
}
const updateAction = async () => {
    updateVisible.value = false;
    await installUpdate();
};
const installAction = async () => {
    await relaunch()
}
</script>

<style lang="less" scoped>
</style>