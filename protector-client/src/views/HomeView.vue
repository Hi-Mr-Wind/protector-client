<script setup lang="ts">

import Message from "@/components/Message.vue";
import FriendList from "@/components/FriendList.vue";
</script>

<template>
  <el-row>
    <el-col :span="6">
      <div class="main-list">
       <FriendList/>
      </div>
    </el-col>

    <el-col :span="18">
      <el-divider direction="vertical" />
      <div class="main-mes">
        <Message/>
      </div>
    </el-col>
  </el-row>
</template>

<style scoped>

</style>