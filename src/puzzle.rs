use crate::game::Game;

pub fn main(turns: u32) {
    let mut game = Game::default();
    for _ in 0..turns {
        game.do_move();
    }

    let p = game.current_player().clone();
    let rack: String = p.rack.iter().collect();

    let (moves, eval_val) = p.gen_moves(game.get_board_mut());

    let mut s = game.get_board().to_string();
    s = format!("{}{}{}{}", s, "\n", rack, "\n");

    for m in moves.iter().take(50) {
        s = format!(
            "{}{}",
            s,
            format!(
                "{} {} {} {}\n",
                m.position.to_str(m.direction),
                game.get_board_mut().format(&m, true),
                m.score,
                m.eval(1.0, eval_val)
            )
        );
    }

    println!("{}", s);
}
