# aoc-helper
aoc-helper is my Advent Of Code cli helper program.

# Usage
Why would you want to do that?

## install
you'll figure it out

## create ~/.config/aoc-helper/config.toml
or these on macos and windows but I didn't and won't test them
`$HOME/Library/Application Support/dev.Benedek03.aoc-helper`
`{FOLDERID_RoamingAppData}\Benedek03\aoc-helper\config`
config.toml must contain:
```
session="<your session cookie>"
root_dir="<the path to your solutions>"
```
aoc-helper will create `<year>/<day>/` subdirectories in root_dir and save your inputs there. 

## aoc-helper help
```
Usage: aoc-helper [OPTIONS] [COMMAND]

Commands:
  fetch   downloads input
  submit  submits an answer TODO
  open    opens the latest puzzle in the default browser
  help    Print this message or the help of the given subcommand(s)

Options:
  -y, --year <YEAR>  [default: latest]
  -d, --day <DAY>    [default: latest]
  -h, --help         Print help information
  -V, --version      Print version information
```