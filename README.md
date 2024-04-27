# Leptos CSR proxying requests to server

## Problem Being Solved

Working on [Lemmy UI's Leptos reimplementation](https://github.com/LemmyNet/lemmy-ui-leptos), I have to make code that works running in multiple contexts. To borrow a point from that project's README:

> use isomorphic Leptos code to ensure that features work in the following contexts:
>
> - SSR only - server side rendering only. Search engine bots and browsers with diverse technical requirements (JS and WASM are disabled) must be able to read and interact with all core features. There will be sophisticated (non-core) features where we agree this is not possible
> - Hydrate - features progressively enhance from being rendered on the server to running almost entirely in the browser (JS and WASM are available). Feature logic must handle this context switch gracefully
> - CSR only - client side rendering only - when a mobile/desktop app framework target is agreed upon (e.g. Tauri) all UI and interaction code is bundled into an app that communicates directly with its Lemmy instance

Looking at the Leptos examples, I found some that worked in 1 or 2 of these contexts, but none that worked in all 3. Further, when trying to implement this, I found it annoying needing to write both server actions and locally run submit handlers. Wouldn't it be great, I thought, if I could only just write the server actions and have them work in all contexts? After some discussion with [gbj](https://github.com/gbj) and a few other people in the Leptos discord server, I learned that I can still have my serverfns hit my actual server even if using CSR instead of hydrated SSR. This still left me with an issue.

[Lemmy](https://github.com/LemmyNet/lemmy) is set up in such a way that the server doesn't really care what frontend is used, and its APIs are set up to use JSON instead of multipart/form-data, which is what `ActionForm` uses. As such, the server for `lemmy-ui-leptos` is only used for 3 things:

1. Server side rendering HTML to send to the browser.
2. Using server fns that can handle multipart/form-data from form submissions and make appropriate requests to the backend server.
3. Managing the session with an HTTP only cookie.

When the app is running in CSR mode with trunk, it only cares about point 2 (it still uses 3, but the cookie doesn't necessarily need to be HTTP only in CSR mode).

I want to make my server functions work across all 3 uses cases, but I can't do that in a way that I can directly make requests to the backend server in all cases. What is to be done.

## My Solution

I solved this by using [Trunk's proxy](https://trunkrs.dev/configuration/#proxy). Expressed simply:

1. I run my SSR + hydrate server with `cargo leptos serve` at localhost:3000.
2. I set up the proxy to forward requests made to server fns to the Leptos SSR server.
3. I run the app in another terminal window with `trunk serve --watch .` at localhost:4000.

The CSR app then works the same as it would in SSR or hydrated.

## Testing with this repo

To test SSR and hydrated, just run `cargo leptos serve` and go to localhost:3000. Notice that the counter works both when JavaScript is enabled and when it is disabled. To test CSR, keep the previous server running, open another terminal window, and run `trunk serve --watch .`. You can then use the CSR app at localhost:4000.

## Difficulties Encountered

There are 2 major difficulties I came across when working on this.

The first you likely already noticed. Currently, the only way I know how to make this work is by running a separate terminal for the cargo leptos server and the trunk server. Running `cargo leptos serve &` did not cause it to run in the background like I expected, making it so I couldn't handle this as a hook specificed in Trunk.toml.

The second is more subtle and more annoying. When running cargo leptos and trunk at the same time, 2 wasm files get generated. With the way this project is setup, the names are `counter_isomorphic` (for the hydrate) and `counter-isomorphic` (for CSR). Yes, it is confusing. Getting this to work required making sure to run the 2 servers using nightly rust and specifying `data-target-name` in index.html. Annoyingly, the `output-name` option for cargo leptos doesn't actually seem to set the output name, making using the same name but with hyphens in one case and underscores in the other neccessary.

## Further improvements

Since none of the server rendering logic is needed in the server the app proxies requests to when running in CSR, it's worth config gating that logic.
