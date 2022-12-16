<template>
    <el-card class="v-demand-item margin-bottom-md" shadow="hover">
        <div class="flex-row">
            <span class="demand-name flex-item-one font-size-md text-title font-weight-bold text-overflow" @click="handleOpenDetail">{{ demand.name }}</span>
            <DemandStatus :demand="demand" @change="handleChange" />
        </div>
        <div class="demand-desc font-size text-sub margin-v text-overflow-two">{{ demand.desc }}</div>
        <el-space :size="10">
            <ActionItem v-if="demand.demand_link" type="demand" :value="demand.demand_link" />
            <ActionItem v-if="demand.ui_link" type="ui" :value="demand.ui_link" />
            <ActionItem v-if="demand.api_link" type="api" :value="demand.api_link" />
            <!-- <ActionItem v-if="demand.code_path" type="code" :value="demand.code_path" /> -->
            <ActionItem v-if="demand.publish_link" type="publish" :value="demand.publish_link" />
            <div v-if="demand.publish_date" class="font-size text-warning">{{ publishDateTip }}</div>
        </el-space>
        <!-- <el-space class="margin-top-sm" :size="10" wrap>
            <LinkItem v-for="temp in demand.extra_links" :key="temp.link" :closable="false" :linkItem="temp" />
        </el-space> -->
    </el-card>
</template>

<script setup lang="ts">
import dayjs from 'dayjs';
import { computed, toRefs } from 'vue';
import { DemandItemType } from '../common/types';
import { DEMAND_STATUS_LIST } from '../constants/demand';

const props = defineProps<{ demand: DemandItemType }>();
const { demand } = toRefs(props);
const emits = defineEmits(['detail', 'refresh']);

const statusInfo = computed(() => {
    return DEMAND_STATUS_LIST.filter(temp => demand.value.status == temp.value)[0];
});

const publishDateTip = computed(() => {
    if (!demand.value.publish_date) return '';
    const now = dayjs();
    const time = dayjs(demand.value.publish_date);
    const day = time.diff(now, 'day', false);
    if (day < 0) return '';
    if (day > 1) return `${Math.floor(day)}天后上线`;
    return '今天上线';
});

const handleOpenDetail = () => {
    emits('detail', demand.value);
}

const handleChange = () => {
    emits('refresh');
}
</script>

<style lang="less" scoped>
.v-demand-item {
    user-select: none;
    border-radius: @border-radius;
    .demand-name {
        cursor: pointer;
        &:hover {
            color: @primary;
        }
    }
    .demand-desc {
        height: 45px;
    }
}
</style>