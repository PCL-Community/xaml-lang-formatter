<script lang="ts" setup>
import {basicSetup} from 'codemirror'
import {xml} from '@codemirror/lang-xml'
import {EditorState} from '@codemirror/state'
import {EditorView, placeholder as cmPlaceholder} from '@codemirror/view'

const props = withDefaults(defineProps<{
  modelValue: string
  placeholder?: string
}>(), {
  placeholder: 'Paste ResourceDictionary XAML here...'
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const editorHost = ref<HTMLDivElement | null>(null)
const editorView = shallowRef<EditorView | null>(null)

const formatterTheme = EditorView.theme({
  '&': {
    minHeight: '34rem',
    height: '100%',
    borderRadius: '1rem',
    overflow: 'hidden',
    backgroundColor: '#020617',
    color: '#e2e8f0',
    border: '1px solid rgba(148, 163, 184, 0.2)'
  },
  '&.cm-focused': {
    outline: '2px solid rgba(16, 185, 129, 0.45)',
    outlineOffset: '2px'
  },
  '.cm-scroller': {
    minHeight: '34rem',
    fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
    fontSize: '0.875rem',
    lineHeight: '1.65'
  },
  '.cm-content': {
    padding: '1rem 0',
    caretColor: '#34d399'
  },
  '.cm-line': {
    padding: '0 1rem'
  },
  '.cm-gutters': {
    backgroundColor: '#020617',
    color: '#64748b',
    borderRight: '1px solid rgba(148, 163, 184, 0.14)'
  },
  '.cm-activeLine': {
    backgroundColor: 'rgba(16, 185, 129, 0.08)'
  },
  '.cm-activeLineGutter': {
    backgroundColor: 'rgba(16, 185, 129, 0.10)',
    color: '#a7f3d0'
  },
  '.cm-selectionBackground, &.cm-focused .cm-selectionBackground': {
    backgroundColor: 'rgba(45, 212, 191, 0.28)'
  },
  '.cm-placeholder': {
    color: '#64748b'
  },
  '.cm-tooltip': {
    backgroundColor: '#0f172a',
    color: '#e2e8f0',
    border: '1px solid rgba(148, 163, 184, 0.2)'
  }
}, {dark: true})

function syncEditorContent(value: string) {
  const view = editorView.value

  if (!view) {
    return
  }

  const current = view.state.doc.toString()

  if (current === value) {
    return
  }

  view.dispatch({
    changes: {
      from: 0,
      to: current.length,
      insert: value
    }
  })
}

onMounted(() => {
  if (!editorHost.value) {
    return
  }

  const state = EditorState.create({
    doc: props.modelValue,
    extensions: [
      basicSetup,
      xml(),
      formatterTheme,
      EditorView.lineWrapping,
      cmPlaceholder(props.placeholder),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          emit('update:modelValue', update.state.doc.toString())
        }
      })
    ]
  })

  editorView.value = new EditorView({
    state,
    parent: editorHost.value
  })
})

watch(() => props.modelValue, syncEditorContent)

onBeforeUnmount(() => {
  editorView.value?.destroy()
  editorView.value = null
})
</script>

<template>
  <div ref="editorHost" class="codemirror-shell"/>
</template>

<style scoped>
.codemirror-shell {
  min-height: 34rem;
}

.codemirror-shell :deep(.cm-editor) {
  width: 100%;
}
</style>
