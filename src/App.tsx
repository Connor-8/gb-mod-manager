import { createEffect, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { readDir, BaseDirectory, FileEntry, exists } from "@tauri-apps/api/fs";
import { watch, watchImmediate } from "tauri-plugin-fs-watch-api";
import "./App.css";

function App() {
  // const [greetMsg, setGreetMsg] = createSignal("");
  const [entries, setEntries] = createSignal<FileEntry[]>([]);
  const [mods, setMods] = createSignal<string[]>();

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name: name() }));
  // }

  // Reads the `$APPDATA/users` directory recursively

  async function processEntries() {
    console.log("Bruh");
    const e = entries();
    for (const entry of e) {
      console.log(`Entry: ${entry.path}`);
    }
  }

  const mods_entry_point =
    "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Ground Branch\\GroundBranch\\Content\\GroundBranch";
  //C:\Program Files (x86)\Steam\steamapps\common\Ground Branch\GroundBranch\Content\GroundBranch\Patches\Custom
  // gb file path `C:\\Program Files (x86)\\Steam\\steamapps\\common\\Ground Branch`
  createEffect(async () => {
    setEntries(entries);
    invoke("check_if_dir_exists", {
      path: mods_entry_point,
    }).then((res) => console.log("Completed!", res));

    invoke("read_mods_dir").then((res) => {
      console.log("Completed!", res);
      setMods(res);
    });

    invoke("get_inventory_category_dirs").then((res) => {
      console.log("Inventory Categories!", res);
    });


    //  const entires = await readDir("src\\mods". {dir: 'src\\mods'})
    //  console.log(entires)
  });

  const handleReadModFiles = (modName: string) => {
    invoke("copy_mod", { modName }).then((res) => {
      console.log("Completed!", res);
    });
  };

  return (
    <div class="container">
      <h1>Ground Branch Mod Manager</h1>
      <div class="row">
        <button onClick={processEntries}>+Load New Mod</button>
      </div>
      {mods()?.map((mod) => (
        <div class="row">
          <div class="col">{mod}</div>
          <div class="col">
            <button onClick={() =>handleReadModFiles(mod)}>Enable</button>
            <button>Disable</button>
            <button>Remove</button>
          </div>
        </div>
      ))}
    </div>
  );
}

export default App;
