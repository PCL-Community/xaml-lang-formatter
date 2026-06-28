<script lang="ts" setup>
import {useHead} from 'nuxt/app'

useHead({
  title: 'XAML Lang Formatter',
  meta: [
    {
      name: 'description',
      content: 'Format WPF ResourceDictionary localization files in your browser with Rust WebAssembly.'
    }
  ]
})

const heroPreview = `<!-- Meta -->
<sys:String x:Key="Meta.Code">en-US</sys:String>
<sys:String x:Key="Meta.Name">English (US)</sys:String>

<!-- Common -->
<sys:String x:Key="Common.App">App</sys:String>

<!-- Common.Action -->
<sys:String x:Key="Common.Action.Open">Open</sys:String>`

const cliSnippet = `cargo run -- ./Lang --group-threshold 5
cargo run -- ./Lang --check
cargo run -- ./Lang --dry-run`

const webSnippet = `cd web
bun install
bun run wasm:build
bun --bun run dev`

const deploySnippet = `cd web
NUXT_APP_BASE_URL=/xaml-lang-formatter/ \\
  bun run build`
</script>

<template>
  <main class="min-h-screen text-default">
    <HeaderBar/>

    <UContainer class="space-y-8 py-8 lg:space-y-10 lg:py-12">
      <section class="grid items-center gap-8 lg:grid-cols-[1.05fr_0.95fr]">
        <div class="space-y-6">
          <div class="flex flex-wrap gap-2">
            <UBadge color="primary" variant="soft">Rust WASM</UBadge>
            <UBadge color="success" variant="soft">Local-first</UBadge>
            <UBadge color="neutral" variant="soft">Nuxt 4 + Nuxt UI 4</UBadge>
          </div>

          <div class="space-y-4">
            <h1 class="max-w-4xl text-4xl font-bold tracking-tight text-highlighted sm:text-5xl lg:text-7xl">
              Make XAML localization files feel beautifully organized.
            </h1>
            <p class="max-w-2xl text-lg leading-8 text-muted">
              A modern formatter workspace for WPF ResourceDictionary language files. Clean grouping,
              deterministic output, and instant formatting powered by Rust WebAssembly directly in your browser.
            </p>
          </div>

          <div class="flex flex-wrap gap-3">
            <UButton
                icon="i-lucide-wand-sparkles"
                label="Open workspace"
                size="xl"
                to="#formatter"
            />
            <UButton
                color="neutral"
                icon="i-simple-icons-github"
                label="View GitHub"
                size="xl"
                target="_blank"
                to="https://github.com/PCL-Community/xaml-lang-formatter"
                variant="outline"
            />
          </div>
        </div>

        <UCard class="glass-panel overflow-hidden border-white/10 bg-white/70 shadow-2xl shadow-slate-950/10 dark:bg-slate-900/65 dark:shadow-black/25">
          <div class="space-y-5 p-1">
            <div class="flex items-start justify-between gap-4 rounded-2xl border border-white/10 bg-slate-950/[0.03] p-5 dark:bg-white/[0.02]">
              <div class="space-y-2">
                <p class="text-sm font-medium text-muted">What makes it useful</p>
                <h2 class="text-2xl font-semibold text-highlighted">Focused on language resources, not generic XML prettifying.</h2>
              </div>
              <span class="grid size-12 shrink-0 place-items-center rounded-2xl bg-gradient-to-br from-primary/15 to-emerald-400/15 text-primary ring-1 ring-white/10">
                <UIcon class="size-6" name="i-lucide-sparkles"/>
              </span>
            </div>

            <div class="grid gap-3 sm:grid-cols-3">
              <div class="rounded-2xl border border-white/10 bg-muted/25 p-4">
                <p class="text-xs uppercase tracking-[0.18em] text-muted">Grouping</p>
                <p class="mt-2 text-sm text-toned">Sorts by <code class="rounded bg-muted px-1 py-0.5 text-xs">x:Key</code> and keeps sections readable.</p>
              </div>
              <div class="rounded-2xl border border-white/10 bg-muted/25 p-4">
                <p class="text-xs uppercase tracking-[0.18em] text-muted">Deterministic</p>
                <p class="mt-2 text-sm text-toned">Regenerates comments and produces predictable output every time.</p>
              </div>
              <div class="rounded-2xl border border-white/10 bg-muted/25 p-4">
                <p class="text-xs uppercase tracking-[0.18em] text-muted">Private</p>
                <p class="mt-2 text-sm text-toned">Runs entirely in your browser through WebAssembly.</p>
              </div>
            </div>

            <div>
              <div class="mb-3 flex items-center gap-2 text-xs uppercase tracking-[0.2em] text-slate-400">
                <span class="size-2 rounded-full bg-emerald-400"/>
                live formatting preview
              </div>
              <HighlightedCode :code="heroPreview" compact language="xml"/>
            </div>
          </div>
        </UCard>
      </section>

      <ClientOnly>
        <FormatterPanel/>
        <template #fallback>
          <UCard class="glass-panel border-white/10 bg-white/70 p-6 dark:bg-slate-900/60">
            <div class="flex items-center gap-3 text-muted">
              <UIcon class="size-5 animate-spin" name="i-lucide-loader-circle"/>
              Loading formatter workspace...
            </div>
          </UCard>
        </template>
      </ClientOnly>

      <section id="developer-notes" class="grid gap-6 lg:grid-cols-3">
        <UCard class="glass-panel border-white/10 bg-white/70 dark:bg-slate-900/60">
          <template #header>
            <div class="flex items-center gap-2 font-semibold">
              <UIcon class="size-4 text-primary" name="i-lucide-terminal-square"/>
              CLI
            </div>
          </template>
          <HighlightedCode :code="cliSnippet" compact empty-text="" language="bash"/>
        </UCard>

        <UCard class="glass-panel border-white/10 bg-white/70 dark:bg-slate-900/60">
          <template #header>
            <div class="flex items-center gap-2 font-semibold">
              <UIcon class="size-4 text-primary" name="i-lucide-code-xml"/>
              Web development
            </div>
          </template>
          <HighlightedCode :code="webSnippet" compact empty-text="" language="bash"/>
        </UCard>

        <UCard class="glass-panel border-white/10 bg-white/70 dark:bg-slate-900/60">
          <template #header>
            <div class="flex items-center gap-2 font-semibold">
              <UIcon class="size-4 text-primary" name="i-lucide-rocket"/>
              Deployment
            </div>
          </template>
          <HighlightedCode :code="deploySnippet" compact empty-text="" language="bash"/>
        </UCard>
      </section>
    </UContainer>
  </main>
</template>
