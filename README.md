<p align="center">
  <img src="./.github/assets/gethere-logo-700x160.png" alt="gethere logo" />
</p>

# gethere Server

A backend for public transport app. Part of **the gethere project**.

![License](https://img.shields.io/github/license/krystejj/gethere-server?label=License)
![Latest Release](https://img.shields.io/github/v/release/krystejj/gethere-server?label=Latest%20Release)
![Issues](https://img.shields.io/github/issues/krystejj/gethere-server?label=Issues)
![Pull Requests](https://img.shields.io/github/issues-pr/krystejj/gethere-server?label=Pull%20Requests)
![Discussions](https://img.shields.io/github/discussions/krystejj/gethere-server?label=Discussions)

> [!CAUTION]
> **This software is in VERY, and I really mean VERY early stage of development!** It still doesn't have the most basic features and could even not work at all.

**gethere server** is a modern, free and open-source backend for public transportation app, as a part of **the gethere project**. It hosts an API which frontends can use. gethere allows you to easily navigate public transportation around the city, giving you the ability to search and plan your route, track live vehicles, check schedules and much more. gethere aims to be as user-friendly and powerful as possible. You can host it entirely on your own.

## 💻 Clients / Frontends

Currently only one main client is in development, that is the [Svelte frontend](https://github.com/krystejj/gethere-svelte-frontend). However, client for Android and iOS is also planned, but web frontend is one with highest priority for now because of very early stage of development. You can create a new client by yourself if you like.

In development, unstable:

- [Svelte frontend](https://github.com/krystejj/gethere-svelte-frontend) - web, main

## 💾 Available Data

For now, the main focus is on [Warsaw](https://en.wikipedia.org/wiki/Warsaw) because it is my city of residence and capital of my country, [Poland](https://en.wikipedia.org/wiki/Poland). Support for other cities is also planned, so Warsaw wouldn't be only city that will be available in gethere. Below you can see what data is available and what is not.

| _>> Warsaw <<_             | Bus | Tram | Metro | Suburban Train |
| -------------------------- | --- | ---- | ----- | -------------- |
| **Timetables**             | ✔️  | ✔️   | ✔️    | ✔️             |
| **Live Vehicles Location** | ✔️  | ✔️   | ❌    | ✔️             |

## 📖 Project Management

All notable changes to this project will be documented in the [changelog file](CHANGELOG.md). The format of that file is based on [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/).

This project adheres to [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html).

If you would find any bug, some type of issue or you have an idea for improving a project, you can file a report using [issues page](https://github.com/krystejj/gethere-server/issues) in this repo or create a [pull request](https://github.com/krystejj/gethere-server/pulls). [Discussions](https://github.com/krystejj/gethere-server/discussions) are also available.

Information about security lies in [security policy file](SECURITY.md).

## 🙏 Used Projects and Credits

This is a list of projects used in development of this project:

- [Rust](https://www.rust-lang.org/) - an incredibly fast and reliable programming language that guarantees memory and thread safety.
- [Actix](https://actix.rs/) - a powerful and extremely fast web framework for Rust.

💗 Big thanks to the creators and all contributors of these projects.

## 📜 License

This project is provided under the terms of the **GNU General Public License v3.0**, a free and open-source license. For more information, see the [license file](LICENSE.md).
