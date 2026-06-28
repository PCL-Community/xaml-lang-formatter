import {useNuxtApp} from "nuxt/app";

export const SAMPLE_XAML = `<ResourceDictionary
    xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
    xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
    xmlns:sys="clr-namespace:System;assembly=mscorlib"
    xml:space="preserve">

    <sys:String x:Key="Common.Action.Test.Test2">2</sys:String>
    <sys:String x:Key="Common.App">App</sys:String>
    <sys:String x:Key="Meta.Name">English (US)</sys:String>
    <sys:String x:Key="Common.Action.Close">Close</sys:String>
    <sys:String x:Key="Meta.Code">en-US</sys:String>
    <sys:String x:Key="Common.Action.Open">Open</sys:String>
    <sys:String x:Key="Common.Action.Test.Test1">1</sys:String>

</ResourceDictionary>
`

export function useXamlFormatter() {
    const {$xamlFormatter} = useNuxtApp()

    function createTimestamp(date = new Date()) {
        const pad = (value: number) => String(value).padStart(2, '0')

        return [
            date.getFullYear(),
            '-',
            pad(date.getMonth() + 1),
            '-',
            pad(date.getDate()),
            'T',
            pad(date.getHours()),
            ':',
            pad(date.getMinutes()),
            ':',
            pad(date.getSeconds())
        ].join('')
    }

    function format(input: string, options: {
        groupThreshold: number
        timestamp: string
    }) {
        return $xamlFormatter.format(input, options)
    }

    function downloadText(filename: string, content: string) {
        const blob = new Blob([content], {
            type: 'application/xml;charset=utf-8'
        })

        const url = URL.createObjectURL(blob)
        const link = document.createElement('a')

        link.href = url
        link.download = filename
        link.click()

        URL.revokeObjectURL(url)
    }

    return {
        createTimestamp,
        downloadText,
        format,
        sample: SAMPLE_XAML
    }
}
