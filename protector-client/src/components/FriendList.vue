<script setup lang="ts">
import {tableData, click_list} from "../js/friendList"

</script>

<template>
  <el-row :gutter="20">
    <el-col :span="1"></el-col>
      <!-- 功能区 -->
    <el-col :span="5" style="width: 200px; height: 650px;display: flex; justify-content: center;">
      <div class="function">
        <div>
          <el-image style="margin-top: 10px;width: 45px;height: 45px"
                    src="https://himg.bdimg.com/sys/portraitn/item/public.1.e4dd1768.84SwiF7xBNN1zG5o1z6wJw"/>
        </div>
      </div>
    </el-col>

    <el-col :span="17">
      <!--好友列表-->
      <div class="heard-list">
        <el-table :data="tableData" class="friend-list"
                  height="650"
                  @cell-click="click_list">
          <el-table-column>
            <template v-slot="scope">
              <img :src="scope.row.img_heard" alt="Avatar" style="width: 35px; height: 35px;">
            </template>
          </el-table-column>
          <el-table-column prop="name"/>
        </el-table>
      </div>
    </el-col>
  </el-row>
</template>

<style scoped>

</style>