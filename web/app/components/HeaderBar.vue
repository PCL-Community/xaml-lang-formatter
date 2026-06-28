<template>
  <header class="sticky top-0 z-30 border-b border-white/10 bg-slate-950/70 backdrop-blur-xl">
    <UContainer class="flex h-18 items-center justify-between gap-4">
      <NuxtLink class="group flex items-center gap-3" to="/">
        <span
            class="grid size-11 place-items-center rounded-2xl bg-gradient-to-br from-primary to-emerald-400 text-slate-950 shadow-lg shadow-primary/25 transition-transform duration-200 group-hover:scale-105">
          <UIcon class="size-5" name="i-lucide-braces"/>
        </span>
        <div>
          <div class="font-semibold tracking-tight text-toned">XAML Lang Formatter</div>
          <div class="text-xs text-muted">Rust + WebAssembly formatter workspace</div>
        </div>
      </NuxtLink>

      <nav class="flex items-center gap-2">
        <UButton
            color="neutral"
            icon="i-lucide-terminal-square"
            label="CLI"
            to="#developer-notes"
            variant="ghost"
        />
        <UButton
            color="neutral"
            icon="i-simple-icons-github"
            label="GitHub"
            target="_blank"
            to="https://github.com/PCL-Community/xaml-lang-formatter"
            variant="outline"
        />
      </nav>
    </UContainer>
  </header>
</template>
