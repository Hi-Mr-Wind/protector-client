<script setup lang="ts">
import {user,message_list,getBackgroundColor,input_message} from "@/js/message"

</script>

<template>
  <el-container>
    <el-header>
      <div style="display: flex; align-items: center;">
        <el-image class="user-header" src="https://sms4j.com/developer/me.jpg"/>
        <el-text class="mx-1">{{ user.username }}</el-text>
      </div>
    </el-header>

    <el-main>
      <div class="mes-frame">
        <el-divider />
        <div class="message" v-for="(item,index) in message_list" :key="index" :style="{backgroundColor: getBackgroundColor(item.source)}">
          <el-text class="mes-1">{{item.mesUser}}：</el-text>
          <el-text class="mes-1">{{item.mes}}</el-text>
        </div>
      </div>
    </el-main>

    <el-footer>
    <!-- 消息发送框-->
      <el-divider />
      <el-input style="height: 200px" type="textarea" v-model="input_message" :rows="9" placeholder="请输入消息……" />
      <el-button style="width: 150px" type="primary" round>发送</el-button>
    </el-footer>
  </el-container>
</template>

<style src="../styles/message.css"></style>