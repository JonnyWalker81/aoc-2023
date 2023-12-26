let string_to_list s = List.init (String.length s) (String.get s)

let is_digit = function
  | '0' .. '9' -> true
  | _ -> false
;;

let find_digit line rev = line

let process line =
  let first = List.find is_digit (string_to_list line) in
  let rev_line = List.rev (string_to_list line) in
  let last = List.find is_digit rev_line in
  let s = Fmt.str "%c%c" first last in
  s
;;

let process2 line =
  let first = find_digit line false in
  let last = find_digit line true in
  let s = Fmt.str "%c%c" first last in
  s
;;

let part1 input =
  let lines = Str.split (Str.regexp "[ \n]+") input in
  let vals = List.map process lines in
  let sum = List.fold_left (fun acc l -> acc + int_of_string l) 0 vals in
  Fmt.pr "%d\n" sum
;;

let part2 input =
  let lines = Str.split (Str.regexp "[ \n]+") input in
  let vals = List.map process2 lines in
  let sum = List.fold_left (fun acc l -> acc + int_of_string l) 0 vals in
  Fmt.pr "%d\n" sum
;;
