let input = System.IO.File.ReadAllLines("./input.txt")

let splitted = input |> Array.map (fun x -> x.Split(' '))

// print input
printfn "%A" input[7]
