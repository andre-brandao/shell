
import { tick, untrack } from "svelte";

// could also be a promise
type SetupFn = () => void | (() => void);
// type WacherSetup = SetupFn | Promise<SetupFn>;
export function createKeyedWatcher() {
  let watchers = new Map();

  return {
    watch(setup: SetupFn) {
      if ($effect.tracking()) {
        $effect(() => {
          let entry = watchers.get(setup);
          if (!entry) {
            console.log("Setup watcher", setup);
            const cleanup = untrack(setup);
            entry = [0, cleanup];
            watchers.set(setup, entry);
          }
          entry[0]++;

          return () => {
            tick().then(() => {
              entry[0]--;
              if (entry[0] === 0) {
                console.log("Cleanup watcher", setup);
                entry[1]?.(); // Run cleanup
                watchers.delete(setup);
              }
            });
          };
        });
      }
    },
  };
}
