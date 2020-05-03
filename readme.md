# Yew doc examples

project to collect some examples used for the documentation

## Pages

- [ ] page 1: simple component
- [ ] page 2: internal state

- [ ]  

## How to use

> Rust & wasm are dependencies for this project

- download the project

```bash
git clone https://github.com/rlasjunies/yew-project-template

```

- build the spa project

In another command line

```bash
wasm-pack build --debug --no-typescript --out-name spa --target web
```

- build the server project

> important! do not start the server before the **end** of the compilation of the spa. If not, an error is shown starting the server ... ~ path not found ... and the spa will not be loaded

```bash
cd server
cargo run
```

- open your webbrowser at the URL <http://127.0.0.1::8080>

## Reminder

If you are using vs-code with the `rls-vscode` extension, do not forget to add the folder in vscode `Workspace`

> `File > Add Folder to Workspace...` select the main folder of the template not `spa` neither `server`
