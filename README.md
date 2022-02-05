## Wordle-RS
A program to solve any [Wordle](https://www.powerlanguage.co.uk/wordle/) puzzle, written in Rust.

### How to use
The basic loop goes like this:
1. Program comes up with a word
2. You pass along the 'code' that Wordle gives you
   1. Wordle syntax goes like this (case in-sensitive):
   
        🟩: O

        🟨: -

        ⬛: X

        i.e. a wordle response of 🟩⬛⬛🟨⬛ becomes OXX-O
3. Repeat

#### The program does not always get the answer within 6 tries, but it does get the answer
