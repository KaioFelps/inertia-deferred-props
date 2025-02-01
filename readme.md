# Testing Deferred Props

Until the version being tested here (2.0.3), There is a problem with inertiajs/react `Deferred` component.

This repo contains a custom `Deferred` which fixes this issue by:
1. memoizing `keys` (to avoid many changes to `loaded` state);
2. setting `loaded` back to `false` everytime a navigation starts.

## The Problem
The "/built-in" route uses the Inertia built-in `Deferred` component.
By clicking on the button, it will perform a post request to a ping pong route (which only returns back).
This is enough to trigger the error, as stated in [this issue](https://github.com/inertiajs/inertia/issues/2221).

At "/custom", by clicking the button and nothing but the expected behavior from `Deferred` happens.

It looks like it only happens with the React Adapter. Svelte and Vue `Deferred` components seem to work just
as expected.

## Running
To run this project, you'll need to have both Rust and Cargo installed. Then, follow the steps:

Copy the environment variables:
```bash
cp .env.example .env
```

In one terminal, type:
```bash
# install node modules and start the vite dev server
npm i && npm run dev
```

Start the application in another terminal:
```bash
cargo run
```

## Switching between React, Vue and Svelte
Change the `CLIENT` key from your environment variables to `svelte` to enable Svelte template engine.
Set it to `vue` and restart to enable Vue template engine. Remove it or change to `react` to use React
as the template engine.

Restart the Actix Web server everytime you change this key, so that it recompiles the root template.
