<template>
    <el-drawer custom-class="v-demand-detail" v-model="show" title="需求详情" direction="rtl" size="50%"
        :before-close="handleClose" :show-close="false" :append-to-body="true">
        <template #header>
            <DrawerHeader title="需求详情" @close="handleClose" />
        </template>
        <div class="padding-all-md">
            <div class="font-size-md font-weight-bold text-title">{{ detail?.name }} <DemandStatus :demand="detail" /></div>
            <div class="font-size text-sub margin-v-md">{{ detail?.desc }}</div>
            <el-space class="margin-top" :size="10">
                <ActionItem type="demand" :value="detail?.demand_link" />
                <ActionItem v-if="detail?.ui_link" type="ui" :value="detail?.ui_link" />
                <ActionItem v-if="detail?.api_link" type="api" :value="detail?.api_link" />
                <ActionItem v-if="detail?.code_path" type="code" :value="detail?.code_path" />
                <ActionItem v-if="detail?.publish_link" type="publish" :value="detail?.publish_link" />
            </el-space>
            <div>
                <el-space class="margin-top" :size="10">
                    <LinkItem v-for="temp in detail?.extra_links" :key="temp.link" :closable="false" :linkItem="temp" />
                </el-space>
            </div>
        </div>
        <template #footer v-if="!onlyShow">
            <div class="action-footer padding-all-md flex-row">
                <el-button type="success" @click="handleDesktop">{{ winInfo ? '从桌面移除' : '放到桌面' }}</el-button>
                <EditorDemand is-edit :info="detail" @refresh="handleRefresh" />
                <el-button type="danger" @click="handleDelete">删除</el-button>
            </div>
        </template>
    </el-drawer>
</template>

<script setup lang="ts">
import { onMounted, ref, toRefs, watch } from 'vue';
import { DemandItemType, WinInfoValue } from '../common/types';
import { invoke } from '@tauri-apps/api/tauri';
import { WebviewWindow } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
const props = defineProps<{ show: boolean, detail: DemandItemType, onlyShow: boolean }>();
const { show, detail, onlyShow } = toRefs(props);
const emits = defineEmits(['close', 'refresh']);
const winInfo = ref<WinInfoValue | undefined>(undefined);

watch(show, async (val, oldVal) => {
    if (val) {
        await getWinInfo();
    }
});

onMounted(async () => {
    // 添加监听刷新数据
    listen('refresh-win', async () => {
        await getWinInfo();
    });
});

const getWinInfo = async () => {
    // 获取是否有小窗
    const win_list = <WinInfoValue[]>await invoke('get_wins');
    let list = win_list.filter(temp => temp.demand_id === detail.value.id);
    winInfo.value = list.length > 0 ? list[0] : undefined;
}

const handleClose = () => {
    emits('close');
}

const handleRefresh = () => {
    emits('refresh');
    handleClose();
};

const handleCloseDestopWin = async () => {
    // 关闭窗口
    const win = WebviewWindow.getByLabel(winInfo.value?.demand_id || '');
    console.log('获取的窗口为 ->', win);
    win && win.close();
    // 删除数据
    await invoke('delete_win', { id: winInfo.value?.demand_id });
    await getWinInfo();
}

const handleDesktop = async () => {
    console.log('handleDesktop', winInfo);
    if (winInfo.value) {
        await handleCloseDestopWin();
    } else {
        await invoke('open_demand', { id: detail.value.id });
    }
    await getWinInfo();
};

const handleDelete = async () => {
    await invoke('delete_demand', { id: detail.value.id });
    if (winInfo.value) {
        await handleCloseDestopWin();
    }
    emits('refresh');
    handleClose();
};

</script>

<style lang="less">
.v-demand-detail {
    --el-drawer-padding-primary: 0;

    .el-drawer__header {
        margin-bottom: 0;
    }

    .action-footer {
        border-top: @border-mini;
    }
}
</style>