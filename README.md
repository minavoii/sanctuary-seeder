# Sanctuary Seeder

[<img alt="github" src="https://img.shields.io/badge/github-minavoii/unity--random-8da0cb?labelColor=555555&logo=github" height="20">](https://github.com/minavoii/sanctuary-seeder)

A seed finder and checker for [Monster Sanctuary](https://store.steampowered.com/app/814370/Monster_Sanctuary/).

## About

The game generates a seed used to randomize the contents of the Bravery, Randomizer, and Relics of Chaos game modes.

The monsters you find are different depending on both the seed and which modes you play on: Randomizer, Bravery, or both (relics being generated after them).

This uses a reimplementation of the game's algorithm to generate all possible seeds and stores them in a database (as `seeds.db`).

The database is then queried to find any seed based on criteria, such as where a monster or relic can be found, or whether it is available or not.

## Downloads

| Platform | Link                                                                                                                                                                                                                                                                                                                                                                            |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Windows  | [Setup x64](https://github.com/minavoii/sanctuary-seeder/releases/latest/download/sanctuary-seeder-Windows-x64-setup.exe) / [Portable x64](https://github.com/minavoii/sanctuary-seeder/releases/latest/download/sanctuary-seeder-Windows-x64.zip) / [Portable ARM64](https://github.com/minavoii/sanctuary-seeder/releases/latest/download/sanctuary-seeder-Windows-ARM64.zip) |
| MacOS    | [x64](https://github.com/minavoii/sanctuary-seeder/releases/latest/download/sanctuary-seeder-Darwin-x64.zip) / [Arm64](https://github.com/minavoii/sanctuary-seeder/releases/latest/download/sanctuary-seeder-Darwin-ARM64.zip)                                                                                                                                                 |
| Linux    | [x64](https://github.com/minavoii/sanctuary-seeder/releases/latest/download/sanctuary-seeder-Linux-x64.zip) / [Arm64](https://github.com/minavoii/sanctuary-seeder/releases/latest/download/sanctuary-seeder-Linux-ARM64.zip)                                                                                                                                                   |

If you're unsure which one to get, you're probably looking for the [setup](https://github.com/minavoii/sanctuary-seeder/releases/latest/download/sanctuary-seeder-Windows-x64-setup.exe).

## Screenshots

![A visual of available Bravery eggs](docs/eggs.png?raw=true "Title")

![The seed finder](docs/finder.png?raw=true "Title")
