const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;
function cl(text){console.log(text)}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

function syncfolder(info){
  document.querySelector('#syncfolder').innerHTML = "<legend>SyncFolder</legend>"
  info.forEach(element => {
    const block = document.createElement('div');
    block.className = "block"


    const checkbox = document.createElement('input');
    checkbox.type = 'checkbox';
    checkbox.id = 'myCheckbox';
    checkbox.name = 'myCheckbox';
    checkbox.checked = true;
    block.append(checkbox)

    const name = document.createElement('div');
    name.textContent = element.split("/")[1]
    cl(element.split("/")[1])
    block.append(name)

    const container = document.querySelector('#syncfolder');
    container.appendChild(block);  
  });
  
}

function loot_val(info){
  let target = info.split('\\')
  document.querySelector("#target").textContent = target[target.length - 1];
}

function config(){
  invoke('config_setup')
  invoke('start').then(loot => {loot_val(loot['target']); syncfolder(loot['folder']); cl(loot['target'])})
}
window.addEventListener("DOMContentLoaded", () => {
  invoke('start').then(loot => {
    loot_val(loot['target'])
    syncfolder(loot['folder'])
  })
});
