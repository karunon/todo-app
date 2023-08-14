use std::collections::HashMap;

// make structure
struct Todo {
    map: HashMap<String, bool>,
    
    // Example:
    // {
    //     "Learning rust": true,
    //     "Shopping": true
    // }
}

// make Todo's methods
impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    // // この引数が実体なのは、selfの所有権を持つことで、他プログラムによる更新を防ぐため？
    // fn save(self) -> Result<(), std::io::Error> {
    //     let mut content = String::new();
    //     for (k, v) in self.map {
    //         let record = format!("{}\t{}\n", k, v);
    //         content.push_str(&record);
    //     }
    //     std::fs::write("./db.txt", content)
    // }
    // Boxはヒープ領域に値を確保する方法（＝ヒープ上に置かれた値へのポインタ）
    // can't understand to return file system's error or Serde's error
    //  So, return error's pointer. Also, dyn is trait.
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("./data/db.json")?;
        // 第１引数でwrite(true)を確認し、第２引数で構造をjsonとして出力
        serde_json::to_writer_pretty(content, &self.map)?;
        Ok(())
    }

    fn new() -> Result<Todo, std::io::Error> {
        // OpenOptions -> false(default)
        // 最後の「?」：エラー処理の一種。
        //      ・Resultを返す関数内でしか使えない
        //      ・match式とほぼ同じ動作
        //      ・ResultがOkならOkの中身を、Errならreturn処理でエラーは呼び出し元のコードに委譲する
        let list = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("./data/db.json")?;
        // let mut content = String::new();
        // // listの全ファイルの中身をcontentに書き出す
        // list.read_to_string(&mut content)?;
        // let map: HashMap<String, bool> = content
        //     // 各行に対するイテレータ（集合したデータ構造に対する繰り返し処理）を作成
        //     .lines()
        //     // クロージャー（|hogehoge|のこと）
        //     //      その場限りの用途で使用。関数の呼び出しとまったく同じ。入力変数は「||」で囲む
        //     //      外部の環境にある変数を補足することができる
        //     // line.splitn(2, '\t') -> tab文字で2つに分ける。collectは分割した文字をVector型に変換
        //     .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        //     // 先ほどのVector型をタプルに変換
        //     .map(|v| (v[0], v[1]))
        //     // kをString型に。vをbool型に
        //     .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
        //     .collect();
        // // Todo型構造体のreturn
        // Ok(Todo {map})
        match serde_json::from_reader(list) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok (Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        // keyがあればそれに対応しているbool型をfalseに
        // Option型はSomeとNone(何も値はない)
        match self.map.get_mut(key) {
            // *vは参照外し
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    // nth(0) is a program. nth(1) is a first args.
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    // println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialisation of db failed");

    // let mut todo = Todo {
    //     map: HashMap::new(),
    // };

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        // 元の所有者はsave(main関数ではなく？)だが、この瞬間だけcomplete()がitemを借りている
        match todo.complete(&item) {
            // itemを利用できる
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            }
        }
    }
}
