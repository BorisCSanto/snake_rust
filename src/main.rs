#![windows_subsystem = "windows"]
extern crate piston_window;
extern crate rand;

mod color;
mod draw;
mod game;
mod menu;
mod snake;
mod wall;

use piston_window::*;

use crate::color::BACK_COLOR;
use crate::draw::to_coord;
use crate::game::Game;
use crate::menu::Bouton;
use crate::menu::BoutonType;
use crate::menu::Menu;
use crate::menu::Niveau;
use crate::menu::Taille;
use crate::menu::Vitesse;

fn main() {
    let bouton_largeur: Bouton = Bouton::new("Largeur".to_owned(), BoutonType::Largeur, false);
    let bouton_hauteur: Bouton = Bouton::new("Hauteur".to_owned(), BoutonType::Hauteur, false);
    let bouton_vitesse: Bouton = Bouton::new("Vitesse".to_owned(), BoutonType::Vitesse, false);
    let bouton_niveau: Bouton = Bouton::new("Niveau".to_owned(), BoutonType::Niveau, false);
    let bouton_depart: Bouton = Bouton::new("Commencer".to_owned(), BoutonType::Depart, true);

    let (width, height) = (50, 50);

    let mut menu: Menu = Menu::new(
        width,
        height,
        Taille::Moyen,
        Taille::Moyen,
        Vitesse::Normal,
        Niveau::Debutant,
        [
            bouton_largeur,
            bouton_hauteur,
            bouton_vitesse,
            bouton_niveau,
            bouton_depart,
        ],
        0,
        false,
    );

    let mut window: PistonWindow =
        WindowSettings::new("Jeu du serpent", [to_coord(width), to_coord(height)])
            .resizable(true)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut glyphs = window.load_font("assets/Coolvetica.ttf").unwrap();

    let mut game = Game::new(10, 10, 0.0, 1);
    let mut switcher = false;
    while let Some(event) = window.next() {
        match menu.to_continue() {
            true => {
                if !switcher {
                    switcher = true;
                    window.set_size([to_coord(menu.get_width()), to_coord(menu.get_height())]);
                    game = Game::new(
                        menu.get_width(),
                        menu.get_height(),
                        menu.get_vitesse(),
                        menu.get_niveau(),
                    );
                }
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    game.key_pressed(key);
                    if game.return_to_menu() {
                        switcher = false;
                        menu.return_to_menu();
                    }
                }
                window.draw_2d(&event, |c, g, device| {
                    clear(BACK_COLOR, g);
                    game.draw(&c, g, &mut glyphs);
                    glyphs.factory.encoder.flush(device);
                });

                event.update(|arg| {
                    game.update(arg.dt);
                });
            }
            false => {
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    menu.key_pressed(key);
                }
                window.set_size([to_coord(width), to_coord(height)]);
                window.draw_2d(&event, |c, g, device| {
                    clear(BACK_COLOR, g);
                    menu.draw(&c, g, &mut glyphs);
                    glyphs.factory.encoder.flush(device)
                });
            }
        }
    }
}
