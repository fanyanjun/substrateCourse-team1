
use structopt::StructOpt;
use std::fmt;

#[derive(Debug, StructOpt)]
struct CliArgs {
    #[structopt(long)]
    owinship: bool,  //是否测试所有权

    #[structopt(short, long)]
    debug: bool,  //开启debug

    #[structopt(subcommand)]   //子命令
    operation: Option<Ops>,
}

//自定义打印
impl fmt::Display for CliArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "owinship:{},debug:{},operation:{:#?}", self.owinship,self.debug,self.operation)
    }
}

#[derive(Debug, StructOpt)]
enum Ops {
    Iters {
        isbn: Option<bool>,
    },
    Remove {
        isbn: Option<String>,
    },
    Status,
}

fn main() {

    println!("Hello, substrate!");

    let opt = CliArgs::from_args();

    println!("自定义打印 Display: {}", opt);

    let isowinship = opt.owinship;
    if isowinship==true{
         //所有权转移
         let x = String::from("hello substrate");

         println!("x所有权转移给y");
         let y = x;
         println!("y:{}", y);
    }

    println!("iterator test");
    //iterator 还没实现

}
