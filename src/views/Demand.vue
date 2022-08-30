
<template>
    <div class="v-demand-page" data-tauri-drag-region>

        <div class="background-element">
            <css-doodle>
                :doodle {
                @grid: 2x8 / 100vmin;
                }
                @place-cell: center;
                width: @rand(10vmin, 99vmin);
                height: @rand(10vmin, 99vmin);
                transform: translate(@rand(-200%, 300%), @rand(-100%, 200%)) scale(@rand(.8, 1.8)) skew(@rand(45deg));
                clip-path: polygon(
                @r(0, 100%) @r(0, 100%),
                @r(30%, 60%) @r(0%, 30%),
                @r(60%, 100%) @r(0%, 50%),
                @r(60%, 100%) @r(50%, 100%),
                @r(30%, 60%) @r(60%, 100%),
                @r(0, 30%) @r(60%, 100%)
                );
                background: @pick(#f44336, #e91e63, #9c27b0, #673ab7, #3f51b5, #60569e, #e6437d, #ebbf4d, #00bcd4,
                #03a9f4,
                #2196f3, #009688, #5ee463, #f8e645, #ffc107, #ff5722, #43f8bf);
                opacity: @rand(.3, .8);
                animation: colorChange @rand(2.1s, 6.1s) infinite @rand(-.3s, -2.5s) linear alternate;

                @keyframes colorChange {
                    100% {
                        left: 0;
                        top: 0;
                        filter: hue-rotate(360deg);
                    }
                }
            </css-doodle>
        </div>
        <div class="demand-content padding-all-md" data-tauri-drag-region>
            <div data-tauri-drag-region class="demand-name margin-bottom-md flex-row">
                <span data-tauri-drag-region :title="demandInfo?.name"
                    class="flex-item-one font-size-lg font-weight-bold text-title text-overflow">{{ demandInfo?.name }}</span>
                <DemandStatus v-if="demandInfo" :demand="demandInfo" @change="getDemandInfo" />
            </div>
            <div class="demand-desc font-size margin-bottom-md" data-tauri-drag-region>{{ demandInfo?.desc }}</div>
            <div class="action-list margin-bottom-md" data-tauri-drag-region>
                <el-space :size="10" data-tauri-drag-region>
                    <ActionItem v-if="demandInfo?.demand_link" type="demand" :value="demandInfo?.demand_link" />
                    <ActionItem v-if="demandInfo?.ui_link" type="ui" :value="demandInfo?.ui_link" />
                    <ActionItem v-if="demandInfo?.api_link" type="api" :value="demandInfo?.api_link" />
                    <ActionItem v-if="demandInfo?.code_path" type="code" :value="demandInfo?.code_path" />
                    <ActionItem v-if="demandInfo?.publish_link" type="publish" :value="demandInfo?.publish_link" />
                </el-space>
            </div>
            <div class="extra-action" data-tauri-drag-region>
                <el-space class="margin-top-sm" :size="10" wrap>
                    <LinkItem v-for="temp in demandInfo?.extra_links" :key="temp.link" :closable="false"
                        :linkItem="temp" />
                </el-space>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
import hotkeys from "hotkeys-js";
import { debounce } from "lodash";
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { DemandItemType } from "../common/types";
const router = useRoute();
const { demandId } = router.params;
const demandInfo = ref<DemandItemType>();

const getDemandInfo = async () => {
    const item: DemandItemType = await invoke('get_demand', { id: demandId });
    console.log('获取的需求为 -> ', item);
    demandInfo.value = item;
};

onMounted(async () => {
    await getDemandInfo();
    const demandWin = WebviewWindow.getByLabel(demandId as string);
    await demandWin?.show();
    // 绑定移动事件
    appWindow.listen('tauri://move', debounce(async ({ event, payload }) => {
        console.log('move ->', payload);
        await invoke('update_position', { id: demandId, x: payload.x, y: payload.y });
        // await refreshData();
    }, 500));
    hotkeys('command+W', () => {
        demandWin?.close();
        // 关闭
        invoke('delete_win', { id: demandId });
        emit('refresh-win');
    });
    listen('refresh-demand', getDemandInfo);
    // appWindow.listen('tauri://close-requested', async () => {
    //     // 关闭
    //     await invoke('delete_win', { id: demandId });
    //     emit('refresh-win');
    // });
});
</script>

<style lang="less" scoped>
.v-demand-page {
    // border-radius: 20px;
    // border: 2px solid @gray;
    background-color: @background-color;
    user-select: none;

    .background-element {
        height: 100vh;
        width: 100vh;
        z-index: 5;

        &::before {
            content: "";
            position: fixed;
            top: 0;
            left: 0;
            bottom: 0;
            right: 0;
            backdrop-filter: blur(30px);
            z-index: 1;
        }
    }

    .demand-content {
        position: fixed;
        top: 0;
        left: 0;
        bottom: 0;
        right: 0;
        height: 100vh;
        z-index: 10;
        background-color: transparent;
    }

    .demand-name {
        cursor: default;
        height: 30px;
    }

    .demand-desc {
        height: 40px;
        .text-overflow-line(2);
    }
}
</style>