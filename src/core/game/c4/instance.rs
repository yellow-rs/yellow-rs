pub struct C4Instance<Board: BoardPlayable> {
    msg: Message,      // Message to manipulate
    http: Arc<Http>,   // Http object to interact with message
    board_data: Board, // Board data wrapper
    board_canvas: ImageSurfaceWrapper,
    players_pair: [User; 2],
    avatars: [ImageSurfaceWrapper; 2],
    turns: u8,
    over: bool,
}
