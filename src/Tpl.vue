<template>
    <router-view v-if="isDemandPage" />
    <MainLayout v-else>
        <el-config-provider :locale="zhCn">
            <router-view />
        </el-config-provider>
    </MainLayout>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import MainLayout from './views/layout/MainLayout.vue';
import zhCn from 'element-plus/lib/locale/lang/zh-cn'
import './styles/main.less';
const router = useRoute();

const isDemandPage = computed(() => {
    return router.path.indexOf('demand') > -1;
});

</script>

<style scoped>
</style>