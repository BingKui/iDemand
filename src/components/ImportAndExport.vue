<template>
    <div class="flex-row-end flex-item-one">
        <el-button type="primary" @click="importDataAction">导入数据</el-button>
        <el-button type="danger" @click="exportDataAction">导出数据</el-button>
    </div>
</template>

<script setup lang="ts">
import { } from 'vue';
import { open, save } from '@tauri-apps/api/dialog';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api/tauri';
import { DemandItemType } from '../common/types';
import { errorNotice, successNotice } from '../common/notice';
const importDataAction = async () => {
    const selected = await open({
        multiple: false,
        title: '选择导入的数据文件',
        filters: [{
            name: 'iDemandData',
            extensions: ['idemand']
        }]
    });
    console.log('选择的文件为 ->', selected);
    if (!selected) return;
    // Read the text file in the `$APPCONFIG/app.conf` path
    const contents = await readTextFile(selected as string);
    console.log('内容为 ->', contents);
    try {
        // 获取本地所有的数据
        const localList: DemandItemType[] = await invoke('get_demands') || [];
        const list = JSON.parse(contents);
        list.forEach(async (item: DemandItemType) => {
            const flag = localList.filter((temp: DemandItemType) => temp.name == item.name && temp.demand_link == temp.demand_link).length > 0;
            // 如果存在，不做处理
            // 不存在添加
            if (!flag) {
                await invoke('add_demand', {
                    item,
                })
            }
        });
        successNotice('导入成功！');
    } catch (error) {
        console.log('报错');
        errorNotice('导入错误，请重试！');
    }
}

const exportDataAction = async () => {
    // 获取所有的数据
    const list = await invoke('get_demands');
    console.log('获取的所有数据为 ->', list);
    // 选择导出的地址
    // Open a selection dialog for image files
    const selected = await save({
        title: '选择导出地址',
        filters: [{
            name: 'iDemandData',
            extensions: ['idemand']
        }]
    });
    console.log('地址为 ->', selected);
    if (!selected) return;
    try {
        // Read the text file in the `$APPCONFIG/app.conf` path
        await writeTextFile(selected as string, JSON.stringify(list));
        successNotice('导出成功成功！');
    } catch (error) {
        errorNotice('导出失败，请重试！');
    }
}
</script>

<style scoped>

</style>