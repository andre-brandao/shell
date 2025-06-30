import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";


type ActiveClient = {
  class: string;
  title: string;
}
export const activeClient: ActiveClient = $state({
  class: "",
  title: ""
})



export const init = async () => {
  const unlistenAC = await listen<{ class: string; title: string } | null>("active-client", (event) => {
    if (event.payload) {
      activeClient.class = event.payload.class;
      activeClient.title = event.payload.title;
    }
  })
  return () => {
    unlistenAC()
  }
}
