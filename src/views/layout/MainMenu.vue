<template>
    <div class="v-main-menu-container">
        <Logo />
        <el-menu default-active="/" class="main-menu font-size-lg" router>
            <el-menu-item index="/">
                <ListOne class="flex-row-center margin-right" size="20" theme="outline" :strokeWidth="4" />
                <span class="font-weight-bold">需求</span>
            </el-menu-item>
            <el-menu-item index="/ongoing">
                <Lightning class="flex-row-center margin-right" size="20" theme="outline" :strokeWidth="4" />
                <span class="font-weight-bold">进行中</span>
            </el-menu-item>
            <el-menu-item index="/complate">
                <List class="flex-row-center margin-right" size="20" theme="outline" :strokeWidth="4" />
                <span class="font-weight-bold">已完成</span>
            </el-menu-item>
            <el-menu-item index="/deploy">
                <Send class="flex-row-center margin-right" size="20" theme="outline" :strokeWidth="4" />
                <span class="font-weight-bold">已发布</span>
            </el-menu-item>
            <!-- <el-menu-item index="/report">
                <TableReport class="flex-row-center margin-right" size="20" theme="outline" :strokeWidth="4" />
                <span class="font-weight-bold">报表</span>
            </el-menu-item> -->
            <el-menu-item index="/setting">
                <SettingTwo class="flex-row-center margin-right" size="20" theme="outline" :strokeWidth="4" />
                <span class="font-weight-bold">设置</span>
            </el-menu-item>
            <el-menu-item index="/about">
                <Info class="flex-row-center margin-right" size="20" theme="outline" :strokeWidth="4" />
                <span class="font-weight-bold">关于</span>
            </el-menu-item>
        </el-menu>
    </div>
</template>

<script setup lang="ts">
import Logo from './Logo.vue';
import { ListOne, List, Send, Lightning, TableReport, SettingTwo, Info } from '@icon-park/vue-next';

</script>

<style lang="less" scoped>
.v-main-menu-container {
    // padding-top: 40px;
    .main-menu {
        border-right: none;
        user-select: none;
    }
}
</style>