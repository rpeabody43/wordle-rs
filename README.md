## Wordle-RS
A program to solve any [Wordle](https://www.powerlanguage.co.uk/wordle/) puzzle, written in Rust.


### Installation
Navigate to the 'Releases' tab on GitHub and install the wordle-rs zip file, extract it and run the exe inside

At the time of writing this README the latest release is v0.1.0

### How to use
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

#### The program does not always get the answer within 6 tries, but it does get the answer
