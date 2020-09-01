use cairo::{Context, Format, ImageSurface};

use bytes::buf::BufExt;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct ImageSurfaceWrapper(ImageSurface);

pub struct C4GFX {
    board_canvas: HashMap<u64, ImageSurfaceWrapper>,
    avatars: HashMap<u64, ImageSurfaceWrapper>,
}

impl C4GFX {
    pub fn canvas_init(&mut self, id: u64) {
        let mut board = File::open("assets/images/board7x6.png").unwrap();
        let img = ImageSurfaceWrapper(ImageSurface::create_from_png(&mut board).unwrap());
        self.board_canvas.insert(id, img);
    }

    // Downloads the avatar if it doesn't exist in the cache yet
    async fn grab_user_avatar(&mut self, face: &str, player: u64) -> Option<()> {
        let avatar_url = format!("{}.png?size=64", face.rsplitn(2, ".").nth(1)?);

        let res = reqwest::get(&avatar_url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        self.avatars.insert(
            player,
            ImageSurfaceWrapper(ImageSurface::create_from_png(&mut res.reader()).unwrap()),
        )?;

        Some(())
    }

    pub async fn generate_board(&mut self, id: u64, pos: [usize; 2]) -> Option<()> {
        const ROW: [f64; 6] = [32., 95., 157., 219., 282., 345.];
        const COLUMN: [f64; 7] = [39., 104., 169., 234., 300., 365., 430.];

        let board = self.board_canvas.get(&id)?;

        let ctx = Context::new(&board.0);

        ctx.new_path();
        ctx.arc(COLUMN[pos[0]], ROW[pos[1]], 31.75, 0.0, PI * 2.0);
        ctx.close_path();
        ctx.clip();

        /*
        ctx.set_source_surface(
            //&self.avatars[(self.turns % 2) as usize].0,
            &ava.0,
            COLUMN[pos[0]] - 32.,
            ROW[pos[1]] - 32.,
        );

        ctx.paint();

        let id = format!("{}.png", id);
        let mut file = File::create(&id).expect("could not create file");

        board.0.write_to_png(&mut file)?
        */
        None
    }
}
