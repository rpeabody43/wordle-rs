## Wordle-RS
A program to solve any [Wordle](https://www.nytimes.com/games/wordle/index.html) or [Survivle](https://lazyguyy.github.io/survivle/) puzzle, written in Rust.


### Installation
Navigate to the 'Releases' tab on GitHub and install the wordle-rs zip file, extract it and run the exe inside

At the time of writing this README the latest release is v1.0.0

### How to use
There are two modes, Wordle solver and Survivle solver.
The Wordle solver attempts to guess the answer in as *few* tries as possible,
the Survivle solver attempts to guess as *many* times as possible without getting the answer.

The basic loop goes like this:
1. Program comes up with a word
2. You pass along the 'code' that Wordle gives you
   1. Wordle syntax goes like this (case in-sensitive):
   <pre>
   🟩 (Green) : O

   🟨 (Yellow): -

   ⬛ (Gray)  : X

   i.e. a wordle response of 🟩⬛⬛🟨⬛ becomes OXX-O
   </pre>
3. Repeat

### Project Outline
- The source code is located in ./src, with `main.rs` as the entrypoint
