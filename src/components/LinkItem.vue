<template>
    <el-tag class="v-link-item pointer-element" effect="dark" type="success" @click="handleClickAction" :closable="closable" @close="handleClose">
        {{ linkItem.text }}
    </el-tag>
</template>

<script setup lang="ts">
import { toRefs } from 'vue';
import { ExtraLinkItem } from '../common/types';
import { open } from '@tauri-apps/api/shell';
const emits = defineEmits(['close'])

const props = defineProps<{ linkItem: ExtraLinkItem, closable: boolean; }>();
const { linkItem, closable } = toRefs(props);

const handleClickAction = async () => {
    await open(linkItem.value.link);
};

const handleClose = () => {
    emits('close', linkItem.value);
}
</script>

<style scoped>
</style>