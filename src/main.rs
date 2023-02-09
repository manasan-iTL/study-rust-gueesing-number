use std::io; // std＝標準ライブラリ > io(入出力のライブラリ)

// main()→エントリポイント
fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    // 変数はデフォルトではImmutable
    // = →　束縛（代入ではない）　guessはStringのnew()で生成される値に束縛される
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess)
}
