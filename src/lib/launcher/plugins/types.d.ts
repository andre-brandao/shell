import type { Component, ComponentProps } from "svelte"

export type LauncherPluginProps = { input: string }

export type LauncherPluginExports = {
  onEnterPressed?: () => void | Promise<void>
  onInputChanged?: (input: string) => void | Promise<void>
}

export type LauncherComponent = Component<LauncherPluginProps, LauncherPluginExports>

export type LauncherPlugin = {
  prefix: `:${string}`
  description: string
  component: LauncherComponent
}

export interface LauncherStateConfig {
  plugins: LauncherPlugin[]
}
