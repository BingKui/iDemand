<template>
    <el-card class="v-setting-item">
        <div class="setting-content flex-column-between">
            <div class="font-size font-weight-bold">{{ label }}</div>
            <div class="font-size-sm text-content">{{ tip }}</div>
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
    border-radius: 0;
    user-select: none;
    cursor: default;
    .setting-content {
        height: 80px;
    }
}
</style>