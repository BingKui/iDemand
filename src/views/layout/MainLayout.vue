<template>
    <div>
        <CloseOne class="close-icon" theme="filled" size="16" fill="#ed4014" :strokeWidth="2" @click="handleClose" />
        <el-container class="v-main-layout">
            <el-aside class="left-side" width="200px" data-tauri-drag-region>
                <MainMenu />
            </el-aside>
            <el-main class="right-container">
                <slot></slot>
            </el-main>
        </el-container>
    </div>
</template>

<script setup lang="ts">
import { CloseOne } from '@icon-park/vue-next';
import { WebviewWindow } from '@tauri-apps/api/window';
import hotkeys from 'hotkeys-js';
import { onMounted } from 'vue';
import MainMenu from './MainMenu.vue';

const handleClose = () => {
    WebviewWindow.getByLabel('main')?.hide();
};

onMounted(() => {
    // 展示
    WebviewWindow.getByLabel('main')?.show();
    // 关闭
    hotkeys('command+W', handleClose);
});
</script>

<style lang="less" scoped>
.close-icon {
    position: fixed;
    left: @gap;
    top: @gap;
    cursor: pointer;
}
.v-main-layout {
    height: 100vh;
    .left-side {
        background-color: @white;
    }
    .right-container {
        height: 100vh;
        --el-main-padding: 0;
    }
}
</style>