[![Netlify Status](https://api.netlify.com/api/v1/badges/14e21263-4c90-4e21-a45a-7d69d9dbd6a8/deploy-status)](https://app.netlify.com/sites/rust-netlify-function-neon/deploys)

## Rust Netlify Function ([neon][neon])

We'll use `yarn` and `yarn workspaces` to build our functions in their own directories.

[Tutorial can be found here][tutorial]

[![](https://www.netlify.com/img/deploy/button.svg)][deploy]

This project uses [Rust][rust] and requires you have it installed to build the rust hello library. The deploy will work without you installing rust, but you will not be able to modify until you can compile your rust library and commit back to your repository.

[deploy]: https://app.netlify.com/start/deploy?repository=https://github.com/talves/rust-netlify-function-neon
[rust]: https://www.rust-lang.org/
[tutorial]: https://tony.alves.dev/garden/rust-netlify-function-neon
[neon]: https://neon-bindings.com/
