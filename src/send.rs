use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // { }で囲っていることにより、'rc' が'.await'の前にdropする
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }
        // 'rc' はもはや使用されない
        // タスクがスケジュールあに戻る時には破棄されている
        yield_now().await;
    });
}
