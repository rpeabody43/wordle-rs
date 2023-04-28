# wordle-rs
A program to solve any [Wordle](https://www.nytimes.com/games/wordle/index.html) or [Survivle](https://lazyguyy.github.io/survivle/) puzzle, written in Rust. Now with a web version [here!](https://rpeabody43.github.io/wordle-rs/)

## Web App
### Usage
Upon visiting the GitHub Pages site, you'll be greeted by a toggle switch for the best guesses or worst guesses, and the corresponding starting word based on that switch.

The color of each letter can be changed by clicking, once for yellow, twice for green. Change the color of each letter accordingly with Wordle-Survivle and click "Next Word". Repeat until you've come up with an answer.


#### Why did the "Next Word" button disappear?
This can happen for two reasons:

1. You reached the answer, awesome!
2. The solver ran out of possible words that fit the criteria. This can either happen because the correct word isn't in my list, or you entered conflicting codes (i.e. marking the same letter green and then gray in the next word).

## Console App
### Installation
Navigate to the 'Releases' tab on GitHub, download the wordle-rs executable and run it.

At the time of writing this README the latest release is v1.0.2

### Usage
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

As with the web app, the program may exit early if conflicting information is given or the answer isn't in the list.

## Project Outline
- The source code is located in `./src`, with `main.rs` as the entrypoint and `console.rs` and `webapp.rs` for their respective frontend methods.
- The main solver instance logic is located in `/game/solve.rs`.
