import type { Component, ComponentProps } from "svelte"

export type LauncherPluginComponentProps = { input: string }




export type LauncherPlugin = {
  prefix: `:${string}`
  description: string
  component: Component<LauncherPluginComponentProps>
}

export interface LauncherStateConfig {
  plugins: LauncherPlugin[]
}
