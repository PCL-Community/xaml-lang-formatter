type ProcessLike = {
    env?: Record<string, string | undefined>
}

const processLike = (globalThis as typeof globalThis & {
    process?: ProcessLike
}).process

const baseURL = processLike?.env?.NUXT_APP_BASE_URL || '/'

export default defineNuxtConfig({
    compatibilityDate: '2026-06-28',

    modules: ['@nuxt/ui'],

    css: ['~/assets/css/main.css'],

    app: {
        baseURL
    },

    nitro: {
        preset: 'github_pages'
    },

    typescript: {
        strict: true
    },

    ui: {
        theme: {
            colors: ['primary', 'secondary', 'success', 'info', 'warning', 'error']
        }
    },

    fonts: {
        provider: 'bunny'
    }
})
