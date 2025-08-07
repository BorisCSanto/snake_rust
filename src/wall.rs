use piston_window::{Context, G2d};
use rand::{Rng, rng};
use std::collections::LinkedList;

use crate::{color::WALL_COLOR, draw::draw_block};

#[derive(Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Wall {
    mur: LinkedList<Block>,
    grille: Vec<Vec<i32>>,
}

impl Wall {
    pub fn new(l: i32, w: i32, h: i32) -> Wall {
        let mut blocks: LinkedList<Block> = LinkedList::new();

        match l {
            2 | 3 => {
                let mut height = 5;

                for _ in 1..l {
                    let taille = height * 2;
                    // lignes du haut
                    let mut construct = true;
                    for n in 1..w {
                        if n % (5 + height) == 0 && n != 1 && taille < w - n {
                            if construct == true {
                                construct = false;
                                for i in 0..taille {
                                    blocks.push_back(Block {
                                        x: n + i,
                                        y: height,
                                    });
                                }
                            } else {
                                construct = true
                            }
                        }
                    }

                    // lignes du bas
                    construct = true;
                    for n in 1..w {
                        if n % (5 + height) == 0 && n != 1 && taille < w - n {
                            if construct == true {
                                construct = false;
                                for i in 0..taille {
                                    blocks.push_back(Block {
                                        x: n + i,
                                        y: h - height - 1,
                                    });
                                }
                            } else {
                                construct = true
                            }
                        }
                    }

                    //lignes de gauche
                    construct = true;
                    for n in 1..h {
                        if n % (5 + height) == 0 && n != 1 && taille < h - n {
                            if construct == true {
                                construct = false;
                                for i in 0..taille {
                                    blocks.push_back(Block {
                                        x: height,
                                        y: n + i,
                                    });
                                }
                            } else {
                                construct = true
                            }
                        }
                    }

                    //lignes de droite
                    construct = true;
                    for n in 1..h {
                        if n % (5 + height) == 0 && n != 1 && taille < h - n {
                            if construct == true {
                                construct = false;
                                for i in 0..taille {
                                    blocks.push_back(Block {
                                        x: w - height,
                                        y: n + i,
                                    });
                                }
                            } else {
                                construct = true
                            }
                        }
                    }

                    height += 5;
                }
            }
            4 => {
                let mut decal = 5;
                let mut random = rng();
                while decal < w / 2 && decal < h / 2 {
                    for n in decal..(w - decal) {
                        if random.random_range(1..10) != 5 {
                            blocks.push_back(Block { x: n, y: decal });
                        }
                    }
                    for m in (decal)..(h - decal) {
                        if random.random_range(1..10) != 5 {
                            blocks.push_back(Block { x: w - decal, y: m });
                        }
                    }
                    for n in decal..(w - decal) {
                        if random.random_range(1..10) != 5 {
                            blocks.push_back(Block {
                                x: n,
                                y: h - decal - 1,
                            });
                        }
                    }
                    for m in (decal + 5)..(h - decal) {
                        if random.random_range(1..10) != 5 {
                            blocks.push_back(Block { x: decal, y: m });
                        }
                    }
                    decal += 5;
                }
            }
            _ => {}
        }

        let larg = w as usize;
        let hei = h as usize;
        let grille = vec![vec![0; hei]; larg];
        Wall {
            mur: blocks,
            grille: grille,
        }
    }

    pub fn make_grille(&mut self) {
        for block in &self.mur {
            self.grille[block.x as usize][block.y as usize] = 1;
        }
    }

    pub fn draw(&mut self, context: &Context, g: &mut G2d) {
        for block in &self.mur {
            draw_block(WALL_COLOR, block.x, block.y, context, g);
        }
    }

    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        self.grille[x as usize][y as usize] == 1
    }
}
