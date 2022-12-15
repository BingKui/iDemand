<template>
    <div class="v-demand-status-list">
        <div class="list-header flex-row margin-all-md">
            <!-- 筛选条件 -->
            <span  v-if="haveAdd" class="margin-right"><EditorDemand info="" @refresh="getDemandList" /></span>
            <div class="input-container">
                <el-input v-model="searchValue" size="large" @input="handleSearchValueChange" placeholder="输入名字或描述，进行搜索">
                    <template #prefix>
                        <div class="search-prefix flex-row-center">
                            <Search theme="outline" size="16" fill="#333" :strokeWidth="2" />
                        </div>
                    </template>
                </el-input>
            </div>
        </div>
        <PerfectScrollbar class="list-container padding-all-md">
            <el-row :gutter="15" v-if="demandFilterList.length > 0">
                <el-col :span="12" v-for="item in demandFilterList" :key="item.id">
                    <DemandItem :demand="item" @detail="handleDetail" @refresh="getDemandList" />
                </el-col>
            </el-row>
            <el-empty v-else :image-size="200" description="暂无需求" />
        </PerfectScrollbar>
        <DemandDetail :detail="detailItem" :show="showDetail" :onlyShow="status == 4" @close="handleDetailClose" @refresh="getDemandList" />
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, Ref, toRefs } from 'vue';
import { AllApplication, HamburgerButton, SettingTwo, Search } from '@icon-park/vue-next';
import { PerfectScrollbar } from 'vue3-perfect-scrollbar';
import { invoke } from '@tauri-apps/api/tauri';
import { debounce } from 'lodash';
import { DemandItemType } from '../common/types';

const demandList: Ref<DemandItemType[]> = ref([]);
const demandFilterList: Ref<DemandItemType[]> = ref([]);
const detailItem: Ref<DemandItemType | undefined> = ref<DemandItemType>();
const showDetail = ref<boolean>(false);
const searchValue = ref('');

const props = withDefaults(defineProps<{ status: number, haveAdd: boolean }>(), {
    status: 1,
    haveAdd: false,
});

const { status, haveAdd } = toRefs(props);

onMounted(async () => {
    await getDemandList();
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
    const list: DemandItemType[] = await invoke('get_demands_by_status', { status: status.value });
    demandList.value = list;
    demandFilterList.value = list;
    console.log('获取的数据为 -> ', list);
};

</script>

<style lang="less" scoped>
.v-demand-status-list {
    width: 100%;
    height: 100vh;
    overflow: hidden;

    .list-container {
        height: calc(100vh - @headerHeight);
    }

    .list-header {
        cursor: default;
        // border-bottom: @border-mini;
        // height: @headerHeight;
        border-radius: @border-radius;
        background-color: transparent;
        .search-prefix {
            line-height: 1;
            height: 100%;
        }
        .input-container {
            flex: 1;
            box-shadow: var(--el-box-shadow-light);
            border-radius: @border-radius;
        }
    }
}
</style>