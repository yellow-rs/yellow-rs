#[derive(Debug, Clone)]
struct ImageSurfaceWrapper(ImageSurface);

fn canvas_init() -> ImageSurfaceWrapper {
    let mut board = std::fs::File::open("assets/images/board7x6.png").unwrap();
    ImageSurfaceWrapper(ImageSurface::create_from_png(&mut board).unwrap())
}

impl Default for ImageSurfaceWrapper {
    fn default() -> Self {
        Self(ImageSurface::create(Format::Rgb30, 128, 128).unwrap())
    }
}

async fn update_canvas(&mut self, pos: [usize; 2]) -> String {
    const ROW: [f64; 6] = [32., 95., 157., 219., 282., 345.];
    const COLUMN: [f64; 7] = [39., 104., 169., 234., 300., 365., 430.];

    let ctx = cairo::Context::new(&self.board_canvas.0);

    ctx.new_path();
    ctx.arc(COLUMN[pos[0]], ROW[pos[1]], 31.75, 0.0, PI * 2.0);
    ctx.close_path();
    ctx.clip();

    ctx.set_source_surface(
        &self.avatars[(self.turns % 2) as usize].0,
        COLUMN[pos[0]] - 32.,
        ROW[pos[1]] - 32.,
    );

    ctx.paint();

    let msg_id = format!("{}.png", self.msg.id.0);
    let mut file = std::fs::File::create(&msg_id).expect("Couldn't create file.");

    self.board_canvas
        .0
        .write_to_png(&mut file)
        .expect("Couldn’t write to png");

    msg_id
}

async fn send_msg(&self, file: &tokio::fs::File) {
    let _ = ChannelId(617407223395647520)
        .send_message(&self.http, move |m| {
            m.content(self.msg.id.0).add_file(AttachmentType::File {
                file,
                filename: "any.png".to_string(),
            })
        })
        .await;
    tokio::fs::remove_file(format!("{}.png", self.msg.id.0))
        .await
        .unwrap();
}

async fn grab_user_avatar(&mut self, player: usize) -> ImageSurfaceWrapper {
    let face = &self.players_pair[player].face();
    let avatar_url = format!("{}.png?size=64", face.rsplitn(2, ".").nth(1).unwrap());

    let res = reqwest::get(&avatar_url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    ImageSurfaceWrapper(ImageSurface::create_from_png(&mut res.reader()).unwrap())
}
