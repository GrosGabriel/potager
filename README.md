# This is a project for a gardening app.

The app will be used locally.
You can add cultures you planted, to keep track of when you did it. You can add notes and pictures and see statistics about your garden.

## Build

The app uses tauri, and some rust inside tauri to build specific database (SQLite) calls.
The frontend is made in svelte.


## How to use

The command below is used for development.

```bash
npx tauri dev
```

You can build the project with : 

```bash
npx tauri build
```

The build is found in `src-tauri/target/release`. The executable app can be launched from any folder, it launches the application and creates a database `potager.db` and a folder `images`.