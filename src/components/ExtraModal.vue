<template>
    <Plus class="margin-left pointer-element" theme="outline" size="24" fill="#333" :strokeWidth="2"
        @click="handleAddExtraLink" />
    <el-dialog v-model="dialogVisible" title="增加额外链接" width="50%" append-to-body>
        <el-form :model="form" ref="ruleFormRef" :rules="rules" label-width="80px">
            <el-form-item label="名称" prop="text">
                <el-input v-model="form.text" placeholder="输入链接名称" />
            </el-form-item>
            <el-form-item label="地址" prop="link">
                <el-input v-model="form.link" type="textarea" placeholder="输入链接地址" />
            </el-form-item>
        </el-form>
        <template #footer>
            <el-button @click="handleClose(ruleFormRef)">取消</el-button>
            <el-button type="primary" @click="submitForm(ruleFormRef)">增加</el-button>
        </template>
    </el-dialog>
</template>

<script setup lang="ts">
import { reactive, ref } from "vue";
import { FolderOpen, Plus } from '@icon-park/vue-next';
import type { FormInstance, FormRules } from "element-plus";
import { validUrl } from "../common/utils.js";
const emits = defineEmits(['add']);
const ruleFormRef = ref<FormInstance>();
const dialogVisible = ref(false);
const form = reactive({
    text: '',
    link: '',
});

const rules = reactive<FormRules>({
    text: [
        { required: true, message: '链接名称不能为空', trigger: 'blur' },
    ],
    link: [
        { required: true, message: '链接地址不能为空', trigger: 'blur' },
        { validator: validUrl, trigger: 'blur' },
    ],
});

const handleAddExtraLink = () => {
    dialogVisible.value = true;
    console.log('展示', dialogVisible.value);
}

const handleClose = (formEl: FormInstance | undefined) => {
    dialogVisible.value = false;
    formEl?.resetFields()
}

const submitForm = async (formEl: FormInstance | undefined) => {
    if (!formEl) return
    await formEl.validate((valid, fields) => {
        if (valid) {
            console.log(form);
            emits('add', {
                ...form,
            });
            handleClose(formEl);
            console.log('submit!')
        } else {
            console.log('error submit!', fields)
        }
    })
}

</script>

<style scoped>
</style>