with builtins;
let 
    lib = import <nixpkgs/lib>;
    file_name = ./input.txt;
    input = readFile file_name;
    second = l: elemAt l 1;
    lines = filter (x: x != []) (map (x: map lib.strings.toInt (filter (x: x != "") (lib.strings.splitString " " x))) (lib.strings.splitString "\n" input));
    inside_out = l: [(map head l) (map second l)];
    locations = inside_out lines;
    sorted = {left = sort lessThan (head locations); right = sort lessThan (second locations);};
    abs = n: if n < 0 then -n else n;
    zip = l1: l2: if l1 != [] && l2 != [] then [[(head l1) (head l2)]] ++ (zip (tail l1) (tail l2)) else []; 
    distances = map (x: abs ((head x) - (second x))) (zip sorted.left sorted.right);
    sum = lib.lists.foldl add 0;
    similar = map (x: (mul x (lib.count (y: x == y) sorted.right))) sorted.left;
in 
{
    part1 = sum distances;
    part2 = sum similar;
}
