<script lang="ts" setup>
const props = withDefaults(defineProps<{
  code: string
  language?: 'xml' | 'bash'
  emptyText?: string
  compact?: boolean
}>(), {
  language: 'xml',
  emptyText: 'Nothing to preview yet.',
  compact: false
})

function escapeHtml(value: string) {
  return value
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
}

function highlightXml(source: string) {
  const escaped = escapeHtml(source)

  return escaped.replace(/(&lt;!--[\s\S]*?--&gt;)|(&lt;\/?)([A-Za-z_][\w:.-]*)([\s\S]*?)(&gt;)/g, (match, comment, bracket, tagName, attrs = '', end) => {
    if (comment) {
      return `<span class="tok-comment">${comment}</span>`
    }

    const highlightedAttrs = String(attrs)
        .replace(/([\w:-]+)(=)(&quot;.*?&quot;)/g, '<span class="tok-attr">$1</span><span class="tok-punct">$2</span><span class="tok-string">$3</span>')

    return `<span class="tok-punct">${bracket}</span><span class="tok-tag">${tagName}</span>${highlightedAttrs}<span class="tok-punct">${end}</span>`
  })
}

function highlightBash(source: string) {
  const escaped = escapeHtml(source)
  return escaped.split(/\r?\n/).map((line) => {
    let result = line

    result = result.replace(/(".*?"|'.*?')/g, '<span class="tok-string">$1</span>')
    result = result.replace(/(\s|^)(--?[\w-]+)/g, '$1<span class="tok-flag">$2</span>')
    result = result.replace(/^(\s*)([A-Za-z_][\w:-]*)/, '$1<span class="tok-command">$2</span>')
    result = result.replace(/(\\)$/g, '<span class="tok-punct">$1</span>')

    return result
  }).join('\n')
}

const highlightedHtml = computed(() => {
  if (!props.code.trim()) {
    return `<span class="tok-muted">${escapeHtml(props.emptyText)}</span>`
  }

  return props.language === 'bash'
      ? highlightBash(props.code)
      : highlightXml(props.code)
})
</script>

<template>
  <div class="overflow-auto rounded-2xl border border-white/10 bg-slate-950/95 shadow-inner">
    <pre :class="compact ? 'min-h-0 p-4 text-xs leading-6' : 'min-h-[24rem] p-4 text-sm leading-7'"
         class="text-slate-100"><code v-html="highlightedHtml"/></pre>
  </div>
</template>

<style>
.tok-tag {
  color: #7dd3fc;
}

.tok-attr {
  color: #93c5fd;
}

.tok-string {
  color: #86efac;
}

.tok-comment {
  color: #94a3b8;
}

.tok-command {
  color: #67e8f9;
}

.tok-flag {
  color: #c4b5fd;
}

.tok-punct {
  color: #cbd5e1;
}

.tok-muted {
  color: #64748b;
}
</style>
