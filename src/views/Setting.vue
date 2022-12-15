<template>
    <div class="v-setting-page padding-all-md">
        <el-row :gutter="15">
            <el-col :span="12">
                <SettingItem label="代码编辑器" tip="设置打开代码的编辑器。">
                    <div class="setting-shell flex-row-end">
                        <el-radio-group v-model="sysInfo.codeApp">
                            <el-radio v-for="item in CODE_APP_LIST" :key="item.value" :label="item.value" border>
                                <span class="flex-row-center" :title="item.text">
                                    <img :src="item.icon" width="30" height="30" alt="">
                                </span>
                            </el-radio>
                        </el-radio-group>
                    </div>
                </SettingItem>
            </el-col>
            <!-- 导出/导出数据 -->
            <el-col :span="12">
                <SettingItem label="导入/导出" tip="可以导入数据，也可以导出数据">
                    <ImportAndExport />
                </SettingItem>
            </el-col>
            <!-- 清空操作日志，会把报表数据清除 -->
            <!-- <el-col :span="12">
                <SettingItem label="每日提醒" tip="每日提醒待完成的任务信息。">
                    <el-switch v-model="isNotice" />
                    <el-time-select v-if="isNotice" class="setting-time margin-left" v-model="noticeTime" start="08:00"
                        step="00:05" end="22:00" placeholder="选择时间" @change="handleTimeChange" />
                </SettingItem>
            </el-col> -->
        </el-row>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { ElNotification } from 'element-plus'
import { emit } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri';
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process';
import { Check } from '@icon-park/vue-next';
import { AppSettingValue } from '../common/types';
import { getAppSettingData } from '../api/setting';
import { CODE_APP_LIST, APP_DEFAULT_SETTING } from '../constants/setting';
const codeApp = ref('vscode');
const isNotice = ref(false);
const isAuto = ref(false);
const noticeTime = ref('');

const sysInfo = ref<AppSettingValue>(APP_DEFAULT_SETTING);

onMounted(async () => {
    await getSystemSetting();
});

const getSystemSetting = async () => {
    const settingInfo: AppSettingValue = await getAppSettingData();
    console.log('settingInfo -> ', settingInfo);
    sysInfo.value = settingInfo;

};

const handleCheckUpdate = async () => {
    const update = await checkUpdate();
    if (update.shouldUpdate) {
        console.log(`Installing update ${update.manifest?.version}, ${update.manifest?.date}`);
        await installUpdate();
        await relaunch();
    }
};

const handleTimeChange = async (time: string) => {
    noticeTime.value = time;
    await changeSetting();
}

const changeSetting = async () => {
    const setting = {
        is_notice: isNotice.value,
        notice_time: noticeTime.value,
        auto_update: isAuto.value,
    };
    await emit('change-setting', setting);
    ElNotification({
        title: '修改成功',
        message: '修改设置成功！',
        type: 'success',
    })
    await getSystemSetting();
}

const handleAutoChange = async (value: boolean | any) => {
    console.log('是否开启 ->', value);
    isAuto.value = value;
    await changeSetting();
}

</script>

<style lang="less">
.v-setting-page {
    .el-radio__input {
        display: none;
    }

    .el-radio__label {
        padding-left: 0;
    }

    .el-radio {
        padding: 0;
        width: 40px;
        height: 40px;
        display: flex;
        align-items: center;
        justify-content: center;
        margin-right: 0;
        margin-left: @gap;

        &.is-checked {
            position: relative;
            background-color: var(--el-menu-hover-bg-color);
            // &::after,
            // &::before {
            //     content: '';
            //     position: absolute;
            //     color: @white;
            //     width: 0;
            //     height: 0;
            //     border-style: solid;
            //     border-width: 4px;
            //     border-color: @primary;
            // }

            // &::after {
            //     border-right-color: transparent;
            //     border-bottom-color: transparent;
            //     left: 0;
            //     top: 0;
            // }

            // &::before {
            //     border-left-color: transparent;
            //     border-top-color: transparent;
            //     bottom: 0;
            //     right: 0;
            // }
        }
    }

    .setting-shell {
        width: 100%;
    }

    .setting-time {
        width: 120px;
    }
}
</style>