<script lang="ts" setup>
const toast = useToast()
const {createTimestamp, downloadText, format, sample} = useXamlFormatter()

const content = ref('')
const error = ref('')
const fileName = ref('formatted.xaml')
const groupThreshold = ref(5)
const manualTimestamp = ref('')
const useManualTimestamp = ref(false)
const lastAction = ref<'source' | 'formatted'>('source')
const fileInput = ref<HTMLInputElement | null>(null)

const currentTimestamp = computed(() => {
  const manual = manualTimestamp.value.trim()

  if (useManualTimestamp.value && manual) {
    return manual
  }

  return createTimestamp()
})

const lineCount = computed(() => content.value ? content.value.split(/\r?\n/).length : 0)
const byteCount = computed(() => new Blob([content.value]).size)
const statusLabel = computed(() => lastAction.value === 'formatted' ? 'Formatted result' : 'Source document')

function loadSample() {
  fileName.value = 'sample.xaml'
  content.value = sample
  error.value = ''
  lastAction.value = 'source'
}

function clearContent() {
  fileName.value = 'formatted.xaml'
  content.value = ''
  error.value = ''
  lastAction.value = 'source'

  if (fileInput.value) {
    fileInput.value.value = ''
  }
}

function openFilePicker() {
  fileInput.value?.click()
}

async function onFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]

  if (!file) {
    return
  }

  fileName.value = file.name
  content.value = await file.text()
  error.value = ''
  lastAction.value = 'source'
}

function formatNow() {
  error.value = ''

  try {
    content.value = format(content.value, {
      groupThreshold: groupThreshold.value,
      timestamp: currentTimestamp.value
    })
    lastAction.value = 'formatted'

    toast.add({
      title: 'Formatted in place',
      description: 'The editor content has been replaced with the formatted XAML.',
      icon: 'i-lucide-check-circle'
    })
  } catch (err) {
    error.value = err instanceof Error ? err.message : String(err)

    toast.add({
      title: 'Format failed',
      description: error.value,
      color: 'error',
      icon: 'i-lucide-circle-alert'
    })
  }
}

async function copyContent() {
  await navigator.clipboard.writeText(content.value)

  toast.add({
    title: 'Copied',
    description: 'Editor content copied to clipboard.',
    icon: 'i-lucide-copy-check'
  })
}

function downloadContent() {
  downloadText(fileName.value || 'formatted.xaml', content.value)
}
</script>

