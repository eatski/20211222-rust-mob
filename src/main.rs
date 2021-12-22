use std::{fs::{read_dir, DirEntry}, io::Error, ffi::OsString};


// 受け入れ基準
// lsコマンドを打つと、現在のディレクトリの中身が表示される
// .から始まるファイルは表示されない

// TODO
// [] ファイルシステムにアクセスする（入力関数）
// [] 出力結果を整形する（整形関数）
// [] 標準出力で表示する（出力関数）

fn input() -> Result<Vec<DirEntry>, std::io::Error>  {
    let entries: Vec<_> = read_dir(".")?.map(
        |entry| entry.expect("failed to read entry")
    ).collect();

    Ok(entries)
}

fn format(entries: Vec<DirEntry>) -> String {
    let entries: Vec<String> = entries.iter().map(|entry| entry.file_name().into_string().expect("failed to convert string")).collect();
    entries.join(" ")
}

fn output(str: String) {
    println!("{}",str)
}

fn main() -> Result<(), std::io::Error> {
    
    let entries = input();
    // println!("{:?}",entries);
    if let Ok(res) = entries {
        let formated = format(res);
        println!("{}", formated);
    } else if let Err(e) = entries {

    }


    Ok(())
}