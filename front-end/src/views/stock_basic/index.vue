<script setup>
import { ref } from "vue";
import {
  ElTable,
  ElTableColumn,
  ElPagination,
  ElRow,
  ElCol,
  ElForm,
  ElFormItem,
} from "element-plus";
import { getBasicQuery } from "./api";

const total = ref(0);
const size = ref(10);
const current = ref(1);
const tableData = ref([]);
const searchForm = ref({});

const handleSizeChange = (newSize) => {
  console.log(newSize);
  fetchTableData({ size: newSize, current: current.value });
};

const handleCurrentChange = (newCurrent) => {
  console.log(newCurrent);
  fetchTableData({ current: newCurrent, size: size.value });
};

const fetchTableData = async (page_params) => {
  const params = {
    size: page_params.size || size.value,
    current: page_params.current || current.value,
  };
  const { ts_code, symbol, name, area } = searchForm.value || {};
  const res = await getBasicQuery({
    ...params,
    ts_code: ts_code || undefined, 
    symbol: symbol || undefined, 
    name: name || undefined, 
    area: area || undefined, 
  });
  const {
    data,
    current: _current,
    size: _size,
    total: _total,
  } = res.data || {};

  tableData.value = data || [];
  current.value = _current || 1;
  size.value = _size || 10;
  total.value = _total || 0;
};
</script>

<template>
  <div class="container">
    <el-form :model="searchForm" label-width="auto">
      <el-row :gutter="10">
        <el-col :span="8">
          <el-form-item label="TS代码" prop="ts_code">
            <el-input v-model="searchForm.ts_code" placeholder="请输入" clearable />
          </el-form-item>
        </el-col>
        <el-col :span="8">
          <el-form-item label="股票代码" prop="symbol">
            <el-input v-model="searchForm.symbol" placeholder="请输入" clearable />
          </el-form-item>
        </el-col>
        <el-col :span="8">
          <el-form-item label="股票名称" prop="name">
            <el-input v-model="searchForm.name" placeholder="请输入" clearable />
          </el-form-item>
        </el-col>
        <el-col :span="8">
          <el-form-item label="地域" prop="area">
            <el-input v-model="searchForm.area" placeholder="请输入" clearable />
          </el-form-item>
        </el-col>
        <el-col :span="8">
          <el-button @click="fetchTableData" type="primary">查询</el-button>
        </el-col>
      </el-row>
    </el-form>

    <el-table :data="tableData" height="500" style="width: 100%">
      <el-table-column prop="ts_code" label="TS代码" width="100" />
      <el-table-column prop="symbol" label="股票代码" width="100" />
      <el-table-column prop="name" label="股票名称" width="100" />
      <el-table-column prop="area" label="地域" width="80" />
      <el-table-column prop="industry" label="所属行业" width="100" />
      <el-table-column prop="cnspell" label="拼音缩写" width="100" />
      <el-table-column prop="market" label="市场类型" width="100" />
      <el-table-column prop="list_date" label="上市日期" width="100" />
      <el-table-column prop="act_name" label="实控人名称" />
      <el-table-column prop="act_ent_type" label="实控人企业性质" width="130" />
    </el-table>
    <el-pagination
      v-model:current-page="current"
      v-model:page-size="size"
      :page-sizes="[10, 20, 50, 100]"
      layout="total, sizes, prev, pager, next, jumper"
      :total="total"
      @size-change="handleSizeChange"
      @current-change="handleCurrentChange"
      class="table-pagination"
    />
  </div>
</template>

<style scoped>
.contianer {
  display: flex;
  padding: 20px;
}
.contianer .table-pagination {
  width: 100%;
  margin-top: 10px;
}
</style>