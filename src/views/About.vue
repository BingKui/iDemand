<template>
    <div class="v-about-page flex-column-center">
        <img src="../assets/appicon.png" width="160" alt="">
        <div class="font-size-32 margin-v-lg">{{ name }}</div>
        <div class="font-size">Version. v{{ version }}</div>
        <div class="font-size-sm margin-top-sm">Copyright © 2022 {{ author }}. All rights reserved.</div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getName, getVersion } from '@tauri-apps/api/app';

const name = ref('');
const version = ref('');
const author = ref('康兵奎');

onMounted(async () => {
    name.value = await getName();
    version.value = await getVersion();
});
</script>

<style lang="less" scoped>
.v-about-page {
    height: 100vh;
}
</style>