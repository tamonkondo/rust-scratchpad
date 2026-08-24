use guessing_game::Pair;

fn main() {
    let xy = Pair {
        x: "Foo",
        y: "Test",
    };
    xy.cmp_display();
}
