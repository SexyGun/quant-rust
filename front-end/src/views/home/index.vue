<script setup>
import { ref } from 'vue'
import { ElButton } from 'element-plus';
import { testGet, testDataBase, testPost, delPost, getPost, testGetStock } from './api.js';
const count = ref(0);
const onClick = async () => {
    count.value += 1;
    // const res = await testGet();
    const res = await testGetStock();
    console.log(res);
    // const res_database = await testDataBase();
    // console.log(res_database);
}

const insertPost = async () => {
    const res = await testPost({
        title: 'lee',
        text: 'asdasdad',
        published: true
    });
    console.log(res);
}

const getPostById = async () => {
    const res = await getPost({
        id: 2
    });
    console.log(res);
}

const delPostById = async () => {
    const res = await delPost({
        id: 2
    });
    console.log(res);
}

</script>

<template>
    <h1>Hello Quant</h1>
    <el-button @click="onClick">点击触发</el-button>
    <div>{{ count }}</div>
    <el-button @click="insertPost">插入数据</el-button>
    <el-button @click="getPostById">获取数据</el-button>
    <el-button @click="delPostById">删除数据</el-button>
</template>