<template>
    <el-card class="v-setting-item">
        <div class="setting-content flex-column-between">
            <div class="font-size-md font-weight-bold">{{ label }}</div>
            <div class="font-size-sm text-sub margin-top-sm">{{ tip }}</div>
            <div class="flex-row margin-top">
                <slot></slot>
            </div>
        </div>
    </el-card>
</template>

<script setup lang="ts">
import { toRefs } from "vue";

const props = defineProps<{ label: string, tip: string }>();
const { label, tip } = toRefs(props);
</script>

<style lang="less" scoped>
.v-setting-item {
    border-radius: @border-radius;
    user-select: none;
    cursor: default;
    .setting-content {
        min-height: 80px;
    }
}
</style>