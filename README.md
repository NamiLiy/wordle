# wordle guess
a simple rust program to find the next best word for wordle based on existing guesses

## Run the code
```
cargo +nightly run --release
```
Play the word suggested by the app in wordle and enter the result a 5 charachter string (b for black square, y for yello square and g for green square)

### Sample
```
Potential words remaining : 12972
Play the word : Some(WordValue { word: "tares", value: 37331 })
Enter result
bbyyb
Potential words remaining : 243
Play the word : Some(WordValue { word: "irone", value: 497 })
Enter result
ygbgg
Potential words remaining : 3
Play the word : Some(WordValue { word: "brick", value: 6 })
Enter result
gggbb
Potential words remaining : 3
Play the word : Some(WordValue { word: "brick", value: 6 })
Enter result
gggbb
Result obtained in 4 tries
```
