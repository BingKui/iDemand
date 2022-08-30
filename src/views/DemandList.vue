<template>
    <div class="v-demand-list-page">
        <div class="main-header flex-row padding-h-md">
            <!-- 筛选条件 -->
            <EditorDemand info="" @refresh="getDemandList" />
            <el-input class="margin-left-md" v-model="searchValue" @input="handleSearchValueChange" placeholder="输入名字或描述，进行搜索">
                <template #prefix>
                    <div class="search-prefix flex-row-center">
                        <Search theme="outline" size="16" fill="#333" :strokeWidth="2" />
                    </div>
                </template>
            </el-input>
        </div>
        <PerfectScrollbar class="list-container padding-all-md">
            <el-row :gutter="15">
                <el-col :span="12" v-for="item in demandFilterList" :key="item.id">
                    <DemandItem :demand="item" @detail="handleDetail" />
                </el-col>
            </el-row>
        </PerfectScrollbar>
        <DemandDetail :detail="detailItem" :show="showDetail" @close="handleDetailClose" @refresh="getDemandList" />
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import { AllApplication, HamburgerButton, SettingTwo, Search } from '@icon-park/vue-next';
import { PerfectScrollbar } from 'vue3-perfect-scrollbar';
import { invoke } from '@tauri-apps/api/tauri';
import { debounce } from 'lodash';
import { DemandItemType } from '../common/types';
import { listen } from '@tauri-apps/api/event';

const demandList: Ref<DemandItemType[]> = ref([]);
const demandFilterList: Ref<DemandItemType[]> = ref([]);
const detailItem: Ref<DemandItemType | undefined> = ref<DemandItemType>();
const showDetail = ref<boolean>(false);
const searchValue = ref('');

onMounted(async () => {
    await getDemandList();
    listen('refresh-demand', getDemandList);
});

const handleSearchValueChange = (val: string) => {
    demandFilterList.value = demandList.value.filter(item => {
        return item.name.indexOf(val) > -1 || item.desc.indexOf(val) > -1;
    })
};

const handleDetail = (item: DemandItemType) => {
    console.log(item);
    detailItem.value = item;
    showDetail.value = true;
}
const handleDetailClose = () => {
    showDetail.value = false;
}

const getDemandList = async () => {
    const list: DemandItemType[] = await invoke('get_demands');
    demandList.value = list;
    demandFilterList.value = list;
    console.log('获取的数据为 -> ', list);
};

</script>

<style lang="less" scoped>
.v-demand-list-page {
    width: 100%;
    height: 100vh;
    overflow: hidden;

    .list-container {
        height: calc(100vh - @headerHeight);
    }

    .main-header {
        cursor: default;
        border-bottom: @border-mini;
        height: @headerHeight;
        // background-color: @white;
        .search-prefix {
            line-height: 1;
            height: 100%;
        }
    }
}
</style>