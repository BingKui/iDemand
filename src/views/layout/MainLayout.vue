<template>
    <div>
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
import { WebviewWindow } from '@tauri-apps/api/window';
import { onMounted } from 'vue';
import MainMenu from './MainMenu.vue';

onMounted(() => {
    // 展示
    WebviewWindow.getByLabel('main')?.show();
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
        border-radius: @border-radius;
        box-shadow: var(--el-box-shadow-light);
    }
    .right-container {
        height: 100vh;
        --el-main-padding: 0;
    }
}
</style>