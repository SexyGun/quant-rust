<script setup>
import { RouterLink, RouterView } from "vue-router";
import { ElContainer, ElHeader, ElMain } from "element-plus";
</script>

<template>
  <div class="common-layout">
    <el-container>
      <el-main>
        <RouterView />
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>
</style>
