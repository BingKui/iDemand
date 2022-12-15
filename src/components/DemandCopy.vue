<template>
    
</template>

<script setup lang="ts">
import { toRefs } from 'vue';
import { DemandItemType } from '../common/types';
const props = defineProps<{ info: DemandItemType; }>();
const { info } = toRefs(props);
</script>

<style scoped>

</style>