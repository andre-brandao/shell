import type { AppDetails } from "$lib/types";
import { Command } from "@tauri-apps/plugin-shell";


export async function launchApp(app: AppDetails) {
  console.log(app);
  let result = await Command.create("exec-sh", [
    "-c",
    app.commandline,
  ]).execute();
}
