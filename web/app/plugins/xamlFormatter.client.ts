import {defineNuxtPlugin, useRuntimeConfig} from "nuxt/app";

type WasmModule = {
    default: (input?: string | URL) => Promise<void>
    format_xaml: (input: string, options: {
        group_threshold: number
        timestamp: string
    }) => { output: string }
}

export default defineNuxtPlugin(async () => {
    const config = useRuntimeConfig()
    const baseURL = config.app.baseURL || '/'

    const wasmUrl = `${baseURL}wasm/xaml_lang_formatter_wasm.js`
    const wasmBinaryUrl = `${baseURL}wasm/xaml_lang_formatter_wasm_bg.wasm`

    const wasm = await import(/* @vite-ignore */ wasmUrl) as WasmModule
    await wasm.default(wasmBinaryUrl)

    return {
        provide: {
            xamlFormatter: {
                format(input: string, options: {
                    groupThreshold: number
                    timestamp: string
                }) {
                    const result = wasm.format_xaml(input, {
                        group_threshold: options.groupThreshold,
                        timestamp: options.timestamp
                    })

                    return result.output
                }
            }
        }
    }
})
