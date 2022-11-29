# aoc-helper
aoc-helper `will be` an Advent Of Code helper cli program. aoc-helper makes it easier to download inputs, etc.

# The idea
`~/.config/aoc-helper/aoc-helper.something`
- cookie
- path to the root dir of the solutions 
- default output: /year/day/input.txt
- default year: latest
- default day: latest
<!-- default `/year/day/part(1:2).lang` -->

`~/.config/aoc-helper/aoc-helper-data.something`
- saves which parts are solved

```
aoc-helper fetch [-d --day <day>] [-y --year <year>] [-c --cookie <path >] [-o --output <file path>]
```
downloads input
```
aoc-helper submit [-d --day <day>] [-y --year <year>] <part> <[-a --answer <answer>] [-c -code <path to source code>]>
```
submits the answer or the stdout of the code ran with the input
```
aoc-helper run <path to source code>
```
compiles and runs the program with the input 
```
aoc-helper open [-d --day <day>] [-y --year <year>] 
```
opens the task 