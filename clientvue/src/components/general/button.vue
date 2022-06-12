<script setup>
defineProps({
  msg: {
    type: String,
    required: true,
  },
});
</script>

<template>
    <button>
        <p>{{ msg }}</p>
    </button>
</template>