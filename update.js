import { readFile, writeFile } from 'node:fs/promises';
import { parse, stringify } from 'smol-toml';

const INFO_APP = {
    name: 'solarlauncher',
    full_name: "Solar Launcher",
    version: '0.8.8',
    description: "launching fnf instances from deep insde your folders, now fixed!",
    author: ['Team Solar Engine'],
    identifier: "net.solarengine.solarlauncher",
};

async function updatePackageJson() {
    try {
        const file = await readFile("package.json", 'utf8')
        const fileContent = JSON.parse(file);

        fileContent["name"] = INFO_APP["name"]
        fileContent["version"] = INFO_APP["version"]
        fileContent["description"] = INFO_APP["description"]

        await writeFile("package.json", `${JSON.stringify(fileContent, null, 2)}\n`, 'utf8');
    } catch (e) {
        console.error(e)
    }
}

async function updateTauriJson() {
    try {
        const file = await readFile("src-tauri/tauri.conf.json", 'utf8')
        const fileContent = JSON.parse(file);

        fileContent["productName"] = INFO_APP["full_name"]
        fileContent["app"]["windows"][0]["title"] = INFO_APP["full_name"]
        fileContent["version"] = INFO_APP["version"]
        fileContent["identifier"] = INFO_APP["identifier"]

        await writeFile("src-tauri/tauri.conf.json", `${JSON.stringify(fileContent, null, 2)}\n`, 'utf8');
    } catch (e) {
        console.error(e)
    }
}

async function updateCargoToml() {
    try {
        const file = await readFile("src-tauri/Cargo.toml", 'utf8')
        const fileContent = parse(file);

        fileContent.package["name"] = INFO_APP["name"]
        fileContent.package["version"] = INFO_APP["version"]
        fileContent.package["description"] = INFO_APP["description"]
        fileContent.package["author"] = INFO_APP["author"]
        fileContent.lib["name"] = INFO_APP["name"] + "_lib"

        await writeFile("src-tauri/Cargo.toml", stringify(fileContent), 'utf8');
    } catch (e) {
        console.error(e)
    }
}

await updatePackageJson()
await updateTauriJson()
await updateCargoToml()