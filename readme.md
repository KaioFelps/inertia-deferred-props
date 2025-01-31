# Testing Deferred Props

Until the version being test here (2.0.3), Inertia has a problem with its `Deferred` component (at least, for React).

This repo contains a custom `Deferred` component that fixes this issue by:
1. memoizing `keys` (to avoid many changes to `loaded` state);
2. setting `loaded` back to `false` everytime a navigation starts.

## The Problem
The "/built-in" route is built using the Inertia built-in `Deferred` component (from the React package).
By clicking on the button, it will perform a post request to a ping pong route, which only returns back.
This is enough to trigger the error, as stated in [this issue](https://github.com/inertiajs/inertia/issues/2221).

At "/custom", you can click the button and check that nothing actually happen, but the expected behavior
from the `Deferred` component.

## Running
To run this project, you'll need to have both Rust and Cargo installed. Then, follow the steps:

In one terminal, type:
```bash
# start the vite dev server
npm i && npm run dev
```

Start the application in another terminal:
```bash
cargo run
```
