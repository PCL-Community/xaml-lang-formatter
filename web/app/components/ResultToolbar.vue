<script lang="ts" setup>
defineProps<{
  disabled?: boolean
}>()

const emit = defineEmits<{
  copy: []
  download: []
  clear: []
}>()
</script>

<template>
  <div class="flex flex-wrap items-start justify-between gap-3">
    <div class="space-y-1">
      <div class="flex items-center gap-2">
        <span
            class="inline-flex size-8 items-center justify-center rounded-xl bg-primary/12 text-primary ring-1 ring-primary/20">
          <UIcon class="size-4" name="i-lucide-sparkles"/>
        </span>
        <div>
          <h3 class="text-base font-semibold text-highlighted">Formatted output</h3>
          <p class="text-sm text-muted">Review, copy, or download the normalized ResourceDictionary.</p>
        </div>
      </div>
    </div>

    <div class="flex flex-wrap gap-2">
      <UButton
          :disabled="disabled"
          color="neutral"
          icon="i-lucide-copy"
          label="Copy"
          variant="outline"
          @click="emit('copy')"
      />
      <UButton
          :disabled="disabled"
          color="neutral"
          icon="i-lucide-download"
          label="Download"
          variant="outline"
          @click="emit('download')"
      />
      <UButton
          :disabled="disabled"
          color="neutral"
          icon="i-lucide-eraser"
          label="Clear"
          variant="ghost"
          @click="emit('clear')"
      />
    </div>
  </div>
</template>
