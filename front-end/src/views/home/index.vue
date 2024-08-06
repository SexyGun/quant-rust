<script setup>
import { ref } from 'vue'
import { ElButton } from 'element-plus';
import { testGet } from './api.js';
const count  = ref(0);
const onClick = async () => {
    count.value += 1;
    const res = await testGet();
    console.log(res);
}

</script>

<template>
    <h1>Hello Quant</h1>
    <el-button @click="onClick">点击触发</el-button>
    <div>{{ count }}</div>
</template>