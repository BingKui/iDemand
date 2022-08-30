<template>
    <el-popover placement="bottom" ref="popoverRef" popper-class="v-demand-status" trigger="click" :show-arrow="false" :offset="3">
        <template #reference>
            <el-tag class="pointer-element" effect="dark" size="small"
                :style="`--el-tag-border-color: ${statusInfo.color}`" :color="statusInfo.color">{{ statusInfo.text }}
            </el-tag>
        </template>
        <div class="demand-status-list flex-column">
            <el-tag class="margin-bottom-sm pointer-element" v-for="item in statusList" :key="item.value" effect="dark" size="small"
                :style="`--el-tag-border-color: ${item.color}`" :color="item.color"
                @click="() => handleChangeStatus(item)">
                {{ item.label }}</el-tag>
        </div>
    </el-popover>
</template>

<script setup lang="ts">
import { computed, ref, toRefs, unref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { DemandItemType } from '../common/types';
import { DEMAND_STATUS_LIST } from '../constants/demand';
import { emit } from '@tauri-apps/api/event';
const props = defineProps<{ demand: DemandItemType }>();
const { demand } = toRefs(props);
const emits = defineEmits(['change']);
const popoverRef = ref();

const statusList = computed(() => {
    return DEMAND_STATUS_LIST.filter(temp => demand.value.status !== temp.value);
});

const statusInfo = computed(() => {
    return DEMAND_STATUS_LIST.filter(temp => demand.value.status == temp.value)[0];
});

const handleChangeStatus = async (item: any) => {
    unref(popoverRef).delayHide?.();
    unref(popoverRef).popperRef?.delayHide?.();
    await invoke('update_status', { id: demand.value.id, status: item.value })
    emits('change');
    emit('refresh-win');
    emit('refresh-demand');
}
</script>

<style lang="less">

.v-demand-status {
    outline: none;
    width: 63px !important;
    min-width: 63px !important;
    --el-popover-padding: @gap-sm;
    padding: @gap-sm;
    padding-bottom: 0 !important;
    .demand-status-list {
        width: 53px;
    }

    &:focus,
    &:active {
        outline: none;
    }

    .el-dropdown-link {
        outline: none;

        &:focus,
        &:active {
            outline: none;
        }
    }

    .el-dropdown-menu {
        padding: 0;
    }

    .el-dropdown-menu__item {
        padding: @gap-sm;
    }
}
</style>