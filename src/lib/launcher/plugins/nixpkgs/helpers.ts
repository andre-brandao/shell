import { Command } from "@tauri-apps/plugin-shell";

export type NhSearchResult = {
  package_attr_name: string;
  package_attr_set: string;
  package_pname: string;
  package_pversion: string;
  package_platforms: string[];
  package_outputs: string[];
  package_default_output: string;
  package_programs: string[];
  package_license_set: string[];
  package_description: string;
  package_longDescription: string | null;
  package_hydra: string | null;
  package_system: string;
  package_homepage: string[];
  package_position: string;
};

export type NhSearchResponse = {
  query: string;
  channel: string;
  elapsed_ms: number;
  results: NhSearchResult[];
};

export async function launchApp(app: NhSearchResult) {
  let result = await Command.create("exec-sh", [
    "-c",
    `nix run nixpkgs#${app.package_attr_name}`,
  ]).execute();
  console.log(result);
}

export async function searchNixpkgs(query: string = ""): Promise<NhSearchResponse> {

  let result = await Command.create("exec-sh", [
    "-c",
    `nh search --limit 25 --json "${query}"`,
  ]).execute();

  // if(result.stderr)

  return JSON.parse(result.stdout) as NhSearchResponse;

}
