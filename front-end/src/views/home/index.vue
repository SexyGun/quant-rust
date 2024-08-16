<script setup>
import Card from "@/components/Card.vue";
const capabilityList = [
  {
    header: "大盘股票汇总",
    content: "总有一款 A 股匹配您",
    image:
      "https://wx2.sinaimg.cn/bmiddle/006ARE9vgy1fu4j4kq3pwj31gf0u0gnj.jpg",
    targetRoute: "/stock",
  },
  {
    header: "Todo 股票排名（选股指南？）",
    content: "计算 Rps 排名并展示",
    image: "https://tva3.sinaimg.cn/large/006rPqrTgy1gof3kgnqq8j30yh0voqa8.jpg",
    targetRoute: "/stock-select",
  },
  {
    header: "Todo 关注股票",
    content:
      "展示选择股票的基本信息，提供几种基本的量化算法的买入卖出点，并展示回测效果",
    image:
      "https://img1.baidu.com/it/u=1721606346,478465930&fm=253&fmt=auto&app=138&f=JPEG?w=800&h=460",
    targetRoute: "/stock",
  },
  {
    header: "Todo 套利？",
    content: "目前还不知道什么叫套利 😊",
    image:
      "https://tvax3.sinaimg.cn/crop.0.0.444.444.1024/006A3pM6ly8fog8ajr6dxj30ch0cc16a.jpg?KID=imgbed,tva&Expires=1640361538&ssig=0Kbx2SfkFf",
    targetRoute: "/stock",
  },
];
const openTargetRoute = (url) => {
  window.open(url);
}
</script>

<template>
  <div>
    <h1 class="title">选择你的韭菜</h1>

    <div id="app" class="container">
      <card
        v-for="ca in capabilityList"
        :key="ca.header"
        :data-image="ca.image"
        @click="openTargetRoute(ca.targetRoute)"
      >
        <template #header>
          <h1>{{ ca.header }}</h1>
        </template>
        <template #content>
          <p>{{ ca.content }}</p>
        </template>
      </card>
    </div>
  </div>
</template>

<style lang="scss" scoped>
body {
  margin: 40px 0;
  font-family: "Raleway";
  font-size: 14px;
  font-weight: 500;
  background-color: #bcaaa4;
  -webkit-font-smoothing: antialiased;
}

.title {
  font-family: "Raleway";
  font-size: 24px;
  font-weight: 700;
  color: #5d4037;
  text-align: center;
}

p {
  line-height: 1.5em;
}

h1 + p,
p + p {
  margin-top: 10px;
}

.container {
  padding: 40px 80px;
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}
</style>