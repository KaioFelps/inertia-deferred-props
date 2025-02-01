<script setup lang="ts">
  defineProps<{ name: string }>();
</script>

<template>
    <span class="px-4 py-2 bg-blue-100 font-medium text-slate-700">
        {{ name }}
    </span>
</template>