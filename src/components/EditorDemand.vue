<template>
    <el-button type="primary" size="large" @click="handleAddAction">{{ isEdit ? '编辑' : '新增' }}需求</el-button>
    <el-drawer custom-class="v-editor-demand" v-model="showEditor" direction="rtl" size="50%"
        :before-close="handleClose" :close-on-click-modal="false" :show-close="false" :append-to-body="true">
        <template #header>
            <DrawerHeader :title="`${isEdit ? '编辑' : '新增'}需求`" @close="handleClose" />
        </template>
        <PerfectScrollbar class="list-container padding-all-md">
            <el-form :model="form" ref="ruleFormRef" :rules="rules" label-width="80px">
                <el-form-item label="名称" prop="name">
                    <el-input v-model="form.name" placeholder="输入需求名称" />
                </el-form-item>
                <el-form-item label="地址" prop="demand_link">
                    <el-input v-model="form.demand_link" placeholder="输入需求地址" />
                </el-form-item>
                <el-form-item label="描述" prop="desc">
                    <el-input v-model="form.desc" type="textarea" placeholder="输入需求描述" />
                </el-form-item>
                <el-form-item label="设计稿" prop="ui_link">
                    <el-input v-model="form.ui_link" placeholder="输入UI地址" />
                </el-form-item>
                <el-form-item label="接口" prop="api_link">
                    <el-input v-model="form.api_link" placeholder="输入API地址" />
                </el-form-item>
                <el-form-item label="项目地址" prop="code_path">
                    <div class="flex-row flex-item-one">
                        <el-input class="flex-item-one" v-model="form.code_path" placeholder="选择项目地址" />
                        <FolderOpen class="margin-left pointer-element" theme="outline" size="24" fill="#333"
                            :strokeWidth="2" @click="handleSelectPath" />
                    </div>
                </el-form-item>
                <el-form-item label="发布地址" prop="publish_link">
                    <el-input v-model="form.publish_link" placeholder="输入发布地址" />
                </el-form-item>
                <el-form-item label="发布时间" prop="publish_date">
                    <el-date-picker v-model="form.publish_date" type="date" placeholder="选择发布时间" />
                </el-form-item>
                <el-form-item label="额外链接" prop="extra_links">
                    <el-space class="margin-top-sm" :size="10" wrap>
                        <LinkItem v-for="temp in form.extra_links" :key="temp.link" :linkItem="temp" closable
                            @close="handleMinusExtraLink" />
                    </el-space>
                    <ExtraModal v-if="form.extra_links.length < 5" @add="handleAddExtraLink" />
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="submitForm(ruleFormRef)">{{ isEdit ? '更新' : '新增' }}</el-button>
                    <el-button @click="resetForm(ruleFormRef)">重置</el-button>
                </el-form-item>
            </el-form>
        </PerfectScrollbar>
    </el-drawer>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref, toRefs, watch } from 'vue';
import { PerfectScrollbar } from 'vue3-perfect-scrollbar';
import { FolderOpen, Plus } from '@icon-park/vue-next';
import { message, open } from '@tauri-apps/api/dialog';
import { homeDir } from '@tauri-apps/api/path';
import type { FormInstance, FormRules } from 'element-plus';
import { DemandFormValue, DemandItemType, ExtraLinkItem } from '../common/types';
import { createDemandItem, editorDemandItem, validUrl } from '../common/utils';
import { invoke } from '@tauri-apps/api/tauri';
import { ElMessage } from 'element-plus'

const showEditor = ref(false);
const props = defineProps<{ isEdit?: boolean; info: DemandItemType; }>();
const { isEdit, info } = toRefs(props);
const ruleFormRef = ref<FormInstance>();
const emits = defineEmits(['refresh']);

const defaultValue = {
    name: '',
    desc: '',
    publish_date: 0,
    demand_link: '',
    ui_link: '',
    api_link: '',
    code_path: '',
    publish_link: '',
    extra_links: <ExtraLinkItem[]>[],
};

watch(info, (val, oldVal) => {
    if (val) {
        console.log('内容编辑修改')
        const { name, desc, demand_link, ui_link, api_link, code_path, publish_date, publish_link, extra_links } = val;
        form.name = name;
        form.desc = desc;
        form.publish_date = publish_date;
        form.demand_link = demand_link;
        form.ui_link = ui_link;
        form.api_link = api_link;
        form.code_path = code_path;
        form.publish_link = publish_link;
        form.extra_links = extra_links;
    }
});

const handleAddAction = () => {
    showEditor.value = true;
};

const handleClose = () => {
    showEditor.value = false;
    ruleFormRef.value?.resetFields();
}

// 选择本地地址
const handleSelectPath = async () => {
    const tempDir = await homeDir();
    const selectedPath = <string>await open({
        defaultPath: tempDir,
        title: '选择项目地址',
        directory: true,
    });
    console.log('选择的地址为 => ', selectedPath);
    form.code_path = selectedPath || '';
}

const handleAddExtraLink = (item: ExtraLinkItem) => {
    // 限制不能名称重复
    const list = form.extra_links.filter(temp => temp.text == item.text);
    if (list.length > 0) {
        ElMessage({
            showClose: false,
            message: '额外链接名称不能重复！',
            type: 'error',
        })
    } else {
        form.extra_links.push(item);
    }
}

const handleMinusExtraLink = (item: ExtraLinkItem) => {
    const list = form.extra_links.filter(temp => temp.text !== item.text);
    form.extra_links = list;
}

let form = reactive(<DemandFormValue>{
    name: '',
    desc: '',
    publish_date: 0,
    demand_link: '',
    ui_link: '',
    api_link: '',
    code_path: '',
    publish_link: '',
    extra_links: <ExtraLinkItem[]>[],
});


const rules = reactive<FormRules>({
    name: [
        { required: true, message: '需求名称不能为空', trigger: 'blur' },
    ],
    demand_link: [
        { required: true, message: '需求链接不能为空', trigger: 'blur' },
        { validator: validUrl, trigger: 'blur' },
    ],
    ui_link: [
        { validator: validUrl, trigger: 'blur' },
    ],
    api_link: [
        { validator: validUrl, trigger: 'blur' },
    ],
    publish_link: [
        { validator: validUrl, trigger: 'blur' },
    ],
});

const submitForm = async (formEl: FormInstance | undefined) => {
    if (!formEl) return
    await formEl.validate(async (valid, fields) => {
        if (valid) {
            const params = { ...form };
            if (isEdit?.value) {
                const item = editorDemandItem(params, info.value);
                console.log('更新参数为 ->', item);
                await invoke('update_demand', { item });
                emits('refresh');
                handleClose();
            } else {
                const item = createDemandItem(params);
                console.log('提交参数为 ->', item);
                await invoke('add_demand', { item });
                emits('refresh');
                handleClose();
            }
        } else {
            console.log('error submit!', fields)
        }
    })
}

const resetForm = (formEl: FormInstance | undefined) => {
    if (!formEl) return
    formEl.resetFields()
}

</script>

<style lang="less">
.v-editor-demand {
    --el-drawer-padding-primary: 0;

    .el-drawer__header {
        margin-bottom: 0;
    }
}
</style>