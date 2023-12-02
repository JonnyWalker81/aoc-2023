open Ocaml
open Core

let () =
  let contents = In_channel.read_all "./input/01/input.txt" in
  Aoc01.part1 contents