<template>
  <section id="formatter" class="space-y-5">
    <div class="grid gap-3 lg:grid-cols-[minmax(0,1fr)_auto] lg:items-center">
      <div>
        <div class="flex flex-wrap items-center gap-2">
          <UBadge color="primary" variant="soft">CodeMirror 6</UBadge>
          <UBadge color="success" variant="soft">In-place format</UBadge>
          <UBadge color="neutral" variant="soft">Single editor</UBadge>
        </div>
        <h2 class="mt-3 text-2xl font-semibold tracking-tight text-highlighted">One editor. Paste, format, keep working.</h2>
        <p class="mt-2 max-w-3xl text-sm text-muted">
          The web UI now behaves like a code editor: load or paste XAML, run the formatter, and the same editor is replaced with the formatted result.
        </p>
      </div>

      <div class="flex flex-wrap gap-2">
        <UButton
            color="neutral"
            icon="i-lucide-file-code-2"
            label="Load sample"
            variant="outline"
            @click="loadSample"
        />
        <UButton
            :disabled="!content"
            color="neutral"
            icon="i-lucide-trash-2"
            label="Reset"
            variant="ghost"
            @click="clearContent"
        />
      </div>
    </div>

    <UCard class="glass-panel overflow-hidden border-white/10 bg-white/70 shadow-2xl shadow-slate-950/10 dark:bg-slate-900/65 dark:shadow-black/25">
      <div class="border-b border-white/10 px-5 py-4">
        <div class="flex flex-wrap items-center justify-between gap-3">
          <div class="flex items-center gap-3">
            <span class="grid size-10 place-items-center rounded-2xl bg-gradient-to-br from-primary/20 to-emerald-400/20 text-primary ring-1 ring-white/10">
              <UIcon class="size-5" name="i-lucide-code-xml"/>
            </span>
            <div>
              <p class="font-semibold text-highlighted">XAML editor</p>
              <p class="text-sm text-muted">Syntax-highlighted CodeMirror workspace with formatter controls.</p>
            </div>
          </div>

          <div class="flex flex-wrap items-center gap-2 text-xs text-muted">
            <span class="rounded-full bg-muted/80 px-3 py-1 ring-1 ring-white/10">{{ statusLabel }}</span>
            <span class="rounded-full bg-muted/80 px-3 py-1 ring-1 ring-white/10">{{ lineCount }} lines</span>
            <span class="rounded-full bg-muted/80 px-3 py-1 ring-1 ring-white/10">{{ byteCount }} bytes</span>
            <span class="rounded-full bg-primary/10 px-3 py-1 text-primary ring-1 ring-primary/20">WASM active</span>
          </div>
        </div>
      </div>

      <div class="grid gap-5 p-5 xl:grid-cols-[minmax(0,1fr)_320px]">
        <div class="space-y-4">
          <UCard class="border-white/10 bg-white/75 dark:bg-slate-950/35">
            <div class="space-y-4">
              <div class="flex flex-wrap items-center justify-between gap-3">
                <div class="space-y-1">
                  <div class="flex items-center gap-2">
                    <span class="inline-flex size-8 items-center justify-center rounded-xl bg-primary/12 text-primary ring-1 ring-primary/20">
                      <UIcon class="size-4" name="i-lucide-file-code-2"/>
                    </span>
                    <div>
                      <h3 class="text-base font-semibold text-highlighted">Editor</h3>
                      <p class="text-sm text-muted">Edit source or formatted XAML in the same CodeMirror field.</p>
                    </div>
                  </div>
                </div>

                <div class="flex flex-wrap gap-2">
                  <UButton
                      :disabled="!content"
                      color="neutral"
                      icon="i-lucide-copy"
                      label="Copy"
                      size="sm"
                      variant="outline"
                      @click="copyContent"
                  />
                  <UButton
                      :disabled="!content"
                      color="neutral"
                      icon="i-lucide-download"
                      label="Download"
                      size="sm"
                      variant="outline"
                      @click="downloadContent"
                  />
                </div>
              </div>

              <div class="flex flex-wrap items-center justify-between gap-3">
                <div class="flex flex-wrap items-center gap-2">
                  <UBadge color="neutral" variant="soft">{{ fileName }}</UBadge>
                  <UBadge color="neutral" variant="subtle">{{ lineCount }} lines</UBadge>
                </div>

                <input
                    ref="fileInput"
                    accept=".xaml,.xml"
                    class="hidden"
                    type="file"
                    @change="onFileChange"
                >
                <UButton
                    color="neutral"
                    icon="i-lucide-upload"
                    label="Load file"
                    size="sm"
                    variant="outline"
                    @click="openFilePicker"
                />
              </div>

              <CodeMirrorEditor
                  v-model="content"
                  placeholder="Paste ResourceDictionary XAML here, then click Format now..."
              />
            </div>
          </UCard>
        </div>

        <div class="space-y-4">
          <UCard class="border-white/10 bg-slate-950/[0.04] dark:bg-white/[0.03]">
            <template #header>
              <div class="flex items-center justify-between gap-3">
                <div>
                  <p class="font-semibold text-highlighted">Controls</p>
                  <p class="text-sm text-muted">Format the current editor content in place.</p>
                </div>
                <UBadge color="primary" variant="soft">Config</UBadge>
              </div>
            </template>

            <div class="space-y-5">
              <UFormField
                  help="How many items under the same prefix are required before creating a nested section."
                  label="Group threshold"
              >
                <UInputNumber v-model="groupThreshold" :min="1"/>
              </UFormField>

              <UFormField help="Enable this if you want to override the browser-generated local timestamp." label="Manual timestamp">
                <div class="flex items-center justify-between gap-3 rounded-2xl border border-default/50 bg-muted/30 px-3 py-2">
                  <div>
                    <p class="text-sm font-medium text-highlighted">Custom formatter timestamp</p>
                    <p class="text-xs text-muted">Use your own yyyy-MM-ddTHH:mm:ss value.</p>
                  </div>
                  <USwitch v-model="useManualTimestamp"/>
                </div>
              </UFormField>

              <UInput
                  v-if="useManualTimestamp"
                  v-model="manualTimestamp"
                  icon="i-lucide-clock-3"
                  placeholder="2026-06-28T15:21:30"
              />

              <div class="rounded-2xl border border-primary/15 bg-primary/8 p-4">
                <div class="flex items-start gap-3">
                  <span class="mt-0.5 inline-flex size-8 items-center justify-center rounded-xl bg-primary/12 text-primary ring-1 ring-primary/20">
                    <UIcon class="size-4" name="i-lucide-clock-3"/>
                  </span>
                  <div class="space-y-1">
                    <p class="text-sm font-medium text-highlighted">Active timestamp</p>
                    <p class="font-mono text-sm text-primary">{{ currentTimestamp }}</p>
                  </div>
                </div>
              </div>

              <UButton
                  :disabled="!content.trim() || groupThreshold < 1"
                  block
                  class="h-12"
                  icon="i-lucide-wand-sparkles"
                  label="Format now"
                  size="lg"
                  @click="formatNow"
              />
            </div>
          </UCard>

          <UAlert
              color="success"
              description="The formatter replaces the editor content locally through Rust WebAssembly. Nothing is uploaded."
              icon="i-lucide-shield-check"
              title="Privacy first"
              variant="soft"
          />

          <UAlert
              v-if="error"
              :description="error"
              color="error"
              icon="i-lucide-circle-alert"
              title="Format failed"
              variant="soft"
          />
        </div>
      </div>
    </UCard>
  </section>
</template>
