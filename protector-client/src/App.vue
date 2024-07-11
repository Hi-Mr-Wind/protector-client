<template>
  <HomeView/>
</template>

<script>
import {defineComponent} from "vue";
import Message from "@/components/Message.vue";
import HomeView from "@/views/HomeView.vue";

export default defineComponent({
  components: {HomeView}
})
</script>

<style src="./styles/main.css">
</style>