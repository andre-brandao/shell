import type { Component, ComponentProps } from "svelte"

export type LauncherPluginComponentProps = { input: string }

export type LauncherPluginExports = {
  onEnterPressed?: () => void | Promise<void>
  onInputChanged?: (input: string) => void | Promise<void>
}

export type LauncherComponent = Component<LauncherPluginComponentProps, LauncherPluginExports>

export type LauncherPlugin = {
  prefix: `:${string}`
  description: string
  component: LauncherComponent
}

export interface LauncherStateConfig {
  plugins: LauncherPlugin[]
}
