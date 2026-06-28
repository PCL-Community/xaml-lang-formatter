export {}

declare module '#app' {
    interface NuxtApp {
        $xamlFormatter: {
            format(input: string, options: {
                groupThreshold: number
                timestamp: string
            }): string
        }
    }
}
