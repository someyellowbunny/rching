use gram::Hexagram;

mod cast;
mod yinyang;
mod line;
mod gram;

fn main() {
    let h = Hexagram::new();

    if !h.pure() {
        let changes = !h;
        println!("{}\n{}\n\n{}\n{}", h, h.name(), changes, changes.name());
    } else {
        println!("{}\n{}", h, h.name());
    }
}
