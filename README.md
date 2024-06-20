# Worldtree SDK

The Worldtree SDK comprises the Worldtree compiler, CLI, and browser packaging: everything you need to turn a directory of YAML files into a narrative game you can publish on the web.

## About the code

The code in this repository is under active development, as I work toward version 1.0. Until that time, the Worldtree SDK is available as a **technology preview** of the Worldtree Engine. **It's suitable for making small games for fun, but not stable enough for a serious project.** I hope to reach 1.0 by **Fall 2024**.

The compiler and CLI are written in Rust. This is my first real Rust project, and I'd appreciate any advice on how to improve this code.

The browser packaging is written in TypeScript, with UI components written using React and packaged using ESBuild.

Both the CLI/compiler and the browser packaging are broken up into many internal modules and libraries, but these libraries are not meant for public consumption **yet**. The APIs are not stable, and they are only able to interoperate because they are all being developed together inside this repository. The structure of this repository itself and the number of repositories making up the Worldtree SDK and engine components are not themselves stable.

## Building from source

The best way to get started using the SDK is by downloading a compiled binary or installer. If you'd like to build this source code yourself, you'll need to first install a couple of prerequisites:

- Node v20.x with Corepack enabled
- Rust 1.77+

Inside the `engine` directory, install Node dependencies:

```
pnpm install
```

Inside the `cli` directory, compile the CLI binary:

```
cargo build
```

## Contributing

The best way to contribute at this time is to report bugs and submit feature requests via GitHub Issues.

## License

Copyright 2024 Doug Valenta.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see [https://www.gnu.org/licenses/](https://www.gnu.org/licenses/).